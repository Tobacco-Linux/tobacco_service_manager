use crate::backend::{EnablementStatus, ServiceInfo, ServiceStatus};
use gtk4::{Align, Box, Label, ListBoxRow, Orientation, Separator, prelude::*};

#[derive(Debug, Clone)]
pub struct ServiceData {
    pub name: String,
    pub status: ServiceStatus,
    pub enablement: EnablementStatus,
}

impl ServiceData {
    pub fn matches_query(&self, query: &str) -> bool {
        query.is_empty() || self.name.to_lowercase().contains(&query.to_lowercase())
    }

    pub fn matches_filters(&self, status_filter: &str, enablement_filter: &str) -> bool {
        let status_matches = status_filter == "All"
            || super::service_filter::format_status(&self.status) == status_filter;
        let enablement_matches = enablement_filter == "All"
            || super::service_filter::format_enablement(&self.enablement) == enablement_filter;
        status_matches && enablement_matches
    }
}

pub fn create_service_entry(service: &ServiceInfo) -> (ServiceData, ListBoxRow) {
    let service_data = ServiceData {
        name: service.name.clone(),
        status: service.status.clone(),
        enablement: service.enablement_status.clone(),
    };

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
            .label(&format!(
                "Status: {}",
                super::service_filter::format_status(&service.status)
            ))
            .halign(Align::Start)
            .css_classes(super::service_filter::get_status_css_classes(
                &service.status,
            ))
            .build(),
    );

    info_box.append(
        &Label::builder()
            .label(&format!(
                "Enablement: {}",
                super::service_filter::format_enablement(&service.enablement_status)
            ))
            .halign(Align::Start)
            .css_classes(super::service_filter::get_enablement_css_classes(
                &service.enablement_status,
            ))
            .build(),
    );

    row_box.append(&info_box);

    let row = ListBoxRow::builder()
        .name(&service_data.name)
        .child(&row_box)
        .build();

    (service_data, row)
}
