use gtk::{prelude::*, Box, Button, Label, Orientation, TextBuffer, TextTagTable, TextView};

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

    view.append(&home_button);
    view.append(&title_label);
    view.append(&description_label);
    view.append(&steps_label);
    view.append(&steps_text_view);
    view.append(&note_label);

    view
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

    view.append(&home_button);
    view.append(&title_label);
    view.append(&description_label);
    view.append(&steps_label);
    view.append(&steps_text_view);
    view.append(&note_label);

    view
}

