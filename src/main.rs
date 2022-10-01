use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let thinker = Application::builder()
        .application_id("org.example.ThinkEr")
        .build();

    thinker.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(1000)
            .default_height(500)
            .title("Welcome to ThinkEr!!")
            .build();

        win.show_all();
    });

    thinker.run();
}
