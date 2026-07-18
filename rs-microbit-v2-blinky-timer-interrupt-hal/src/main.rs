#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use embedded_hal::digital::StatefulOutputPin;
use nrf52833_hal::{gpio, pac::{self, interrupt}, timer};
use panic_halt as _;

const BLINK_PERIOD_MS: u32 = 500;
const TICKS_PER_MS: u32 = timer::Timer::<pac::TIMER0>::TICKS_PER_SECOND / 1_000;


// Center LED of the 5×5 matrix.
// LED is driven by:
//   Column 3 -> P0.31 (active low)
//   Row 3    -> P0.15 (active high)

struct SharedPeripherals {
    row_3_pin: gpio::Pin<gpio::Output<gpio::PushPull>>,
    timer0: timer::Timer<pac::TIMER0, timer::Periodic>,
}

static SHARED_PERIPHERALS: Mutex<RefCell<Option<SharedPeripherals>>> =
    Mutex::new(RefCell::new(None));

#[interrupt]
fn TIMER0() {
    critical_section::with(|cs| {
        let mut shared = SHARED_PERIPHERALS.borrow(cs).borrow_mut();
        let shared = shared.as_mut().unwrap();
        shared.timer0.reset_event();
        shared.row_3_pin.toggle().unwrap();
    });
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let pins = gpio::p0::Parts::new(peripherals.P0);
    let mut timer0 = timer::Timer::new(peripherals.TIMER0).into_periodic();
    timer0.enable_interrupt();
    timer0.start(BLINK_PERIOD_MS * TICKS_PER_MS);

    let _col_3 = pins.p0_31.into_push_pull_output(gpio::Level::Low);
    let row_3 = pins.p0_15.into_push_pull_output(gpio::Level::High).degrade();

    critical_section::with(|cs| {
        *SHARED_PERIPHERALS.borrow(cs).borrow_mut() = Some(SharedPeripherals {
            row_3_pin: row_3,
            timer0
        });
    });

    pac::NVIC::unpend(pac::Interrupt::TIMER0);
    unsafe { pac::NVIC::unmask(pac::Interrupt::TIMER0) };

    loop {
        cortex_m::asm::wfi();
    }
}
