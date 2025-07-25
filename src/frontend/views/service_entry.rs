use crate::backend::{EnablementStatus, ServiceInfo, ServiceStatus};
use gtk4::{Align, Box, Label, ListBoxRow, Orientation, Separator, prelude::BoxExt};

pub fn create_service_entry(service: &ServiceInfo) -> ListBoxRow {
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
    ListBoxRow::builder()
        .child(&row_box)
        .name(
            format_enablement(&service.enablement_status).to_owned()
                + format_status(&service.status),
        ) // extremely clunky way to store data on a widget but i dont have to write in and unsafe thingy so..
        .build()
}

pub fn format_status(status: &ServiceStatus) -> &'static str {
    match status {
        ServiceStatus::Active => "Active",
        ServiceStatus::Inactive => "Inactive",
        ServiceStatus::Failed => "Failed",
        ServiceStatus::Activating => "Activating",
        ServiceStatus::Deactivating => "Deactivating",
        ServiceStatus::Unknown(_) => "Unknown",
    }
}

pub fn format_enablement(enablement: &EnablementStatus) -> &'static str {
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
        _ => &["dim-label"],
    }
}
