extern crate wtk;
use wtk::prelude::*;

fn main() {
    let mut app = App::<SDLBackend>::new("wtk example");
    let button = Button::new("clickme", |b| {
        b.set_text("clicked");
    }).shared();
    app.add_widget(button.clone());
    app.run();
}
