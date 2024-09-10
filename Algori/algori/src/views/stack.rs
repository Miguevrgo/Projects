use crate::views::help::create_view_stack as create_help_view;
use gtk::{prelude::*, Box, Button, DrawingArea, Entry, Orientation};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

struct AppState {
    elements: RefCell<Vec<i32>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            elements: RefCell::new(Vec::new()),
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
    let push_entry = Entry::new();
    let push_button = Button::with_label("Push");
    let pop_button = Button::with_label("Pop");
    let help_button = Button::new();
    push_button.set_widget_name("push-button");
    pop_button.set_widget_name("pop-button");
    help_button.set_widget_name("help-button");

    controls.append(&home_button);
    controls.append(&push_entry);
    controls.append(&push_button);
    controls.append(&pop_button);
    controls.append(&help_button);

    view.append(&controls);
    let drawing_area = DrawingArea::new();
    drawing_area.set_vexpand(true);
    drawing_area.set_hexpand(true);
    view.append(&drawing_area);
    let state = Rc::new(AppState::new());
    drawing_area.set_draw_func({
        let state = state.clone();
        move |_, cr, width, height| {
            let elements = state.elements.borrow();
            let element_height = 30.0;
            let element_width = width as f64 - 20.0;
            let total_height = elements.len() as f64 * (element_height + 5.0) - 5.0;
            let offset_y = (height as f64 - total_height) / 2.0;
            cr.set_source_rgb(0.1568, 0.1725, 0.2039);
            cr.paint().unwrap();
            for (i, &element) in elements.iter().enumerate() {
                let y = offset_y + i as f64 * (element_height + 5.0);
                cr.set_source_rgb(0.0, 0.0, 1.0);
                cr.rectangle(10.0, y, element_width, element_height);
                cr.fill().unwrap();
                cr.set_source_rgb(1.0, 1.0, 1.0);
                cr.set_font_size(element_height / 2.5);
                let text = format!("{}", element);
                let extents = cr.text_extents(&text).unwrap();
                let text_x = 10.0 + (element_width - extents.width()) / 2.0;
                let text_y = y + (element_height + extents.height()) / 2.0;
                cr.move_to(text_x, text_y);
                cr.show_text(&text).unwrap();
            }
        }
    });
    push_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            let value = if let Ok(value) = push_entry.text().parse() {
                value
            } else {
                rand::thread_rng().gen_range(0..=100)
            };
            state.elements.borrow_mut().push(value);
            drawing_area.queue_draw();
        }
    });
    pop_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            state.elements.borrow_mut().pop();
            drawing_area.queue_draw();
        }
    });
    help_button.connect_clicked({
        let stack_clone = stack.clone();
        move |_| {
            let help_view = create_help_view(&stack_clone);
            stack_clone.add_named(&help_view, Some("Help"));
            stack_clone.set_visible_child_name("Help");
        }
    });
    view
}
