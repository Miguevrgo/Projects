use gtk::prelude::*;
use gtk::{Button, ComboBoxText, DrawingArea, Entry, Label, Orientation};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub struct Bst<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> Bst<T> {
    pub fn new() -> Self {
        Bst { root: None }
    }
}

impl<T> Bst<T>
where
    T: Ord + Clone,
{
    pub fn insert(&mut self, new_val: T) {
        let new_node = Box::new(Node {
            value: new_val,
            left: None,
            right: None,
        });
        Self::push_node(new_node, &mut self.root);
    }

    pub fn delete(&mut self, val: T) {
        self.root = Self::delete_node(self.root.take(), val);
    }

    fn delete_node(node: Option<Box<Node<T>>>, val: T) -> Option<Box<Node<T>>> {
        match node {
            Some(mut n) => {
                use std::cmp::Ordering;
                match val.cmp(&n.value) {
                    Ordering::Less => {
                        n.left = Self::delete_node(n.left.take(), val);
                        Some(n)
                    }
                    Ordering::Greater => {
                        n.right = Self::delete_node(n.right.take(), val);
                        Some(n)
                    }
                    Ordering::Equal => {
                        if n.left.is_none() {
                            return n.right;
                        }
                        if n.right.is_none() {
                            return n.left;
                        }
                        let min_larger_node = Self::find_min(n.right.take().unwrap());
                        n.value = min_larger_node.value.clone();
                        n.right = Self::delete_node(n.right.take(), n.value.clone());
                        Some(n)
                    }
                }
            }
            None => None,
        }
    }

    fn find_min(mut node: Box<Node<T>>) -> Box<Node<T>> {
        while let Some(left) = node.left.take() {
            node = left;
        }
        node
    }

    fn push_node(new_node: Box<Node<T>>, current_node: &mut Option<Box<Node<T>>>) {
        if let Some(node) = current_node {
            use std::cmp::Ordering;
            match node.value.cmp(&new_node.value) {
                Ordering::Less | Ordering::Equal => Self::push_node(new_node, &mut node.right),
                Ordering::Greater => Self::push_node(new_node, &mut node.left),
            }
        } else {
            *current_node = Some(new_node);
        }
    }
}

fn draw_tree<T: std::fmt::Display>(
    tree: &Bst<T>,
    cr: &gtk::cairo::Context,
    width: f64,
    height: f64,
) {
    if let Some(ref root) = tree.root {
        let x = width / 2.0;
        let y = 50.0;
        let offset = width / 4.0;
        draw_node(root, cr, x, y, offset, height);
    }
}

fn draw_node<T: std::fmt::Display>(
    node: &Node<T>,
    cr: &gtk::cairo::Context,
    x: f64,
    y: f64,
    offset: f64,
    height: f64,
) {
    let node_radius = 30.0;
    let vertical_spacing = height / 10.0;

    cr.set_source_rgb(0.2, 0.6, 0.2);
    cr.arc(x, y, node_radius, 0.0, 2.0 * std::f64::consts::PI);
    cr.fill_preserve()
        .expect("Unable to set background for node in Tree view");
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.stroke().unwrap();

    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.set_font_size(22.0);
    let text = node.value.to_string();
    let extents = cr.text_extents(&text).expect("Unable to extent text");
    cr.move_to(x - extents.width() / 2.0, y + extents.height() / 2.0);
    cr.show_text(&text).expect("Unable to show text in node");

    if let Some(ref left) = node.left {
        let new_x = x - offset;
        let new_y = y + vertical_spacing;

        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.move_to(
            x - node_radius * (offset / (offset + vertical_spacing)).sqrt(),
            y + node_radius * (vertical_spacing / (offset + vertical_spacing)).sqrt(),
        );
        cr.line_to(
            new_x + node_radius * (offset / (offset + vertical_spacing)).sqrt(),
            new_y - node_radius * (vertical_spacing / (offset + vertical_spacing)).sqrt(),
        );
        cr.stroke().expect("Unable to stroke in Tree view");
        draw_node(left, cr, new_x, new_y, offset / 2.0, height);
    }

    if let Some(ref right) = node.right {
        let new_x = x + offset;
        let new_y = y + vertical_spacing;

        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.move_to(
            x + node_radius * (offset / (offset + vertical_spacing)).sqrt(),
            y + node_radius * (vertical_spacing / (offset + vertical_spacing)).sqrt(),
        );
        cr.line_to(
            new_x - node_radius * (offset / (offset + vertical_spacing)).sqrt(),
            new_y - node_radius * (vertical_spacing / (offset + vertical_spacing)).sqrt(),
        );
        cr.stroke().expect("Unable to stroke in Tree view");
        draw_node(right, cr, new_x, new_y, offset / 2.0, height);
    }
}

pub fn create_view(stack: &gtk::Stack) -> gtk::Box {
    let view = gtk::Box::new(Orientation::Horizontal, 10);
    let stack_clone = stack.clone();
    let home_button = Button::with_label("Home");
    home_button.set_widget_name("home-button");
    home_button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("Home");
    });

    let controls = gtk::Box::new(Orientation::Vertical, 10);
    let push_entry = Entry::new();
    let push_label = Button::with_label("Insert");
    push_label.set_widget_name("push-button");
    let pop_label = Button::with_label("Delete");
    pop_label.set_widget_name("pop-button");
    let tree_label = Label::new(Some("Tree type"));
    let tree_combo = ComboBoxText::new();
    tree_combo.append_text("Binary Search Tree");
    let select_button = Button::with_label("Select");

    controls.append(&home_button);
    controls.append(&push_entry);
    controls.append(&push_label);
    controls.append(&pop_label);
    controls.append(&tree_label);
    controls.append(&tree_combo);
    controls.append(&select_button);

    view.append(&controls);

    let drawing_area = DrawingArea::new();
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);

    view.append(&drawing_area);

    let tree = Rc::new(RefCell::new(Bst::new()));

    push_label.connect_clicked({
        let drawing_area = drawing_area.clone();
        let tree = tree.clone();
        let push_entry = push_entry.clone();
        move |_| {
            let value: i32 = push_entry
                .text()
                .parse()
                .unwrap_or(rand::thread_rng().gen_range(1..=100));
            tree.borrow_mut().insert(value);
            drawing_area.queue_draw();
        }
    });

    pop_label.connect_clicked({
        let drawing_area = drawing_area.clone();
        let tree = tree.clone();
        let push_entry = push_entry.clone();
        move |_| {
            let value: i32 = push_entry
                .text()
                .parse()
                .unwrap_or(rand::thread_rng().gen_range(1..=100));
            tree.borrow_mut().delete(value);
            drawing_area.queue_draw();
        }
    });

    select_button.connect_clicked({
        let drawing_area = drawing_area.clone();
        let tree = tree.clone();
        move |_| {
            let tree_type = tree_combo
                .active_text()
                .unwrap_or_else(|| "Binary Search Tree".into())
                .to_string();
            *tree.borrow_mut() = match tree_type.as_str() {
                "Binary Search Tree" => Bst::new(),
                _ => Bst::new(),
            };
            drawing_area.queue_draw();
        }
    });

    drawing_area.set_draw_func(move |_, cr, width, height| {
        cr.set_source_rgb(0.1568, 0.1725, 0.2039);
        cr.paint().expect("Unable to paint background");
        let tree = tree.borrow();
        draw_tree(&tree, cr, width as f64, height as f64);
    });

    view
}
