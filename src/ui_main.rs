mod ui_certlist;
mod ui_viewcert;
use crate::database;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::ApplicationWindow;

const MAIN_SPAICING: i32 = 5;
const APPLICATION_NAME: &str = "Certificator";

pub fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    window.set_title(APPLICATION_NAME);
    window.set_default_size(1024, 768);
    window.set_border_width(MAIN_SPAICING as u32);
    window.set_position(gtk::WindowPosition::Center);

    let main_layout = gtk::Box::new(gtk::Orientation::Vertical, MAIN_SPAICING);
    main_layout.set_border_width(5);
    let lists_layout = gtk::Box::new(gtk::Orientation::Horizontal, MAIN_SPAICING);
    lists_layout.set_halign(gtk::Align::Fill);
    lists_layout.set_valign(gtk::Align::Fill);

    let cert_view = ui_viewcert::view_cert();

    let cert_list = ui_certlist::cert_list();
    let ca_list = ui_certlist::cert_list();
    let csr_list = ui_certlist::cert_list();

    let menu = make_menu();

    window.add(&main_layout);
    main_layout.add(&menu);
    main_layout.add(&lists_layout);
    lists_layout.add(&ui_certlist::wrap("CSRs", &csr_list));
    lists_layout.add(&ui_certlist::wrap("Certificates", &cert_list));
    lists_layout.add(&ui_certlist::wrap("Keys", &ca_list));
    main_layout.add(&cert_view);

    cert_list.connect_cursor_changed(move |treeview| {
        let selection = treeview.get_selection();
        if let Some((model, iter)) = selection.get_selected() {
            let thumbprint = model
                .get_value(&iter, 1)
                .get::<String>()
                .expect("Couldn't get string value");
            let cert_repr = get_certificate_representation(thumbprint);
            display_cert(cert_repr, &cert_view)
        }
    });

    window.show_all();
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


fn make_menu() -> gtk::Box {
    let lists_layout = gtk::Box::new(gtk::Orientation::Horizontal, MAIN_SPAICING);
    lists_layout.set_halign(gtk::Align::Fill);
    let import_button = gtk::Button::new_with_mnemonic("_Import") ;
    lists_layout.add(&import_button);
    import_button.connect_clicked(move |_| {
        show_import_window()
    });
    return lists_layout
}

fn show_import_window() {
    println!("ololo")
}