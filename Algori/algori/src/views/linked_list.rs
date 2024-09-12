use gtk::{prelude::*, Box, Button, DrawingArea, Entry, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn add_front(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.take(),
        }));
        if self.tail.is_none() {
            self.tail = Some(new_node.clone());
        }
        self.head = Some(new_node);
    }

    fn add_back(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(Node { value, next: None }));
        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }
        self.tail = Some(new_node);
    }

    fn remove(&mut self) {
        if let Some(head) = self.head.take() {
            self.head = head.borrow_mut().next.take();
            if self.head.is_none() {
                self.tail = None;
            }
        }
    }

    fn iter(&self) -> LinkedListIterator {
        LinkedListIterator {
            current: self.head.clone(),
        }
    }
}

struct LinkedListIterator {
    current: Option<Rc<RefCell<Node>>>,
}

impl Iterator for LinkedListIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current| {
            let current_borrowed = current.borrow();
            self.current.clone_from(&current_borrowed.next);
            current_borrowed.value
        })
    }
}

struct AppState {
    list: RefCell<LinkedList>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            list: RefCell::new(LinkedList::new()),
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

    let add_entry = Entry::new();
    let add_front_button = Button::with_label("Add Front");
    add_front_button.set_widget_name("add-front-button");
    let add_back_button = Button::with_label("Add Back");
    add_back_button.set_widget_name("add-back-button");
    let remove_button = Button::with_label("Remove");
    remove_button.set_widget_name("remove-button");

    controls.append(&add_entry);
    controls.append(&add_front_button);
    controls.append(&add_back_button);
    controls.append(&remove_button);
    view.append(&controls);

    let drawing_area = DrawingArea::new();
    drawing_area.set_vexpand(true);
    drawing_area.set_hexpand(true);
    view.append(&drawing_area);

    let state = Rc::new(AppState::new());

    drawing_area.set_draw_func({
        let state = state.clone();
        move |_, cr, width, height| {
            let list = state.list.borrow();
            let num_elements = list.iter().count();
            if num_elements == 0 {
                return;
            }

            let total_spacing = width as f64 - 20.0;
            let node_width = total_spacing / (num_elements as f64 * 1.5 - 0.5);
            let node_height = node_width / 2.0;
            let spacing = node_width / 2.0;
            let mut current_x = 10.0;

            cr.set_source_rgb(0.1568, 0.1725, 0.2039);
            cr.paint().unwrap();

            for value in list.iter() {
                cr.set_source_rgb(0.0, 0.0, 1.0);
                cr.rectangle(
                    current_x,
                    height as f64 / 2.0 - node_height / 2.0,
                    node_width,
                    node_height,
                );
                cr.fill().unwrap();

                cr.set_source_rgb(1.0, 1.0, 1.0);
                cr.set_font_size(node_height / 2.0);
                let text = format!("{}", value);
                let extents = cr.text_extents(&text).unwrap();
                let text_x = current_x + (node_width - extents.width()) / 2.0;
                let text_y = height as f64 / 2.0 - node_height / 2.0
                    + (node_height + extents.height()) / 2.0;
                cr.move_to(text_x, text_y);
                cr.show_text(&text).unwrap();

                current_x += node_width + spacing;

                // Draw arrow
                if current_x < width as f64 {
                    cr.set_source_rgb(1.0, 1.0, 1.0);
                    cr.move_to(current_x - spacing, height as f64 / 2.0);
                    cr.line_to(current_x - spacing / 2.0, height as f64 / 2.0);
                    cr.stroke().unwrap();
                    cr.move_to(current_x - spacing / 2.0 - 5.0, height as f64 / 2.0 - 5.0);
                    cr.line_to(current_x - spacing / 2.0, height as f64 / 2.0);
                    cr.line_to(current_x - spacing / 2.0 - 5.0, height as f64 / 2.0 + 5.0);
                    cr.fill().unwrap();
                }
            }
        }
    });

    add_front_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        let add_entry = add_entry.clone();
        move |_| {
            if let Ok(value) = add_entry.text().parse() {
                state.list.borrow_mut().add_front(value);
                drawing_area.queue_draw();
            }
        }
    });

    add_back_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        let add_entry = add_entry.clone();
        move |_| {
            if let Ok(value) = add_entry.text().parse() {
                state.list.borrow_mut().add_back(value);
                drawing_area.queue_draw();
            }
        }
    });

    remove_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            state.list.borrow_mut().remove();
            drawing_area.queue_draw();
        }
    });

    view
}

