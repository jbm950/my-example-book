#![no_main]
#![no_std]

use cortex_m_rt::entry;
use fugit::RateExtU32;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::GpioExt,
    pac,
    rcc::{self, RccExt},
    timer::PwmExt,
};
use tb6612fng::{DriveCommand, Motor};

// Pins to use with Motor Driver:
// PA5 (D13) PWM
// PA6 (D12) A IN 2
// PA7 (D11) A IN 3
// PB6 (D10) Standby
//
// Note, pin choices were made arbitrarily other than PWM was chosen to match
// up with my PWM example.

const DUTY_PERCENT: u8 = 50;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi());
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    let pwm_pin = gpioa.pa5.into_alternate();
    let (_, (ch1, ..)) = dp.TIM2.pwm_hz(1.kHz(), &mut rcc);
    let mut pwm = ch1.with(pwm_pin);
    pwm.enable();

    let a_in_1 = gpioa.pa6.into_push_pull_output();
    let a_in_2 = gpioa.pa7.into_push_pull_output();

    // Library doesn't seem to account for standby pin
    let mut standby = gpiob.pb6.into_push_pull_output();
    standby.set_high();

    let mut motor = Motor::new(a_in_1, a_in_2, pwm).unwrap();
    motor.drive(DriveCommand::Forward(DUTY_PERCENT)).unwrap();

    loop {
        cortex_m::asm::wfi();
    }
}
