use cursive::{
    theme::{Color, ColorStyle},
    Printer, View,
};
use std::collections::VecDeque;

pub struct History {
    data: VecDeque<String>,
}

impl History {
    pub fn new() -> Self {
        let data = vec![
            String::from("L"),
            String::from("R"),
            String::from("U"),
            String::from("D")
            ];
        let data = VecDeque::from(data);
        Self { data }
    }
}

impl View for History {
    fn draw(&self, printer: &Printer) {
        for (i, v) in self.data.iter().enumerate() {
        let background_style = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(100, 100, 100));
        printer.with_color(background_style, |printer| {
            printer.print((0, i+1), &format!("   {:>1}   ",&v));
        });
        }
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(10,10)
    }
}