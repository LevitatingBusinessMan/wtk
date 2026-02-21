use log::{Level, LevelFilter, Metadata, Record};

extern crate wtk;
use wtk::prelude::*;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    let mut app = App::<SDLBackend>::new("WTK button example");
    let button = Button::new("clickme", |b| {
        b.set_text("clicked");
    });
    app.add_widget(button.shared());
    app.run();
}
