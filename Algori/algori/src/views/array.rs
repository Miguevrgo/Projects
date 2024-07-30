use gtk::{prelude::*, Box, Button, DrawingArea};
use rand::Rng;

///TODO: Change Constant to User input
const NUM_ELEMENTS: usize = 10;
const ELEMENT_WIDTH: i32 = 75;
const MAX_ELEMENT_HEIGHT: i32 = 400;

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

    drawing_area.set_draw_func(move |_, cr, _width, height| {
        let mut random = rand::thread_rng();
        let heights: Vec<i32> = (0..NUM_ELEMENTS)
            .map(|_| random.gen_range(20..=MAX_ELEMENT_HEIGHT))
            .collect();

        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.paint().unwrap();

        for (i, &elem_height) in heights.iter().enumerate() {
            let x = i as f64 * (ELEMENT_WIDTH + 5) as f64;
            let y = height as f64 - elem_height as f64;

            cr.set_source_rgb(0.0, 0.0, 1.0);
            cr.rectangle(x, y, ELEMENT_WIDTH as f64, elem_height as f64);
            cr.fill().unwrap();
        }
    });

    view.append(&drawing_area);

    view
}
