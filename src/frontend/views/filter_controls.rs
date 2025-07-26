use adw::{ActionRow, PreferencesGroup, prelude::*};
use gtk4::{Align, Box, ComboBoxText, Orientation};

pub fn create_filter_controls() -> (Box, ComboBoxText, ComboBoxText) {
    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let group = PreferencesGroup::builder()
        .title("Service Filters")
        .description("Filter services by status and enablement state")
        .build();

    let (status_row, status_combo) = create_combo_row(
        "Status",
        &[
            "All",
            "Active",
            "Inactive",
            "Failed",
            "Activating",
            "Deactivating",
            "Unknown",
        ],
    );

    let (enablement_row, enablement_combo) = create_combo_row(
        "Enablement",
        &[
            "All",
            "Enabled",
            "Disabled",
            "Static",
            "Indirect",
            "Generated",
            "Transient",
            "Unknown",
        ],
    );

    group.add(&status_row);
    group.add(&enablement_row);
    main_box.append(&group);

    (main_box, status_combo, enablement_combo)
}

fn create_combo_row(title: &str, options: &[&str]) -> (ActionRow, ComboBoxText) {
    let combo = ComboBoxText::builder()
        .valign(Align::Center)
        .css_classes(["compact"])
        .build();
    options.iter().for_each(|option| {
        combo.append_text(option);
    });

    combo.set_active(Some(0));

    let row = ActionRow::builder().title(title).activatable(false).build();

    row.add_suffix(&combo);

    (row, combo)
}
