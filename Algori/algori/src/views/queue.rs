use gtk::{prelude::*, Box, Button, Entry, Orientation};
use rand::Rng;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct AppState {
    queue: RefCell<VecDeque<i32>>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            queue: RefCell::new(VecDeque::new()),
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

    let enqueue_entry = Entry::new();
    let enqueue_button = Button::with_label("Enqueue");
    let dequeue_button = Button::with_label("Dequeue");
    let help_button = Button::new();
    enqueue_button.set_widget_name("push-button");
    dequeue_button.set_widget_name("pop-button");
    help_button.set_widget_name("help-button");

    controls.append(&home_button);
    controls.append(&enqueue_entry);
    controls.append(&enqueue_button);
    controls.append(&dequeue_button);
    controls.append(&help_button); //TODO: connect_clicked
    view.append(&controls);

    let queue_container = Box::new(Orientation::Vertical, 10);
    queue_container.set_widget_name("drawing-background");
    queue_container.set_vexpand(true);
    queue_container.set_hexpand(true);
    view.append(&queue_container);

    let state = Rc::new(AppState::new());

    enqueue_button.connect_clicked({
        let state = state.clone();
        let queue_container = queue_container.clone();
        let enqueue_entry = enqueue_entry.clone();
        move |_| {
            let value = if let Ok(value) = enqueue_entry.text().parse() {
                value
            } else {
                rand::thread_rng().gen_range(0..=100)
            };

            state.queue.borrow_mut().push_back(value);

            let element = Button::with_label(&format!("{}", value));
            element.set_widget_name("square-element");
            queue_container.append(&element);
            queue_container.show();
        }
    });

    dequeue_button.connect_clicked({
        let state = state.clone();
        let queue_container = queue_container.clone();
        move |_| {
            if state.queue.borrow_mut().pop_front().is_some() {
                if let Some(first_child) = queue_container.first_child() {
                    queue_container.remove(&first_child);
                }
            }
        }
    });

    view
}
