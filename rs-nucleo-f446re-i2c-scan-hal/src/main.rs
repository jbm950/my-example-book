#![no_main]
#![no_std]

use core::ops::Range;

use cortex_m_rt::entry;
use fugit::RateExtU32;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    gpio::GpioExt,
    i2c::{I2cExt, Mode},
    pac,
    rcc::{self, RccExt},
};

// Per Table 11 of the Datasheet (page 58)
//   * PB8 -> I2C1 SCL = AF4
//   * PB9 -> I2C1 SDA = AF4

/// Valid 7-bit I2C addresses; 0x00-0x07 and 0x78-0x7F are reserved.
const SCAN_RANGE: Range<u8> = 0x08..0x78;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().expect("peripherals already taken");

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi());
    let gpiob = dp.GPIOB.split(&mut rcc);

    let mut i2c1 = dp
        .I2C1
        .i2c((gpiob.pb8, gpiob.pb9), Mode::standard(100.kHz()), &mut rcc);

    rprintln!("Setup I2C complete");

    let num_devices = SCAN_RANGE
        .filter(|&addr| {
            // Sending only the address is enough to detect whether a device ACKs.
            let found = i2c1.write(addr, &[]).is_ok();
            if found {
                rprintln!("Found device at 0x{:02X}", addr);
            }
            found
        })
        .count();

    rprintln!("Scan complete. {} device(s) found.", num_devices);

    loop {
        cortex_m::asm::nop();
    }
}
