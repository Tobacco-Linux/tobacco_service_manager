use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::backend::{EnablementStatus, ServiceInfo, ServiceStatus};
use adw::prelude::ObjectExt;
use gtk4::{Align, Box, Label, ListBoxRow, Orientation, Separator, prelude::BoxExt};

#[derive(Debug, Clone)]
pub struct ServiceRowData {
    pub name: String,
    pub status: ServiceStatus,
    pub enablement: EnablementStatus,
}

pub type RowMetadata = Rc<RefCell<HashMap<usize, ServiceRowData>>>;

pub fn create_service_entry(
    service: &ServiceInfo,
    metadata: &RowMetadata,
    index: usize,
) -> ListBoxRow {
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

    let status_label = Label::builder()
        .label(&format!("Status: {}", format_status(&service.status)))
        .halign(Align::Start)
        .css_classes(get_status_css_classes(&service.status))
        .build();

    let enablement_label = Label::builder()
        .label(&format!(
            "Enablement: {}",
            format_enablement(&service.enablement_status)
        ))
        .halign(Align::Start)
        .css_classes(get_enablement_css_classes(&service.enablement_status))
        .build();

    info_box.append(&status_label);
    info_box.append(&enablement_label);

    row_box.append(&info_box);

    let row = ListBoxRow::builder().child(&row_box).build();

    metadata.borrow_mut().insert(
        index,
        ServiceRowData {
            name: service.name.clone(),
            status: service.status.clone(),
            enablement: service.enablement_status.clone(),
        },
    );

    unsafe { row.set_data("row_index", index) };

    row
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
