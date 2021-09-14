use gtk::prelude::*;

use crate::prelude::View;
use std::rc::Rc;

pub struct MenuBarView {

}

impl MenuBarView {
    pub fn new() -> Self {
        MenuBarView {}
    }
}

impl View for MenuBarView {
    fn assemble(&self) -> gtk::Widget {
        let gbuilder = gtk::Builder::from_resource("/org/altereigo/npaf/MenuBar.glade");
        let grid: gtk::Grid = gbuilder.object("root").unwrap();
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}

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
        let (p_menubar, p_toolbar, p_workspace) = (
            gbuilder.object::<gtk::Grid>("p_menu_bar").unwrap(),
            gbuilder.object::<gtk::Grid>("p_tool_bar").unwrap(),
            gbuilder.object::<gtk::Grid>("p_workspace").unwrap(),
        );
        let menubar = MenuBarView::new().assemble();
        p_menubar.attach(&menubar, 0, 0, 1, 1);
        let grid: gtk::Grid = gbuilder.object("root").unwrap();
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
