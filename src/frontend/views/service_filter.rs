use super::service_entry::ServiceData;
use gtk4::{ListBoxRow, prelude::WidgetExt};

pub fn update_service_visibility(
    service_widgets: &[(ServiceData, ListBoxRow)],
    query: &str,
    status_filter: &str,
    enablement_filter: &str,
) {
    for (service_data, row) in service_widgets {
        let visible = service_data.matches_query(query)
            && service_data.matches_filters(status_filter, enablement_filter);
        row.set_visible(visible);
    }
}

pub fn format_status(status: &crate::backend::ServiceStatus) -> &'static str {
    use crate::backend::ServiceStatus;
    match status {
        ServiceStatus::Active => "Active",
        ServiceStatus::Inactive => "Inactive",
        ServiceStatus::Failed => "Failed",
        ServiceStatus::Activating => "Activating",
        ServiceStatus::Deactivating => "Deactivating",
        ServiceStatus::Unknown(_) => "Unknown",
    }
}

pub fn format_enablement(enablement: &crate::backend::EnablementStatus) -> &'static str {
    use crate::backend::EnablementStatus;
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

pub fn get_status_css_classes(status: &crate::backend::ServiceStatus) -> &'static [&'static str] {
    use crate::backend::ServiceStatus;
    match status {
        ServiceStatus::Active => &["success"],
        ServiceStatus::Failed => &["error"],
        ServiceStatus::Activating | ServiceStatus::Deactivating => &["warning"],
        _ => &["dim-label"],
    }
}

pub fn get_enablement_css_classes(
    enablement: &crate::backend::EnablementStatus,
) -> &'static [&'static str] {
    use crate::backend::EnablementStatus;
    match enablement {
        EnablementStatus::Enabled => &["success"],
        EnablementStatus::Disabled => &["dim-label"],
        EnablementStatus::Static => &["warning"],
        _ => &["dim-label"],
    }
}
