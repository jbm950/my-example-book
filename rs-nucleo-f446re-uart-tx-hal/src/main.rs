#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use embedded_hal_nb::serial::Write as _;
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
) -> serial::Tx<pac::USART2, u8> {
    let gpioa = gpioa.split(rcc);
    let tx = gpioa.pa2.into_alternate();
    let config = serial::Config::default().baudrate(Bps(115_200));

    usart2.tx(tx, config, rcc).unwrap()
}

fn write_all_bytes<W: embedded_hal_nb::serial::Write<u8>>(
    writer: &mut W,
    buf: &[u8],
) -> Result<(), W::Error> {
    for &byte in buf {
        block!(writer.write(byte))?;
    }

    Ok(())
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.freeze(rcc::Config::hsi());
    let mut usart2 = setup_usart2(&mut rcc, peripherals.GPIOA, peripherals.USART2);

    // Use the core::fmt::Write trait directly.
    usart2.write_str("Hello World 1!\r\n").unwrap();

    // Use Rust's formatting macros.
    write!(usart2, "Hello World {}!\r\n", 32).unwrap();

    // Send a raw byte slice.
    write_all_bytes(&mut usart2, b"Hello World 2!\r\n").unwrap();

    // Wait until the final stop bit has completely left the shift register.
    block!(usart2.flush()).unwrap();

    loop {
        cortex_m::asm::wfi();
    }
}
