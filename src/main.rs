mod app;
mod person_editor;
mod prelude;
mod root;
mod gedcom;

use crate::prelude::*;
use gtk::prelude::*;
use std::rc::Rc;

fn main() {
    gtk::init().expect("Could not initialize GTK");

    let tm = app::Application::builder().build().unwrap();

    tm.run();
}
