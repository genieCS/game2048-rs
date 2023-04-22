use crate::history::History;
use crate::lrud::LRUD;
use cursive::{
    event::{Callback, Event, EventResult, Key},
    theme::{Color, ColorStyle},
    views::{ Dialog, TextView},
    Printer, View, XY,
};
use rand::Rng;
use maplit::hashmap;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    pub data: [[u32; 4]; 4],
    current_theme: HashMap<u32, ColorStyle>,
    swap_theme: HashMap<u32, ColorStyle>,
    score: u32,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        let data: [[u32; 4]; 4] = [[0; 4]; 4];
        let current_theme = Self::traditional_colors();
        let swap_theme = Self::various_colors();
        let mut board = Self {
            data,
            current_theme,
            swap_theme,
            score: 0,
        };
        board.insert();
        board.insert();
        board
    }

    pub fn swap_theme(&mut self) {
        std::mem::swap(&mut self.current_theme, &mut self.swap_theme);
    }

    fn traditional_colors() -> HashMap<u32, ColorStyle> {
        hashmap!{
            0 => ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(187, 173, 160)), // Default color for empty cells
            2 => ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 255, 0)), // Light yellow
            4 => ColorStyle::new(Color::Rgb(255, 255, 255), Color::Rgb(192, 192, 192)), // Gray
            8 => ColorStyle::new(Color::Rgb(255, 255, 255), Color::Rgb(0, 255, 0)), // Green
            16 => ColorStyle::new(Color::Rgb(255, 255, 255), Color::Rgb(255, 0, 0)), // Red
            32 => ColorStyle::new(Color::Rgb(255, 255, 255), Color::Rgb(0, 0, 255)), // Blue
            64 => ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 255, 255)), // White
            128 => ColorStyle::new(Color::Rgb(255, 255, 255), Color::Rgb(0, 255, 255)), // Cyan
            256 => ColorStyle::new(Color::Rgb(255, 255, 255), Color::Rgb(255, 0, 255)), // Magenta
            512 => ColorStyle::new(Color::Rgb(255, 255, 255), Color::Rgb(128, 128, 128)), // Dark gray
            1024 => ColorStyle::new(Color::Rgb(255, 255, 255), Color::Rgb(192, 192, 192)), // Light gray
            2048 => ColorStyle::new(Color::Rgb(255, 255, 255), Color::Rgb(165, 42, 42)), // Brown
        }
    }

    fn various_colors() -> HashMap<u32, ColorStyle> {
        hashmap!{
            0 => ColorStyle::new(Color::Rgb(187, 173, 160), Color::Rgb(187, 173, 160)), // Default color for empty cells
            2 => ColorStyle::new(Color::Rgb(255, 192, 203), Color::Rgb(248, 231, 28)), // Light yellow
            4 => ColorStyle::new(Color::Rgb(255, 192, 203), Color::Rgb(242, 202, 24)), // Beige
            8 => ColorStyle::new(Color::Rgb(255, 192, 203), Color::Rgb(245, 159, 27)), // Light orange
            16 => ColorStyle::new(Color::Rgb(255, 192, 203), Color::Rgb(246, 122, 29)), // Orange
            32 => ColorStyle::new(Color::Rgb(255, 192, 203), Color::Rgb(242, 83, 34)), // Light red
            64 => ColorStyle::new(Color::Rgb(255, 192, 203), Color::Rgb(210, 30, 40)), // Red
            128 => ColorStyle::new(Color::Rgb(255, 192, 203), Color::Rgb(241, 45, 98)), // Light pink
            256 => ColorStyle::new(Color::Rgb(255, 192, 203), Color::Rgb(234, 33, 126)), // Pink
            512 => ColorStyle::new(Color::Rgb(255, 192, 203), Color::Rgb(191, 33, 128)), // Light purple
            1024 => ColorStyle::new(Color::Rgb(255, 192, 203), Color::Rgb(149, 33, 135)), // Purple
            2048 => ColorStyle::new(Color::Rgb(255, 192, 203), Color::Rgb(98, 33, 138)), // Bright purple
        }
    }

    fn draw_background(&self, printer: &Printer) {
        let background_style = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(188, 173, 159));
        for i in 0..4 {
            printer.with_color(background_style, |printer| {
                printer.print((0, 6 * i), "o---------o---------o---------o---------o");
                for j in 0..5 {
                    printer.print((10 * j, 6 * i + 1), "|");
                    printer.print((10 * j, 6 * i + 2), "|");
                    printer.print((10 * j, 6 * i + 3), "|");
                    printer.print((10 * j, 6 * i + 4), "|");
                    printer.print((10 * j, 6 * i + 5), "|");

                }
            });
        }
        printer.with_color(background_style, |printer| {
            printer.print(XY::new(0, 24), "o---------o---------o---------o---------o");
        });
    }

    fn draw_board(&self, printer: &Printer) {
        for i in 0..4 {
            for j in 0..4 {
                let num = self.data[i][j];
                let color_style = self.current_theme.get(&num).unwrap();
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

    pub fn restart(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                self.data[i][j] = 0;
            }
        }
        self.score = 0;
        self.insert();
        self.insert();
    }

    fn insert(&mut self) {
        let mut rng = rand::thread_rng();
        let mut r = rng.gen_range(0..4);
        let mut c = rng.gen_range(0..4);
        while self.data[r][c] != 0 {
            r = rng.gen_range(0..4);
            c = rng.gen_range(0..4);
        }
        let vals = vec![2, 4];
        let val = vals[rng.gen_range(0..2)];
        self.data[r][c] = val;
    }

    fn is_full(&self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.data[i][j] == 0 {
                    return false;
                }
            }
        }
        true
    }

    fn push(&mut self, lrud: LRUD) -> EventResult {
        let (score, moved) = match lrud {
            LRUD::Left => self.push_left(),
            LRUD::Right => self.push_right(),
            LRUD::Up => self.push_up(),
            LRUD::Down => self.push_down(),
        };
        self.score += score;
        if moved {
            self.insert();
            if self.is_full() && !self.can_merge() {
                return self.gameover();
            }
        }
        self.event_result(self.score, lrud, moved)
    }

    fn can_merge(&self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self.data[i][j] == self.data[i + 1][j] {
                    return true;
                }
                if self.data[i][j] == self.data[i][j + 1] {
                    return true;
                }
            }
        }
        false
    }

    fn gameover(&self) -> EventResult {
        EventResult::Consumed(Some(Callback::from_fn(move |s| {
            s.add_layer(Dialog::info("Game Over!"));
        })))
    }

    fn event_result(&self, score: u32, lrud: LRUD, moved: bool) -> EventResult {
        EventResult::Consumed(Some(Callback::from_fn(move |s| {
            s.call_on_name("score", |view: &mut TextView| {
                view.set_content(score.to_string());
            });
            if moved {
                s.call_on_name("history", |history: &mut History| {
                    history.update(lrud)
                });
            }
        })))
    }

    fn push_left(&mut self) -> (u32, bool) {
        let score = self.merge_left();
        let moved = self._push_left();
        (score, score != 0 || moved)
    }

    fn merge_left(&mut self) -> u32 {
        let mut score = 0;
        for r in 0..4 {
            let mut i = 0;
            while i < 3 {
                if self.data[r][i] == 0 {
                    i += 1;
                    continue;
                }
                let mut j = i + 1;
                while j < 4 && self.data[r][j] == 0 {
                    j += 1;
                }
                if j == 4 {
                    break;
                }
                if self.data[r][i] == self.data[r][j] {
                    self.data[r][i] *= 2;
                    score += self.data[r][i];
                    self.data[r][j] = 0;
                    i = j + 1;
                } else {
                    i = j;
                }
            }
        }
        score
    }

    fn _push_left(&mut self) -> bool {
        let mut moved = false;
        for r in 0..4 {
            let mut i = 0;
            while i < 4 {
                if self.data[r][i] != 0 {
                    i += 1;
                    continue;
                }
                let mut j = i + 1;
                while j < 4 && self.data[r][j] == 0 {
                    j += 1;
                }
                if j == 4 {
                    break;
                }
                moved = true;
                self.data[r][i] = self.data[r][j];
                self.data[r][j] = 0;
                i = j;
            }
        }
        moved
    }

    fn push_right(&mut self) -> (u32, bool) {
        self.swap_lr();
        let result = self.push_left();
        self.swap_lr();
        result
    }

    fn push_up(&mut self) -> (u32, bool) {
        self.swap_diagnol();
        let result = self.push_left();
        self.swap_diagnol();
        result
    }

    fn push_down(&mut self) -> (u32, bool) {
        self.swap_ud();
        let result = self.push_up();
        self.swap_ud();
        result
    }

    fn swap_lr(&mut self) {
        for r in 0..4 {
            for c in 0..2 {
                self.data[r].swap(c, 3-c);
            }
        }
    }

    fn swap_diagnol(&mut self) {
        for r in 0..4 {
            for c in (r + 1)..4 {
                let tmp = self.data[r][c];
                self.data[r][c] = self.data[c][r];
                self.data[c][r] = tmp;
            }
        }
    }

    fn swap_ud(&mut self) {
        for r in 0..2 {
            for c in 0..4 {
                let tmp = self.data[r][c];
                self.data[r][c] = self.data[3 - r][c];
                self.data[3 - r][c] = tmp;
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

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char('l') | Event::Key(Key::Left) => self.push(LRUD::Left),
            Event::Char('r') | Event::Key(Key::Right) => self.push(LRUD::Right),
            Event::Char('u') | Event::Key(Key::Up) => self.push(LRUD::Up),
            Event::Char('d') | Event::Key(Key::Down) => self.push(LRUD::Down),
            _ => EventResult::Ignored,
        }
    }

    fn take_focus(
        &mut self,
        _source: cursive::direction::Direction,
    ) -> Result<EventResult, cursive::view::CannotFocus> {
        Ok(EventResult::Consumed(None))
    }
}
