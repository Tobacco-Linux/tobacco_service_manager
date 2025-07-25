use adw::{ActionRow, prelude::*};
use gtk4::{Box, ComboBoxText, Orientation};

pub fn create_filter_controls() -> (Box, ComboBoxText, ComboBoxText) {
    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let group = adw::PreferencesGroup::builder()
        .title("Filters")
        .description("Filter services by status and enablement")
        .build();

    let (status_row, status_combo) =
        create_combo_row("Status:", &["All", "Active", "Inactive", "Failed"]);
    let (enablement_row, enablement_combo) =
        create_combo_row("Enablement:", &["All", "Enabled", "Disabled", "Static"]);

    group.add(&status_row);
    group.add(&enablement_row);

    main_box.append(&group);
    (main_box, status_combo, enablement_combo)
}

fn create_combo_row(title: &str, options: &[&str]) -> (ActionRow, ComboBoxText) {
    let combo = ComboBoxText::new();
    for option in options {
        combo.append_text(option);
    }
    combo.set_active(Some(0));
    combo.set_valign(gtk4::Align::Center);
    combo.add_css_class("flat");
    combo.set_size_request(150, -1);

    let row = ActionRow::builder().title(title).build();

    row.add_suffix(&combo);
    row.set_activatable_widget(Some(&combo));

    (row, combo)
}
