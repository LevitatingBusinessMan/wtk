pub mod widgets;
pub mod rect;
pub mod event;
pub mod app;
pub mod prelude;
pub mod backends;
pub mod draw;
mod ffi;

use prelude::*;

fn main() {
    let mut app = App::<SDLBackend>::new("wtk example");
    let button = Button::new("click", || {}).shared();
    app.add_widget(button.clone());
    app.run();
}
