use gtk::{prelude::*, Box, Button, DrawingArea};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

const NUM_ELEMENTS: usize = 10;
const ELEMENT_WIDTH: i32 = 75;
const MAX_ELEMENT_HEIGHT: i32 = 400;
const MARGIN: f64 = 20.0;

pub fn create_view(stack: &gtk::Stack) -> Box {
    let view = Box::new(gtk::Orientation::Vertical, 10);
    let stack_clone = stack.clone();
    let home_button = Button::with_label("Home");

    home_button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("Home");
    });
    view.append(&home_button);

    let drawing_area = DrawingArea::new();
    drawing_area.set_vexpand(true);
    drawing_area.set_hexpand(true);

    let heights: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(
        (0..NUM_ELEMENTS)
            .map(|_| rand::thread_rng().gen_range(20..=MAX_ELEMENT_HEIGHT))
            .collect(),
    ));
    let heights_clone = heights.clone();

    drawing_area.set_draw_func(move |_, cr, width, height| {
        let heights = heights_clone.borrow();
        let total_width = NUM_ELEMENTS as f64 * (ELEMENT_WIDTH + 5) as f64 - 5.0;
        let offset_x = (width as f64 - total_width) / 2.0;

        // Dibujar el fondo
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.paint().unwrap();

        // Dibujar los rectángulos centrados
        for (i, &elem_height) in heights.iter().enumerate() {
            let x = offset_x + i as f64 * (ELEMENT_WIDTH + 5) as f64;
            let y = height as f64 - MARGIN - elem_height as f64;

            cr.set_source_rgb(0.0, 0.0, 1.0);
            cr.rectangle(x, y, ELEMENT_WIDTH as f64, elem_height as f64);
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
    sort_button.connect_clicked(move |_| {
        let mut heights = heights_clone.borrow_mut();
        let mut i = *sorting_index_clone.borrow();
        let mut swapped = false;

        if *sorted_clone.borrow() {
            return;
        }

        if i < NUM_ELEMENTS - 1 {
            for j in 0..NUM_ELEMENTS - 1 - i {
                if heights[j] > heights[j + 1] {
                    heights.swap(j, j + 1);
                    swapped = true;
                }
            }
            i += 1;
        } else {
            *sorted_clone.borrow_mut() = true;
        }

        if swapped {
            *sorting_index_clone.borrow_mut() = i;
        } else {
            *sorted_clone.borrow_mut() = true;
        }

        drawing_area_clone.queue_draw();
    });

    let drawing_area_clone = drawing_area.clone();
    let heights_clone = heights.clone();
    let sorting_index_clone = sorting_index.clone();
    let sorted_clone = sorted.clone();
    shuffle_button.connect_clicked(move |_| {
        let mut heights = heights_clone.borrow_mut();
        let mut rng = rand::thread_rng();

        for i in 0..heights.len() {
            let j = rng.gen_range(0..heights.len());
            heights.swap(i, j);
        }

        *sorting_index_clone.borrow_mut() = 0;
        *sorted_clone.borrow_mut() = false;

        drawing_area_clone.queue_draw();
    });

    view
}