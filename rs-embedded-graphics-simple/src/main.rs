use embedded_graphics::{
    Drawable,
    geometry::{Point, Size},
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    text::{Baseline, Text},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

const DISPLAY_SIZE: Size = Size::new(128, 64);

fn main() {
    let mut display = SimulatorDisplay::new(DISPLAY_SIZE);

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello\nworld!", Point::new(0, 0), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();

    Window::new("embedded-graphics playground", &output_settings).show_static(&display);
}
