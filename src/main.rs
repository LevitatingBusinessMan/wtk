pub mod widgets;
pub mod rect;
pub mod event;
pub mod app;
pub mod prelude;
pub mod backends;
pub mod draw;
pub mod pixels;
mod ffi;

use prelude::*;

fn main() {
    let mut app = App::<SDLBackend>::new("wtk example");
    let button = Button::new("click", |b| {
        b.set_text("newtext");
    }).shared();
    app.add_widget(button.clone());
    app.run();
}
