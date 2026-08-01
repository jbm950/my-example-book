#![no_main]
#![no_std]

use cortex_m_rt::entry;
use fugit::RateExtU32;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::GpioExt,
    hal::delay::DelayNs,
    i2c::{I2c, I2cExt, Mode},
    pac,
    rcc::{self, RccExt},
    timer::TimerExt,
};

// Per Table 11 of the Datasheet (page 58)
//   * PB8 -> I2C1 SCL = AF4
//   * PB9 -> I2C1 SDA = AF4

const SSD1306_ADDR: u8 = 0x3C;
const CTRL_CMD: u8 = 0x00;
const CTRL_DATA: u8 = 0x40;
const PAGE_WIDTH: usize = 128;
const SOLID_PAGE: u8 = 0xFF;

// Command sequence follows the "Software Initialization Flow Chart" on page 64
// of the SSD1306 Manual.
#[rustfmt::skip]
const INIT_CMDS: [u8; 19] = [
    0xAE, // Display off
    
    // Set MUX Ratio
    0xA8, 0x3F,

    // Set Display Offset
    0xD3, 0x00,

    0x40, // Set Display Start Line
    0xA1, // Set Segment Re-map
    0xC8, // Set COM Output Scan Direction

    // Set COM Pins Hardware Configuration
    0xDA, 0x12,

    // Set Contrast Control
    0x81, 0x7F,

    0xA4, // Disable Entire Display On
    0xA6, // Set Normal Display

    // Set Osc Frequency
    0xD5, 0x80,

    // Enable Charge Pump Regulator
    0x8D, 0x14,

    0xAF, // Display On
];

fn send_command(i2c: &mut I2c<pac::I2C1>, cmd: u8) {
    i2c.write(SSD1306_ADDR, &[CTRL_CMD, cmd]).unwrap();
}

fn fill_first_page(i2c: &mut I2c<pac::I2C1>) {
    // --- Point the RAM pointer at page 0, column 0 and fill it solid ---
    send_command(i2c, 0xB0); // Set page start address = page 0
    send_command(i2c, 0x00); // Set lower column start nibble = 0
    send_command(i2c, 0x10); // Set upper column start nibble = 0

    let mut frame = [CTRL_DATA; PAGE_WIDTH + 1];
    frame[1..].fill(SOLID_PAGE); // 128 columns, all bits set -> solid page
    i2c.write(SSD1306_ADDR, &frame).unwrap();
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().expect("peripherals already taken");

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi());

    let mut delay = dp.TIM6.delay_ms(&mut rcc);

    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut i2c1 = dp
        .I2C1
        .i2c((gpiob.pb8, gpiob.pb9), Mode::standard(100.kHz()), &mut rcc);

    for cmd in INIT_CMDS {
        send_command(&mut i2c1, cmd)
    }
    fill_first_page(&mut i2c1);

    delay.delay_ms(5000); // Leave screen on for 5 seconds

    send_command(&mut i2c1, 0xAE); // Display off

    loop {
        cortex_m::asm::wfi();
    }
}
