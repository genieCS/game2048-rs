use crate::board_data::BoardData;
use crate::lrud::LRUD;
use cursive::{
    theme::{BaseColor, Color, ColorStyle},
    Printer, View,
};
use maplit::hashmap;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    data: BoardData,
    theme: HashMap<u32, ColorStyle>,
    background_style: ColorStyle,
}


impl Board {
    pub fn new(background_style: ColorStyle) -> Self {
        Self {
            data: BoardData::new(),
            theme: Self::traditional_colors(background_style),
            background_style,
        }
    }

    fn traditional_colors(default_color: ColorStyle) -> HashMap<u32, ColorStyle> {
        hashmap!{
            0 => default_color,
            2 => ColorStyle::new(Color::Rgb(0, 0, 0), Color::Light(BaseColor::Yellow)), 
            4 => ColorStyle::new(Color::Rgb(0, 0, 0), Color::Light(BaseColor::Green)),
            8 => ColorStyle::new(Color::Rgb(0, 0, 0), Color::Light(BaseColor::Red)),
            16 => ColorStyle::new(Color::Rgb(0, 0, 0), Color::Light(BaseColor::Cyan)),
            32 => ColorStyle::new(Color::Rgb(0, 0, 0), Color::Light(BaseColor::Blue)),
            64 => ColorStyle::new(Color::Rgb(0, 0, 0), Color::Light(BaseColor::Magenta)),
            128 => ColorStyle::new(Color::Rgb(0, 0, 0), BaseColor::Yellow),
            256 => ColorStyle::new(Color::Rgb(0, 0, 0), BaseColor::Green),
            512 => ColorStyle::new(Color::Rgb(0, 0, 0), BaseColor::Red),
            1024 => ColorStyle::new(Color::Rgb(0, 0, 0), BaseColor::Cyan),
            2048 => ColorStyle::new(Color::Rgb(0, 0, 0), BaseColor::Blue),
            4096 => ColorStyle::new(Color::Rgb(0, 0, 0), BaseColor::Magenta),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.background_style);
    }

    pub fn push(&mut self, lrud: LRUD) -> (u32, bool, bool) {
        self.data.push(lrud)
    }

    fn draw_background(&self, printer: &Printer) {
        let len = self.data.len();
        for i in 0..(len + 1){
            printer.with_color(self.background_style, |printer| {
                printer.print((0, 6 * i), "o---------o---------o---------o---------o");
                if i != len {
                    for j in 0..(len + 1) {
                        for k in 1..6 {
                            printer.print((10 * j, 6 * i + k), "|");
                        }
                    }
                }
            });
        }
    }

    fn draw_board(&self, printer: &Printer) {
        let len = self.data.len();
        for i in 0..len {
            for j in 0..len {
                let num = self.data[i][j];
                let color_style = self.theme.get(&num).unwrap();
                let num_str = &num.to_string();
                let num = if num == 0 { "" } else { num_str };
                printer.with_color(*color_style, |printer| {
                    printer.print((10 * j + 1, 6 * i + 1), "         ");
                    printer.print((10 * j + 1, 6 * i + 2), "         ");
                    printer.print((10 * j + 1, 6 * i + 3), &format!(" {:^7} ", num));
                    printer.print((10 * j + 1, 6 * i + 4), "         ");
                    printer.print((10 * j + 1, 6 * i + 5), "         ");
                });
            }
        }
    }
}

impl View for Board {
    fn draw(&self, printer: &Printer) {
        self.draw_background(printer);
        self.draw_board(printer);
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(45, 40)
    }
}