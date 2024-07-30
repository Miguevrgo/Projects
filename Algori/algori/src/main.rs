use gio::Settings;
use gtk::{gdk, prelude::*};
use gtk::{
    gio, glib, Align, Application, ApplicationWindow, Box, Button, CssProvider,
    EventControllerMotion, GestureClick, Grid, Image, ScrolledWindow, Stack, Switch,
};

mod views;

const APP_ID: &str = "org.gtk_rs.Algori";
const NUM_COLUMNS: usize = 4;

/// Creates the default home view for a scrollable window, which displays all the algorithms
/// in a grid layout made up of 4-element rows consisting of an image and a button.
fn create_home_view(stack: &Stack, algorithms: &[(String, String)]) -> Box {
    let grid = Grid::builder().row_spacing(5).column_spacing(20).build();
    grid.set_column_homogeneous(true);
    grid.set_row_homogeneous(true);

    let css_provider = CssProvider::new();
    css_provider.load_from_path("style.css");

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    for (index, (name, image_path)) in algorithms.iter().enumerate() {
        let row = index / NUM_COLUMNS;
        let column = index % NUM_COLUMNS;

        let button = Button::with_label(name);
        button.set_size_request(200, -1);
        let image = Image::from_file(format!("assets/{}", image_path));
        image.set_size_request(275, 275);

        let box_container = Box::new(gtk::Orientation::Vertical, 0);

        if row == 0 {
            box_container.set_margin_top(40);
        }
        if row == (algorithms.len() / NUM_COLUMNS) {
            box_container.set_margin_bottom(10);
        }
        if column == 0 {
            box_container.set_margin_start(30);
        }
        if column == (NUM_COLUMNS - 1) {
            box_container.set_margin_end(30);
        }

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
        let stack_clone = stack.clone();
        gesture_click.connect_released(move |_, _, _, _| {
            stack_clone.set_visible_child_name(&name_clone);
        });

        box_container.add_controller(gesture_click);

        grid.attach(&box_container, column as i32, (row * 2) as i32, 1, 2);
    }

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .child(&grid)
        .build();

    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);

    let home_view = Box::new(gtk::Orientation::Vertical, 0);
    home_view.append(&scrolled_window);

    home_view
}

fn build_ui(app: &Application) {
    let settings = Settings::new(APP_ID);

    let is_dark_mode = settings.boolean("is-dark-mode");

    let dark_mode_switch = Switch::builder()
        .margin_bottom(48)
        .margin_top(48)
        .margin_start(48)
        .margin_end(48)
        .valign(Align::End)
        .halign(Align::End)
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

    let stack = Stack::new();

    let algorithms = [
        ("Array".to_string(), "array.svg".to_string()),
        ("Sorting".to_string(), "sorting.png".to_string()),
        ("Graph".to_string(), "graph.png".to_string()),
        ("Tree".to_string(), "tree.svg".to_string()),
        ("Linked List".to_string(), "linked_list.svg".to_string()),
        ("Hash Table".to_string(), "hash_table.svg".to_string()),
        ("Bit Manipulation".to_string(), "bitwise.svg".to_string()),
        ("Math".to_string(), "math.svg".to_string()),
        ("Stack".to_string(), "stack.svg".to_string()),
        ("Queue".to_string(), "queue.svg".to_string()),
        ("Heap".to_string(), "heap.svg".to_string()),
        ("Trie".to_string(), "queue.svg".to_string()),
        ("Binary Search".to_string(), "queue.svg".to_string()),
        ("Dijkstra".to_string(), "queue.svg".to_string()),
    ];

    let home_view = create_home_view(&stack, &algorithms);
    stack.add_named(&home_view, Some("Home"));

    for (name, _) in &algorithms {
        let algorithm_view = match name.as_str() {
            //TODO: Implement each view in views/
            "Array" => views::array::create_view(&stack),
            "Sorting" => views::sorting::create_view(&stack),
            "Graph" => views::graph::create_view(&stack),
            "Tree" => views::tree::create_view(&stack),
            "Linked List" => views::linked_list::create_view(&stack),
            "Hash Table" => views::hash_table::create_view(&stack),
            "Bit Manipulation" => views::bit_manipulation::create_view(&stack),
            "Math" => views::math::create_view(&stack),
            "Stack" => views::stack::create_view(&stack),
            "Queue" => views::queue::create_view(&stack),
            "Heap" => views::heap::create_view(&stack),
            "Trie" => views::trie::create_view(&stack),
            "Binary Search" => views::binary_search::create_view(&stack),
            "Dijkstra" => views::dijkstra::create_view(&stack),
            _ => Box::new(gtk::Orientation::Vertical, 10),
        };
        stack.add_named(&algorithm_view, Some(name));
    }

    stack.set_visible_child_name("Home");

    let main_container = Box::new(gtk::Orientation::Vertical, 0);
    main_container.append(&stack);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Algori")
        .default_width(window_width)
        .default_height(window_height)
        .child(&main_container)
        .build();

    if is_maximized {
        window.maximize();
    }

    window.present();
}

fn main() -> glib::ExitCode {
    std::env::set_var("GSETTINGS_SCHEMA_DIR", "./schemas");
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}
