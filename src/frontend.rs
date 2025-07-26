use crate::backend::{EnablementStatus, ServiceInfo, ServiceStatus, SystemdServiceManager};
use adw::{Application, HeaderBar, Toast, ToastOverlay, ToastPriority, Window, prelude::*};
use gtk4::{
    Box, Button, ComboBoxText, Label, ListBox, ListBoxRow, ScrolledWindow, SearchEntry, Separator,
};
use std::cell::RefCell;
use std::rc::Rc;

struct ServiceManagerState {
    systemd: SystemdServiceManager,
    service_widgets: Rc<RefCell<Vec<(ServiceData, ListBoxRow)>>>,
    services_list: ListBox,
    status_combo: ComboBoxText,
    enablement_combo: ComboBoxText,
    current_query: Rc<RefCell<String>>,
    toast_overlay: ToastOverlay,
}

impl ServiceManagerState {
    fn refresh_services(&self) {
        for (_, row) in self.service_widgets.borrow_mut().drain(..) {
            self.services_list.remove(&row);
        }

        if let Ok(services) = self.systemd.get_services() {
            let widgets: Vec<(ServiceData, ListBoxRow)> = services
                .into_iter()
                .map(|service| create_service_entry(&service))
                .collect();

            for (_, row) in &widgets {
                self.services_list.append(row);
            }

            *self.service_widgets.borrow_mut() = widgets;
        }

        self.update_visibility();
    }

    fn update_visibility(&self) {
        let query = self.current_query.borrow().clone();
        let status_filter = self
            .status_combo
            .active_text()
            .unwrap_or_else(|| "All".into());
        let enablement_filter = self
            .enablement_combo
            .active_text()
            .unwrap_or_else(|| "All".into());

        update_service_visibility(
            &self.service_widgets.borrow(),
            &query,
            &status_filter,
            &enablement_filter,
        );
    }

    fn handle_service_action(&self, action: &str) {
        let selected_services = get_selected_services(&self.services_list);
        if selected_services.is_empty() {
            self.show_toast("No services selected", ToastPriority::Normal);
            return;
        }

        for service_name in &selected_services {
            let result = match action {
                "Start" => self.systemd.start_unit(service_name),
                "Stop" => self.systemd.stop_unit(service_name),
                "Enable" => self.systemd.enable_unit(service_name),
                "Disable" => self.systemd.disable_unit(service_name),
                _ => continue,
            };

            match result {
                Ok(()) => self.show_toast(
                    &format!("{} operation successful for {}", action, service_name),
                    ToastPriority::Normal,
                ),
                Err(e) => self.show_toast(
                    &format!("Failed to {} {}: {}", action, service_name, e),
                    ToastPriority::High,
                ),
            }
        }

        self.refresh_services();
    }

    fn show_toast(&self, message: &str, priority: ToastPriority) {
        let toast = Toast::builder()
            .title(message)
            .priority(priority)
            .timeout(3)
            .build();
        self.toast_overlay.add_toast(toast);
    }
}

pub fn build_ui(app: &Application) {
    let systemd = SystemdServiceManager::new();
    let services_list = ListBox::builder()
        .selection_mode(gtk4::SelectionMode::Multiple)
        .css_classes(["boxed-list"])
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let toast_overlay = ToastOverlay::new();

    let state = Rc::new(RefCell::new(ServiceManagerState {
        systemd,
        service_widgets: Rc::new(RefCell::new(Vec::new())),
        services_list,
        status_combo: ComboBoxText::new(),
        enablement_combo: ComboBoxText::new(),
        current_query: Rc::new(RefCell::new(String::new())),
        toast_overlay,
    }));

    let sidebar = build_sidebar(Rc::clone(&state));
    let main_content = build_main_content(Rc::clone(&state));

    let window = create_window(app, Rc::clone(&state), sidebar, main_content);

    state.borrow().refresh_services();
    window.present();
}

fn build_sidebar(state: Rc<RefCell<ServiceManagerState>>) -> Box {
    let sidebar = Box::builder()
        .css_classes(["navigation-sidebar"])
        .orientation(gtk4::Orientation::Vertical)
        .margin_start(12)
        .margin_end(12)
        .margin_top(4)
        .margin_bottom(4)
        .spacing(2)
        .build();

    let search_entry = SearchEntry::builder()
        .css_classes(["inline"])
        .placeholder_text("Search names...")
        .build();

    let (filter_controls, status_combo, enablement_combo) = create_filter_controls();

    {
        let mut state = state.borrow_mut();
        state.status_combo = status_combo;
        state.enablement_combo = enablement_combo;
    }

    let refresh_button = Button::builder().icon_name("view-refresh").build();
    setup_refresh_button(refresh_button, Rc::clone(&state));

    let action_callback = {
        let state = Rc::clone(&state);
        move |button: &Button| {
            if let Some(label) = button.label() {
                state.borrow().handle_service_action(&label);
            }
        }
    };

    sidebar.append(&search_entry);
    sidebar.append(&Separator::new(gtk4::Orientation::Horizontal));
    sidebar.append(&filter_controls);
    sidebar.append(&create_service_actions(action_callback));

    let state_search = Rc::clone(&state);
    search_entry.connect_search_changed(move |search| {
        let query = search.text().to_string();
        state_search
            .borrow()
            .current_query
            .borrow_mut()
            .clone_from(&query);
        state_search.borrow().update_visibility();
    });

    let state_status = Rc::clone(&state);
    state.borrow().status_combo.connect_changed(move |_| {
        state_status.borrow().update_visibility();
    });

    let state_enablement = Rc::clone(&state);
    state.borrow().enablement_combo.connect_changed(move |_| {
        state_enablement.borrow().update_visibility();
    });

    sidebar
}

fn build_main_content(state: Rc<RefCell<ServiceManagerState>>) -> Box {
    let services_container = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    let services_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .min_content_width(550)
        .child(&state.borrow().services_list)
        .hexpand(true)
        .vexpand(true)
        .build();

    services_container.append(&services_scroll);
    services_container
}

fn create_window(
    app: &Application,
    state: Rc<RefCell<ServiceManagerState>>,
    sidebar: Box,
    main_content: Box,
) -> Window {
    let sidebar_container = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .width_request(350)
        .build();

    let sidebar_scroll = ScrolledWindow::builder()
        .min_content_width(250)
        .child(&sidebar)
        .vexpand(true)
        .hexpand(false)
        .build();

    sidebar_container.append(&sidebar_scroll);

    let main_box = Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .hexpand(true)
        .vexpand(true)
        .build();

    main_box.append(&sidebar_container);
    main_box.append(&Separator::new(gtk4::Orientation::Vertical));
    main_box.append(&main_content);

    state.borrow().toast_overlay.set_child(Some(&main_box));

    let header = HeaderBar::new();
    header.pack_start(&Button::builder().icon_name("view-refresh").build());

    let vbox = Box::new(gtk4::Orientation::Vertical, 0);
    vbox.append(&header);
    vbox.append(&state.borrow().toast_overlay);

    Window::builder()
        .application(app)
        .default_width(1200)
        .default_height(800)
        .title("Service Manager")
        .content(&vbox)
        .build()
}

fn setup_refresh_button(button: Button, state: Rc<RefCell<ServiceManagerState>>) {
    button.connect_clicked(move |_| {
        state.borrow().refresh_services();
    });
}

fn get_selected_services(list_box: &ListBox) -> Vec<String> {
    list_box
        .selected_rows()
        .iter()
        .filter_map(|row| {
            let name = row.widget_name();
            if !name.is_empty() {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect()
}

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
        let status_matches = status_filter == "All" || format_status(&self.status) == status_filter;
        let enablement_matches =
            enablement_filter == "All" || format_enablement(&self.enablement) == enablement_filter;
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
        .orientation(gtk4::Orientation::Vertical)
        .spacing(6)
        .margin_start(12)
        .margin_end(12)
        .margin_top(9)
        .margin_bottom(9)
        .build();

    row_box.append(
        &Label::builder()
            .label(&service.name)
            .halign(gtk4::Align::Start)
            .css_classes(["heading"])
            .build(),
    );

    row_box.append(
        &Label::builder()
            .label(&service.description)
            .halign(gtk4::Align::Start)
            .wrap(true)
            .justify(gtk4::Justification::Left)
            .css_classes(["caption"])
            .build(),
    );

    row_box.append(&Separator::new(gtk4::Orientation::Horizontal));

    let info_box = Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .spacing(12)
        .build();

    info_box.append(
        &Label::builder()
            .label(&format!("Status: {}", format_status(&service.status)))
            .halign(gtk4::Align::Start)
            .css_classes(get_status_css_classes(&service.status))
            .build(),
    );

    info_box.append(
        &Label::builder()
            .label(&format!(
                "Enablement: {}",
                format_enablement(&service.enablement_status)
            ))
            .halign(gtk4::Align::Start)
            .css_classes(get_enablement_css_classes(&service.enablement_status))
            .build(),
    );

    row_box.append(&info_box);

    let row = ListBoxRow::builder()
        .name(&service_data.name)
        .child(&row_box)
        .build();

    (service_data, row)
}

pub fn create_filter_controls() -> (Box, ComboBoxText, ComboBoxText) {
    let main_box = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let group = adw::PreferencesGroup::builder()
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

fn create_combo_row(title: &str, options: &[&str]) -> (adw::ActionRow, gtk4::ComboBoxText) {
    let combo = ComboBoxText::builder()
        .valign(gtk4::Align::Center)
        .css_classes(["compact"])
        .build();
    options.iter().for_each(|option| {
        combo.append_text(option);
    });
    combo.set_active(Some(0));

    let row = adw::ActionRow::builder()
        .title(title)
        .activatable(false)
        .build();
    row.add_suffix(&combo);

    (row, combo)
}

pub fn create_service_actions<F: Fn(&Button) + 'static + Clone>(button_callback: F) -> Box {
    let main_box = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let group = adw::PreferencesGroup::builder()
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

fn create_action_buttons<F: Fn(&Button) + 'static + Clone>(
    title: &str,
    actions: &[(&str, &str, &str)],
    callback: &F,
) -> adw::ActionRow {
    let button_box = Box::builder()
        .css_classes(["linked"])
        .orientation(gtk4::Orientation::Horizontal)
        .halign(gtk4::Align::End)
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

    let row = adw::ActionRow::builder()
        .title(title)
        .activatable(false)
        .build();
    row.add_suffix(&button_box);
    row
}

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
