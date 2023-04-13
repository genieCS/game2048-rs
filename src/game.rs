use crate::board::Board;
use cursive::{
    view::Nameable,
    views::Dialog,
    Cursive,
    CursiveExt,
};

pub fn run() {
    let mut siv: Cursive = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let board = Board::new().with_name("game_2048");

    let view = Dialog::around(board)
        .title("GAME2048");

    siv.add_layer(view);

    siv.run();
}
