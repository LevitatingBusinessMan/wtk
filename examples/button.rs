extern crate wtk;
use wtk::prelude::*;

fn main() {
    let mut app = App::<SDLBackend>::new("wtk example");
    let button = Button::new("click", |b| {
        b.set_text("newtext");
    }).shared();
    app.add_widget(button.clone());
    app.run();
}
