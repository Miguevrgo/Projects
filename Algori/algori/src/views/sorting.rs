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
    home_button.set_widget_name("home-button");

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

    let sort_label = Label::new(Some("Sorting Method:"));
    controls.append(&sort_label);

    let sort_combo = ComboBoxText::new();
    sort_combo.append_text("Bubble Sort");
    sort_combo.append_text("Selection Sort");
    sort_combo.set_active(Some(0));
    controls.append(&sort_combo);

    let apply_button = Button::with_label("Apply");
    controls.append(&apply_button);

    let help_button = Button::new();
    help_button.set_widget_name("help-button");
    controls.append(&help_button);

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
            }
        }
    });

    view.append(&drawing_area);
    let controls_array = Box::new(Orientation::Vertical, 10);

    let sort_button = Button::with_label("Sort Step by Step");
    sort_button.set_widget_name("sort-shuffle-button");
    controls_array.append(&sort_button);
    sort_button.set_vexpand(true);

    let shuffle_button = Button::with_label("Shuffle");
    shuffle_button.set_widget_name("sort-shuffle-button");
    controls_array.append(&shuffle_button);
    shuffle_button.set_vexpand(true);

    view.append(&controls_array);

    sort_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            bubble_sort_step(&drawing_area, &state);
        }
    });

    shuffle_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            let mut heights = state.heights.borrow_mut();
            let mut rng = rand::thread_rng();

            for i in 0..heights.len() {
                let j = rng.gen_range(0..heights.len());
                heights.swap(i, j);
            }

            *state.sorting_index.borrow_mut() = 0;
            *state.sorted.borrow_mut() = false;
            *state.current_indices.borrow_mut() = None;

            drawing_area.queue_draw();
        }
    });

    apply_button.connect_clicked({
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

    view
}

fn bubble_sort_step(drawing_area: &DrawingArea, state: &Rc<AppState>) {
    let mut heights = state.heights.borrow_mut();
    let mut i = *state.sorting_index.borrow();
    let mut swapped = false;

    if *state.sorted.borrow() {
        return;
    }

    if i < heights.len() - 1 {
        for j in 0..heights.len() - 1 - i {
            *state.current_indices.borrow_mut() = Some((j, j + 1));
            drawing_area.queue_draw();
            if heights[j] > heights[j + 1] {
                heights.swap(j, j + 1);
                swapped = true;
            }
        }
        i += 1;
    } else {
        *state.sorted.borrow_mut() = true;
    }

    if swapped {
        *state.sorting_index.borrow_mut() = i;
    } else {
        *state.sorted.borrow_mut() = true;
    }

    *state.current_indices.borrow_mut() = None;
    drawing_area.queue_draw();
}
