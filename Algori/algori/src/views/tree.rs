use gtk::{prelude::*, Box, Button, Label};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(value: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
        }))
    }
}

pub struct BinaryTree {
    pub root: Option<Rc<RefCell<TreeNode>>>,
}

impl BinaryTree {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, value: i32) {
        let new_node = TreeNode::new(value);
        if let Some(root) = &self.root {
            self.insert_node(root.clone(), new_node);
        } else {
            self.root = Some(new_node);
        }
    }

    fn insert_node(&mut self, node: Rc<RefCell<TreeNode>>, new_node: Rc<RefCell<TreeNode>>) {
        if new_node.borrow().value < node.borrow().value {
            if let Some(left) = node.borrow().left.clone() {
                self.insert_node(left, new_node);
            } else {
                node.borrow_mut().left = Some(new_node);
            }
        } else if let Some(right) = node.borrow().right.clone() {
            self.insert_node(right, new_node);
        } else {
            node.borrow_mut().right = Some(new_node);
        }
    }
}

pub fn create_view(stack: &gtk::Stack) -> Box {
    let view = Box::new(gtk::Orientation::Vertical, 10);
    let label = Label::new(Some("This is the view for Array"));
    let stack_clone = stack.clone();
    let home_button = Button::with_label("Home");
    home_button.set_widget_name("home-button");

    home_button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("Home");
    });

    view.append(&home_button);
    view.append(&label);

    view
}
