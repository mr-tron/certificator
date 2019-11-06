use crate::database;
use gtk::prelude::*;
use std::rc::Rc;

const MAIN_SPAICING: i32 = 8;

pub struct Lists {
    pub container: gtk::Box,
    pub cert_list: gtk::TreeView,
    pub csr_list: gtk::TreeView,
    pub keys_list: gtk::TreeView,
}

impl Lists {
    pub fn new() -> Lists {
        let lists_layout = gtk::Box::new(gtk::Orientation::Horizontal, MAIN_SPAICING);
        lists_layout.set_halign(gtk::Align::Fill);
        lists_layout.set_valign(gtk::Align::Fill);

        let certs_model = Rc::new(create_certs_model());
        let csr_model = Rc::new(create_csrs_model());
        let keys_model = Rc::new(create_keys_model());
        let cert_list = list(certs_model);
        let keys_list = list(keys_model);
        let csr_list = list(csr_model);
            add_certs_columns(&cert_list);
            add_csrs_columns(&csr_list);
            add_keys_columns(&keys_list);
        lists_layout.add(&wrap("CSRs", &csr_list));
        lists_layout.add(&wrap("Certificates", &cert_list));
        lists_layout.add(&wrap("Keys", &keys_list));

        Lists {
            container: lists_layout,
            cert_list: cert_list,
            csr_list: csr_list,
            keys_list: keys_list,
        }
    }
}

fn list(model: Rc<gtk::ListStore>) -> gtk::TreeView {
    let treeview = gtk::TreeView::new_with_model(&*model);
    treeview.set_vexpand(true);
    treeview
}

fn wrap(name: &str, list: &gtk::TreeView) -> gtk::Box {
    let scroll = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scroll.set_hexpand(true);
    scroll.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, MAIN_SPAICING);
    let label = gtk::Label::new(Some(name));
    scroll.add(list);
    vbox.add(&label);
    vbox.add(&scroll);
    vbox
}

fn create_certs_model() -> gtk::ListStore {
    let col_types: [gtk::Type; 2] = [gtk::Type::String, gtk::Type::String];
    let col_indices: [u32; 2] = [0, 1];
    let store = gtk::ListStore::new(&col_types);

    let db = database::DB::new().unwrap();
    let data = db.get_certs(0, 10).unwrap();

    for d in data.iter() {
        let values: [&dyn ToValue; 2] = [&d.name, &d.thumbprint];
        store.set(&store.append(), &col_indices, &values);
    }
    store
}

fn create_csrs_model() -> gtk::ListStore {
    let col_types: [gtk::Type; 2] = [gtk::Type::U32, gtk::Type::String];
    let col_indices: [u32; 2] = [0, 1];
    let store = gtk::ListStore::new(&col_types);

    let db = database::DB::new().unwrap();
    let data = db.get_csrs(0, 10).unwrap();

    for d in data.iter() {
        let values: [&dyn ToValue; 2] = [&d.id, &d.name];
        store.set(&store.append(), &col_indices, &values);
    }
    store
}

fn create_keys_model() -> gtk::ListStore {
    let col_types: [gtk::Type; 4] = [gtk::Type::U32, gtk::Type::String, gtk::Type::String, gtk::Type::String];
    let col_indices: [u32; 4] = [0, 1, 2, 3];
    let store = gtk::ListStore::new(&col_types);

    let db = database::DB::new().unwrap();
    let data = db.get_keys(0, 10).unwrap();

    for d in data.iter() {
        let values: [&dyn ToValue; 4] = [&d.id, &d.name, &d.algo, &d.option];
        store.set(&store.append(), &col_indices, &values);
    }
    store
}

fn add_certs_columns(treeview: &gtk::TreeView) {
    let fields = vec!("Name", "Thumbprint");
    for (i,  field) in fields.iter().enumerate()     {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title(field);
        column.add_attribute(&renderer, "text", i as i32);
        treeview.append_column(&column);
    }

}

fn add_csrs_columns(treeview: &gtk::TreeView) {
    let fields = vec!("ID", "Name");
    for (i,  field) in fields.iter().enumerate()     {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title(field);
        column.add_attribute(&renderer, "text", i as i32);
        treeview.append_column(&column);
    }

}

fn add_keys_columns(treeview: &gtk::TreeView) {
    let fields = vec!("ID", "Name", "Algorithm", "Option");
    for (i,  field) in fields.iter().enumerate()     {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title(field);
        column.add_attribute(&renderer, "text", i as i32);
        treeview.append_column(&column);
    }

}
