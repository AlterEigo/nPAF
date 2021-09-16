use gtk::prelude::*;

use crate::{
    prelude::*,
};

pub struct PersonEditorView {
}

impl PersonEditorView {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for PersonEditorView {
    fn assemble(&self) -> gtk::Widget {
        let gbuilder = gtk::Builder::from_resource("/org/altereigo/npaf/PersonEditor.glade");
        let root: gtk::Grid = gbuilder.object("root").unwrap();
        root.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
