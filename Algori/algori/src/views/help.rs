use gtk::{prelude::*, Box, Button, Label, Orientation};

pub fn create_view(stack: &gtk::Stack) -> Box {
    let view = Box::new(Orientation::Vertical, 10);
    let stack_clone = stack.clone();
    let home_button = Button::with_label("Back");
    home_button.set_widget_name("back-button");

    home_button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("Stack");
    });

    let help_text = r#"
# Algoritmo de Ejemplo

Este es un ejemplo de cómo se puede mostrar la ayuda.

## Pasos del Algoritmo

1. Inicializar
2. Procesar
3. Finalizar

**Nota:** Este es un texto de ejemplo con formato mínimo.
"#;

    let label = Label::new(Some(help_text));
    view.append(&home_button);
    view.append(&label);

    view
}