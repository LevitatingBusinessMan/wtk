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
    let box_ = WBox::horizontal()
        .border(true)
        .padding(5)
        .with(vec![button1, button2])
        .shared();
    app.add_widget(box_);
    app.run();
}
