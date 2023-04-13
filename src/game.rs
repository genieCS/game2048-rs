use crate::board::Board;
use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, DummyView, LinearLayout, TextView},
    Cursive,
    CursiveExt,
};

pub fn run() {
    let mut siv: Cursive = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let board = Board::new().with_name("game_2048");

    let score_view = LinearLayout::vertical()
    .child(TextView::new("SCORE"))
    .child(DummyView)
    .child(TextView::new("0").with_name("score"))
    .fixed_size((10, 5));

    let view = Dialog::around(
        LinearLayout::horizontal()
        .child(board)
        .child(DummyView)
        .child(score_view))
        .title("GAME2048");

    siv.add_layer(view);

    siv.run();
}
