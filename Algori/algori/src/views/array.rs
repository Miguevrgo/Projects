use gtk::{prelude::*, Box, Button, ComboBoxText, DrawingArea, Entry, Label, Orientation};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

const MARGIN: f64 = 20.0;
const NUM_ELEMENTS: usize = 10;

struct AppState {
    heights: RefCell<Vec<i32>>,
    sorting_index: RefCell<usize>,
    sorted: RefCell<bool>,
    current_indices: RefCell<Option<(usize, usize)>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            heights: RefCell::new(Vec::new()),
            sorting_index: RefCell::new(0),
            sorted: RefCell::new(false),
            current_indices: RefCell::new(None),
        }
    }
}

pub fn create_view(stack: &gtk::Stack) -> Box {
    let view = Box::new(Orientation::Horizontal, 10);
    let stack_clone = stack.clone();
    let home_button = Button::with_label("Home");

    home_button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("Home");
    });

    let controls = Box::new(Orientation::Vertical, 10);
    controls.append(&home_button);

    let elements_label = Label::new(Some("Number of Elements:"));
    controls.append(&elements_label);

    let elements_entry = Entry::new();
    elements_entry.set_text("10");
    controls.append(&elements_entry);

    let create_button = Button::with_label("Create Array");
    controls.append(&create_button);

    let insert_label = Label::new(Some("Insert Element:"));
    controls.append(&insert_label);

    let insert_entry = Entry::new();
    controls.append(&insert_entry);

    let insert_button = Button::with_label("Insert");
    controls.append(&insert_button);

    let delete_label = Label::new(Some("Delete Element:"));
    controls.append(&delete_label);

    let delete_entry = Entry::new();
    controls.append(&delete_entry);

    let delete_button = Button::with_label("Delete");
    controls.append(&delete_button);

    view.append(&controls);

    let drawing_area = DrawingArea::new();
    drawing_area.set_vexpand(true);
    drawing_area.set_hexpand(true);

    let state = Rc::new(AppState::new());

    drawing_area.set_draw_func({
        let state = state.clone();
        move |_, cr, width, height| {
            let heights = state.heights.borrow();
            let num_elements = heights.len();
            let element_width = (width as f64 - 2.0 * MARGIN) / num_elements as f64 - 5.0;
            let total_width = num_elements as f64 * (element_width + 5.0) - 5.0;
            let offset_x = (width as f64 - total_width) / 2.0;

            cr.set_source_rgb(0.1568, 0.1725, 0.2039);
            cr.paint().unwrap();

            for (i, &elem_height) in heights.iter().enumerate() {
                let x = offset_x + i as f64 * (element_width + 5.0);
                let y = height as f64 - MARGIN - elem_height as f64;

                if let Some((a, b)) = *state.current_indices.borrow() {
                    if i == a || i == b {
                        cr.set_source_rgb(1.0, 0.0, 0.0);
                    } else {
                        cr.set_source_rgb(0.0, 0.0, 1.0);
                    }
                } else {
                    cr.set_source_rgb(0.0, 0.0, 1.0);
                }

                cr.rectangle(x, y, element_width, elem_height as f64);
                cr.fill().unwrap();

                // Draw memory address and value
                cr.set_source_rgb(1.0, 1.0, 1.0);
                cr.move_to(x, y - 10.0);
                cr.show_text(&format!("{:p}", &heights[i])).unwrap();
                cr.move_to(x, y + elem_height as f64 + 10.0);
                cr.show_text(&format!("{}", heights[i])).unwrap();
            }
        }
    });

    view.append(&drawing_area);

    create_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            let num_elements: usize = elements_entry.text().parse().unwrap_or(NUM_ELEMENTS);
            let mut heights = state.heights.borrow_mut();
            *heights = (0..num_elements)
                .map(|_| rand::thread_rng().gen_range(100..=700))
                .collect();
            drawing_area.queue_draw();
        }
    });

    insert_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            let value: i32 = insert_entry.text().parse().unwrap_or(0);
            let mut heights = state.heights.borrow_mut();
            heights.push(value);
            drawing_area.queue_draw();
        }
    });

    delete_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            let value: i32 = delete_entry.text().parse().unwrap_or(0);
            let mut heights = state.heights.borrow_mut();
            if let Some(pos) = heights.iter().position(|&x| x == value) {
                heights.remove(pos);
            }
            drawing_area.queue_draw();
        }
    });

    view
}