#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal_nb::serial::{Read, Write as _};
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::GpioExt,
    nb::block,
    pac,
    rcc::{self, Rcc, RccExt},
    serial::{self, SerialExt},
    time::Bps,
};

fn setup_usart2(
    rcc: &mut Rcc,
    gpioa: pac::GPIOA,
    usart2: pac::USART2,
) -> serial::Serial<pac::USART2, u8> {
    let gpioa = gpioa.split(rcc);
    let tx_pin = gpioa.pa2.into_alternate();
    let rx_pin = gpioa.pa3.into_alternate();
    let config = serial::Config::default().baudrate(Bps(115_200));

    usart2.serial((tx_pin, rx_pin), config, rcc).unwrap()
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.freeze(rcc::Config::hsi());
    let mut usart2 = setup_usart2(&mut rcc, peripherals.GPIOA, peripherals.USART2);

    loop {
        let byte = match block!(usart2.read()) {
            Ok(b) => b,
            Err(_) => continue,
        };
        block!(usart2.write(byte)).unwrap();
    }
}
