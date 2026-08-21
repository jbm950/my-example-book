#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};
use stm32f4::stm32f446 as pac;

// USART2 TX is on PA2
// USART2 TX -> Stream 6/Channel 4 of DMA 1 per table 28 of the Datasheet (page 204)

const DMA_STREAM: usize = 6; // Stream 6 per datasheet (see above)
const DMA_CHANNEL: u8 = 4; // Channel 4 per datasheet (see above)

static MESSAGE: &[u8] = b"Hello World from DMA!\r\n";

fn configure_usart2(rcc: &pac::RCC, gpioa: &pac::GPIOA, usart2: &pac::USART2) {
    rcc.ahb1enr().modify(|_, w| w.gpioaen().enabled());
    rcc.apb1enr().modify(|_, w| w.usart2en().enabled());

    gpioa.moder().modify(|_, w| w.moder2().alternate());
    // PA2 -> USART2 TX = AF7 per table 11 of the Datasheet (page 58)
    gpioa.afrl().modify(|_, w| w.afrl2().af7());

    // Set baud rate (115200 @ 16 MHz HSI, default reset clocks, no PLL)
    // USARTDIV = 16_000_000 / (16 * 115200) ≈ 8.6875 -> mantissa=8, fraction=11
    usart2.brr().write(|w| unsafe { w.div_mantissa().bits(8).div_fraction().bits(11) });

    usart2.cr3().modify(|_, w| w.dmat().enabled()); // DMA request on TXE
    usart2.cr1().write(|w| w.ue().enabled().te().enabled());
}

fn configure_dma(rcc: &pac::RCC, dma1: &pac::DMA1, usart2: &pac::USART2) {
    rcc.ahb1enr().modify(|_, w| w.dma1en().enabled());

    let stream = dma1.st(DMA_STREAM);

    // Ensure stream is disabled before configuring
    stream.cr().modify(|_, w| w.en().disabled());
    while stream.cr().read().en().bit_is_set() {}

    // Clear stale interrupt flags
    #[rustfmt::skip]
    dma1.hifcr().write(|w| {
        w.ctcif6().clear()
         .chtif6().clear()
         .cteif6().clear()
         .cdmeif6().clear()
         .cfeif6().clear()
    });

    // Addresses and transfer length
    stream.par().write(|w| unsafe { w.pa().bits(usart2.dr().as_ptr() as u32) });
    stream.m0ar().write(|w| unsafe { w.m0a().bits(MESSAGE.as_ptr() as u32) });
    stream.ndtr().write(|w| w.ndt().set(MESSAGE.len() as u16));

    // Configure the transfer
    stream.cr().write(|w| {
        w.chsel().set(DMA_CHANNEL)
         .dir().memory_to_peripheral()
         .minc().incremented()
         .msize().bits8()
         .pinc().fixed()
         .psize().bits8()
         .circ().disabled()
    });
}

fn do_transfer(dma1: &pac::DMA1) {
    let stream = dma1.st(DMA_STREAM);

    // Start the transfer
    stream.cr().modify(|_, w| w.en().enabled());

    // Wait for transfer completion.
    // Error flags are intentionally not handled in this minimal example.
    while dma1.hisr().read().tcif6().bit_is_clear() {}

    // Clear the flag and disable the stream
    dma1.hifcr().write(|w| w.ctcif6().clear());
    stream.cr().modify(|_, w| w.en().disabled());
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    configure_usart2(&dp.RCC, &dp.GPIOA, &dp.USART2);
    configure_dma(&dp.RCC, &dp.DMA1, &dp.USART2);

    do_transfer(&dp.DMA1);

    rprintln!("Transfer Complete!");

    loop {
        cortex_m::asm::nop();
    }
}
