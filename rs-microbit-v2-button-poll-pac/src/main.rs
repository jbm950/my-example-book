#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nrf52833_pac as pac;
use panic_halt as _;

// LED Pins for Middle LED:
//    Row 3 P0.15
//    Col 3 P0.31
const ROW_3_PIN: usize = 15;
const COL_3_PIN: usize = 31;

// Button A P0.14
const BUTTON_A_PIN: usize = 14;

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

fn set_pin_output(p0: &pac::P0, pin: usize, high: bool) {
    // SAFETY: All callers pass valid GPIO pin numbers in the range 0..32.
    unsafe {
        if high {
            p0.outset.write(|w| w.bits(1 << pin));
        } else {
            p0.outclr.write(|w| w.bits(1 << pin));
        }
    }
}

fn set_pin_output_low(p0: &pac::P0, pin: usize) {
    p0.outclr.write(|w| unsafe {
        // SAFETY: All callers pass valid GPIO pin numbers in the range 0..32.
        w.bits(1 << pin)
    });
}

fn pin_is_low(p0: &pac::P0, pin: usize) -> bool {
    (p0.in_.read().bits() & (1 << pin)) == 0
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    configure_led_pins(&peripherals.P0);
    configure_button_pin(&peripherals.P0);

    // Columns sink current, so keep column low while driving row high.
    set_pin_output_low(&peripherals.P0, COL_3_PIN);

    loop {
        set_pin_output(
            &peripherals.P0,
            ROW_3_PIN,
            pin_is_low(&peripherals.P0, BUTTON_A_PIN),
        );
    }
}
