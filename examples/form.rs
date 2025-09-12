extern crate wtk;
use wtk::enclose;
use wtk::prelude::*;
use wtk::widgets::Entry;

fn main() {
    let mut app = App::<SDLBackend>::new("WTK form example");
    app.add_widget(Label::new("Name").shared());
    let name = Entry::new().shared();
    app.add_widget(name.clone());
    app.add_widget(Label::new("Age").shared());
    let age = Entry::new().shared();
    app.add_widget(age.clone());
    app.add_widget(Button::new("SUBMIT",  enclose!((name, age) move |_b| {
        println!("Name: {}", name.clone().borrow().get_text());
        println!("Age: {}", age.clone().borrow().get_text());
    })).shared());
    app.run();
}
