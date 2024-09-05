use gtk::{prelude::*, Box, Button, DrawingArea, Entry, Orientation};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use rand::Rng;

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
    controls.append(&home_button);

    let enqueue_entry = Entry::new();
    let enqueue_button = Button::with_label("Enqueue");
    enqueue_button.set_widget_name("enqueue-button");
    let dequeue_button = Button::with_label("Dequeue");
    dequeue_button.set_widget_name("dequeue-button");

    controls.append(&enqueue_entry);
    controls.append(&enqueue_button);
    controls.append(&dequeue_button);
    view.append(&controls);

    // Crear un contenedor para los elementos de la cola
    let queue_container = Box::new(Orientation::Vertical, 10);
    queue_container.set_widget_name("queue-container");
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

            // Crear un nuevo widget para representar el elemento
            let element = Button::with_label(&format!("{}", value));
            element.set_widget_name("square-element"); // Aplicar clase CSS
            queue_container.append(&element);
            queue_container.show(); // Asegurar que el nuevo widget sea visible
        }
    });

    dequeue_button.connect_clicked({
        let state = state.clone();
        let queue_container = queue_container.clone();
        move |_| {
            if let Some(_) = state.queue.borrow_mut().pop_front() {
                if let Some(first_child) = queue_container.first_child() {
                    queue_container.remove(&first_child); // Eliminar el primer widget en la cola
                }
            }
        }
    });

    view
}
