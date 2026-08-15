#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use fugit::RateExtU32;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    Listen,
    gpio::{GpioExt, Pin},
    hal::pwm::SetDutyCycle,
    pac::{self, Interrupt, NVIC, interrupt},
    rcc::{self, Rcc, RccExt},
    timer::{CaptureChannel, CaptureExt, Event, PwmExt},
};

// TIM2 CH1 is PA5, Set for PWM
// TIM3 CH1 is PA6, Set for Timer Capture
//
// Intended setup: jumper PA5 -> PA6 to loop TIM2's PWM output into TIM3's
// capture input. PA6 is driven by PA5 through the jumper, so no pull is needed.

type Tim3Capture = CaptureChannel<pac::TIM3, 0>;

static TIM3_CAPTURE: Mutex<RefCell<Option<Tim3Capture>>> =
    Mutex::new(RefCell::new(None));

fn setup_pwm(pa5: Pin<'A', 5>, tim2: pac::TIM2, rcc: &mut Rcc) {
    let pwm_pin = pa5.into_alternate();
    let (_, (pwm_ch1, ..)) = tim2.pwm_hz(1.kHz(), rcc);
    let mut pwm = pwm_ch1.with(pwm_pin);
    pwm.set_duty_cycle_percent(50).unwrap();
    pwm.enable();
}

fn setup_capture(pa6: Pin<'A', 6>, tim3: pac::TIM3, rcc: &mut Rcc) -> Tim3Capture {
    let capture_pin = pa6.into_alternate();
    let (mut capture_manager, (capture_ch1, ..)) = tim3.capture_hz(1.MHz(), rcc);
    let mut capture = capture_ch1.with(capture_pin);
    capture.enable();
    capture_manager.listen(Event::C1);

    capture
}

#[interrupt]
fn TIM3() {
    // Example doesn't address overcapture.

    critical_section::with(|cs| {
        let mut capture = TIM3_CAPTURE.borrow(cs).borrow_mut();
        let Some(capture) = capture.as_mut() else {
            return;
        };

        rprintln!("Capture value: {}", capture.get_capture());
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().expect("peripherals already taken");

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi());
    let gpioa = dp.GPIOA.split(&mut rcc);

    setup_pwm(gpioa.pa5, dp.TIM2, &mut rcc);
    let capture = setup_capture(gpioa.pa6, dp.TIM3, &mut rcc);

    critical_section::with(|cs| {
        *TIM3_CAPTURE.borrow(cs).borrow_mut() = Some(capture);
    });

    NVIC::unpend(Interrupt::TIM3);
    // SAFETY: TIM3_CAPTURE is populated and the stale pending bit is cleared
    // above, before the interrupt is unmasked.
    unsafe {
        NVIC::unmask(Interrupt::TIM3);
    }

    loop {
        // WFI stops the RTT connection
        cortex_m::asm::nop();
    }
}
