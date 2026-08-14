#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};
use stm32f4::stm32f446::{self as pac, interrupt, Interrupt, NVIC};

// TIM2 CH1 is PA5, AF1 (Set for PWM)
// TIM3 CH1 is PA6, AF2 (Set for Timer Capture)
//
// Intended setup: jumper PA5 -> PA6 to loop TIM2's PWM output into TIM3's
// capture input. PA6 is left floating since it's always actively driven by PA5;
// no pull needed.

const TIM2_PSC: u16 = 15; // 16 MHz / (15 + 1) = 1 MHz
// PWM frequency = TIM2CLK / ((PSC + 1) * (ARR + 1))
//
// TIM2CLK = 16 MHz (HSI, no PLL, APB1 prescaler = /1, so no x2 timer-clock multiplier)
// After the prescaler: 16 MHz / (15 + 1) = 1 MHz timer tick (1 tick = 1 us)
//
// Target: 1 kHz PWM period
//   ARR + 1 = tick_rate / f_pwm = 1_000_000 / 1_000 = 1000
//   ARR     = 999
//
// Bonus: this also gives 1000 discrete duty-cycle steps (CCR1 = 0..=999),
// so CCR1 = 500 is exactly 50%.
const TIM2_ARR: u32 = 999;
const TIM2_DUTY_CYCLE: u32 = (TIM2_ARR + 1) / 2; // 50% Duty

const TIM3_PSC: u16 = 15; // 16 MHz / (15 + 1) = 1 MHz
const TIM3_ARR: u16 = 0xFFFF; // Max ARR

static SHARED_TIM3: Mutex<RefCell<Option<pac::TIM3>>> = Mutex::new(RefCell::new(None));

fn configure_pwm_output(rcc: &pac::RCC, gpioa: &pac::GPIOA, tim2: &pac::TIM2) {
    // Configure pin
    gpioa.moder().modify(|_, w| w.moder5().alternate());
    gpioa.afrl().modify(|_, w| w.afrl5().af1());

    // Configure timer
    rcc.apb1enr().modify(|_, w| w.tim2en().enabled());

    tim2.psc().write(|w| w.psc().set(TIM2_PSC));
    tim2.arr().write(|w| w.arr().set(TIM2_ARR));

    tim2.ccmr1_output().modify(|_, w|  w.oc1m().pwm_mode1());
    tim2.ccer().modify(|_, w| w.cc1e().enabled());

    tim2.ccr1().write(|w| w.ccr().set(TIM2_DUTY_CYCLE));

    // Force update event to apply the prescaler
    tim2.egr().write(|w| w.ug().update());

    // Start counter
    tim2.cr1().modify(|_, w| w.cen().enabled());
}

fn configure_timer_capture(rcc: &pac::RCC, gpioa: &pac::GPIOA, tim3: &pac::TIM3) {
    // Configure pin
    gpioa.moder().modify(|_, w| w.moder6().alternate());
    gpioa.afrl().modify(|_, w| w.afrl6().af2());

    // Configure timer
    rcc.apb1enr().modify(|_, w| w.tim3en().enabled());

    // Enable capture interrupt
    tim3.dier().modify(|_, w| w.cc1ie().enabled());

    tim3.psc().write(|w| w.psc().set(TIM3_PSC));
    tim3.arr().write(|w| w.arr().set(TIM3_ARR));

    // ICF/ICPSC left at reset (no filter, capture prescaler = 1)
    // Every rising edge captured
    tim3.ccmr1_input().modify(|_, w| w.cc1s().ti1());

    #[rustfmt::skip]
    tim3.ccer().modify(|_, w| {
        w.cc1e().enabled()
         // CC1P/CC1NP = 0/0 → capture on rising edge
         .cc1np().clear_bit()
         .cc1p().rising_edge()
    });

    // Force update event to apply the prescaler
    tim3.egr().write(|w| w.ug().update());

    // Start counter
    tim3.cr1().modify(|_, w| w.cen().enabled());
}

#[interrupt]
fn TIM3() {
    // Example doesn't address overcapture.

    critical_section::with(|cs| {
        let mut tim3 = SHARED_TIM3.borrow(cs).borrow_mut();
        let Some(tim3) = tim3.as_mut() else {
            return;
        };

        // Reading CCR1 clears the interrupt flag
        rprintln!("Capture value: {}", tim3.ccr1().read().bits());
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    dp.RCC.ahb1enr().modify(|_, w| w.gpioaen().enabled());

    configure_pwm_output(&dp.RCC, &dp.GPIOA, &dp.TIM2);
    configure_timer_capture(&dp.RCC, &dp.GPIOA, &dp.TIM3);

    critical_section::with(|cs| {
        *SHARED_TIM3.borrow(cs).borrow_mut() = Some(dp.TIM3);
    });

    NVIC::unpend(Interrupt::TIM3);
    unsafe {
        NVIC::unmask(Interrupt::TIM3);
    }

    loop {
        // WFI stops the RTT connection
        cortex_m::asm::nop();
    }
}
