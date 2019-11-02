use gtk::prelude::*;

const MAIN_SPAICING: i32 = 8;

pub struct View {
    pub container: gtk::Box,
    pub view: gtk::TextView,
}

impl View {
    pub fn new() -> View {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, MAIN_SPAICING);
        let view = view_cert();
        container.add(&view);
        View { container, view }
    }
}

pub fn view_cert() -> gtk::TextView {
    let builder = gtk::TextViewBuilder::new();

    let view = builder
        .editable(false)
        .vexpand(true)
        .cursor_visible(false)
        .monospace(true)
        .build();
    view
}
