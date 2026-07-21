#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::GpioExt,
    hal::delay::DelayNs,
    pac,
    rcc::{Config, RccExt},
    timer::TimerExt,
};

const HALF_PERIOD_MS: u32 = 500;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.freeze(Config::hsi());

    let gpioa = peripherals.GPIOA.split(&mut rcc);
    let mut led = gpioa.pa5.into_push_pull_output();

    let mut delay = peripherals.TIM6.delay_ms(&mut rcc);

    loop {
        led.toggle();
        delay.delay_ms(HALF_PERIOD_MS);
    }
}
