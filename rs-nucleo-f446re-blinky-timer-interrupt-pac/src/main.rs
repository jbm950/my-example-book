#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use panic_halt as _;
use stm32f4::stm32f446::{self as pac, NVIC};
use pac::{interrupt, Interrupt};

// Onboard LED is attached to PA5

const TIM2_PSC_VAL: u16 = 15_999; // 16 MHz / (15_999 + 1) = 1 kHz → 1 ms per tick
const TIMER_PERIOD_TICKS: u32 = 999; // (999 + 1) ticks × 1 ms = 1000 ms = 1 s

struct SharedPeripherals {
    gpioa: pac::GPIOA,
    tim2: pac::TIM2,
}

static SHARED_PERIPHERALS: Mutex<RefCell<Option<SharedPeripherals>>> =
    Mutex::new(RefCell::new(None));

fn configure_led(rcc: &pac::RCC, gpioa: &pac::GPIOA) {
    rcc.ahb1enr().modify(|_, w| w.gpioaen().set_bit());
    gpioa.moder().modify(|_, w| w.moder5().output());
}

fn init_timer(rcc: &pac::RCC, tim2: &pac::TIM2) {
    rcc.apb1enr().modify(|_, w| w.tim2en().set_bit());
    tim2.dier().modify(|_, w| w.uie().set_bit());
    tim2.psc().write(|w| w.psc().set(TIM2_PSC_VAL));
    tim2.arr().write(|w| w.arr().set(TIMER_PERIOD_TICKS));

    // Force update event to load registers into active shadow registers
    tim2.egr().write(|w| w.ug().set_bit());

    // Start counter
    tim2.cr1().modify(|_, w| w.cen().set_bit());

    // Clear the flag that UG generated so the main loop doesn't instantly
    // jump past the first delay
    tim2.sr().modify(|_, w| w.uif().clear());
}

#[interrupt]
fn TIM2() {
    critical_section::with(|cs| {
        let mut shared = SHARED_PERIPHERALS.borrow(cs).borrow_mut();

        if let Some(shared) = shared.as_mut() {
            // Clear the event
            shared.tim2.sr().modify(|_, w| w.uif().clear());

            // Toggle the LED state
            shared.gpioa.odr().modify(|r, w| w.odr5().bit(!r.odr5().bit_is_set()));
        }
    });
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    configure_led(&peripherals.RCC, &peripherals.GPIOA);
    init_timer(&peripherals.RCC, &peripherals.TIM2);

    critical_section::with(|cs| {
        *SHARED_PERIPHERALS.borrow(cs).borrow_mut() = Some(SharedPeripherals {
            gpioa: peripherals.GPIOA,
            tim2: peripherals.TIM2,
        });
    });

    NVIC::unpend(Interrupt::TIM2);
    unsafe {
        NVIC::unmask(Interrupt::TIM2);
    }

    loop {
        cortex_m::asm::wfi();
    }
}
