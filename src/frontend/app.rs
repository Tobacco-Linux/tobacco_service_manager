use super::views::{
    ServiceData, create_filter_controls, create_service_entry, update_service_visibility,
};
use crate::backend::get_services;
use adw::{Application, HeaderBar, Window, prelude::*};
use gtk4::{Box, Button, ListBox, ListBoxRow, Orientation, ScrolledWindow, SearchEntry, Separator};
use std::cell::RefCell;
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

    let refresh_button = Button::builder().icon_name("view-refresh").build();

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
        .build();

    let current_query = Rc::new(RefCell::new(String::new()));
    let service_widgets: Rc<RefCell<Vec<(ServiceData, ListBoxRow)>>> =
        Rc::new(RefCell::new(Vec::new()));

    let refresh_data = {
        let service_widgets = service_widgets.clone();
        let services_list = services_list.clone();

        move || {
            for (_, row) in service_widgets.borrow_mut().drain(..) {
                services_list.remove(&row);
            }
            if let Ok(services) = get_services() {
                let widgets: Vec<(ServiceData, ListBoxRow)> = services
                    .into_iter()
                    .map(|service| create_service_entry(&service))
                    .collect();

                for (_, row) in &widgets {
                    services_list.append(row);
                }

                *service_widgets.borrow_mut() = widgets;
            }
        }
    };

    refresh_data(); // initial loading

    let update_visibility = {
        let service_widgets = service_widgets.clone();
        let status_combo = status_combo.clone();
        let enablement_combo = enablement_combo.clone();
        let current_query = current_query.clone();

        move || {
            let query = current_query.borrow().clone();
            let status_filter = status_combo.active_text().unwrap_or_else(|| "All".into());
            let enablement_filter = enablement_combo
                .active_text()
                .unwrap_or_else(|| "All".into());

            update_service_visibility(
                &service_widgets.borrow(),
                &query,
                &status_filter,
                &enablement_filter,
            );
        }
    };

    let update_visibility_search = update_visibility.clone();
    let current_query_search = current_query.clone();
    search_entry.connect_search_changed(move |search| {
        *current_query_search.borrow_mut() = search.text().to_string();
        update_visibility_search();
    });

    let update_visibility_status = update_visibility.clone();
    status_combo.connect_changed(move |_| {
        update_visibility_status();
    });

    let update_visibility_enablement = update_visibility.clone();
    enablement_combo.connect_changed(move |_| {
        update_visibility_enablement();
    });

    let refresh_button_handler = refresh_data.clone();
    refresh_button.connect_clicked(move |_| {
        refresh_button_handler();
        update_visibility();
    });

    let main_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .vexpand(true)
        .build();

    let sidebar_container = Box::builder()
        .orientation(Orientation::Vertical)
        .width_request(350)
        .build();

    let sidebar_scroll = ScrolledWindow::builder()
        .min_content_width(250)
        .child(&sidebar)
        .vexpand(true)
        .hexpand(false)
        .build();

    sidebar_container.append(&sidebar_scroll);

    let services_container = Box::builder()
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    let services_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .min_content_width(550)
        .child(&services_list)
        .hexpand(true)
        .vexpand(true)
        .build();

    services_container.append(&services_scroll);

    main_box.append(&sidebar_container);
    main_box.append(&Separator::new(Orientation::Vertical));
    main_box.append(&services_container);

    sidebar_container.set_hexpand(false);
    sidebar_container.set_vexpand(true);
    services_container.set_hexpand(true);
    services_container.set_vexpand(true);

    let header = HeaderBar::new();
    header.pack_start(&refresh_button);

    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.append(&header);
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
