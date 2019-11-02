mod ui_certlist;
mod ui_viewcert;
use crate::database;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::ApplicationWindow;

const MAIN_SPAICING: i32 = 5;
const APPLICATION_NAME: &str = "Certificator";

pub struct CertificatorGUI {
    pub window: ApplicationWindow,
    pub menu: Menu,
    pub lists: Lists,
    pub view: View,
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

        let cert_view = View::new();
        let menu = Menu::new();
        let lists = Lists::new() ;
        main_layout.add(&menu.container);
        main_layout.add(&lists.container);
        main_layout.add(&cert_view.container);

        window.add(&main_layout);

        let t = cert_view.view.clone();

        lists.cert_list.connect_cursor_changed(move |treeview| {
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
        window.show_all();
        CertificatorGUI {
            window,
            lists,
            view: cert_view,
            menu,
        }
    }
}

pub struct Menu {
    container: gtk::Box
}


impl Menu {
    fn new() -> Menu {
            let lists_layout = gtk::Box::new(gtk::Orientation::Horizontal, MAIN_SPAICING);
    lists_layout.set_halign(gtk::Align::Fill);
    let import_button = gtk::Button::new_with_mnemonic("_Import");
    lists_layout.add(&import_button);
    import_button.connect_clicked(move |_| show_import_window());

        Menu{
            container: lists_layout
        }
    }
}
pub struct Lists {
    pub container: gtk::Box,
    pub cert_list: gtk::TreeView,
    pub csr_list: gtk::TreeView,
    pub keys_list: gtk::TreeView,
}



impl Lists{
    fn new() -> Lists {
        let lists_layout = gtk::Box::new(gtk::Orientation::Horizontal, MAIN_SPAICING);
        lists_layout.set_halign(gtk::Align::Fill);
        lists_layout.set_valign(gtk::Align::Fill);

        let cert_list = ui_certlist::cert_list();
        let keys_list = ui_certlist::cert_list();
        let csr_list = ui_certlist::cert_list();
        lists_layout.add(&ui_certlist::wrap("CSRs", &csr_list));
        lists_layout.add(&ui_certlist::wrap("Certificates", &cert_list));
        lists_layout.add(&ui_certlist::wrap("Keys", &keys_list));

        Lists {
            container: lists_layout,
            cert_list: cert_list,
            csr_list: csr_list,
            keys_list: keys_list
        }
    }
}
pub struct View {
    container: gtk::Box,
    view: gtk::TextView,
}


impl View {
    fn new() -> View {
        let container =  gtk::Box::new(gtk::Orientation::Horizontal, MAIN_SPAICING);
        let view = ui_viewcert::view_cert();
        container.add(&view);
        View {
            container,
            view
        }
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
    println!("ololo")
}
