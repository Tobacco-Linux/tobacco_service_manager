use adw::{
    Application,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib,
};
mod backend;
mod frontend;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.tobacco_linux.ServiceManager")
        .build();

    app.connect_activate(|app| {
        frontend::build_ui(app);
    });

    app.run()
}
