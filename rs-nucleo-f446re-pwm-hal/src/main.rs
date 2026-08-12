#![no_main]
#![no_std]

use cortex_m_rt::entry;
use fugit::RateExtU32;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::GpioExt, hal::pwm::SetDutyCycle, pac, rcc::{self, RccExt}, timer::PwmExt
};

// Onboard LED is attached to PA5

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().expect("peripherals already taken");

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi());
    let gpioa = dp.GPIOA.split(&mut rcc);

    let led_pin = gpioa.pa5.into_alternate();

    let (_, (ch1, ..)) = dp.TIM2.pwm_hz(1.kHz(), &mut rcc);
    let mut led_pwm = ch1.with(led_pin);
    led_pwm.set_duty_cycle_percent(50).unwrap();
    led_pwm.enable();

    loop {
        cortex_m::asm::wfi();
    }
}
