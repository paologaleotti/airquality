use core::fmt::Debug;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    text::{Baseline, Text},
    Drawable,
};
use format_no_std::show;
use ssd1306::mode::BufferedGraphicsMode;
use ssd1306::prelude::WriteOnlyDataCommand;
use ssd1306::size::DisplaySize;
use ssd1306::Ssd1306;

use crate::consts::UI_LINE_HEIGHT;

pub enum UiLine {
    L1,
    L2,
    L3,
    L4,
    L5,
}

impl UiLine {
    fn to_u8(&self) -> u8 {
        match self {
            UiLine::L1 => 0,
            UiLine::L2 => 1,
            UiLine::L3 => 2,
            UiLine::L4 => 3,
            UiLine::L5 => 4,
        }
    }
}

pub trait Display: DrawTarget {
    fn flush(&mut self) -> Result<(), Self::Error>;
}

impl<DI, SIZE> Display for Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush()
    }
}

pub struct UiEngine<D>
where
    D: Display,
{
    display: D,
}

impl<D> UiEngine<D>
where
    D: Display<Color = BinaryColor>,
    D::Error: Debug,
{
    pub fn new(display: D) -> Self {
        UiEngine { display }
    }

    pub fn print_text(&mut self, text: &str, label: Option<&str>, line: UiLine) {
        let mut buf = [0u8; 64];

        let text_to_draw = match label {
            Some(l) => show(&mut buf, format_args!("{}: {}", l, text)).unwrap(),
            None => text,
        };

        let y_position = line.to_u8() * UI_LINE_HEIGHT;

        Text::with_baseline(
            text_to_draw,
            Point::new(0, y_position as i32),
            self.default_style(),
            Baseline::Top,
        )
        .draw(&mut self.display)
        .unwrap();
    }

    pub fn print_centered(&mut self, text: &str) {
        let text_width = text.len() as i32 * 6;
        let x_position = (128 - text_width) / 2;

        Text::with_baseline(
            text,
            Point::new(x_position, 32),
            self.default_style(),
            Baseline::Middle,
        )
        .draw(&mut self.display)
        .unwrap();
    }

    pub fn flush(&mut self) {
        self.display.flush().unwrap();
    }

    pub fn clear(&mut self) {
        self.display.clear(BinaryColor::Off).unwrap()
    }

    fn default_style(&self) -> MonoTextStyle<'static, BinaryColor> {
        MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build()
    }
}
