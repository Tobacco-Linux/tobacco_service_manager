use super::views::{
    create_filter_controls, create_service_entry, filter_services, search_services,
    service_entry::ServiceRowData,
};
use crate::backend::get_services;
use adw::{Application, HeaderBar, Window, prelude::*};
use gtk4::{Box, ListBox, ListBoxRow, Orientation, ScrolledWindow, SearchEntry, Separator};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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

    let (filter_controls, status_combo, enablement_combo) = create_filter_controls();

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
    let row_metadata: Rc<RefCell<HashMap<usize, ServiceRowData>>> =
        Rc::new(RefCell::new(HashMap::new()));

    if let Ok(services) = get_services() {
        let widgets: Vec<ListBoxRow> = services
            .into_iter()
            .enumerate()
            .map(|(index, service)| create_service_entry(&service, &row_metadata, index))
            .collect();

        for widget in &widgets {
            services_list.append(widget);
        }

        *service_widgets.borrow_mut() = widgets;
    }

    let service_widgets_search = service_widgets.clone();
    let row_metadata_search = row_metadata.clone();
    search_entry.connect_search_changed(move |search| {
        let query = search.text().to_string();
        search_services(&service_widgets_search, &row_metadata_search, &query);
    });

    let service_widgets_status = service_widgets.clone();
    let row_metadata_status = row_metadata.clone();
    let enablement_combo_filter = enablement_combo.clone();
    status_combo.connect_changed(move |combo| {
        if let Some(status_text) = combo.active_text() {
            if let Some(enablement_text) = enablement_combo_filter.active_text() {
                filter_services(
                    &service_widgets_status,
                    &row_metadata_status,
                    &status_text,
                    &enablement_text,
                );
            }
        }
    });

    let service_widgets_enablement = service_widgets.clone();
    let row_metadata_enablement = row_metadata.clone();
    let status_combo_filter = status_combo.clone();
    enablement_combo.connect_changed(move |combo| {
        if let Some(enablement_text) = combo.active_text() {
            if let Some(status_text) = status_combo_filter.active_text() {
                filter_services(
                    &service_widgets_enablement,
                    &row_metadata_enablement,
                    &status_text,
                    &enablement_text,
                );
            }
        }
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
