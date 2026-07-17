#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nrf52833_pac as pac;
use panic_halt as _;

// Col 3 P0.31
// Row 3 P0.15

const ROW_3_PIN: usize = 15;
const COL_3_PIN: usize = 31;
const TIMER0_PRESCALER: u8 = 4; // 16 MHz / 2^4 = 1 MHz
const DELAY_US: u32 = 1_000_000; // Microseconds, for use with 1 MHz clock

fn configure_p0_pins(p0: &pac::P0) {
    for pin_num in [COL_3_PIN, ROW_3_PIN] {
        p0.pin_cnf[pin_num].write(|w| {
            w.dir().output()
            .input().disconnect()
            .pull().disabled()
            .drive().s0s1()
            .sense().disabled()
        });
    }
}

fn configure_timer0(timer: &pac::TIMER0) {
    timer.prescaler.write(|w| unsafe { w.prescaler().bits(TIMER0_PRESCALER) });
    timer.mode.write(|w| w.mode().timer());
    timer.bitmode.write(|w| w.bitmode()._32bit());
}

fn set_pin(p0: &pac::P0, pin: usize) {
    p0.outset.write(|w| unsafe { w.bits(1 << pin) });
}

fn clear_pin(p0: &pac::P0, pin: usize) {
    p0.outclr.write(|w| unsafe { w.bits(1 << pin) });
}

/// Block execution of program waiting for a specified delay using timer 0
///
/// ## Parameters
/// - `timer`: access to TIMER0
/// - `us`: amount of time to delay in microseconds (assumes TIMER0 runs at 1 MHz)
fn timer0_delay(timer: &pac::TIMER0, us: u32) {
    // Reset timer
    timer.tasks_stop.write(|w| unsafe { w.bits(1) });
    timer.tasks_clear.write(|w| unsafe { w.bits(1) });
    timer.events_compare[0].write(|w| unsafe { w.bits(0) }); // Clear event

    timer.cc[0].write(|w| unsafe { w.bits(us) });
    timer.tasks_start.write(|w| unsafe { w.bits(1) });

    // Busy loop until the timer completes
    while timer.events_compare[0].read().bits() == 0 {};
    timer.tasks_stop.write(|w| unsafe { w.bits(1) });
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    configure_p0_pins(&peripherals.P0);
    configure_timer0(&peripherals.TIMER0);

    // Columns sink current, so keep column low while driving row high.
    clear_pin(&peripherals.P0, COL_3_PIN);

    loop {
        set_pin(&peripherals.P0, ROW_3_PIN);
        timer0_delay(&peripherals.TIMER0, DELAY_US);
        clear_pin(&peripherals.P0, ROW_3_PIN);
        timer0_delay(&peripherals.TIMER0, DELAY_US);
    }
}
