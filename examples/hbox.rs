extern crate wtk;
use wtk::{prelude::*, widgets::HBox};

fn main() {
    let mut app = App::<SDLBackend>::new("WTK button example");
    let button1 = Button::new("clickme", |b| {
        b.set_text("clicked");
    }).shared();
    let button2 = Button::new("clickme", |b| {
        b.set_text("clicked");
    }).shared();
    let mut box_ = HBox::new();
    box_.add_widget(button1);
    box_.add_widget(button2);
    app.add_widget(box_.shared());
    app.run();
}
