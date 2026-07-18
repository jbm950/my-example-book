#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use nrf52833_pac as pac;
use pac::interrupt;
use panic_halt as _;

// LED Pins for Middle LED:
//    Row 3 P0.15
//    Col 3 P0.31
const ROW_3_PIN: usize = 15;
const COL_3_PIN: usize = 31;

// Button A P0.14
const BUTTON_A_PIN: usize = 14;
const BUTTON_GPIOTE_CHANNEL: usize = 0;

struct SharedPeripherals {
    p0: pac::P0,
    gpiote: pac::GPIOTE,
}

static SHARED_PERIPHERALS: Mutex<RefCell<Option<SharedPeripherals>>> =
    Mutex::new(RefCell::new(None));

fn configure_led_pins(p0: &pac::P0) {
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

fn configure_button_pin(p0: &pac::P0) {
    #[rustfmt::skip]
    p0.pin_cnf[BUTTON_A_PIN].write(|w| {
        w.dir().input()
            .input().connect()
            .pull().pullup()
            .drive().s0s1()  // Ignored for input pins; included for completeness
            .sense().disabled()
    });
}

fn configure_gpiote(gpiote: &pac::GPIOTE) {
    #[rustfmt::skip]
    gpiote.config[BUTTON_GPIOTE_CHANNEL].write(|w| {
        w.mode().event()
            .polarity().toggle();

        unsafe {
            w.psel().bits(BUTTON_A_PIN as u8)
        }
    });
    gpiote.intenset.write(|w| w.in0().set());
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
fn GPIOTE() {
    critical_section::with(|cs| {
        let mut shared = SHARED_PERIPHERALS.borrow(cs).borrow_mut();
        let shared = shared.as_mut().unwrap();

        if shared.gpiote.events_in[BUTTON_GPIOTE_CHANNEL].read().events_in().is_generated() {
            shared.gpiote.events_in[BUTTON_GPIOTE_CHANNEL].reset();

            // Interrupt is triggered by button press and release.
            // Therefore, need to determine the state of the button.
            let button_is_pressed = (shared.p0.in_.read().bits() & (1 << BUTTON_A_PIN)) == 0;
            set_pin_output(&shared.p0, ROW_3_PIN, button_is_pressed);
        }
    });
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    configure_led_pins(&peripherals.P0);
    configure_button_pin(&peripherals.P0);
    configure_gpiote(&peripherals.GPIOTE);

    // Columns sink current, so keep column low while driving row high.
    set_pin_output(&peripherals.P0, COL_3_PIN, false);

    critical_section::with(|cs| {
        *SHARED_PERIPHERALS.borrow(cs).borrow_mut() = Some(SharedPeripherals {
            p0: peripherals.P0,
            gpiote: peripherals.GPIOTE,
        });
    });

    pac::NVIC::unpend(pac::Interrupt::GPIOTE);
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE) };

    loop {
        cortex_m::asm::wfi();
    }
}
