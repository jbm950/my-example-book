#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    dma::{
        MemoryToPeripheral, Stream6, StreamsTuple, Transfer, config::DmaConfig, traits::StreamISR,
    },
    gpio::GpioExt,
    pac,
    rcc::{self, Rcc, RccExt},
    serial::{self, SerialExt},
    time::Bps,
};

// USART2 TX is on PA2
// USART2 TX -> Stream 6/Channel 4 of DMA 1 per table 28 of the Datasheet (page 204)

#[rustfmt::skip]
type UartDmaTx = Transfer <
    Stream6<pac::DMA1>,
    4,
    serial::Tx<pac::USART2, u8>,
    MemoryToPeripheral,
    &'static [u8],
>;

static MESSAGE: &[u8] = b"Hello World from DMA!\r\n";

fn configure_uart(
    rcc: &mut Rcc,
    gpioa: pac::GPIOA,
    usart2: pac::USART2,
) -> serial::Tx<pac::USART2, u8> {
    let gpioa = gpioa.split(rcc);
    let tx = gpioa.pa2.into_alternate();
    let config = serial::Config::default()
        .baudrate(Bps(115_200))
        .dma(serial::config::DmaConfig::Tx);

    usart2.tx(tx, config, rcc).unwrap()
}

fn configure_transfer(
    rcc: &mut Rcc,
    dma1: pac::DMA1,
    usart2: serial::Tx<pac::USART2, u8>,
) -> UartDmaTx {
    let streams = StreamsTuple::new(dma1, rcc);
    let dma_config = DmaConfig::default().memory_increment(true);

    Transfer::init_memory_to_peripheral(streams.6, usart2, MESSAGE, None, dma_config)
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi());

    let usart2 = configure_uart(&mut rcc, dp.GPIOA, dp.USART2);
    let mut transfer = configure_transfer(&mut rcc, dp.DMA1, usart2);

    // start() hands us &mut Tx<USART2, u8> for last-moment peripheral setup
    // (e.g. clearing flags); nothing needed here.
    transfer.start(|_tx| {});

    // Example doesn't account for is_transfer_error()
    while !transfer.is_transfer_complete() {}
    transfer.clear_transfer_complete();

    rprintln!("Transfer complete");

    loop {
        cortex_m::asm::wfi();
    }
}
