use gtk::prelude::*;

use crate::{
    prelude::*,
};

pub struct PersonEditorView {
    gbuilder: gtk::Builder
}

impl PersonEditorView {
    pub fn new() -> Self {
        Self {
            gbuilder: gtk::Builder::from_resource("/org/altereigo/npaf/PersonEditor.glade")
        }
    }
}

impl View for PersonEditorView {
    fn assemble(&self) -> gtk::Widget {
        let root: gtk::Grid = self.gbuilder.object("root").unwrap();
        root.dynamic_cast::<gtk::Widget>().unwrap()
    }
}

impl Windowed for PersonEditorView {
    fn assemble_window(&self) -> gtk::Window {
        let wdg = self.assemble();
        let wdw: gtk::Window = self.gbuilder.object("top").unwrap();
        wdw.set_child(Some(&wdg));
        wdw
    }
}
