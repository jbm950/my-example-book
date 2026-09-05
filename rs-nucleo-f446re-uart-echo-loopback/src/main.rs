#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal_nb::serial::{Read, Write as _};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    gpio::GpioExt,
    nb::block,
    pac,
    rcc::{self, Rcc, RccExt},
    serial::{self, SerialExt},
    time::Bps,
};

// Loopback will be UART1 Tx -> UART 3 Rx
// UART1 Tx on PB6 (D10)
// UART3 Rx on PC5 (Pin 6 on right side)

fn setup_usart1(
    rcc: &mut Rcc,
    gpiob: pac::GPIOB,
    usart1: pac::USART1,
) -> serial::Tx<pac::USART1, u8> {
    let gpiob = gpiob.split(rcc);
    let tx = gpiob.pb6.into_alternate();
    let config = serial::Config::default().baudrate(Bps(115_200));

    usart1.tx(tx, config, rcc).unwrap()
}

fn setup_usart3(
    rcc: &mut Rcc,
    gpioc: pac::GPIOC,
    usart3: pac::USART3,
) -> serial::Rx<pac::USART3, u8> {
    let gpioc = gpioc.split(rcc);
    let rx = gpioc.pc5.into_alternate();
    let config = serial::Config::default().baudrate(Bps(115_200));

    usart3.rx(rx, config, rcc).unwrap()
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peripherals = pac::Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.freeze(rcc::Config::hsi());
    let mut usart1_tx = setup_usart1(&mut rcc, peripherals.GPIOB, peripherals.USART1);
    let mut usart3_rx = setup_usart3(&mut rcc, peripherals.GPIOC, peripherals.USART3);

    for byte in b"Hello World Loopback!\r\n" {
        block!(usart1_tx.write(*byte)).unwrap();
        block!(usart1_tx.flush()).unwrap();

        let response = block!(usart3_rx.read()).unwrap();
        rprintln!("Received: {}", char::from(response));
    }

    loop {
        // wfi messes with RTT logging
        cortex_m::asm::nop();
    }
}
