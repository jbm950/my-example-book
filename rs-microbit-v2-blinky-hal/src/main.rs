#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::StatefulOutputPin};
use nrf52833_hal::{gpio, pac, timer};
use panic_halt as _;

const BLINK_PERIOD_MS: u32 = 500;

// Center LED of the 5×5 matrix.
// LED is driven by:
//   Column 3 -> P0.31 (active low)
//   Row 3    -> P0.15 (active high)

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let pins = gpio::p0::Parts::new(peripherals.P0);
    let mut timer0 = timer::Timer::new(peripherals.TIMER0);

    let _col_3 = pins.p0_31.into_push_pull_output(gpio::Level::Low);
    let mut row_3 = pins.p0_15.into_push_pull_output(gpio::Level::High);

    loop {
        row_3.toggle().unwrap();
        timer0.delay_ms(BLINK_PERIOD_MS);
    }
}
