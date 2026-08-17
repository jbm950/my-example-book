#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    gpio::GpioExt,
    hal_02::Qei,
    pac,
    qei::QeiExt,
    rcc::{self, RccExt},
};

// Quadrature Generator (TIM2):
// TIM2 CH1 is PA5, AF1 (Signal A)
// TIM2 CH2 is PA1, AF1 (Signal B)
//
// Quadrature Decoder (TIM1):
// TIM1 CH1 is PA8 (Signal A)
// TIM1 CH2 is PA9 (Signal B)

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

fn configure_tim2_as_generator(rcc: &mut rcc::Rcc, tim2: &pac::TIM2) {
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

    // CCR1 = 0 makes Signal A toggle at the period boundary. CCR2 is offset
    // by 500 ticks, producing the 90° phase relationship with Signal B.
    tim2.ccr1().write(|w| w.ccr().set(0));
    tim2.ccr2().write(|w| w.ccr().set(TIM2_QUAD_OFFSET)); // Signal B lags 90 degrees
    tim2.egr().write(|w| w.ug().set_bit()); // force new CCR values into active regs immediately

    tim2.ccer()
        .modify(|_, w| w.cc1e().set_bit().cc2e().set_bit());

    // Start counter
    tim2.cr1().modify(|_, w| w.cen().set_bit());
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi());
    let gpioa = dp.GPIOA.split(&mut rcc);

    // AF number must be explicit here — nothing downstream infers it,
    // unlike sig_a_dec/sig_b_dec below, which get their AF pinned by qei()'s
    // trait bound.
    //
    // Configuring AF mode is done as a side effect of into_alternate(); the
    // resulting pin handle isn't needed afterward since nothing else reads
    // or reconfigures these pins.
    let _sig_a_gen = gpioa.pa5.into_alternate::<1>();
    let _sig_b_gen = gpioa.pa1.into_alternate::<1>();
    let sig_a_dec = gpioa.pa8.into_alternate();
    let sig_b_dec = gpioa.pa9.into_alternate();

    configure_tim2_as_generator(&mut rcc, &dp.TIM2);

    let decoder = dp.TIM1.qei((sig_a_dec, sig_b_dec), &mut rcc);

    let mut last = decoder.count();
    loop {
        let value = decoder.count();
        if value != last {
            rprintln!("Count: {}, Direction: {:?}", value, decoder.direction());
            last = value;
        }
    }
}
