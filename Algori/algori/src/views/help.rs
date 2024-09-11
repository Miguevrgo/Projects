use gtk::{
    gio, glib, Application, ApplicationWindow, Box, Button, CssProvider, EventControllerMotion,
    GestureClick, Grid, Image, ScrolledWindow, Stack,
};
use gtk::{prelude::*, Label, Orientation, TextBuffer, TextTagTable, TextView};

pub fn create_view_stack(stack: &gtk::Stack) -> Box {
    let view = Box::new(Orientation::Vertical, 10);
    view.set_widget_name("help-view");
    let stack_clone = stack.clone();
    let home_button = Button::with_label("Back");
    home_button.set_widget_name("back-button");

    home_button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("Stack");
    });

    let title_label = Label::new(Some("Stack"));
    title_label.set_widget_name("help-title");

    let description_label = Label::new(Some("stack data structure"));
    description_label.set_widget_name("help-description");

    let steps_label = Label::new(Some("## Pasos del Algoritmo"));
    steps_label.set_widget_name("help-steps-title");

    let steps_content = r#"
1. Inicializar
2. Procesar
3. Finalizar
"#;
    let steps_buffer = TextBuffer::new(Some(&TextTagTable::new()));
    steps_buffer.set_text(steps_content);

    let steps_text_view = TextView::new();
    steps_text_view.set_buffer(Some(&steps_buffer));
    steps_text_view.set_widget_name("help-steps-content");
    steps_text_view.set_editable(false);
    steps_text_view.set_wrap_mode(gtk::WrapMode::Word);

    let note_label = Label::new(Some(
        "**Nota:** Este es un texto de ejemplo con formato mínimo.",
    ));
    note_label.set_widget_name("help-note");

    let box_container = Box::new(gtk::Orientation::Vertical, 0);
    box_container.append(&home_button);
    view.append(&home_button);
    view.append(&title_label);
    view.append(&description_label);
    view.append(&steps_label);
    view.append(&steps_text_view);
    view.append(&note_label);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .child(&view)
        .build();

    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);

    let home_view = Box::new(gtk::Orientation::Vertical, 0);
    home_view.append(&scrolled_window);

    home_view
}

pub fn create_view_array(stack: &gtk::Stack) -> Box {
    let view = Box::new(Orientation::Vertical, 10);
    view.set_widget_name("help-view");

    let stack_clone = stack.clone();
    let home_button = Button::with_label("Back");
    home_button.set_widget_name("back-button");

    home_button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("Stack");
    });

    let title_label = Label::new(Some("Array"));
    title_label.set_widget_name("help-title");

    let subtitle_introduction = Label::new(Some(" 1. Introduction"));
    subtitle_introduction.set_widget_name("help-subtitle");

    let introduction_content = Label::new(Some(
        "
Array is one of the most used and useful data structures, it can be defined
as a collection of elements contiguous in memory, most of the times, of the
same type (or size).
    ",
    ));

    let subtitle_motivation = Label::new(Some("2. Motivation"));
    subtitle_motivation.set_widget_name("help-subtitle");

    let motivation_content = Label::new(Some("Array data structures with its different implementations,
including dynamic ones, such as vector in C++ or Vec in Rust, are used as a solution for much of the 
problems involved in daily programming, providing an efficient in both space and time solution"));

    let subtitle_complexity = Label::new(Some("3. Complexity"));
    subtitle_complexity.set_widget_name("help-subtitle");
    let note_label = Label::new(Some(
        "**Nota:** Este es un texto de ejemplo con formato mínimo.",
    ));
    note_label.set_widget_name("help-note");

    view.append(&home_button);
    view.append(&title_label);
    view.append(&subtitle_introduction);
    view.append(&introduction_content);
    view.append(&subtitle_motivation);
    view.append(&motivation_content);
    view.append(&note_label);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .child(&view)
        .build();

    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);

    let home_view = Box::new(gtk::Orientation::Vertical, 0);
    home_view.append(&scrolled_window);

    home_view
}
