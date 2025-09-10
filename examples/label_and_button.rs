extern crate wtk;
use wtk::prelude::*;

fn main() {
    let mut app = App::<SDLBackend>::new("WTK can do multiple widgets!!!");
    let label = Label::new("WTK widgets are so cool omg");
    let button = Button::new("clickme", |b| {
        b.set_text("clicked");
    });
    app.add_widget(label.shared());
    app.add_widget(button.shared());
    app.run();
}
