extern crate wtk;
use wtk::prelude::*;

fn main() {
    let mut app = App::<SDLBackend>::new("WTK box example");
    let button1 = Button::new("clickme", |b| {
        b.set_text("clicked");
    }).shared();
    let button2 = Button::new("clickme", |b| {
        b.set_text("clicked");
    }).shared();
    let mut box_ = WBox::new(Orientation::Horizontal);
    box_.add_widget(button1);
    box_.add_widget(button2);
    box_.set_margin(5);
    box_.set_border(true);
    app.add_widget(box_.shared());
    app.run();
}
