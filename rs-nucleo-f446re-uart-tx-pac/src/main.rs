#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4::stm32f446 as pac;

// USART2 TX is on PA2

fn setup_usart2(rcc: &pac::RCC, gpioa: &pac::GPIOA, usart2: &pac::USART2) {
    rcc.ahb1enr().modify(|_, w| w.gpioaen().set_bit());
    rcc.apb1enr().modify(|_, w| w.usart2en().set_bit());

    gpioa.moder().modify(|_, w| w.moder2().alternate());
    // PA2 -> USART2 TX = AF7 per table 11 of the Datasheet (page 58)
    gpioa.afrl().modify(|_, w| w.afrl2().af7());

    // Set baud rate (115200 @ 16 MHz HSI, default reset clocks, no PLL)
    // USARTDIV = 16_000_000 / (16 * 115200) ≈ 8.6875 -> mantissa=8, fraction=11
    usart2.brr().write(|w| unsafe { w.div_mantissa().bits(8).div_fraction().bits(11) });
    usart2.cr1().write(|w| w.ue().set_bit().te().set_bit());
}

fn send_bytes(usart2: &pac::USART2, bytes: &[u8]) {
    for &byte in bytes {
        while usart2.sr().read().txe().bit_is_clear() {}
        usart2.dr().write(|w| unsafe { w.bits(byte as u16) });
    }
    // Wait until the final stop bit has completely left the shift register.
    while usart2.sr().read().tc().bit_is_clear() {}
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    setup_usart2(&dp.RCC, &dp.GPIOA, &dp.USART2);
    send_bytes(&dp.USART2, b"Hello World!\r\n");

    loop {
        cortex_m::asm::wfi();
    }
}
