use cursive::{
    theme::ColorStyle,
    Printer, View,
};

pub struct Score {
    background_color: ColorStyle,
    main_color: ColorStyle,
    score: u32,
}

impl Score {
    pub fn new(background_color: ColorStyle, main_color: ColorStyle) -> Self {
        Self {
            background_color,
            main_color,
            score: 0,
         }
    }

    pub fn add_score(&mut self, score: u32) {
        self.score += score;
    }

    pub fn reset(&mut self) {
        self.score = 0;
    }
}

impl View for Score {
    fn draw(&self, printer: &Printer) {
        printer.with_color(self.main_color, |printer| {
            printer.print((0,0), &format!(" {:^13} ", "SCORE"));
            printer.print((0,1), &format!(" {:^13} ",&self.score));
        });
        printer.with_color(self.background_color, |printer| {
            printer.print((0,4), &format!(" {:^13} ", "SHORTCUT"));
            printer.print((0,5), &format!(" {:13} ", "↑, u: up"));
            printer.print((0,6), &format!(" {:13} ", "↓, d: down"));
            printer.print((0,7), &format!(" {:13} ", "←, l: left"));
            printer.print((0,8), &format!(" {:13} ", "→, r: right"));
            printer.print((0,9), &format!(" {:13} ", "n: new game"));
            printer.print((0,10), &format!(" {:13} ",  "q: quit"));
        });
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(15, 11)
    }
}