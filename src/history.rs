use cursive::{
    theme::{ColorStyle},
    Printer, View, XY,
};
use std::{collections::VecDeque };
use crate::lrud::{LRUD};

pub struct History {
    data: VecDeque<LRUD>,
    background_color: ColorStyle,
    main_color: ColorStyle,
    capacity: usize,
}


impl History {
    pub fn new(background_color: ColorStyle, main_color: ColorStyle, capacity: usize) -> Self {
        Self {
            data: VecDeque::new(),
            background_color,
            main_color,
            capacity,
        }
    }

    pub fn update(&mut self, lrud: LRUD) {
        if self.data.len() == self.capacity {
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

impl View for History {
    fn draw(&self, printer: &Printer) {
        printer.with_color(self.main_color, |printer| {
            printer.print(XY::new(0,0), " HISTORY ");
        });
        if self.data.is_empty() {
            return
        }
        printer.with_color(self.background_color, |printer| {
            for (i, v) in self.data.iter().take(self.data.len() - 1).enumerate() {
                printer.print((0, self.data.len() - i), &format!(" {} ", v));
            }
        });
        printer.with_color(self.main_color, |printer| {
            printer.print((0, 1), &format!(" {} ", self.data.back().unwrap()));
        });
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(13,self.capacity + 3)
    }
}