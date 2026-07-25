#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4::stm32f446 as pac;

// USART2 TX is on PA2
// USART2 RX is on PA3

fn setup_usart2(rcc: &pac::RCC, gpioa: &pac::GPIOA, usart2: &pac::USART2) {
    rcc.ahb1enr().modify(|_, w| w.gpioaen().set_bit());
    rcc.apb1enr().modify(|_, w| w.usart2en().set_bit());

    #[rustfmt::skip]
    gpioa.moder().modify(|_, w| {
        w.moder2().alternate()
         .moder3().alternate()
    });

    // Per Table 11 of the Datasheet (page 58)
    //   * PA2 -> USART2 TX = AF7
    //   * PA3 -> USART2 RX = AF7
    #[rustfmt::skip]
    gpioa.afrl().modify(|_, w| {
        w.afrl2().af7()
         .afrl3().af7()
    });

    // Set baud rate (115200 @ 16 MHz HSI, default reset clocks, no PLL)
    // USARTDIV = 16_000_000 / (16 * 115200) ≈ 8.6875 -> mantissa=8, fraction=11
    usart2
        .brr()
        .write(|w| unsafe { w.div_mantissa().bits(8).div_fraction().bits(11) });
    usart2
        .cr1()
        .write(|w| w.ue().set_bit().te().set_bit().re().set_bit());
}

fn echo_byte(usart2: &pac::USART2) {
    // Busy loop waiting for an incoming byte
    while usart2.sr().read().rxne().bit_is_clear() {}
    let byte = usart2.dr().read().dr().bits() as u8;

    // Make sure TX is clear before writing next byte
    while usart2.sr().read().txe().bit_is_clear() {}
    usart2.dr().write(|w| unsafe { w.bits(byte as u16) });
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    setup_usart2(&dp.RCC, &dp.GPIOA, &dp.USART2);

    loop {
        echo_byte(&dp.USART2);
    }
}
