use adw::{ActionRow, PreferencesGroup, prelude::*};
use gtk4::{Align, Box, Button, Orientation};

pub fn create_service_actions<F: Fn(&Button) + 'static + Copy>(button_callback: F) -> Box {
    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let group = PreferencesGroup::builder()
        .title("Service Actions")
        .description("Perform actions on selected services")
        .build();

    let state_row = create_action_buttons(
        "State",
        &[
            ("Start", "Start service", "media-playback-start-symbolic"),
            ("Stop", "Stop service", "media-playback-stop-symbolic"),
        ],
        &button_callback,
    );

    let enablement_row = create_action_buttons(
        "Enablement",
        &[
            ("Enable", "Enable auto-start", "system-run-symbolic"),
            ("Disable", "Disable auto-start", "window-close-symbolic"),
        ],
        &button_callback,
    );

    group.add(&state_row);
    group.add(&enablement_row);
    main_box.append(&group);

    main_box
}

fn create_action_buttons<F: Fn(&Button) + 'static + Copy>(
    title: &str,
    actions: &[(&str, &str, &str)],
    callback: &F,
) -> ActionRow {
    let button_box = Box::builder()
        .css_classes(["linked"])
        .orientation(Orientation::Horizontal)
        .halign(Align::End)
        .margin_top(6)
        .margin_bottom(6)
        .build();

    for (label, tooltip, icon) in actions {
        let button = Button::builder()
            .icon_name(*icon)
            .tooltip_text(*tooltip)
            .label(*label)
            .build();

        let callback_clone = callback.clone();
        button.connect_clicked(move |btn| callback_clone(btn));
        button_box.append(&button);
    }

    let row = ActionRow::builder().title(title).activatable(false).build();

    row.add_suffix(&button_box);
    row
}
