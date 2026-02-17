extern crate wtk;
use wtk::prelude::*;
fn main() {
    let mut app = App::<SDLBackend>::new("WTK button example");
    let button = Button::new("clickme!", |b| {
        b.set_text("clicked");
    }).shared();
    let button2 = Button::new("clickme!", |b| {
        b.set_text("clicked");
    }).shared();
    let mut box_ = WBox::new(Orientation::Horizontal);
    box_.add_widget(button);
    box_.add_widget(button2);
    let main_widget = Centered::new(
        Centered::new(box_.shared(), Orientation::Horizontal, 800).shared(),
        Orientation::Vertical,
        600
    ).shared();
    app.add_widget(main_widget);
    app.run();
}
