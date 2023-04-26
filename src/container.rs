use crate::board::Board;
use crate::history::History;
use crate::lrud::LRUD;
use crate::score::Score;

use cursive::{
    event::{Callback, Event, EventResult, Key},
    theme::{BaseColor, Color, ColorStyle},
    views::Dialog,
    Printer, Vec2, View,
};


pub struct Container {
    pub board: Board,
    pub history: History,
    pub score: Score,
    board_size: Vec2,
    history_size: Vec2,
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Container {
    pub fn new() -> Self {
        let background_color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(188, 173, 159));
        let main_color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Light(BaseColor::Yellow));

        let mut board = Board::new(background_color);
        let mut history = History::new(background_color, main_color, 12);
        let score = Score::new(background_color, main_color);
        let board_size = board.required_size(cursive::Vec2::new(0, 0));
        let history_size = history.required_size(cursive::Vec2::new(0, 0));
        Self {
            board,
            history,
            score,
            board_size,
            history_size,
        }
    }

    pub fn new_game(&mut self) {
        self.board.reset();
        self.history.clear();
        self.score.reset();
    }

    fn push(&mut self, lrud: LRUD) -> EventResult {
        let (score, moved, over) = self.board.push(lrud);
        if moved {
            self.history.update(lrud);
            self.score.add_score(score);
        }
        EventResult::Consumed(
            if over
            {
                Some(
                    Callback::from_fn(move |s|{ 
                        s.add_layer(Dialog::info("Game Over! Press 'n' for new game."))
                    }))
            } else { 
                None 
            })
    }
}

impl View for Container {
    fn draw(&self, printer: &Printer) {
        let x_padding = 1;
        let y_padding = 1;
        let board_padding = cursive::Vec2::new(x_padding, y_padding);
        let history_padding = cursive::Vec2::new(board_padding.x + self.board_size.x, y_padding);
        let score_padding = cursive::Vec2::new(history_padding.x + self.history_size.x, y_padding);

        let board_printer = printer.offset(board_padding);
        let history_printer = printer.offset(history_padding);
        let score_printer = printer.offset(score_padding);

        self.board.draw(&board_printer);
        self.history.draw(&history_printer);
        self.score.draw(&score_printer);
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        let board = self.board.required_size(_constraint);
        let history = self.history.required_size(_constraint);
        let score = self.score.required_size(_constraint);
        cursive::Vec2::new(board.x + history.x + score.x + 5, std::cmp::max(board.y, std::cmp::max(history.y, score.y)))
    }

    fn on_event(&mut self, event: cursive::event::Event) -> cursive::event::EventResult {
        match event {
            Event::Char('l') | Event::Key(Key::Left) => self.push(LRUD::Left),
            Event::Char('r') | Event::Key(Key::Right) => self.push(LRUD::Right),
            Event::Char('u') | Event::Key(Key::Up) => self.push(LRUD::Up),
            Event::Char('d') | Event::Key(Key::Down) => self.push(LRUD::Down),
            Event::Char('n') => {
                self.new_game();
                EventResult::Consumed(None)
            }
            _ => EventResult::Ignored,
        }
    }
}