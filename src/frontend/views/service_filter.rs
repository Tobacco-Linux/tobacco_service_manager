use gtk4::{ListBox, ListBoxRow, prelude::WidgetExt};
use std::cell::RefCell;

pub fn search_services(
    service_widgets: &RefCell<Vec<(String, ListBoxRow)>>,
    services_list: &ListBox,
    query: &str,
) {
    let widgets = service_widgets.borrow();

    while let Some(child) = services_list.first_child() {
        services_list.remove(&child);
    }

    widgets
        .iter()
        .filter(|(name, _)| query.is_empty() || name.to_lowercase().contains(&query.to_lowercase()))
        .for_each(|(_, row)| services_list.append(row));
}

pub fn filter_services(
    service_widgets: &RefCell<Vec<(String, ListBoxRow)>>,
    services_list: &ListBox,
    status_filter: &str,
    enablement_filter: &str,
) {
    let widgets = service_widgets.borrow();

    while let Some(child) = services_list.first_child() {
        services_list.remove(&child);
    }

    widgets
        .iter()
        .filter(|(_, service)| {
            (status_filter == "All" || service.widget_name().contains(status_filter))
                && (enablement_filter == "All" || service.widget_name().contains(enablement_filter))
        })
        .for_each(|(_, row)| services_list.append(row));
}
