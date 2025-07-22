use adw::{Application, HeaderBar, Window, prelude::*};
use gtk4::{
    Align, Box, Label, ListBox, Orientation, Paned, PolicyType, ScrolledWindow, SearchEntry,
    Widget, glib,
};

const GLOBAL_MARGIN: i32 = 0;
const APP_ID: &str = "org.tobacco_linux.ServiceManager";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &adw::Application) {
    let scrolled_sidebar = wrap_in_scroller(&sidebar());
    scrolled_sidebar.set_size_request(250, -1);

    let scrolled_services = wrap_in_scroller(&service_list());
    scrolled_services.set_size_request(550, -1);

    let paned = Paned::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .vexpand(true)
        .hexpand(true)
        .build();
    paned.set_start_child(Some(&scrolled_sidebar));
    paned.set_end_child(Some(&scrolled_services));

    main_window(&app, &headerify(&paned)).present();
}

fn main_window(app: &Application, content: &impl IsA<Widget>) -> Window {
    Window::builder()
        .application(app)
        .default_width(800)
        .default_height(600)
        .title("Service Manager")
        .content(content)
        .build()
}

fn headerify(content: &impl IsA<Widget>) -> Box {
    let box_widget = Box::new(Orientation::Vertical, 0);
    box_widget.append(&HeaderBar::new());
    box_widget.append(content);
    box_widget
}

fn sidebar() -> ListBox {
    let search_bar = SearchEntry::builder()
        .css_classes(vec![String::from("inline")])
        .placeholder_text("Search...")
        .width_request(50)
        .build();

    let list_box = ListBox::builder()
        .margin_start(GLOBAL_MARGIN)
        .margin_end(GLOBAL_MARGIN)
        .margin_bottom(GLOBAL_MARGIN)
        .margin_top(GLOBAL_MARGIN)
        .css_classes(vec![String::from("navigation-sidebar")])
        .build();

    list_box.append(&search_bar);

    list_box
}

fn service_list() -> ListBox {
    let list_box = ListBox::builder().build();
    for number in 0..=100 {
        let label = Label::new(Some(&number.to_string()));
        label.set_size_request(-1, 30);
        list_box.append(&label);
    }

    list_box
}

fn wrap_in_scroller(child: &impl IsA<Widget>) -> ScrolledWindow {
    ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_width(360)
        .child(child)
        .build()
}
