use crate::board::Board;
use cursive::{
    view::{Nameable, Resizable, Selector},
    views::{Dialog, DummyView, LinearLayout, TextView, Button},
    Cursive,
    CursiveExt,
    XY,
};

pub fn run() {
    let mut siv: Cursive = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let board = Board::new().with_name("game_2048");

    let score_view = LinearLayout::vertical()
    .child(TextView::new("SCORE"))
    .child(TextView::new("0").with_name("score"))
    .child(DummyView)
    .child(Button::new("New Game", |s| {
        s.call_on_name("score", |view: &mut TextView| {
            view.set_content("0");
        });
        s.call_on_name("game_2048", |board: &mut Board| {
            board.restart();
        });
        let game = "game_2048";
        s.focus(&Selector::Name(game));
    }))
    .fixed_size(XY::new(10, 5));

    let view = Dialog::around(
        LinearLayout::horizontal()
        .child(board)
        .child(DummyView)
        .child(score_view))
        .title("GAME2048");

    siv.add_layer(view);

    siv.run();
}
