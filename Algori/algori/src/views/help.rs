use gtk::{
    prelude::*, Box, Button, Label, Orientation, ScrolledWindow, TextBuffer, TextTagTable, TextView,
};

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
        "Array is one of the most used and useful data structures, it can be defined
as a collection of multiple elements under a single variable name. 
    ",
    ));
    introduction_content.set_widget_name("help-content");

    let subtitle_motivation = Label::new(Some("2. Motivation"));
    subtitle_motivation.set_widget_name("help-subtitle");

    let motivation_content = Label::new(Some("Array data structures with its different implementations,
including dynamic ones, such as vector in C++ or Vec in Rust, are used as a solution for much of the 
problems involved in daily programming, providing an efficient in both space and time solution"));
    motivation_content.set_widget_name("help-content");

    let subtitle_description = Label::new(Some("3. Description"));
    subtitle_description.set_widget_name("help-subtitle");

    let description_content = Label::new(Some("An array is, or at least should be, contiguous spaces in memory of elements
of the same type, most used languages are strict about this and you can always assume it. The reason for this is that 
accessing some index of an array is done in constant time, O(1), because as all elements are the same size and contiguous,
finding the memory address of the element is done by a simple formula:
"));
    description_content.set_widget_name("help-content");

    let formula_content = Label::new(Some("address = base_address + (index * size_of_element)"));
    formula_content.set_widget_name("help-note");

    let description_content_2 = Label::new(Some("Arrays data structures are frequently divided into dynamic and static arrays,
this meaning that the size of the array is fixed or can be changed during runtime. This is a very important distinction,
as it can change the complexity of the operations over the array, this is due to the fact that an static array memory is
reserved at compile time, while a dynamic array memory is reserved at runtime. This allows the dynamic array to grow while
running, but it also means that the memory is maybe reallocated several times, this operation is costly, which would result
in a higher complexity when pushing elements to the array, however, the array implementation includes two numerical values
regarding the array, the capacity and the length, the capacity is the amount of memory that has been reserved for the array,
and the length is the amount of elements that are currently in the array. When the size meets the capacity, the array is reallocated,
reserving a bigger amount of memory, and copying the elements to the new memory space. This reallocation usually doubles the capacity
even though there is some discussion about the best growth factor. This reallocation provides an amortized constant time complexity
for pushing elements to the array, but it is important to note that the reallocation is a costly operation, so whenever possible,
reserving size for the array is a good practice, specially before for loops where a push operation is done several times."));
    description_content_2.set_widget_name("help-content");

    let subtitle_complexity = Label::new(Some("4. Complexity"));
    subtitle_complexity.set_widget_name("help-subtitle");

    view.append(&home_button);
    view.append(&title_label);
    view.append(&subtitle_introduction);
    view.append(&introduction_content);
    view.append(&subtitle_motivation);
    view.append(&motivation_content);
    view.append(&subtitle_description);
    view.append(&description_content);
    view.append(&formula_content);
    view.append(&description_content_2);

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
