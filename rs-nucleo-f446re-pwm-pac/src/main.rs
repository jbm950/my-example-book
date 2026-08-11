#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4::stm32f446 as pac;

// Onboard LED is attached to PA5

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
const DUTY_CYCLE: u32 = (TIM2_ARR + 1) / 2; // 50% Duty

fn configure_pwm_output(rcc: &pac::RCC, gpioa: &pac::GPIOA) {
    rcc.ahb1enr().modify(|_, w| w.gpioaen().set_bit());
    gpioa.moder().modify(|_, w| w.moder5().alternate());
    gpioa.afrl().modify(|_, w| w.afrl5().af1());
}

fn init_timer(rcc: &pac::RCC, tim2: &pac::TIM2) {
    rcc.apb1enr().modify(|_, w| w.tim2en().set_bit());

    tim2.psc().write(|w| w.psc().set(TIM2_PSC));
    tim2.arr().write(|w| w.arr().set(TIM2_ARR));

    tim2.ccmr1_output().modify(|_, w| {
        w.oc1m().pwm_mode1().oc1pe().set_bit() // Enable CCR1 preload
    });
    tim2.ccer().modify(|_, w| w.cc1e().set_bit());
    tim2.cr1().modify(|_, w| w.arpe().set_bit());

    tim2.ccr1().write(|w| w.ccr().set(DUTY_CYCLE));

    // Force update event to load registers into active shadow registers
    tim2.egr().write(|w| w.ug().set_bit());

    // Start counter
    tim2.cr1().modify(|_, w| w.cen().set_bit());
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    configure_pwm_output(&peripherals.RCC, &peripherals.GPIOA);
    init_timer(&peripherals.RCC, &peripherals.TIM2);

    loop {
        cortex_m::asm::wfi();
    }
}
