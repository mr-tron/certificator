use gtk::prelude::*;

const MAIN_SPAICING: i32 = 8;

pub struct Menu {
    pub container: gtk::Box,
    pub import_button: gtk::Button,
}

impl Menu {
    pub fn new() -> Menu {
        let lists_layout = gtk::Box::new(gtk::Orientation::Horizontal, MAIN_SPAICING);
        lists_layout.set_halign(gtk::Align::Fill);
        let import_button = gtk::Button::new_with_mnemonic("_Import");
        lists_layout.add(&import_button);

        Menu {
            container: lists_layout,
            import_button,
        }
    }
}
