use gtk::{prelude::*, Box as gtkBox, Button, ComboBoxText, DrawingArea, Entry, Label};
use rand::Rng;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bst<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> Default for Bst<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<T> Bst<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(value: T) -> Self {
        let root = Box::new(Node {
            value,
            left: None,
            right: None,
        });

        Self { root: Some(root) }
    }
}

impl<T> Bst<T>
where
    T: Ord,
{
    pub fn insert(&mut self, new_val: T) {
        let new_node = Box::new(Node {
            value: new_val,
            left: None,
            right: None,
        });
        Self::push_node(new_node, &mut self.root);
    }

    fn push_node(new_node: Box<Node<T>>, current_node: &mut Option<Box<Node<T>>>) {
        if let Some(node) = current_node {
            use std::cmp::Ordering;
            match node.value.cmp(&new_node.value) {
                Ordering::Less | Ordering::Equal => Self::push_node(new_node, &mut node.left),
                Ordering::Greater => Self::push_node(new_node, &mut node.right),
            }
        } else {
            current_node.insert(new_node);
        }
    }
}

pub fn create_view(stack: &gtk::Stack) -> gtkBox {
    let view = gtkBox::new(gtk::Orientation::Horizontal, 10);
    let stack_clone = stack.clone();
    let home_button = Button::with_label("Home");
    home_button.set_widget_name("home-button");
    home_button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("Home");
    });

    let controls = gtkBox::new(gtk::Orientation::Vertical, 10);
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

    push_label.connect_clicked({
        let drawing_area = drawing_area.clone();
        move |_| {
            let value: i32 = push_entry
                .text()
                .parse()
                .unwrap_or(rand::thread_rng().gen_range(1..=100));
            // Do it whenever a tree selected Bst::from(value);
        }
    });

    select_button.connect_clicked({
        let drawing_area = drawing_area.clone();
        move |_| {
            let tree_type = tree_combo
                .active_text()
                .unwrap_or_else(|| "Binary Search Tree".into())
                .to_string();
            let tree = match tree_type.as_str() {
                "Binary Search Tree" => 1,
                _ => 2,
            };
        }
    });

    view
}
