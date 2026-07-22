#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4::stm32f446 as pac;

// Onboard LED is attached to PA5
// User Button is attached to PC13

fn configure_led(rcc: &pac::RCC, gpioa: &pac::GPIOA) {
    rcc.ahb1enr().modify(|_, w| w.gpioaen().set_bit());
    gpioa.moder().modify(|_, w| w.moder5().output());
}

fn configure_button(rcc: &pac::RCC, gpioc: &pac::GPIOC) {
    rcc.ahb1enr().modify(|_, w| w.gpiocen().set_bit());
    gpioc.moder().modify(|_, w| w.moder13().input());
    gpioc.pupdr().modify(|_, w| w.pupdr13().floating());
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    configure_led(&peripherals.RCC, &peripherals.GPIOA);
    configure_button(&peripherals.RCC, &peripherals.GPIOC);

    loop {
        // User Button pulls PC13 low when pressed.
        let button_is_pressed = peripherals.GPIOC.idr().read().idr13().is_low();

        peripherals
            .GPIOA
            .odr()
            .modify(|_, w| w.odr5().bit(button_is_pressed));
    }
}
