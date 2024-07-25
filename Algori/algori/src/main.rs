use gio::Settings;
use gtk::prelude::*;
use gtk::{gio, glib, Align, Application, ApplicationWindow, Button, Grid, Image, Switch};

const APP_ID: &str = "org.gtk_rs.Algori";
const NUM_COLUMNS: usize = 4;

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

    let grid = Grid::new();
    grid.set_column_homogeneous(true);
    grid.set_row_homogeneous(true);

    let algorithms = [
        ("Array", "array.png"),
        ("Sorting", "sorting.gif"),
        ("Graph", "graph.gif"),
        ("Tree", "tree.gif"),
    ];

    for (index, (name, image_path)) in algorithms.iter().enumerate() {
        let row = index / NUM_COLUMNS;
        let column = index % NUM_COLUMNS;

        let button = Button::with_label(name);
        button.set_size_request(window_width / 4, -1);
        let image = Image::from_file(format!("assets/{}", image_path));
        image.set_size_request(window_width / 4, window_height / 4);

        let name = name.to_string();

        button.connect_clicked(move |_| {
            println!("Clicked on {}", name);
        });

        grid.attach(&image, column as i32, (row * 2) as i32, 1, 1);
        grid.attach(&button, column as i32, (row * 2 + 1) as i32, 1, 1);
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Algori")
        .default_width(window_width)
        .default_height(window_height)
        .child(&grid)
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
