use gtk::{prelude::*, Box, Button, Label};

use super::help::create_view_binary_search as create_help_view;

pub fn create_view(stack: &gtk::Stack) -> Box {
    let view = Box::new(gtk::Orientation::Vertical, 10);
    let label = Label::new(Some("This is the view for Array"));
    let stack_clone = stack.clone();
    let home_button = Button::with_label("Home");
    home_button.set_widget_name("home-button");

    home_button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("Home");
    });

    let help_button = Button::new();
    help_button.set_widget_name("help-button");

    view.append(&home_button);
    view.append(&help_button);
    view.append(&label);

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
