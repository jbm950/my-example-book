// Project uses raw bit accesses rather than typed accessors for educational
// purposes.

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4::stm32f446 as pac;

const GPIOAEN_BIT: u8 = 0;
const GPIO_BSRR_RESET_OFFSET: u8 = 16;

const PA5_PIN: u8 = 5;
const PA5_MODER_SHIFT: u8 = PA5_PIN * 2; // MODER has 2 bits per pin
const OUTPUT_MODE: u32 = 0b01;
const PA5_MODER_MASK: u32 = 0b11 << PA5_MODER_SHIFT;

// TIM6 is a basic timer which is all that's needed for this example
const APB1_TIM6_ENABLE_BIT: u8 = 4;
const TIM6_CEN_BIT: u8 = 0;
const TIM6_PSC_VAL: u32 = 15_999; // 16 MHz / (15_999 + 1) = 1 kHz → 1 ms per tick
const TIM6_UG_BIT: u8 = 0;
const TIM6_UIF_BIT: u8 = 0;
const TIMER_PERIOD_TICKS: u32 = 999; // (999 + 1) ticks × 1 ms = 1000 ms = 1 s

fn configure_led(rcc: &pac::RCC, gpioa: &pac::GPIOA) {
    rcc.ahb1enr()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << GPIOAEN_BIT)) });

    gpioa.moder().modify(|r, w| unsafe {
        let mut val = r.bits();
        val &= !PA5_MODER_MASK; // Clear existing PA5 MODER bits
        val |= OUTPUT_MODE << PA5_MODER_SHIFT; // Set to PA5 to output
        w.bits(val)
    });
}

fn configure_timer(rcc: &pac::RCC, tim6: &pac::TIM6) {
    rcc.apb1enr()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << APB1_TIM6_ENABLE_BIT)) });
    tim6.psc().write(|w| unsafe { w.bits(TIM6_PSC_VAL) });
    tim6.arr().write(|w| unsafe { w.bits(TIMER_PERIOD_TICKS) });

    // Force update event to load registers into active shadow registers
    tim6.egr().write(|w| unsafe { w.bits(1 << TIM6_UG_BIT) });

    // Start counter
    tim6
        .cr1()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << TIM6_CEN_BIT)) });

    // Clear the flag that UG generated so the main loop doesn't instantly
    // jump past the first delay
    tim6.sr().write(|w| unsafe { w.bits(0) });
}

fn set_pin_output(gpioa: &pac::GPIOA, pin: u8, high: bool) {
    // Bits 0..15 set, bits 16..31 reset
    let shift = if high { pin } else { pin + GPIO_BSRR_RESET_OFFSET };

    // SAFETY: Writing arbitrary bit patterns to BSRR is safe. The caller
    // guarantees `pin` is a valid GPIO pin number.
    gpioa.bsrr().write(|w| unsafe { w.bits(1 << shift) });
}

fn tim6_wait_for_period(timer: &pac::TIM6) {
    // Wait for update event
    while timer.sr().read().bits() & (1 << TIM6_UIF_BIT) == 0 {}

    // Clear event
    timer.sr().write(|w| unsafe { w.bits(0) });
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    configure_led(&peripherals.RCC, &peripherals.GPIOA);
    configure_timer(&peripherals.RCC, &peripherals.TIM6);

    loop {
        set_pin_output(&peripherals.GPIOA, PA5_PIN, true);
        tim6_wait_for_period(&peripherals.TIM6);
        set_pin_output(&peripherals.GPIOA, PA5_PIN, false);
        tim6_wait_for_period(&peripherals.TIM6);
    }
}
