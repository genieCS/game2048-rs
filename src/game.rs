use crate::board::Board;
use crate::history::History;
use cursive::{
    view::{Nameable, Selector},
    views::{Button, Dialog, DummyView, LinearLayout, TextView},
    Cursive, CursiveExt,
};

pub fn run() {
    let mut siv: Cursive = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let board = Board::new().with_name("game_2048");

    let shortcut = LinearLayout::vertical()
    .child(TextView::new("SHORTCUT"))
    .child(TextView::new("↑, u: up"))
    .child(TextView::new("↓, d: down"))
    .child(TextView::new("←, l: left"))
    .child(TextView::new("→, r: right"))
    .child(TextView::new("n: new game"))
    .child(TextView::new("q: quit"))
    .child(TextView::new("t: swap theme"));

    let manual_view = LinearLayout::vertical()
        .child(TextView::new("SCORE"))
        .child(TextView::new("0").with_name("score"))
        .child(DummyView)
        .child(Button::new("New Game", new_game))
        .child(DummyView)
        .child(shortcut);

    siv.add_global_callback('n', new_game);
    siv.add_global_callback('t', |s| {
        s.call_on_name("game_2048", |board: &mut Board| {
            board.swap_theme();
        });
    });

    let history = History::new().with_name("history");
    let history = LinearLayout::vertical()
    .child(TextView::new("HISTORY"))
    .child(history);

    let view = Dialog::around(
        LinearLayout::horizontal()
            .child(board)
            .child(DummyView)
            .child(history)
            .child(DummyView)
            .child(manual_view),
    )
    .title("GAME2048");

    siv.add_layer(view);

    siv.run();
}

fn new_game(s: &mut Cursive) {
    s.call_on_name("score", |view: &mut TextView| {
        view.set_content("0");
    });
    s.call_on_name("game_2048", |board: &mut Board| {
        board.restart();
    });
    s.call_on_name("history", |history: &mut History| {
        history.clear();
    });
    let game = "game_2048";
    s.focus(&Selector::Name(game)).unwrap();   
}