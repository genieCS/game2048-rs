use crate::backend::backend;
use crate::container::Container;

use cursive::{
    view::{Nameable, Selector},
    Cursive,
};

pub fn run() {
    let mut siv: Cursive = Cursive::new();
    siv.add_global_callback('q', |s| s.quit());
    
    let container = Container::new().with_name("container");
    siv.add_layer(container);
    siv.focus(&Selector::Name("container")).unwrap();
    
    let siv = std::sync::Mutex::new(siv);
    siv.lock().unwrap().run_with(|| backend());
}
