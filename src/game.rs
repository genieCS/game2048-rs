use crate::board::Board;
use cursive::{
    view::Nameable,
    views::{Dialog, LinearLayout},
    Cursive,
    CursiveExt,
};

pub fn run() {
    let mut siv: Cursive = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let mut board: Board = Board::new();   
    board.insert();
    board.insert();

    let view = Dialog::around(
        LinearLayout::vertical()
            .child(board.with_name("game_2048")),
    )
    .title("GAME2048");

    siv.add_layer(view);

    siv.run();
}
