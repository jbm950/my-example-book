#![no_main]
#![no_std]


use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4::stm32f446 as pac;

// Per Table 11 of the Datasheet (page 58)
//   * PA4 -> SPI1 NSS = AF5
//   * PA5 -> SPI1 SCK = AF5
//   * PA6 -> SPI1 MISO = AF5
//   * PA7 -> SPI1 MOSI = AF5
//
// Per Table 11 of the Datasheet (page 59)
//   * PB12 -> SPI2 NSS = AF5
//   * PB13 -> SPI2 SCK = AF5
//   * PB14 -> SPI2 MISO = AF5
//   * PB15 -> SPI2 MOSI = AF5

/// SPI1 baud rate prescaler: fPCLK2 / 8.
/// With default HSI clocks (APB2 = 16 MHz), this yields a 2 MHz SCK.
const SPI1_BAUD_DIV8: u8 = 0b010;

fn setup_spi_controller(rcc: &pac::RCC, gpioa: &pac::GPIOA, spi1: &pac::SPI1) {
    // SPI1 is set up as the controller
    rcc.ahb1enr().modify(|_, w| w.gpioaen().set_bit());
    rcc.apb2enr().modify(|_, w| w.spi1en().set_bit());

    // PA4 (NSS) is plain GPIO output, software controlled
    gpioa.moder().modify(|_, w| w.moder4().output());
    gpioa.bsrr().write(|w| w.bs4().set_bit());

    // Set up PA5 (SCK), PA6 (MISO) and PA7 (MOSI)
    #[rustfmt::skip]
    gpioa.moder().modify(|_, w| {
        w.moder5().alternate()
         .moder6().alternate()
         .moder7().alternate()
    });

    #[rustfmt::skip]
    gpioa.afrl().modify(|_, w| {
        w.afrl5().af5()
         .afrl6().af5()
         .afrl7().af5()
    });

    #[rustfmt::skip]
    spi1.cr1().write(|w| unsafe {
        w.br().bits(SPI1_BAUD_DIV8)
         .mstr().set_bit()
         .cpol().clear_bit()   // clock idle low
         .cpha().clear_bit()   // sample on first (leading) clock edge
         // Use software NSS management; PA4 is the physical CS GPIO.
         .ssm().set_bit()
         .ssi().set_bit() // Internal NSS high
    });
}

fn setup_spi_peripheral(rcc: &pac::RCC, gpiob: &pac::GPIOB, spi2: &pac::SPI2) {
    // SPI2 is set up as the peripheral
    rcc.ahb1enr().modify(|_, w| w.gpioben().set_bit());
    rcc.apb1enr().modify(|_, w| w.spi2en().set_bit());

    // SPI2
    // Set up PB12 (NSS), PB13 (SCK), PB14 (MISO), PB15 (MOSI)
    #[rustfmt::skip]
    gpiob.moder().modify(|_, w| {
        w.moder12().alternate()
         .moder13().alternate()
         .moder14().alternate()
         .moder15().alternate()
    });

    #[rustfmt::skip]
    gpiob.afrh().modify(|_, w| {
        w.afrh12().af5()
         .afrh13().af5()
         .afrh14().af5()
         .afrh15().af5()
    });

    #[rustfmt::skip]
    spi2.cr1().write(|w| {
        w.mstr().clear_bit()
         .cpol().clear_bit()   // clock idle low
         .cpha().clear_bit()   // sample on first (leading) clock edge
         .ssm().clear_bit()
    });
}

fn enable_spi(spi1: &pac::SPI1, spi2: &pac::SPI2) {
    spi2.cr1().modify(|_, w| w.spe().set_bit()); // Peripheral
    spi1.cr1().modify(|_, w| w.spe().set_bit()); // Controller
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    setup_spi_controller(&dp.RCC, &dp.GPIOA, &dp.SPI1);
    setup_spi_peripheral(&dp.RCC, &dp.GPIOB, &dp.SPI2);
    enable_spi(&dp.SPI1, &dp.SPI2);

    let cmd_bytes = [0xAA, 0x55, 0x12, 0x34];
    let periph_bytes = [0xDE, 0xAD, 0xBE, 0xEF];

    for (i, (&cmd_byte, &periph_byte)) in cmd_bytes.iter().zip(periph_bytes.iter()).enumerate() {
        // Preload the peripheral's outgoing byte before the controller
        // starts generating clock pulses.
        dp.SPI2.dr().write(|w| unsafe { w.dr().bits(periph_byte as u16) });

        // Assert CS (active low)
        dp.GPIOA.bsrr().write(|w| w.br4().set_bit());

        // Send controller byte
        while dp.SPI1.sr().read().txe().bit_is_clear() {}
        dp.SPI1.dr().write(|w| unsafe { w.dr().bits(cmd_byte as u16) });

        // Both bytes were transmitted/received simultaneously.
        while dp.SPI1.sr().read().rxne().bit_is_clear() {}
        let rx_controller = dp.SPI1.dr().read().dr().bits() as u8;

        while dp.SPI2.sr().read().rxne().bit_is_clear() {}
        let rx_peripheral = dp.SPI2.dr().read().dr().bits() as u8;

        // Deassert CS after bus no longer busy
        while dp.SPI1.sr().read().bsy().bit_is_set() {}
        dp.GPIOA.bsrr().write(|w| w.bs4().set_bit());

        rprintln!(
            "[{}] C_TX={:#04x} -> P_RX={:#04x} {}   |   P_TX={:#04x} -> C_RX={:#04x} {}",
            i,
            cmd_byte, rx_peripheral, if cmd_byte == rx_peripheral { "OK" } else { "MISMATCH" },
            periph_byte, rx_controller, if periph_byte == rx_controller { "OK" } else { "MISMATCH" }
        );
    }

    rprintln!("Done.");

    loop {
        cortex_m::asm::nop();
    }
}
