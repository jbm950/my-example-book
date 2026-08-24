#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::GpioExt,
    pac,
    rcc::{self, RccExt},
};

// Pins to use with Motor Driver:
// PA5 (D13) PWM
// PA6 (D12) A IN 2
// PA7 (D11) A IN 3
// PB6 (D10) Standby
//
// Note, pin choices were made arbitrarily other than PWM was chosen to match
// up with my PWM example.

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi());
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    let mut pwm = gpioa.pa5.into_push_pull_output();
    let mut a_in_1 = gpioa.pa6.into_push_pull_output();
    let mut a_in_2 = gpioa.pa7.into_push_pull_output();
    let mut standby = gpiob.pb6.into_push_pull_output();

    pwm.set_high();
    a_in_1.set_high();
    a_in_2.set_low();
    standby.set_high();

    loop {
        cortex_m::asm::wfi();
    }
}
