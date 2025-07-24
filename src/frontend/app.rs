use super::views::{create_service_entry, filter_services};
use crate::backend::get_services;
use adw::{Application, HeaderBar, Window, prelude::*};
use gtk4::{Box, ListBox, Orientation, ScrolledWindow, SearchEntry, SelectionMode, Separator};

pub fn build_ui(app: &Application) {
    let search_entry = SearchEntry::builder()
        .css_classes(["inline"])
        .placeholder_text("Search services...")
        .hexpand(true)
        .build();

    let sidebar = ListBox::builder()
        .css_classes(["navigation-sidebar"])
        .selection_mode(SelectionMode::None)
        .build();
    sidebar.append(&search_entry);

    let services_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let service_widgets = std::cell::RefCell::new(Vec::new());

    if let Ok(services) = get_services() {
        let widgets: Vec<_> = services
            .into_iter()
            .map(|service| {
                let name = service.name.clone();
                (name, create_service_entry(&service))
            })
            .collect();

        widgets
            .iter()
            .for_each(|(_, row)| services_list.append(row));
        *service_widgets.borrow_mut() = widgets;
    }

    let service_widgets_clone = service_widgets.clone();
    let services_list_clone = services_list.clone();
    search_entry.connect_search_changed(move |search| {
        let query = search.text().to_lowercase();
        filter_services(&service_widgets_clone, &services_list_clone, &query);
    });

    let main_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .vexpand(true)
        .build();

    main_box.append(
        &ScrolledWindow::builder()
            .min_content_width(250)
            .child(&sidebar)
            .vexpand(true)
            .build(),
    );
    main_box.append(&Separator::new(Orientation::Vertical));

    main_box.append(
        &ScrolledWindow::builder()
            .min_content_width(550)
            .child(&services_list)
            .hexpand(true)
            .vexpand(true)
            .build(),
    );

    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.append(&HeaderBar::new());
    vbox.append(&main_box);

    Window::builder()
        .application(app)
        .default_width(900)
        .default_height(650)
        .title("Service Manager")
        .content(&vbox)
        .build()
        .present();
}
