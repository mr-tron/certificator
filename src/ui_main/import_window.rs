use crate::database;
use gtk::prelude::*;

const MAIN_SPAICING: i32 = 5;

pub struct ImportWindow {
    pub window: gtk::Window,
    import_button: gtk::Button,
    input: gtk::TextView,
}

impl ImportWindow {
    pub fn new() -> ImportWindow {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        let main_layout = gtk::Box::new(gtk::Orientation::Vertical, MAIN_SPAICING);
        let input = text_input();
        let import_button = gtk::Button::new_with_label("Import!");
        main_layout.add(&input);
        main_layout.add(&import_button);
        window.add(&main_layout);
        window.set_title("title");
        window.set_default_size(600, 600);
        window.show_all();

        let w = ImportWindow {
            window,
            import_button,
            input,
        };
        w.set_callbacks();
        w
    }

    fn set_callbacks(&self) {
        let t = self.input.clone();
        let w = self.window.clone();
        self.import_button.connect_clicked(move |_| {
            let buf = t.get_buffer().expect("df");
            let (start, end) = buf.get_bounds();
            let text = buf.get_text(&start, &end, false).expect("dfs");

            let cert = database::X509CertificateDB::from_pem(text.as_str());
            let db = database::DB::new().unwrap();
            match db.save_cert(cert) {
                Ok(_) => w.close(),
                Err(e) => println!("{}", e),
            };
        });
    }
}

fn text_input() -> gtk::TextView {
    let builder = gtk::TextViewBuilder::new();

    let t = builder
        .editable(true)
        .vexpand(true)
        .hexpand(true)
        .cursor_visible(false)
        .monospace(true)
        .build();
    t
}
