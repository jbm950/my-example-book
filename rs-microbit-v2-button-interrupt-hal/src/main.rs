#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use embedded_hal::digital::{InputPin, OutputPin};
use nrf52833_hal::{
    gpio, gpiote,
    pac::{self, interrupt},
};
use panic_halt as _;

// Center LED of the 5×5 matrix.
// LED is driven by:
//   Column 3 -> P0.31 (active low)
//   Row 3    -> P0.15 (active high)
//
// Button A -> P0.14 (active low)

struct SharedPeripherals {
    row_3_pin: gpio::Pin<gpio::Output<gpio::PushPull>>,
    button_pin: gpio::Pin<gpio::Input<gpio::PullUp>>,
    gpiote: gpiote::Gpiote,
}

static SHARED_PERIPHERALS: Mutex<RefCell<Option<SharedPeripherals>>> =
    Mutex::new(RefCell::new(None));

#[interrupt]
fn GPIOTE() {
    critical_section::with(|cs| {
        if let Some(shared) = SHARED_PERIPHERALS.borrow(cs).borrow_mut().as_mut() {
            shared.gpiote.channel0().reset_events();

            // Mechanical bounce may generate multiple interrupts, but because
            // the LED is driven from the current button level (rather than
            // toggled), it settles to the correct final state.
            let button_pressed = shared.button_pin.is_low().unwrap();
            shared.row_3_pin.set_state(button_pressed.into()).unwrap();
        }
    });
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let pins = gpio::p0::Parts::new(peripherals.P0);

    let _col_3 = pins.p0_31.into_push_pull_output(gpio::Level::Low);
    let mut row_3 = pins.p0_15.into_push_pull_output(gpio::Level::Low).degrade();
    let mut button_pin = pins.p0_14.into_pullup_input().degrade();

    let gpiote = gpiote::Gpiote::new(peripherals.GPIOTE);
    gpiote
        .channel0()
        .input_pin(&button_pin)
        .toggle()
        .enable_interrupt();

    // Sync initial state in case button is pressed on boot
    let initial_state = button_pin.is_low().unwrap();
    row_3.set_state(initial_state.into()).unwrap();

    critical_section::with(|cs| {
        *SHARED_PERIPHERALS.borrow(cs).borrow_mut() = Some(SharedPeripherals {
            row_3_pin: row_3,
            button_pin,
            gpiote,
        });
    });

    pac::NVIC::unpend(pac::Interrupt::GPIOTE);
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE) };

    loop {
        cortex_m::asm::wfi();
    }
}
