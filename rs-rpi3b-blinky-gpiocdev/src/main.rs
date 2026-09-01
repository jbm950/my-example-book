use gpiocdev::{line::Value, Request};
use std::time::Duration;

const GPIO_LED_PIN: u32 = 17; // Using GPIO 17 (pin 11) for the example

fn main() -> gpiocdev::Result<()> {
    let request = Request::builder()
        .on_chip("/dev/gpiochip0")
        .with_consumer("rs-rpi3b-blinky")
        .with_line(GPIO_LED_PIN)
        .as_output(Value::Inactive)
        .request()?;

    loop {
        request.set_value(GPIO_LED_PIN, Value::Active)?;
        std::thread::sleep(Duration::from_secs(1));
        request.set_value(GPIO_LED_PIN, Value::Inactive)?;
        std::thread::sleep(Duration::from_secs(1));
    }
}
