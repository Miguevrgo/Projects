use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.Algori";

fn build_id(app: &Application) {
    let button = Button::builder()
        .label("Algoritmo X")
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    button.connect_clicked(|button| {
        button.set_label("Hello World");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Algori")
        .default_width(1600)
        .default_height(900)
        .child(&button)
        .build();

    window.present();
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_id);

    app.run()
}
