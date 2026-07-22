#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::{Edge, ExtiPin, GpioExt, Output, PC13, PushPull, gpioa::PA5},
    pac::{self, interrupt},
    rcc::{Config, Enable, Rcc, RccExt},
    syscfg::SysCfgExt,
};

// Onboard LED is attached to PA5
// User Button is attached to PC13

const TIM2_PSC_VAL: u16 = 15_999; // 16 MHz / (15_999 + 1) = 1 kHz → 1 ms per tick
const DEBOUNCE_PERIOD_TICKS: u32 = 29; // (29 + 1) ticks × 1 ms = 30 ms

struct SharedPeripherals {
    led: PA5<Output<PushPull>>,
    tim2: pac::TIM2,
    button: PC13,
}

static SHARED_PERIPHERALS: Mutex<RefCell<Option<SharedPeripherals>>> =
    Mutex::new(RefCell::new(None));

fn setup_led(gpioa: pac::GPIOA, rcc: &mut Rcc) -> PA5<Output<PushPull>> {
    let gpioa = gpioa.split(rcc);
    gpioa.pa5.into_push_pull_output()
}

fn setup_button(
    gpioc: pac::GPIOC,
    rcc: &mut Rcc,
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

/// Configures TIM2 as a one-pulse-mode debounce timer.
/// One-pulse mode isn't exposed by the `Counter` HAL abstraction,
/// so this configures the peripheral directly via PAC registers.
fn configure_debounce_timer(rcc: &mut Rcc, tim2: &pac::TIM2) {
    pac::TIM2::enable(rcc);
    tim2.psc().write(|w| w.psc().set(TIM2_PSC_VAL));
    tim2.arr().write(|w| w.arr().set(DEBOUNCE_PERIOD_TICKS));
    tim2.cr1().modify(|_, w| w.opm().set_bit());

    // Force update event to load registers into active shadow registers
    tim2.egr().write(|w| w.ug().set_bit());
}

#[interrupt]
fn EXTI15_10() {
    critical_section::with(|cs| {
        let mut shared = SHARED_PERIPHERALS.borrow(cs).borrow_mut();
        let Some(shared) = shared.as_mut() else { return };

        if !shared.button.check_interrupt() {
            return
        }
        shared.button.clear_interrupt_pending_bit();

        // Only change LED state if debounce timer is not active
        if shared.tim2.cr1().read().cen().bit_is_clear() {
            // User Button pulls PC13 low when pressed.
            let button_is_pressed = shared.button.is_low();

            shared.led.set_state(button_is_pressed.into());
            shared.tim2.cr1().modify(|_, w| w.cen().set_bit());
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
    configure_debounce_timer(&mut rcc, &peripherals.TIM2);

    critical_section::with(|cs| {
        *SHARED_PERIPHERALS.borrow(cs).borrow_mut() = Some(SharedPeripherals {
            led,
            tim2: peripherals.TIM2,
            button,
        });
    });

    pac::NVIC::unpend(pac::Interrupt::EXTI15_10);
    unsafe { pac::NVIC::unmask(pac::Interrupt::EXTI15_10) };

    loop {
        cortex_m::asm::wfi();
    }
}
