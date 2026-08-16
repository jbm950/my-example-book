#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};
use stm32f4::stm32f446 as pac;

// Quadrature Generator (TIM2):
// TIM2 CH1 is PA5, AF1 (Signal A)
// TIM2 CH2 is PA1, AF1 (Signal B)
//
// Quadrature Decoder (TIM1):
// TIM1 CH1 is PA8, AF1 (Signal A)
// TIM1 CH2 is PA9, AF1 (Signal B)

const TIM2_PSC: u16 = 15; // 16 MHz / (15 + 1) = 1 MHz
// TIM2CLK = 16 MHz (HSI, no PLL, APB1 prescaler = /1, so no x2 timer-clock multiplier)
// After the prescaler: 16 MHz / (15 + 1) = 1 MHz timer tick (1 tick = 1 us)
//
// Target: 1000 tick period (chosen to make quadrature offset simple)
//   ARR + 1 = tick_rate
//   ARR     = 999
//
// Timer period = 1000 us.
// Toggle mode changes the output once per timer period, so each output
// has a 2000 us (500 Hz) period.
// A 90° phase shift is therefore 500 us = 500 timer ticks.
const TIM2_ARR: u32 = 999;
const _: () = assert!(
    (TIM2_ARR + 1) % 2 == 0,
    "ARR + 1 must be even for exact 90° quadrature offset"
);
const TIM2_QUAD_OFFSET: u32 = (TIM2_ARR + 1) / 2; // 90° = quarter of the 2×(ARR + 1) output period


const TIM1_ARR: u16 = u16::MAX; // Max ARR

fn configure_quadrature_pins(rcc: &pac::RCC, gpioa: &pac::GPIOA) {
    rcc.ahb1enr().modify(|_, w| w.gpioaen().set_bit());

    #[rustfmt::skip]
    gpioa.moder().modify(|_, w| {
        w.moder5().alternate() // Generator Signal A
         .moder1().alternate() // Generator Signal B
         .moder8().alternate() // Encoder Signal A
         .moder9().alternate() // Encoder Signal B
    });

    #[rustfmt::skip]
    gpioa.afrl().modify(|_, w| {
        w.afrl5().af1() // Generator Signal A
         .afrl1().af1() // Generator Signal B
    });

    #[rustfmt::skip]
    gpioa.afrh().modify(|_, w| {
        w.afrh8().af1() // Encoder Signal A
         .afrh9().af1() // Encoder Signal B
    });
}

fn configure_tim2_as_generator(rcc: &pac::RCC, tim2: &pac::TIM2) {
    rcc.apb1enr().modify(|_, w| w.tim2en().set_bit());

    tim2.psc().write(|w| w.psc().set(TIM2_PSC));
    tim2.arr().write(|w| w.arr().set(TIM2_ARR));
    tim2.cr1().modify(|_, w| w.arpe().set_bit());

    // Force both outputs to the same initial level (high) before
    // switching to toggle mode. Toggle mode flips the output relative to
    // whatever level it's currently holding, so *which* level you force
    // here — not just that both start from the same one — determines whether
    // Signal B leads or lags Signal A once CEN is set. Empirically:
    // force_active() on both yields B lagging A by 90°; force_inactive() on
    // both flips the relationship to a 90° lead instead.
    #[rustfmt::skip]
    tim2.ccmr1_output().modify(|_, w| {
        w.oc1m().force_active() // Signal A
         // Signal B — see note above: this ordering gives a 90° lag, not lead
         .oc2m().force_active()
    });

    // Force update event to load registers into active shadow registers
    tim2.egr().write(|w| w.ug().set_bit());

    #[rustfmt::skip]
    tim2.ccmr1_output().modify(|_, w| {
        w.oc1m().toggle() // Signal A
         .oc1pe().set_bit() // Enable CCR1 preload
         .oc2m().toggle() // Signal B
         .oc2pe().set_bit() // Enable CCR2 preload
    });

    // CCR1 = 0: Signal A's toggle event coincides with counter reset (CNT
    // wraps to 0), i.e. it toggles at the very start of every period
    tim2.ccr1().write(|w| w.ccr().set(0));
    tim2.ccr2().write(|w| w.ccr().set(TIM2_QUAD_OFFSET)); // Signal B lags 90 degrees
    tim2.egr().write(|w| w.ug().set_bit()); // force new CCR values into active regs immediately

    tim2.ccer()
        .modify(|_, w| w.cc1e().set_bit().cc2e().set_bit());

    // Start counter
    tim2.cr1().modify(|_, w| w.cen().set_bit());
}

fn configure_tim1_as_encoder(rcc: &pac::RCC, tim1: &pac::TIM1) {
    rcc.apb2enr().modify(|_, w| w.tim1en().set_bit());

    // Use the full 16-bit counter range so the encoder can accumulate position
    // without wrapping at a smaller application-defined limit.
    tim1.arr().write(|w| w.arr().set(TIM1_ARR));

    #[rustfmt::skip]
    tim1.ccmr1_input().modify(|_, w| {
        w.cc1s().ti1()
         .cc2s().ti2()
    });

    #[rustfmt::skip]
    tim1.ccer().modify(|_, w| {
        w.cc1e().enabled()
         .cc2e().enabled()
    });

    // Count on both edges of both encoder inputs for 4× quadrature resolution.
    tim1.smcr().modify(|_, w| w.sms().encoder_mode_3());

    tim1.cr1().modify(|_, w| w.cen().enabled());
}

fn get_count(tim1: &pac::TIM1) -> u16 {
    tim1.cnt().read().cnt().bits()
}

fn get_direction(tim1: &pac::TIM1) -> &str {
    if tim1.cr1().read().dir().bit_is_set() { "down" } else { "up" }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    configure_quadrature_pins(&dp.RCC, &dp.GPIOA);
    configure_tim2_as_generator(&dp.RCC, &dp.TIM2);
    configure_tim1_as_encoder(&dp.RCC, &dp.TIM1);

    let mut last = get_count(&dp.TIM1);
    loop {
        let value = get_count(&dp.TIM1);
        if value != last {
            rprintln!("Count: {}, Direction: {}", value, get_direction(&dp.TIM1));
            last = value;
        }
    }
}
