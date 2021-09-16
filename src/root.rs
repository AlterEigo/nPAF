use gtk::prelude::*;

use crate::{
    prelude::*,
};
use std::rc::Rc;

#[derive(Default)]
pub struct ClickEmitter {
    callbacks: Vec<Box<dyn Fn(())>>
}

impl ClickEmitter {
    pub fn emit(&self) {
        for it in self.callbacks.iter() {
            (it)(());
        }
    }
}

impl EventEmitter<()> for ClickEmitter {
    fn subscribe<T: Fn(()) + 'static>(&mut self, cb: T)
    {
        self.callbacks.push(Box::new(cb));
    }
}

#[derive(Default)]
pub struct MenuBarView {
}

impl MenuBarView {
    pub fn new() -> Self {
        Default::default()
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

#[derive(Default)]
pub struct ToolBarView {
    pub add_person_evt: Rc<ClickEmitter>
}

impl ToolBarView {
    pub fn new() -> Self {
        Default::default()
    }
}

impl View for ToolBarView {
    fn assemble(&self) -> gtk::Widget {
        let gbuilder = gtk::Builder::from_resource("/org/altereigo/npaf/Toolbar.glade");
        let root: gtk::Grid = gbuilder.object("root").unwrap();
        let btn: gtk::Button = gbuilder.object("b_add_person").unwrap();
        let emitter = self.add_person_evt.clone();
        btn.connect_clicked(move |_| {
            emitter.emit();
        });
        root.show();
        root.dynamic_cast::<gtk::Widget>().unwrap()
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
        let root: gtk::Grid = gbuilder.object("root").unwrap();
        let (p_menubar, p_toolbar, p_workspace) = (
            gbuilder.object::<gtk::Grid>("p_menu_bar").unwrap(),
            gbuilder.object::<gtk::Grid>("p_tool_bar").unwrap(),
            gbuilder.object::<gtk::Grid>("p_workspace").unwrap(),
        );
        let menubar = MenuBarView::new().assemble();
        p_menubar.attach(&menubar, 0, 0, 1, 1);
        let toolbar = ToolBarView::new().assemble();
        p_toolbar.attach(&toolbar, 0, 0, 1, 1);
        root.set_row_homogeneous(false);
        root.show();
        root.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
