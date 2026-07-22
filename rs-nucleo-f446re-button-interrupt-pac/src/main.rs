#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use panic_halt as _;
use stm32f4::stm32f446 as pac;
use pac::interrupt;

// Onboard LED is attached to PA5
// User Button is attached to PC13

struct SharedPeripherals {
    gpioa: pac::GPIOA,
    gpioc: pac::GPIOC,
    exti: pac::EXTI,
}

static SHARED_PERIPHERALS: Mutex<RefCell<Option<SharedPeripherals>>> =
    Mutex::new(RefCell::new(None));

fn configure_led(rcc: &pac::RCC, gpioa: &pac::GPIOA) {
    rcc.ahb1enr().modify(|_, w| w.gpioaen().set_bit());
    gpioa.moder().modify(|_, w| w.moder5().output());
}

fn configure_button(rcc: &pac::RCC, gpioc: &pac::GPIOC) {
    // Set up button
    rcc.ahb1enr().modify(|_, w| w.gpiocen().set_bit());
    gpioc.moder().modify(|_, w| w.moder13().input());
    gpioc.pupdr().modify(|_, w| w.pupdr13().floating());
}

fn configure_interrupt(rcc: &pac::RCC, syscfg: &pac::SYSCFG, exti: &pac::EXTI) {
    // Set EXTI channel 13 to watch GPIOC (Button Pin = PC13)
    rcc.apb2enr().modify(|_, w| w.syscfgen().set_bit());
    syscfg.exticr4().modify(|_, w| w.exti13().pc());

    // Turn on EXTI channel 13
    exti.imr().modify(|_, w| w.mr13().set_bit());

    // Enable rising and falling edge triggers for toggle
    exti.ftsr().modify(|_, w| w.tr13().set_bit());
    exti.rtsr().modify(|_, w| w.tr13().set_bit());
}

#[interrupt]
fn EXTI15_10() {
    critical_section::with(|cs| {
        let mut shared = SHARED_PERIPHERALS.borrow(cs).borrow_mut();

        if let Some(shared) = shared.as_mut() {
            if shared.exti.pr().read().pr13().bit_is_set() {
                shared.exti.pr().write(|w| w.pr13().clear_bit_by_one());

                // User Button pulls PC13 low when pressed.
                let button_is_pressed = shared.gpioc.idr().read().idr13().is_low();

                shared.gpioa.odr().modify(|_, w| w.odr5().bit(button_is_pressed));
            }
        }
    });
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    configure_led(&peripherals.RCC, &peripherals.GPIOA);
    configure_button(&peripherals.RCC, &peripherals.GPIOC);
    configure_interrupt(&peripherals.RCC, &peripherals.SYSCFG, &peripherals.EXTI);

    critical_section::with(|cs| {
        *SHARED_PERIPHERALS.borrow(cs).borrow_mut() = Some(SharedPeripherals {
            gpioa: peripherals.GPIOA,
            gpioc: peripherals.GPIOC,
            exti: peripherals.EXTI,
        });
    });

    pac::NVIC::unpend(pac::Interrupt::EXTI15_10);
    unsafe { pac::NVIC::unmask(pac::Interrupt::EXTI15_10) };

    loop {
        cortex_m::asm::wfi();
    }
}
