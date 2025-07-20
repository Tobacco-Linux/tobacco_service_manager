use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};
use gtk4 as gtk;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.tobacco_linux.service_manager")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(600)
            .title("Service Manager")
            .build();
        window.present();
    });

    app.run()
}
