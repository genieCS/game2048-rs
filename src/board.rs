use cursive::{
    event::{Event, EventResult},
    theme::{ Color, ColorStyle},
    View,
    Printer, XY,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    pub data: [[u32; 4]; 4],
    pub num2color: HashMap<u32, ColorStyle>,
}

impl Board {
    pub fn new() -> Self {
        let data: [[u32; 4]; 4] = [[0; 4]; 4];
        let num2color = Self::init_num2color();
        Self {
            data,
            num2color,
        }
    }

    pub fn init_num2color() -> HashMap<u32, ColorStyle> {
        let mut map = HashMap::new();
        map.insert(0, ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 0, 0)));  // Red
        map.insert(2, ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 165, 0)));   // Orange
        map.insert(4, ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 255, 0)));    // Yellow
        map.insert(8, ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(0, 255, 0)));      // Green
        map.insert(16, ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(0, 255, 255)));    // Cyan
        map.insert(32, ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(0, 0, 255)));      // Blue
        map.insert(64, ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(128, 0, 128)));    // Purple
        map.insert(128, ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 0, 255)));    // Magenta
        map.insert(256, ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(128, 128, 128)));  // Gray
        map.insert(512, ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 255, 255)));  // White
        map.insert(1024, ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 192, 203)));  // Pink
        map.insert(2048, ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 0, 128)));  // Rose
        map
    }

    fn draw_background(&self, printer: &Printer) {
        let background_style = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(188, 173, 159));
        for i in (0..4)
        .into_iter()
        {
            printer.with_color(background_style, |printer| {
                printer.print((0,4*i), "o------o------o------o------o");
                for j in (0..4)
                .into_iter()
                {
                    printer.print((7*j,4*i+1), "|");
                    printer.print((7*j,4*i+2), "|");
                    printer.print((7*j,4*i+3), "|");

                }
                printer.print((28,4*i+1), "|");
                printer.print((28,4*i+2), "|");
                printer.print((28,4*i+3), "|");              
            });
        };
        printer.with_color(background_style, |printer| {
            printer.print(XY::new(0,16), "o------o------o------o------o");
        });
    }

    fn fill_board(&self, printer: &Printer) {
        for i in (0..4)
        .into_iter()
        {
            for j in (0..4)
            .into_iter()
            {
                let num = self.data[i][j];
                let color_style = self.num2color.get(&num).unwrap();
                let num_str = &num.to_string();
                let num = if num == 0 {""} else { num_str };
                printer.with_color(*color_style, |printer| {
                    printer.print((7*j+1,4*i+1), "      ");
                    printer.print((7*j+1,4*i+2), &format!(" {:>4} ", num));
                    printer.print((7*j+1,4*i+3), "      ");

                });
            }
        }
    }
}

impl View for Board {
    fn draw(&self, printer: &Printer) {
        self.draw_background(printer);
        self.fill_board(printer);
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
