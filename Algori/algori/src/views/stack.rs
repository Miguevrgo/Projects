use gtk::{prelude::*, Box, Button, DrawingArea, Entry, Orientation, ScrolledWindow, TextBuffer, TextTag, TextTagTable, TextView, Window};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
struct AppState {
    elements: RefCell<Vec<i32>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            elements: RefCell::new(Vec::new()),
        }
    }
}

fn highlight_code(code: &str) -> TextBuffer {
    let buffer = TextBuffer::new(Some(&TextTagTable::new()));
    let tag_table = buffer.tag_table();

    // Crear tags para los diferentes estilos
    let header1_tag = TextTag::new(Some("header1"));
    header1_tag.set_property("weight", &700);
    header1_tag.set_property("scale", &1.5);
    tag_table.add(&header1_tag);

    let header2_tag = TextTag::new(Some("header2"));
    header2_tag.set_property("weight", &700);
    header2_tag.set_property("scale", &4.0);
    tag_table.add(&header2_tag);

    for line in code.lines() {
        if line.starts_with("## ") {
            buffer.insert_with_tags(&mut buffer.end_iter(), line, &[&header2_tag]);
        } else if line.starts_with("# ") {
            buffer.insert_with_tags(&mut buffer.end_iter(), line, &[&header1_tag]);
        } else {
            buffer.insert(&mut buffer.end_iter(), line);
        }
        buffer.insert(&mut buffer.end_iter(), "\n");
    }

    buffer
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
    let push_entry = Entry::new();
    let push_button = Button::with_label("Push");
    push_button.set_widget_name("push-button");
    let pop_button = Button::with_label("Pop");
    pop_button.set_widget_name("pop-button");

    controls.append(&push_entry);
    controls.append(&push_button);
    controls.append(&pop_button);
    let help_button = Button::new();
    help_button.set_widget_name("help-button");
    controls.append(&help_button);
    view.append(&controls);

    let drawing_area = DrawingArea::new();
    drawing_area.set_vexpand(true);
    drawing_area.set_hexpand(true);
    view.append(&drawing_area);

    let state = Rc::new(AppState::new());

    drawing_area.set_draw_func({
        let state = state.clone();
        move |_, cr, width, height| {
            let elements = state.elements.borrow();
            let element_height = 30.0;
            let element_width = width as f64 - 20.0;
            let total_height = elements.len() as f64 * (element_height + 5.0) - 5.0;
            let offset_y = (height as f64 - total_height) / 2.0;

            cr.set_source_rgb(0.1568, 0.1725, 0.2039);
            cr.paint().unwrap();

            for (i, &element) in elements.iter().enumerate() {
                let y = offset_y + i as f64 * (element_height + 5.0);
                cr.set_source_rgb(0.0, 0.0, 1.0);
                cr.rectangle(10.0, y, element_width, element_height);
                cr.fill().unwrap();

                cr.set_source_rgb(1.0, 1.0, 1.0);
                cr.set_font_size(element_height / 2.5);
                let text = format!("{}", element);
                let extents = cr.text_extents(&text).unwrap();
                let text_x = 10.0 + (element_width - extents.width()) / 2.0;
                let text_y = y + (element_height + extents.height()) / 2.0;
                cr.move_to(text_x, text_y);
                cr.show_text(&text).unwrap();
            }
        }
    });

    push_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            let value = if let Ok(value) = push_entry.text().parse() {
                value
            } else {
                rand::thread_rng().gen_range(0..=100)
            };

            state.elements.borrow_mut().push(value);
            drawing_area.queue_draw();
        }
    });

    pop_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            state.elements.borrow_mut().pop();
            drawing_area.queue_draw();
        }
    });

    help_button.connect_clicked(move |_| {
        let help_window = Window::new();
        help_window.set_title(Some("Help"));
        help_window.set_default_size(800, 600);
    
        let scrolled_window = ScrolledWindow::new();
        let text_view = TextView::new();
        text_view.set_editable(false);
        text_view.set_cursor_visible(false);
    
        let markdown_text = r#"
# Algoritmo de Ejemplo
    
Este es un ejemplo de cómo se puede mostrar la ayuda.
    
## Pasos del Algoritmo
    
1. Inicializar
2. Procesar
3. Finalizar
    
**Nota:** Este es un texto de ejemplo con formato mínimo.
"#;
    
        let buffer = highlight_code(markdown_text);
    
        text_view.set_buffer(Some(&buffer));
        scrolled_window.set_child(Some(&text_view));
        help_window.set_child(Some(&scrolled_window));
        help_window.show();
    });

    view
}
