use gtk::{
    gdk, prelude::*, Box, Button, ComboBoxText, Dialog, DrawingArea, Entry, Label, Orientation,
    TextBuffer, TextTag, TextTagTable, TextView, Window,
};
use std::cell::RefCell;
use std::rc::Rc;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

const NUM_ELEMENTS: usize = 10;

struct AppState {
    elements: RefCell<Vec<i32>>,
    capacity: RefCell<usize>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            elements: RefCell::new(Vec::new()),
            capacity: RefCell::new(NUM_ELEMENTS),
        }
    }
}

const RUST_VECTOR_CODE: &str = r#"
pub struct Vector<T> {
    elements: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Vector { elements: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }
}
"#;

const CPP_VECTOR_CODE: &str = r#"
#include <vector>

template <typename T>
class Vector {
public:
    Vector() : elements() {}

    void push(const T& value) {
        elements.push_back(value);
    }

    T* get(size_t index) {
        if (index < elements.size()) {
            return &elements[index];
        }
        return nullptr;
    }

private:
    std::vector<T> elements;
};
"#;

const C_VECTOR_CODE: &str = r#"
#include <stdlib.h>

typedef struct {
    int* elements;
    size_t size;
    size_t capacity;
} Vector;

void vector_init(Vector* vector) {
    vector->elements = malloc(sizeof(int) * 10);
    vector->size = 0;
    vector->capacity = 10;
}

void vector_push(Vector* vector, int value) {
    if (vector->size == vector->capacity) {
        vector->capacity *= 2;
        vector->elements = realloc(vector->elements, sizeof(int) * vector->capacity);
    }
    vector->elements[vector->size++] = value;
}

int* vector_get(Vector* vector, size_t index) {
    if (index < vector->size) {
        return &vector->elements[index];
    }
    return NULL;
}
"#;

fn highlight_code(code: &str, language: &str) -> TextBuffer {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = match language {
        "Rust" => ss.find_syntax_by_extension("rs").unwrap(),
        "C++" => ss.find_syntax_by_extension("cpp").unwrap(),
        "C" => ss.find_syntax_by_extension("c").unwrap(),
        _ => ss.find_syntax_plain_text(),
    };
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let buffer = TextBuffer::new(Some(&TextTagTable::new()));
    let tag_table = buffer.tag_table();

    for line in LinesWithEndings::from(code) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ss).unwrap();
        for (style, text) in ranges {
            let tag = TextTag::new(None);
            tag.set_foreground(Some(
                &gdk::RGBA::new(
                    style.foreground.r as f32 / 255.0,
                    style.foreground.g as f32 / 255.0,
                    style.foreground.b as f32 / 255.0,
                    style.foreground.a as f32 / 255.0,
                )
                .to_string(),
            ));
            tag_table.add(&tag);
            buffer.insert_with_tags(&mut buffer.end_iter(), text, &[&tag]);
        }
    }

    buffer
}

pub fn create_view(stack: &gtk::Stack) -> Box {
    let view = Box::new(Orientation::Horizontal, 10);
    let stack_clone = stack.clone();
    let home_button = Button::with_label("Home");
    home_button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("Home");
    });

    let controls = Box::new(Orientation::Vertical, 10);
    controls.append(&home_button);

    let elements_label = Label::new(Some("Number of Elements:"));
    controls.append(&elements_label);
    let elements_entry = Entry::new();
    elements_entry.set_text("10");
    controls.append(&elements_entry);
    let create_button = Button::with_label("Create Array");
    controls.append(&create_button);

    let insert_label = Label::new(Some("Insert Element:"));
    controls.append(&insert_label);
    let insert_entry = Entry::new();
    controls.append(&insert_entry);
    let insert_button = Button::with_label("Insert");
    controls.append(&insert_button);

    let delete_label = Label::new(Some("Delete Element:"));
    controls.append(&delete_label);
    let delete_entry = Entry::new();
    controls.append(&delete_entry);
    let delete_button = Button::with_label("Delete");
    controls.append(&delete_button);

    let language_label = Label::new(Some("Code Language:"));
    let language_combo = ComboBoxText::new();

    language_combo.append_text("Rust");
    language_combo.append_text("C++");
    language_combo.append_text("C");

    let show_button = Button::with_label("Show Code");
    let language_box = Box::new(Orientation::Horizontal, 10);
    language_box.append(&language_label);
    language_box.append(&language_combo);
    controls.append(&language_box);
    controls.append(&show_button);
    view.append(&controls);

    let drawing_area = DrawingArea::new();
    drawing_area.set_vexpand(true);
    drawing_area.set_hexpand(true);

    let state = Rc::new(AppState::new());
    drawing_area.set_draw_func({
        let state = state.clone();
        move |_, cr, width, height| {
            let elements = state.elements.borrow();
            let capacity = *state.capacity.borrow();
            let num_elements = elements.len();
            let element_size = (width as f64 - 20.0) / capacity as f64 - 5.0;
            let total_width = capacity as f64 * (element_size + 5.0) - 5.0;
            let offset_x = (width as f64 - total_width) / 2.0;

            cr.set_source_rgb(0.1568, 0.1725, 0.2039);
            cr.paint().unwrap();

            for i in 0..capacity {
                let x = offset_x + i as f64 * (element_size + 5.0);
                let y = (height as f64 - element_size) / 2.0;

                if i < num_elements {
                    cr.set_source_rgb(0.0, 0.0, 1.0);
                } else {
                    cr.set_source_rgb(0.5, 0.5, 0.5);
                }

                cr.rectangle(x, y, element_size, element_size);
                cr.fill().unwrap();

                if i < num_elements {
                    cr.set_source_rgb(1.0, 1.0, 1.0);
                    cr.set_font_size(element_size / 2.5);

                    let text = format!("{}", elements[i]);
                    let extents = cr.text_extents(&text).unwrap();
                    let text_x = x + (element_size - extents.width()) / 2.0;
                    let text_y = y + (element_size + extents.height()) / 2.0;

                    cr.move_to(text_x, text_y);
                    cr.show_text(&text).unwrap();
                }
            }
        }
    });

    view.append(&drawing_area);
    create_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            let num_elements: usize = elements_entry.text().parse().unwrap_or(NUM_ELEMENTS);
            let mut elements = state.elements.borrow_mut();
            *elements = (0..num_elements).map(|x| x as i32).collect();
            *state.capacity.borrow_mut() = num_elements;
            drawing_area.queue_draw();
        }
    });

    insert_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            let value: i32 = insert_entry.text().parse().unwrap_or(0);
            let mut elements = state.elements.borrow_mut();
            let mut capacity = state.capacity.borrow_mut();
            if elements.len() == *capacity {
                *capacity *= 2;
            }
            elements.push(value);
            drawing_area.queue_draw();
        }
    });

    delete_button.connect_clicked({
        let state = state.clone();
        let drawing_area = drawing_area.clone();
        move |_| {
            let value: i32 = delete_entry.text().parse().unwrap_or(0);
            let mut elements = state.elements.borrow_mut();
            if let Some(pos) = elements.iter().position(|&x| x == value) {
                elements.remove(pos);
            }
            drawing_area.queue_draw();
        }
    });

    show_button.connect_clicked(move |_| {
        let language = language_combo.active_text().unwrap().to_string();
        let code = match language.as_str() {
            "Rust" => RUST_VECTOR_CODE,
            "C++" => CPP_VECTOR_CODE,
            "C" => C_VECTOR_CODE,
            _ => "",
        };
        let buffer = highlight_code(code, &language);
        let dialog = Dialog::with_buttons(
            Some("Vector Implementation"),
            Some(&Window::default()),
            gtk::DialogFlags::MODAL,
            &[("Close", gtk::ResponseType::Close)],
        );
        let content_area = dialog.content_area();
        let text_view = TextView::new();
        text_view.set_buffer(Some(&buffer));
        content_area.append(&text_view);
        dialog.show();
        dialog.connect_response(|dialog, _| {
            dialog.close();
        });
    });
    view
}

