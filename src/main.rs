extern crate gio;
extern crate gtk;
mod database;
mod models;
mod ui_main;
use gio::prelude::*;
use gtk::Application;

const APPLICATION_ID: &str = "in.subbot.certificator";

fn main() {
    let application = Application::new(Some(APPLICATION_ID), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_startup(|app| {
        let _gui = ui_main::CertificatorGUI::new(app);
        return;
    });

    application.connect_activate(|_| {});

    application.run(&[]);
}
