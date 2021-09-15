//!
//! TaskManager's root
//!

use gio::prelude::*;
use gtk::prelude::*;

use crate::prelude::*;
use crate::root::RootView;
use std::rc::Rc;

pub struct Application {
    gtk_app: gtk::Application,
}

#[derive(Default,Clone)]
pub struct ApplicationBuilder {
}

impl ApplicationBuilder {
    pub fn build(self) -> Result<Application> {
        Ok(Application {
            gtk_app: gtk::Application::builder()
                .application_id("org.altereigo.ae-task-manager")
                .build(),
        })
    }
}

impl Application {
    pub fn builder() -> ApplicationBuilder {
        Default::default()
    }

    fn assemble_root(&self) -> gtk::Widget {
        let mut view = RootView::new();
        view.assemble()
    }

    fn init_window(&self) -> gtk::ApplicationWindow {
        let gbuilder = gtk::Builder::from_resource("/org/altereigo/npaf/AppWindow.glade");
        gbuilder.object("root").unwrap()
    }

    pub fn run(&self) -> i32 {
        Application::load_resources();
        let root = self.assemble_root();
        let window = self.init_window();
        window.set_child(Some(&root));
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_resource("/org/altereigo/npaf/style.css");
        gtk::StyleContext::add_provider_for_screen(
            &window.screen().unwrap(),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        self.gtk_app.connect_activate(move |app| {
            window.set_application(Some(app));
            window.present();
        });
        self.gtk_app.run()
    }

    fn load_resources() {
        let bytes = include_bytes!("../../resources/resources.gresource");
        let resource_data = glib::Bytes::from(&bytes[..]);
        let res = gio::Resource::from_data(&resource_data).unwrap();
        gio::resources_register(&res);
    }
}
