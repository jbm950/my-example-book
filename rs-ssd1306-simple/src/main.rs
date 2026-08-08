#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_graphics::{
    Drawable,
    geometry::Point,
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    text::{Baseline, Text},
};
use fugit::RateExtU32;
use panic_halt as _;
use ssd1306::{
    I2CDisplayInterface, Ssd1306, mode::DisplayConfig, prelude::DisplayRotation,
    size::DisplaySize128x64,
};
use stm32f4xx_hal::{
    gpio::GpioExt,
    i2c::{I2cExt, Mode},
    pac,
    rcc::{self, RccExt},
};

// Nucleo-F446RE / STM32F446RE alternate-function mapping:
// Per Table 11 of the Datasheet (page 58)
//   * PB8 -> I2C1 SCL
//   * PB9 -> I2C1 SDA

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().expect("peripherals already taken");

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi());

    let gpiob = dp.GPIOB.split(&mut rcc);
    let i2c1 = dp
        .I2C1
        .i2c((gpiob.pb8, gpiob.pb9), Mode::standard(100.kHz()), &mut rcc);

    let interface = I2CDisplayInterface::new(i2c1);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().expect("ssd1306 init failed — check I2C wiring/address");
    display.clear_buffer();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello\nworld!", Point::new(0, 0), text_style, Baseline::Top)
        .draw(&mut display)
        .expect("drawing to display buffer failed");

    display.flush().expect("failed to flush buffer to display");

    loop {
        cortex_m::asm::wfi();
    }
}
