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
    gpio::GpioExt,
    hal::{delay::DelayNs, pwm::SetDutyCycle},
    pac::{self, Interrupt, NVIC, interrupt},
    rcc::{self, RccExt},
    timer::{CaptureChannel, CaptureExt, CaptureFilter, Event, PwmExt, TimerExt},
};

// Pins to use with Motor Driver:
// PA5 (D13) PWM
// PA6 (D12) A IN 2
// PA7 (D11) A IN 3
// PB6 (D10) Standby
//
// Pins for Timer Capture:
// TIM3 CH1 is PB4
//
// Note, pin choices were made arbitrarily other than PWM was chosen to match
// up with my PWM example.

const TIMER_FREQ_HZ: u32 = 1_000_000;
const PULSES_PER_REV: u32 = 12;
const GEAR_RATIO: u32 = 90;

const TOTAL_SAMPLES: usize = 10;

struct SharedResources {
    capture: CaptureChannel<pac::TIM3, 0>,
    prev_count: Option<u32>,
    sample_index: usize,
    samples: [u32; TOTAL_SAMPLES],
}

static SHARED_RESOURCES: Mutex<RefCell<Option<SharedResources>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn TIM3() {
    // Example doesn't address overcapture.

    critical_section::with(|cs| {
        let mut shared = SHARED_RESOURCES.borrow(cs).borrow_mut();
        let Some(shared) = shared.as_mut() else {
            return;
        };

        // The first pass primes the previous count value
        let Some(prev_count) = shared.prev_count else {
            shared.prev_count = Some(shared.capture.get_capture());
            return;
        };

        if shared.sample_index != TOTAL_SAMPLES {
            let captured = shared.capture.get_capture();
            let delta = (captured as u16).wrapping_sub(prev_count as u16) as u32;
            let rpm = (TIMER_FREQ_HZ * 60) / (delta * PULSES_PER_REV * GEAR_RATIO);

            shared.samples[shared.sample_index] = rpm;
            shared.sample_index += 1;
            shared.prev_count = Some(captured);
        } else {
            let avg_rpm = shared.samples.iter().sum::<u32>() as f64 / shared.samples.len() as f64;
            rprintln!("Samples: {:?}, Avg: {}", shared.samples, avg_rpm);
            shared.capture.disable();

            NVIC::pend(Interrupt::TIM3);
            NVIC::mask(Interrupt::TIM3);
        }
    });
}

fn start_capture() {
    critical_section::with(|cs| {
        let mut shared = SHARED_RESOURCES.borrow(cs).borrow_mut();
        let Some(shared) = shared.as_mut() else {
            return;
        };

        shared.capture.enable();
    });

    NVIC::unpend(Interrupt::TIM3);
    // SAFETY: SHARED_RESOURCES is populated and the stale pending bit is
    // cleared above, before the interrupt is unmasked.
    unsafe {
        NVIC::unmask(Interrupt::TIM3);
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi());
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    let pwm_pin = gpioa.pa5.into_alternate();
    let (_, (pwm_ch1, ..)) = dp.TIM2.pwm_hz(1.kHz(), &mut rcc);
    let mut pwm = pwm_ch1.with(pwm_pin);
    pwm.set_duty_cycle_percent(10).unwrap();

    let mut a_in_1 = gpioa.pa6.into_push_pull_output();
    let mut a_in_2 = gpioa.pa7.into_push_pull_output();
    let mut standby = gpiob.pb6.into_push_pull_output();

    let capture_pin = gpiob.pb4.into_alternate().internal_pull_up(true);
    let (mut capture_manager, (capture_ch1, ..)) = dp.TIM3.capture_hz(1.MHz(), &mut rcc);
    let mut capture = capture_ch1.with(capture_pin);
    capture.set_filter(CaptureFilter::FckIntN8);
    capture_manager.listen(Event::C1);

    // Turn on motor
    pwm.enable();
    a_in_1.set_high();
    a_in_2.set_low();
    standby.set_high();

    critical_section::with(|cs| {
        *SHARED_RESOURCES.borrow(cs).borrow_mut() = Some(SharedResources {
            capture,
            prev_count: None,
            sample_index: 0,
            samples: [0u32; TOTAL_SAMPLES],
        });
    });

    let mut delay = dp.TIM6.delay_ms(&mut rcc);
    delay.delay_ms(1000); // Let the motor settle for 1 second before taking measurements

    start_capture();

    loop {
        cortex_m::asm::nop();
    }
}
