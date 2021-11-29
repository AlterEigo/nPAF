mod app;
mod person_editor;
mod prelude;
mod root;
mod gedcom;

use crate::prelude::*;
use gtk::prelude::*;
use std::rc::Rc;
use gedcom::*;

fn main() {
    let mut parser: GedParser = Default::default();
    let data = std::fs::File::open("data_sample.ged").unwrap();
    let unparsed = parser.count_unparsed(&data);
    let data = std::fs::File::open("data_sample.ged").unwrap();
    if unparsed > 0 {
        println!("Did not parse exactly {} lines.", unparsed);
    }
    let res = parser.parse(&data);
    if let Ok(parsed) = res {
        println!("{:#?}", parsed);
    } else {
        println!("An error occured: '{:#?}'", res);
    }
}

fn main2() {
    gtk::init().expect("Could not initialize GTK");

    let tm = app::Application::builder().build().unwrap();

    tm.run();
}
