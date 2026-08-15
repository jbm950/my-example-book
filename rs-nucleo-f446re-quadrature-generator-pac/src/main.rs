#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4::stm32f446 as pac;

// TIM2 CH1 is PA5, AF1 (Signal A)
// TIM2 CH2 is PA1, AF1 (Signal B)

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

fn configure_quadrature_pins(rcc: &pac::RCC, gpioa: &pac::GPIOA) {
    rcc.ahb1enr().modify(|_, w| w.gpioaen().set_bit());
    gpioa.moder().modify(|_, w| {
        w.moder5()
            .alternate() // Signal A
            .moder1()
            .alternate() // Signal B
    });
    gpioa.afrl().modify(|_, w| {
        w.afrl5()
            .af1() // Signal A
            .afrl1()
            .af1() // Signal B
    });
}

fn configure_tim2(rcc: &pac::RCC, tim2: &pac::TIM2) {
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

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    configure_quadrature_pins(&peripherals.RCC, &peripherals.GPIOA);
    configure_tim2(&peripherals.RCC, &peripherals.TIM2);

    loop {
        cortex_m::asm::wfi();
    }
}
