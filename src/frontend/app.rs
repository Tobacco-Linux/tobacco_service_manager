use super::views::{create_service_entry, search_services};
use crate::{
    backend::get_services,
    frontend::views::{create_filter_controls, filter_services},
};
use adw::{Application, HeaderBar, Window, prelude::*};
use gtk4::{Box, ListBox, Orientation, ScrolledWindow, SearchEntry, Separator};

pub fn build_ui(app: &Application) {
    let search_entry = SearchEntry::builder()
        .css_classes(["inline"])
        .placeholder_text("Search names...")
        .build();

    let sidebar = Box::builder()
        .css_classes(["navigation-sidebar"])
        .orientation(Orientation::Vertical)
        .margin_start(12)
        .margin_end(12)
        .margin_top(4)
        .margin_bottom(4)
        .spacing(2)
        .build();

    let (filter_controls, status, enablement) = create_filter_controls();

    sidebar.append(&search_entry);
    sidebar.append(&Separator::new(Orientation::Vertical));
    sidebar.append(&filter_controls);

    let services_list = ListBox::builder()
        .selection_mode(gtk4::SelectionMode::Multiple)
        .css_classes(["boxed-list"])
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .hexpand(false)
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
        search_services(&service_widgets_clone, &services_list_clone, &query);
    });

    let service_widgets_clone = service_widgets.clone();
    let services_list_clone = services_list.clone();
    let enablement_clone = enablement.clone();
    status.connect_changed(move |combo| {
        filter_services(
            &service_widgets_clone,
            &services_list_clone,
            &combo.active_text().unwrap().as_str(),
            &enablement_clone.active_text().unwrap().as_str(),
        );
    });

    let service_widgets_clone = service_widgets.clone();
    let services_list_clone = services_list.clone();
    enablement.connect_changed(move |combo| {
        filter_services(
            &service_widgets_clone,
            &services_list_clone,
            &status.active_text().unwrap().as_str(),
            &combo.active_text().unwrap().as_str(),
        );
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
            .hscrollbar_policy(gtk4::PolicyType::Never)
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
        .default_width(1200)
        .default_height(800)
        .title("Service Manager")
        .content(&vbox)
        .build()
        .present();
}
