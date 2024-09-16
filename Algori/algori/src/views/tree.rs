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
                Ordering::Less | Ordering::Equal => Self::push_node(new_node, &mut node.right),
                Ordering::Greater => Self::push_node(new_node, &mut node.left),
            }
        } else {
            current_node.insert(new_node);
        }
    }
}

fn draw_tree<T: std::fmt::Display>(tree: &Bst<T>, cr: &gtk::cairo::Context, width: f64) {
    if let Some(ref root) = tree.root {
        let x = width / 2.0;
        let y = 50.0;
        let offset = 200.0;
        draw_node(root, cr, x, y, offset);
    }
}

fn draw_node<T: std::fmt::Display>(
    node: &Node<T>,
    cr: &gtk::cairo::Context,
    x: f64,
    y: f64,
    offset: f64,
) {
    cr.set_source_rgb(0.2, 0.6, 0.2);
    cr.arc(x, y, 30.0, 0.0, 2.0 * std::f64::consts::PI);
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
        let new_y = y + 100.0;

        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.move_to(x - 25.0, y + 25.0);
        cr.line_to(new_x, new_y);
        cr.stroke().expect("Unable to stroke in Tree view");
        draw_node(left, cr, new_x, new_y, offset);
    }
    if let Some(ref right) = node.right {
        let new_x = x - offset;
        let new_y = y + 100.0;

        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.move_to(x + 25.0, y + 25.0);
        cr.line_to(new_x, new_y);
        cr.stroke().expect("Unable to stroke in Tree view");
        draw_node(right, cr, new_x, new_y, offset);
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
