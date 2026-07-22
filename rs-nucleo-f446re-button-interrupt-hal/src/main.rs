#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::{Edge, ExtiPin, GpioExt, Output, PC13, PushPull, gpioa::PA5},
    pac::{self, interrupt},
    rcc::{Config, RccExt},
    syscfg::SysCfgExt,
};

// Onboard LED is attached to PA5
// User Button is attached to PC13

struct SharedPeripherals {
    led: PA5<Output<PushPull>>,
    button: PC13,
}

static SHARED_PERIPHERALS: Mutex<RefCell<Option<SharedPeripherals>>> =
    Mutex::new(RefCell::new(None));

fn setup_led(gpioa: pac::GPIOA, rcc: &mut pac::RCC) -> PA5<Output<PushPull>> {
    let gpioa = gpioa.split(rcc);
    gpioa.pa5.into_push_pull_output()
}

fn setup_button(
    gpioc: pac::GPIOC,
    rcc: &mut pac::RCC,
    syscfg: pac::SYSCFG,
    exti: &mut pac::EXTI,
) -> PC13 {
    let gpioc = gpioc.split(rcc);
    let mut button = gpioc.pc13.into_floating_input();

    let mut syscfg = syscfg.constrain(rcc);
    button.make_interrupt_source(&mut syscfg);
    button.trigger_on_edge(exti, Edge::RisingFalling);
    button.enable_interrupt(exti);

    button
}

#[interrupt]
fn EXTI15_10() {
    critical_section::with(|cs| {
        let mut shared = SHARED_PERIPHERALS.borrow(cs).borrow_mut();

        if let Some(shared) = shared.as_mut() {
            if shared.button.check_interrupt() {
                shared.button.clear_interrupt_pending_bit();

                // User Button pulls PC13 low when pressed.
                let button_is_pressed = shared.button.is_low();

                shared.led.set_state(button_is_pressed.into());
            }
        }
    });
}

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.freeze(Config::hsi());

    let led = setup_led(peripherals.GPIOA, &mut rcc);
    let button = setup_button(
        peripherals.GPIOC,
        &mut rcc,
        peripherals.SYSCFG,
        &mut peripherals.EXTI,
    );

    critical_section::with(|cs| {
        *SHARED_PERIPHERALS.borrow(cs).borrow_mut() = Some(SharedPeripherals { led, button });
    });

    pac::NVIC::unpend(pac::Interrupt::EXTI15_10);
    unsafe { pac::NVIC::unmask(pac::Interrupt::EXTI15_10) };

    loop {
        cortex_m::asm::wfi();
    }
}
