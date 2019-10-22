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
