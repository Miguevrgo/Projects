use gio::Settings;
use gtk::{gdk, prelude::*};
use gtk::{
    gio, glib, Align, Application, ApplicationWindow, Box, Button, CssProvider,
    EventControllerMotion, GestureClick, Grid, Image, Switch,
};

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

    let css_provider = CssProvider::new();
    css_provider.load_from_path("style.css");

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let algorithms = [
        ("Array".to_string(), "array.png".to_string()),
        ("Sorting".to_string(), "sorting.gif".to_string()),
        ("Graph".to_string(), "graph.gif".to_string()),
        ("Tree".to_string(), "tree.gif".to_string()),
        ("Linked List".to_string(), "linked.gif".to_string()),
    ];

    for (index, (name, image_path)) in algorithms.iter().enumerate() {
        let row = index / NUM_COLUMNS;
        let column = index % NUM_COLUMNS;

        let button = Button::with_label(name);
        button.set_size_request(window_width / 4, -1);
        let image = Image::from_file(format!("assets/{}", image_path));
        image.set_size_request(window_width / 4, window_height / 4);

        let box_container = Box::new(gtk::Orientation::Vertical, 0);
        box_container.append(&image);
        box_container.append(&button);

        let motion_controller = EventControllerMotion::new();
        let box_container_clone = box_container.clone();
        motion_controller.connect_enter({
            let box_container_clone = box_container_clone.clone();
            move |_, _, _| {
                box_container_clone.add_css_class("highlight");
            }
        });
        motion_controller.connect_leave({
            let box_container_clone = box_container_clone.clone();
            move |_| {
                box_container_clone.remove_css_class("highlight");
            }
        });

        box_container.add_controller(motion_controller);

        let gesture_click = GestureClick::new();
        let name_clone = name.clone();
        gesture_click.connect_released(move |_, _, _, _| {
            println!("Clicked on {}", name_clone);
            //TODO: Logic of new page
        });

        box_container.add_controller(gesture_click);

        grid.attach(&box_container, column as i32, (row * 2) as i32, 1, 2);
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
