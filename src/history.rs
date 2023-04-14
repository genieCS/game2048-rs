use cursive::{
    theme::{Color, ColorStyle},
    Printer, View,
};
use std::collections::VecDeque;
use crate::lrud::{LRUD};

pub struct History {
    data: VecDeque<LRUD>,
}


impl History {
    pub fn new() -> Self {
        Self { data: VecDeque::new() }
    }

    pub fn update(&mut self, lrud: LRUD) {
        if self.data.len() == 12 {
            self.data.pop_front();
        }
        self.data.push_back(lrud);
    }

    pub fn clear(&mut self) {
        while !self.data.is_empty() {
            self.data.pop_back();
        }
    }
}

impl Default for History {
        fn default() -> Self {
            Self::new()
        }
}

impl View for History {
    fn draw(&self, printer: &Printer) {
        for (i, v) in self.data.iter().enumerate() {
        let background_style = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(100, 100, 100));
        printer.with_color(background_style, |printer| {
            printer.print((0, 12-i), &format!(" {:>5} ",&v));
        });
        }
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(13,13)
    }
}