use adw::{Application, HeaderBar, Window, prelude::*};
use gtk4::{
    Align, Box, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, SearchEntry,
    SelectionMode, glib,
};

mod backend;
use backend::get_services;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.tobacco_linux.ServiceManager")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let search_entry = SearchEntry::builder()
        .css_classes(["inline"])
        .placeholder_text("Search...")
        .build();

    let sidebar = ListBox::builder()
        .css_classes(["navigation-sidebar"])
        .selection_mode(SelectionMode::None)
        .build();
    sidebar.append(&search_entry);

    let services = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .margin_top(4)
        .margin_bottom(4)
        .build();

    if let Ok(service_list) = get_services() {
        for service in service_list {
            services.append(
                &ListBoxRow::builder()
                    .child(
                        &Label::builder()
                            .label(&service)
                            .halign(Align::Start)
                            .margin_start(8)
                            .margin_top(6)
                            .margin_bottom(6)
                            .build(),
                    )
                    .build(),
            );
        }
    }

    let main_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .vexpand(true)
        .build();

    let scrolled_sidebar = ScrolledWindow::builder()
        .min_content_width(250)
        .child(&sidebar)
        .vexpand(true)
        .build();

    let scrolled_services = ScrolledWindow::builder()
        .min_content_width(550)
        .child(&services)
        .hexpand(true)
        .vexpand(true)
        .build();

    main_box.append(&scrolled_sidebar);
    main_box.append(&scrolled_services);

    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.append(&HeaderBar::new());
    vbox.append(&main_box);

    let window = Window::builder()
        .application(app)
        .default_width(800)
        .default_height(600)
        .title("Service Manager")
        .content(&vbox)
        .build();

    window.present();
}
