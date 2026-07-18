#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use nrf52833_pac as pac;
use pac::interrupt;
use panic_halt as _;

const ROW_3_PIN: usize = 15; // P0.15
const COL_3_PIN: usize = 31; // P0.31
const TIMER0_PRESCALER: u8 = 4; // 16 MHz / 2^4 = 1 MHz
const TOGGLE_PERIOD_US: u32 = 1_500_000; // Microseconds, for use with 1 MHz clock

struct SharedPeripherals {
    p0: pac::P0,
    timer0: pac::TIMER0,
}

static SHARED_PERIPHERALS: Mutex<RefCell<Option<SharedPeripherals>>> =
    Mutex::new(RefCell::new(None));

fn configure_p0_pins(p0: &pac::P0) {
    for pin_num in [COL_3_PIN, ROW_3_PIN] {
        #[rustfmt::skip]
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
    timer
        .prescaler
        .write(|w| unsafe { w.prescaler().bits(TIMER0_PRESCALER) });
    timer.mode.write(|w| w.mode().timer());
    timer.bitmode.write(|w| w.bitmode()._32bit());
    timer.intenset.write(|w| w.compare0().set());

    timer.tasks_stop.write(|w| unsafe { w.bits(1) });
    timer.tasks_clear.write(|w| unsafe { w.bits(1) });
    timer.events_compare[0].write(|w| unsafe { w.bits(0) }); // Clear event
    timer.cc[0].write(|w| unsafe { w.bits(TOGGLE_PERIOD_US) });

    timer.tasks_start.write(|w| unsafe { w.bits(1) });
}

fn set_pin_output(p0: &pac::P0, pin: usize, high: bool) {
    // SAFETY: All callers pass valid GPIO pin numbers in the range 0..32.
    if high {
        p0.outset.write(|w| unsafe { w.bits(1 << pin) });
    } else {
        p0.outclr.write(|w| unsafe { w.bits(1 << pin) });
    }
}

#[interrupt]
fn TIMER0() {
    critical_section::with(|cs| {
        let mut shared = SHARED_PERIPHERALS.borrow(cs).borrow_mut();
        let shared = shared.as_mut().unwrap();

        shared.timer0.tasks_stop.write(|w| unsafe { w.bits(1) });
        shared.timer0.tasks_clear.write(|w| unsafe { w.bits(1) });
        shared.timer0.events_compare[0].write(|w| unsafe { w.bits(0) }); // Clear event

        set_pin_output(
            &shared.p0,
            ROW_3_PIN,
            (shared.p0.out.read().bits() & (1 << ROW_3_PIN)) == 0,
        );

        shared.timer0.tasks_start.write(|w| unsafe { w.bits(1) });
    });
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    configure_p0_pins(&peripherals.P0);
    configure_timer0(&peripherals.TIMER0);

    // Columns sink current, so keep column low while driving row high.
    set_pin_output(&peripherals.P0, COL_3_PIN, false);

    critical_section::with(|cs| {
        *SHARED_PERIPHERALS.borrow(cs).borrow_mut() = Some(SharedPeripherals {
            p0: peripherals.P0,
            timer0: peripherals.TIMER0,
        });
    });

    pac::NVIC::unpend(pac::Interrupt::TIMER0);
    unsafe { pac::NVIC::unmask(pac::Interrupt::TIMER0) };

    loop {
        cortex_m::asm::wfi();
    }
}
