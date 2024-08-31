use gtk::{prelude::*, Box, Button, ComboBoxText, DrawingArea, Entry, Label, Orientation};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

const MARGIN: f64 = 20.0;
const NUM_ELEMENTS: usize = 10;

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

    let sort_label = Label::new(Some("Sorting Method:"));
    controls.append(&sort_label);

    let sort_combo = ComboBoxText::new();
    sort_combo.append_text("Bubble Sort");
    sort_combo.append_text("Selection Sort");
    sort_combo.set_active(Some(0));
    controls.append(&sort_combo);

    let apply_button = Button::with_label("Apply");
    controls.append(&apply_button);

    view.append(&controls);

    let drawing_area = DrawingArea::new();
    drawing_area.set_vexpand(true);
    drawing_area.set_hexpand(true);

    let heights: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let heights_clone = heights.clone();

    let current_indices: Rc<RefCell<Option<(usize, usize)>>> = Rc::new(RefCell::new(None));
    let current_indices_clone = current_indices.clone();

    drawing_area.set_draw_func(move |_, cr, width, height| {
        let heights = heights_clone.borrow();
        let num_elements = heights.len();
        let element_width = (width as f64 - 2.0 * MARGIN) / num_elements as f64 - 5.0;
        let total_width = num_elements as f64 * (element_width + 5.0) - 5.0;
        let offset_x = (width as f64 - total_width) / 2.0;

        cr.set_source_rgb(0.1568, 0.1725, 0.2039);
        cr.paint().unwrap();

        for (i, &elem_height) in heights.iter().enumerate() {
            let x = offset_x + i as f64 * (element_width + 5.0);
            let y = height as f64 - MARGIN - elem_height as f64;

            if let Some((a, b)) = *current_indices_clone.borrow() {
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
    });

    view.append(&drawing_area);

    let sort_button = Button::with_label("Sort Step by Step");
    view.append(&sort_button);

    let shuffle_button = Button::with_label("Shuffle");
    view.append(&shuffle_button);

    let sorting_index: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    let sorted: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));

    let drawing_area_clone = drawing_area.clone();
    let heights_clone = heights.clone();
    let sorting_index_clone = sorting_index.clone();
    let sorted_clone = sorted.clone();
    let current_indices_clone = current_indices.clone();

    sort_button.connect_clicked(move |_| {
        bubble_sort_step(
            &drawing_area_clone,
            &heights_clone,
            &sorting_index_clone,
            &sorted_clone,
            &current_indices_clone,
        );
    });

    let drawing_area_clone = drawing_area.clone();
    let heights_clone = heights.clone();
    let sorting_index_clone = sorting_index.clone();
    let sorted_clone = sorted.clone();
    let current_indices_clone_shuffle = current_indices.clone();

    shuffle_button.connect_clicked(move |_| {
        let mut heights = heights_clone.borrow_mut();
        let mut rng = rand::thread_rng();

        for i in 0..heights.len() {
            let j = rng.gen_range(0..heights.len());
            heights.swap(i, j);
        }

        *sorting_index_clone.borrow_mut() = 0;
        *sorted_clone.borrow_mut() = false;
        *current_indices_clone_shuffle.borrow_mut() = None;

        drawing_area_clone.queue_draw();
    });

    let heights_clone = heights.clone();
    apply_button.connect_clicked(move |_| {
        let num_elements: usize = elements_entry.text().parse().unwrap_or(NUM_ELEMENTS);
        let mut heights = heights_clone.borrow_mut();
        *heights = (0..num_elements)
            .map(|_| rand::thread_rng().gen_range(100..=700))
            .collect();
        drawing_area.queue_draw();
    });

    view
}

fn bubble_sort_step(
    drawing_area: &DrawingArea,
    heights: &Rc<RefCell<Vec<i32>>>,
    sorting_index: &Rc<RefCell<usize>>,
    sorted: &Rc<RefCell<bool>>,
    current_indices: &Rc<RefCell<Option<(usize, usize)>>>,
) {
    let mut heights = heights.borrow_mut();
    let mut i = *sorting_index.borrow();
    let mut swapped = false;

    if *sorted.borrow() {
        return;
    }

    if i < heights.len() - 1 {
        for j in 0..heights.len() - 1 - i {
            *current_indices.borrow_mut() = Some((j, j + 1));
            drawing_area.queue_draw();
            if heights[j] > heights[j + 1] {
                heights.swap(j, j + 1);
                swapped = true;
            }
        }
        i += 1;
    } else {
        *sorted.borrow_mut() = true;
    }

    if swapped {
        *sorting_index.borrow_mut() = i;
    } else {
        *sorted.borrow_mut() = true;
    }

    *current_indices.borrow_mut() = None;
    drawing_area.queue_draw();
}