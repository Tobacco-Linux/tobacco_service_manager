use super::views::{
    ServiceData, create_filter_controls, create_service_entry, update_service_visibility,
};
use crate::backend::SystemdServiceManager;
use crate::frontend::views::create_service_actions;
use adw::{Application, HeaderBar, Toast, ToastOverlay, ToastPriority, Window, prelude::*};
use gtk4::{
    Box, Button, ComboBoxText, ListBox, ListBoxRow, Orientation, ScrolledWindow, SearchEntry,
    Separator,
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
        .orientation(Orientation::Vertical)
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
    sidebar.append(&Separator::new(Orientation::Horizontal));
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
        .orientation(Orientation::Vertical)
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
        .orientation(Orientation::Vertical)
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
        .orientation(Orientation::Horizontal)
        .hexpand(true)
        .vexpand(true)
        .build();

    main_box.append(&sidebar_container);
    main_box.append(&Separator::new(Orientation::Vertical));
    main_box.append(&main_content);

    state.borrow().toast_overlay.set_child(Some(&main_box));

    let header = HeaderBar::new();
    header.pack_start(&Button::builder().icon_name("view-refresh").build());

    let vbox = Box::new(Orientation::Vertical, 0);
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
