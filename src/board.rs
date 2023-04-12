use cursive::{
    event::{Event, EventResult},
    View,
    Printer,
};

#[derive(Debug)]
pub struct Board {
    pub data: [[u32; 4]; 4],
}

impl Board {
    pub fn new() -> Self {
        let board: [[u32; 4]; 4] = [[0; 4]; 4];
        Self {
            data: board,
        }
    }
}

impl View for Board {
    fn draw(&self, printer: &Printer) {
        for (i, _) in (1..5)
        .into_iter()
        .enumerate()
        {
            printer.print((0,4*i), "o----o----o----o----o");
            printer.print((0,4*i+1), "|    |    |    |    |");
            printer.print((0,4*i+2), "|    |    |    |    |");
            printer.print((0,4*i+3), "|    |    |    |    |");

        }

        printer.print((0,16), "o----o----o----o----o");
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(100, 100)
    }

    fn on_event(&mut self, _: Event) -> EventResult {
        EventResult::Consumed(None)
    }

    fn take_focus(
            &mut self,
            _source: cursive::direction::Direction,
        ) -> Result<EventResult, cursive::view::CannotFocus> {
        Ok(EventResult::Consumed(None))
    }
}
