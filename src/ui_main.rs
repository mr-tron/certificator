mod import_window;
mod ui_certlist;
mod ui_menu;
mod ui_viewcert;
use crate::database;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::ApplicationWindow;

const MAIN_SPAICING: i32 = 5;
const APPLICATION_NAME: &str = "Certificator";

pub struct CertificatorGUI {
    pub window: ApplicationWindow,
    pub menu: ui_menu::Menu,
    pub lists: ui_certlist::Lists,
    pub view: ui_viewcert::View,
}

impl CertificatorGUI {
    pub fn new(application: &gtk::Application) -> CertificatorGUI {
        let window = ApplicationWindow::new(application);
        window.set_title(APPLICATION_NAME);
        window.set_default_size(1024, 768);
        window.set_border_width(MAIN_SPAICING as u32);
        window.set_position(gtk::WindowPosition::Center);

        let main_layout = gtk::Box::new(gtk::Orientation::Vertical, MAIN_SPAICING);
        main_layout.set_border_width(5);

        let cert_view = ui_viewcert::View::new();
        let menu = ui_menu::Menu::new();
        let lists = ui_certlist::Lists::new();
        main_layout.add(&menu.container);
        main_layout.add(&lists.container);
        main_layout.add(&cert_view.container);

        window.add(&main_layout);
        window.show_all();

        let gui = CertificatorGUI {
            window,
            lists,
            view: cert_view,
            menu,
        };
        gui.set_callbacks();
        gui
    }
    fn set_callbacks(&self) {
        let t = self.view.view.clone();

        self.lists
            .cert_list
            .connect_cursor_changed(move |treeview| {
                let selection = treeview.get_selection();
                if let Some((model, iter)) = selection.get_selected() {
                    let thumbprint = model
                        .get_value(&iter, 1)
                        .get::<String>()
                        .expect("Couldn't get string value");
                    let cert_repr = get_certificate_representation(thumbprint);
                    display_cert(cert_repr, &t)
                }
            });
        self.menu
            .import_button
            .connect_clicked(move |_| show_import_window());
    }
}

fn display_cert(cert: String, cert_view: &gtk::TextView) {
    cert_view
        .get_buffer()
        .expect("Couldn't get window")
        .set_text(&cert[..]);
}

fn get_certificate_representation(thumbprint: String) -> String {
    let db = database::DB::new().unwrap();
    let cert = db.get_cert(thumbprint).unwrap();
    return cert.repr();
}

fn show_import_window() {
    let _window = import_window::ImportWindow::new();
}
