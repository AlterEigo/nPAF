use gtk::prelude::*;

use crate::prelude::View;
use std::rc::Rc;

pub struct RootView {
}

impl RootView {
    pub fn new() -> Self {
        RootView {}
    }
}

impl View for RootView {
    fn assemble(&self) -> gtk::Widget {
        let gbuilder = gtk::Builder::from_resource("/org/altereigo/npaf/Root.glade");
        let grid: gtk::Grid = gbuilder.object("root").unwrap();
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
