use cursive::{
    event::{Event, EventResult},
    theme::{Color, ColorStyle},
    Printer, View, XY,
};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
enum LRUD {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Board {
    pub data: [[u32; 4]; 4],
    pub num2color: HashMap<u32, ColorStyle>,
}

impl Board {
    pub fn new() -> Self {
        let data: [[u32; 4]; 4] = [[0; 4]; 4];
        let num2color = Self::init_num2color();
        Self { data, num2color }
    }

    pub fn init_num2color() -> HashMap<u32, ColorStyle> {
        let mut map = HashMap::new();
        map.insert(
            0,
            ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 0, 0)),
        ); // Red
        map.insert(
            2,
            ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 165, 0)),
        ); // Orange
        map.insert(
            4,
            ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 255, 0)),
        ); // Yellow
        map.insert(
            8,
            ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(0, 255, 0)),
        ); // Green
        map.insert(
            16,
            ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(0, 255, 255)),
        ); // Cyan
        map.insert(
            32,
            ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(0, 0, 255)),
        ); // Blue
        map.insert(
            64,
            ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(128, 0, 128)),
        ); // Purple
        map.insert(
            128,
            ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 0, 255)),
        ); // Magenta
        map.insert(
            256,
            ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(128, 128, 128)),
        ); // Gray
        map.insert(
            512,
            ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 255, 255)),
        ); // White
        map.insert(
            1024,
            ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 192, 203)),
        ); // Pink
        map.insert(
            2048,
            ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 0, 128)),
        ); // Rose
        map
    }

    fn draw_background(&self, printer: &Printer) {
        let background_style = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(188, 173, 159));
        for i in (0..4).into_iter() {
            printer.with_color(background_style, |printer| {
                printer.print((0, 4 * i), "o------o------o------o------o");
                for j in (0..5).into_iter() {
                    printer.print((7 * j, 4 * i + 1), "|");
                    printer.print((7 * j, 4 * i + 2), "|");
                    printer.print((7 * j, 4 * i + 3), "|");
                }
            });
        }
        printer.with_color(background_style, |printer| {
            printer.print(XY::new(0, 16), "o------o------o------o------o");
        });
    }

    fn draw_board(&self, printer: &Printer) {
        for i in (0..4).into_iter() {
            for j in (0..4).into_iter() {
                let num = self.data[i][j];
                let color_style = self.num2color.get(&num).unwrap();
                let num_str = &num.to_string();
                let num = if num == 0 { "" } else { num_str };
                printer.with_color(*color_style, |printer| {
                    printer.print((7 * j + 1, 4 * i + 1), "      ");
                    printer.print((7 * j + 1, 4 * i + 2), &format!(" {:>4} ", num));
                    printer.print((7 * j + 1, 4 * i + 3), "      ");
                });
            }
        }
    }
    pub fn insert(&mut self) {
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

    pub fn is_full(&self) -> bool {
        for i in (0..4).into_iter() {
            for j in (0..4).into_iter() {
                if self.data[i][j] == 0 {
                    return false;
                }
            }
        }
        true
    }

    fn push(&mut self, lrud: LRUD) {
        match lrud {
            LRUD::Left => self.push_left(),
            LRUD::Right=> self.push_right(),
            LRUD::Up => self.push_up(),
            LRUD::Down => self.push_down(),
        }
        if self.is_full() {
            println!("Game Over!");
            return
        }
        self.insert();
    }

    fn push_left(&mut self) {
        for r in (0..4).into_iter() {
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
                    self.data[r][j] = 0;
                    i = j + 1;
                } else {
                    i = j;
                }
            }

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
                self.data[r][i] = self.data[r][j];
                self.data[r][j] = 0;
                i = j + 1;
            }
        }
    }

    fn push_right(&mut self) {
        self.swap_lr();
        self.push_left();
        self.swap_lr();
    }

    fn push_up(&mut self) {
        self.swap_diagnol();
        self.push_left();
        self.swap_diagnol();
    }

    fn push_down(&mut self) {
        self.swap_ud();
        self.push_up();
        self.swap_ud();
    }

    fn swap_lr(&mut self) {
        for r in (0..4).into_iter() {
            for c in (0..2).into_iter() {
                let tmp = self.data[r][c];
                self.data[r][c] = self.data[r][3 - c];
                self.data[r][3 - c] = tmp;
            }
        }
    }

    fn swap_diagnol(&mut self) {
        for r in (0..4).into_iter() {
            for c in (r+1..4).into_iter() {
                let tmp = self.data[r][c];
                self.data[r][c] = self.data[c][r];
                self.data[c][r] = tmp;
            }
        }
    }

    fn swap_ud(&mut self) {
        for r in (0..2).into_iter() {
            for c in (0..4).into_iter() {
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
        cursive::Vec2::new(100, 100)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char('l') => {
                self.push(LRUD::Left);
                EventResult::Consumed(None)
            },
            Event::Char('r') => {
                self.push(LRUD::Right);
                EventResult::Consumed(None)
            },
            Event::Char('u') => {
                self.push(LRUD::Up);
                EventResult::Consumed(None)
            },
            Event::Char('d') => {
                self.push(LRUD::Down);
                EventResult::Consumed(None)
            },
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
