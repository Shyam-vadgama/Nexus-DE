use gtk4::prelude::*;
use libadwaita::prelude::*;
use libadwaita::Application;
use tracing::info;

const APP_ID: &str = "org.nexus.Settings";

fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting NEXUS Settings (ID: {})", APP_ID);

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // If dry-run test mode, exit cleanly
    if std::env::args().any(|arg| arg == "--test-run") {
        info!("Dry run complete.");
        return;
    }

    app.run();
}

fn build_ui(app: &Application) {
    let window = libadwaita::ApplicationWindow::builder()
        .application(app)
        .title("NEXUS Settings")
        .default_width(850)
        .default_height(600)
        .build();

    let content = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(12)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    let label = gtk4::Label::builder()
        .label("NEXUS Desktop Environment Settings")
        .css_classes(vec!["title-1".to_string()])
        .build();

    content.append(&label);
    window.set_content(Some(&content));
    window.present();
}
