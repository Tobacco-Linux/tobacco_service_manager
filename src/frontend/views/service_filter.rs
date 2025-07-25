use adw::glib::object::ObjectExt;
use gtk4::{ListBoxRow, prelude::WidgetExt};
use std::cell::RefCell;

use crate::frontend::views::service_entry::{RowMetadata, format_enablement, format_status};

pub fn search_services(
    service_widgets: &RefCell<Vec<ListBoxRow>>,
    metadata: &RowMetadata,
    query: &str,
) {
    let widgets = service_widgets.borrow();
    let metadata_ref = metadata.borrow();

    for row in widgets.iter() {
        let index = unsafe {
            row.data::<usize>("row_index")
                .expect("Row should have index")
                .read()
        };

        let matches = if query.is_empty() {
            true
        } else {
            metadata_ref
                .get(&index)
                .map(|data| data.name.to_lowercase().contains(&query.to_lowercase()))
                .unwrap_or(false)
        };
        row.set_visible(matches);
    }
}

pub fn filter_services(
    service_widgets: &RefCell<Vec<ListBoxRow>>,
    metadata: &RowMetadata,
    status_filter: &str,
    enablement_filter: &str,
) {
    let widgets = service_widgets.borrow();
    let metadata_ref = metadata.borrow();

    for row in widgets.iter() {
        let index = unsafe {
            row.data::<usize>("row_index")
                .expect("Row should have index")
                .read()
        };

        let matches = metadata_ref
            .get(&index)
            .map(|data| {
                let status_matches =
                    status_filter == "All" || format_status(&data.status) == status_filter;

                let enablement_matches = enablement_filter == "All"
                    || format_enablement(&data.enablement) == enablement_filter;

                status_matches && enablement_matches
            })
            .unwrap_or(false);

        row.set_visible(matches);
    }
}
