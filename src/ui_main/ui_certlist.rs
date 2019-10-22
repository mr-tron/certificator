use crate::database;
use gtk::prelude::*;
use std::rc::Rc;
const MAIN_SPAICING: i32 = 8;

pub fn cert_list() -> gtk::TreeView {
    let model = Rc::new(create_model());
    let treeview = gtk::TreeView::new_with_model(&*model);
    treeview.set_vexpand(true);
    treeview.set_search_column(Columns::Name as i32);

    add_columns(&model, &treeview);
    treeview
}

pub fn wrap(name: &str, list: &gtk::TreeView) -> gtk::Box {
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

fn create_model() -> gtk::ListStore {
    let col_types: [gtk::Type; 2] = [gtk::Type::String, gtk::Type::String];
    let db = database::DB::new().unwrap();

    let data = db.get_certs(0, 10).unwrap();
    let store = gtk::ListStore::new(&col_types);

    let col_indices: [u32; 2] = [0, 1];

    for d in data.iter() {
        let values: [&dyn ToValue; 2] = [&d.name, &d.thumbprint];
        store.set(&store.append(), &col_indices, &values);
    }

    store
}

#[repr(i32)]
enum Columns {
    Name = 0,
    Thumbprint,
}

fn add_columns(model: &Rc<gtk::ListStore>, treeview: &gtk::TreeView) {
    {
        let renderer = gtk::CellRendererText::new();
        let _model_clone = model.clone();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Name");
        column.add_attribute(&renderer, "text", Columns::Name as i32);
        //        column.set_sizing(gtk::TreeViewColumnSizing::Fixed);
        //        column.set_fixed_width(50);
        treeview.append_column(&column);
    }

    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Thumbprint");
        column.add_attribute(&renderer, "text", Columns::Thumbprint as i32);
        column.set_sort_column_id(Columns::Thumbprint as i32);
        treeview.append_column(&column);
    }
}
