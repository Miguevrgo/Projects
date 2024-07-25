use gio::Settings;
use gtk::prelude::*;
use gtk::{
    gio, glib, Align, Application, ApplicationWindow, Box, Button, Image, Orientation, Switch,
};

const APP_ID: &str = "org.gtk_rs.Algori";

fn build_id(app: &Application) {
    let settings = Settings::new(APP_ID);

    let is_dark_mode = settings.boolean("is-dark-mode");

    let dark_mode_switch = Switch::builder()
        .margin_bottom(48)
        .margin_top(48)
        .margin_start(48)
        .margin_end(48)
        .valign(Align::Center)
        .halign(Align::Center)
        .state(is_dark_mode)
        .build();

    let window_width = settings.int("window-width");
    let window_height = settings.int("window-height");
    let is_maximized = settings.boolean("is-maximized");

    dark_mode_switch.connect_state_set(move |_, is_enabled| {
        settings
            .set_boolean("is-dark-mode", is_enabled)
            .expect("Could not set dark mode");
        glib::Propagation::Proceed
    });

    let vbox = Box::new(Orientation::Horizontal, 5);
    vbox.append(&dark_mode_switch);

    let algorithms = vec![
        ("Array", "array.png"),
        ("Sorting", "sorting.gif"),
        ("Graph", "graph.gif"),
        ("Tree", "tree.gif"),
    ];

    let image_size: (i32, i32) = (window_width / 4, window_height / 4);

    for (name, image_path) in algorithms {
        let hbox = Box::new(Orientation::Vertical, 5);
        let button = Button::with_label(name);
        let image = Image::from_file(format!("assets/{}", image_path));
        image.set_size_request(image_size.0, image_size.1);

        button.connect_clicked(move |_| {
            println!("Clicked on {}", name);
        });

        hbox.append(&image);
        hbox.append(&button);
        vbox.append(&hbox);
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Algori")
        .default_width(window_width)
        .default_height(window_height)
        .child(&vbox)
        .build();

    if is_maximized {
        window.maximize();
    }

    window.present();
}

fn main() -> glib::ExitCode {
    std::env::set_var("GSETTINGS_SCHEMA_DIR", "./schemas");
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_id);

    app.run()
}
