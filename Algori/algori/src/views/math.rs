use gtk::{prelude::*, Box, Button, Label};

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
