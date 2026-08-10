#![no_main]
#![no_std]

use cortex_m_rt::entry;
use fugit::RateExtU32;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    gpio::GpioExt, pac, rcc::{self, Rcc, RccExt}, spi::{Mode, Phase, Polarity, SpiExt}
};

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

const SPI_MODE: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::CaptureOnFirstTransition,
};

fn setup_spi_peripheral(rcc: &Rcc, gpiob: &pac::GPIOB, spi2: &pac::SPI2) {
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
         .spe().set_bit()
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().expect("peripherals already taken");

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi());
    let gpioa = dp.GPIOA.split(&mut rcc);

    let mut spi1 = dp.SPI1.spi(
        (Some(gpioa.pa5), Some(gpioa.pa6), Some(gpioa.pa7)),
        SPI_MODE,
        1.MHz(),
        &mut rcc,
    );

    // PA4 (NSS) is plain GPIO, software-controlled
    let mut cs = gpioa.pa4.into_push_pull_output();
    cs.set_high();

    // --- Peripheral: SPI2, deliberately left at the PAC level ---
    //
    // stm32f4xx-hal's slave API only exposes blocking SpiBus methods,
    // which write AND wait in one call. That doesn't fit a single-core
    // polling loop where we need to preload the outgoing byte *before*
    // the controller starts the clock, then separately read what arrived
    // *after*. Rather than reach for interrupts (out of scope for this
    // project) or fight the abstraction, SPI2 stays exactly as
    // configured in the PAC version.
    setup_spi_peripheral(&rcc, &dp.GPIOB, &dp.SPI2);

    let cmd_bytes = [0xAA, 0x55, 0x12, 0x34];
    let periph_bytes = [0xDE, 0xAD, 0xBE, 0xEF];

    for (i, (cmd_byte, periph_byte)) in cmd_bytes.into_iter().zip(periph_bytes).enumerate() {
        // Preload the peripheral's outgoing byte before the controller
        // starts generating clock pulses.
        dp.SPI2.dr().write(|w| unsafe { w.dr().bits(periph_byte as u16) });

        // Assert CS (active low)
        cs.set_low();

        let mut buffer = [cmd_byte];
        spi1.transfer_in_place(&mut buffer).unwrap();
        let rx_controller = buffer[0];

        // Read what the peripheral received, straight from its DR.
        let rx_peripheral = dp.SPI2.dr().read().dr().bits() as u8;

        // Deassert CS
        cs.set_high();

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
