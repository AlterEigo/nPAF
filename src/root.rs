use gtk::prelude::*;

use crate::{person_editor::PersonEditorView, prelude::*};
use std::rc::Rc;

pub enum MenuBarButton {
    Minimize,
    Maximize,
    Close,
}

#[derive(Default)]
pub struct MenuBarView {
    gbuilder: gtk::Builder,
}

impl MenuBarView {
    pub fn new() -> Self {
        Self {
            gbuilder: gtk::Builder::from_resource("/org/altereigo/npaf/MenuBar.glade"),
            ..Default::default()
        }
    }

    pub fn button(&self, name: MenuBarButton) -> gtk::Button {
        let getter = |name| -> gtk::Button { self.gbuilder.object(name).unwrap() };
        match name {
            MenuBarButton::Close => getter("b_close"),
            MenuBarButton::Minimize => getter("b_minimize"),
            MenuBarButton::Maximize => getter("b_maximize"),
        }
    }
}

impl View for MenuBarView {
    fn assemble(&self) -> gtk::Widget {
        let grid: gtk::Grid = self.gbuilder.object("root").unwrap();
        grid.show();
        grid.dynamic_cast::<gtk::Widget>().unwrap()
    }
}

#[derive(Default)]
pub struct ToolBarView {
    gbuilder: gtk::Builder,
}

impl ToolBarView {
    pub fn new() -> Self {
        Self {
            gbuilder: gtk::Builder::from_resource("/org/altereigo/npaf/Toolbar.glade"),
            ..Default::default()
        }
    }

    pub fn on_person_edit<CallbackT: Fn(&gtk::Button) + 'static>(&self, cb: CallbackT) {
        let btn: gtk::Button = self.gbuilder.object("b_add_person").unwrap();
        btn.connect_clicked(cb);
    }
}

impl View for ToolBarView {
    fn assemble(&self) -> gtk::Widget {
        let root: gtk::Grid = self.gbuilder.object("root").unwrap();
        root.show();
        root.dynamic_cast::<gtk::Widget>().unwrap()
    }
}

pub struct RootView {
    gbuilder: gtk::Builder,
    menubar: MenuBarView,
}

impl RootView {
    pub fn new() -> Self {
        RootView {
            gbuilder: gtk::Builder::from_resource("/org/altereigo/npaf/Root.glade"),
            menubar: MenuBarView::new(),
        }
    }

    pub fn on_window_close<CallbackT: Fn(&gtk::Button) + 'static>(&self, f: CallbackT) {
        let btn: gtk::Button = self.menubar.button(MenuBarButton::Close);
        btn.connect_clicked(f);
    }
}

impl View for RootView {
    fn assemble(&self) -> gtk::Widget {
        let root: gtk::Grid = self.gbuilder.object("root").unwrap();
        let (p_menubar, p_toolbar, p_workspace) = (
            self.gbuilder.object::<gtk::Grid>("p_menu_bar").unwrap(),
            self.gbuilder.object::<gtk::Grid>("p_tool_bar").unwrap(),
            self.gbuilder.object::<gtk::Grid>("p_workspace").unwrap(),
        );
        let menubar = self.menubar.assemble();
        p_menubar.attach(&menubar, 0, 0, 1, 1);
        let toolbar = ToolBarView::new();
        toolbar.on_person_edit(|_| {
            let editor = PersonEditorView::new();
            let window = editor.assemble_window();
            window.present();
        });
        let toolbar = toolbar.assemble();
        p_toolbar.attach(&toolbar, 0, 0, 1, 1);
        root.set_row_homogeneous(false);
        root.show();
        root.dynamic_cast::<gtk::Widget>().unwrap()
    }
}
