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
        let grid = gtk::Grid::builder().build();
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
