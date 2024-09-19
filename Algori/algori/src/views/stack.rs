use crate::views::help::create_view_stack as create_help_view;
use gtk::{prelude::*, Box, Button, Entry, Orientation};
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

    let stack_container = Box::new(Orientation::Vertical, 10);
    stack_container.set_widget_name("drawing-background");
    stack_container.set_hexpand(true);
    stack_container.set_vexpand(true);
    view.append(&stack_container);

    let state = Rc::new(AppState::new());

    push_button.connect_clicked({
        let state = state.clone();
        let stack_container = stack_container.clone();
        let push_entry = push_entry.clone();
        move |_| {
            let value = if let Ok(value) = push_entry.text().parse() {
                value
            } else {
                rand::thread_rng().gen_range(0..=100)
            };

            state.elements.borrow_mut().push(value);

            let element = Button::with_label(&format!("{}", value));
            element.set_widget_name("square-element");
            stack_container.append(&element);
            stack_container.show();
        }
    });

    pop_button.connect_clicked({
        let state = state.clone();
        let stack_container = stack_container.clone();
        move |_| {
            if state.elements.borrow_mut().pop().is_some() {
                if let Some(last_child) = stack_container.last_child() {
                    stack_container.remove(&last_child);
                }
            }
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
