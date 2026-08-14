#![no_main]
#![no_std]

use cortex_m_rt::entry;
use fugit::RateExtU32;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};
use stm32f4xx_hal::{
    gpio::GpioExt, hal::pwm::SetDutyCycle, pac, rcc::{self, RccExt}, timer::{CaptureExt, PwmExt}
};

// TIM2 CH1 is PA5, Set for PWM
// TIM3 CH1 is PA6, Set for Timer Capture
//
// Intended setup: jumper PA5 -> PA6 to loop TIM2's PWM output into TIM3's
// capture input. PA6 is driven by PA5 through the jumper, so no pull is needed.

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().expect("peripherals already taken");

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi());
    let gpioa = dp.GPIOA.split(&mut rcc);

    let pwm_pin = gpioa.pa5.into_alternate();
    let (_, (pwm_ch1, ..)) = dp.TIM2.pwm_hz(1.kHz(), &mut rcc);
    let mut pwm = pwm_ch1.with(pwm_pin);
    pwm.set_duty_cycle_percent(50).unwrap();
    pwm.enable();

    let capture_pin = gpioa.pa6.into_alternate();
    let (_, (capture_ch1, ..)) = dp.TIM3.capture_hz(1.MHz(), &mut rcc);
    let mut capture = capture_ch1.with(capture_pin);
    capture.enable();

    let mut last = capture.get_capture();
    loop {
        let value = capture.get_capture();
        if value != last {
            rprintln!("Capture value: {}", value);
            last = value;
        }
    }
}
