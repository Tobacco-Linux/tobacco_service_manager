use adw::{Application, HeaderBar, Window, prelude::*};
use gtk4::{
    Align, Box, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, SearchEntry,
    SelectionMode, Separator, glib,
};

mod backend;
use backend::{EnablementStatus, ServiceInfo, ServiceStatus, get_services};

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
                (name, create_service_row(&service))
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

fn create_service_row(service: &ServiceInfo) -> ListBoxRow {
    let row_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .margin_start(12)
        .margin_end(12)
        .margin_top(9)
        .margin_bottom(9)
        .build();

    row_box.append(
        &Label::builder()
            .label(&service.name)
            .halign(Align::Start)
            .css_classes(["heading"])
            .build(),
    );

    row_box.append(
        &Label::builder()
            .label(&service.description)
            .halign(Align::Start)
            .wrap(true)
            .justify(gtk4::Justification::Left)
            .css_classes(["caption"])
            .build(),
    );

    row_box.append(&Separator::new(Orientation::Horizontal));

    let info_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .build();

    info_box.append(
        &Label::builder()
            .label(&format!("Status: {}", format_status(&service.status)))
            .halign(Align::Start)
            .css_classes(get_status_css_classes(&service.status))
            .build(),
    );

    info_box.append(
        &Label::builder()
            .label(&format!(
                "Enablement: {}",
                format_enablement(&service.enablement_status)
            ))
            .halign(Align::Start)
            .css_classes(get_enablement_css_classes(&service.enablement_status))
            .build(),
    );

    row_box.append(&info_box);
    ListBoxRow::builder().child(&row_box).build()
}

fn format_status(status: &ServiceStatus) -> &'static str {
    match status {
        ServiceStatus::Active => "Active",
        ServiceStatus::Inactive => "Inactive",
        ServiceStatus::Failed => "Failed",
        ServiceStatus::Activating => "Activating",
        ServiceStatus::Deactivating => "Deactivating",
        ServiceStatus::Unknown(_) => "Unknown",
    }
}

fn format_enablement(enablement: &EnablementStatus) -> &'static str {
    match enablement {
        EnablementStatus::Enabled => "Enabled",
        EnablementStatus::Disabled => "Disabled",
        EnablementStatus::Static => "Static",
        EnablementStatus::Indirect => "Indirect",
        EnablementStatus::Generated => "Generated",
        EnablementStatus::Transient => "Transient",
        EnablementStatus::Unknown(_) => "Unknown",
    }
}

fn get_status_css_classes(status: &ServiceStatus) -> &'static [&'static str] {
    match status {
        ServiceStatus::Active => &["success"],
        ServiceStatus::Failed => &["error"],
        ServiceStatus::Activating | ServiceStatus::Deactivating => &["warning"],
        _ => &["dim-label"],
    }
}

fn get_enablement_css_classes(enablement: &EnablementStatus) -> &'static [&'static str] {
    match enablement {
        EnablementStatus::Enabled => &["success"],
        EnablementStatus::Disabled => &["dim-label"],
        EnablementStatus::Static => &["warning"],
        _ => &["caption"],
    }
}

fn filter_services(
    service_widgets: &std::cell::RefCell<Vec<(String, ListBoxRow)>>,
    services_list: &ListBox,
    query: &str,
) {
    let widgets = service_widgets.borrow();

    services_list.first_child().iter().for_each(|_| {
        while let Some(child) = services_list.first_child() {
            services_list.remove(&child);
        }
    });

    widgets
        .iter()
        .filter(|(name, _)| query.is_empty() || name.to_lowercase().contains(query))
        .for_each(|(_, row)| services_list.append(row));
}
