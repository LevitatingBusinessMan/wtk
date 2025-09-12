extern crate wtk;
use wtk::prelude::*;
use std::cell::RefCell;
use std::ops::AddAssign;
use std::rc::Rc;
use wtk::enclose;

fn main() {
    let mut app = App::<SDLBackend>::new("Counter go brrr");
    let count = Rc::new(RefCell::new(0));
    let label = Label::new(format!("Counter: 0")).shared();
    let button1 = Button::new("+", enclose!((count, label) move |_b| {
        let mut count = count.borrow_mut();
        count.add_assign(1);
        label.borrow_mut().set_text(format!("Counter: {count}"));
    })).shared();
    let button2 = Button::new("-", enclose!((count, label) move |_b| {
        let mut count = count.borrow_mut();
        count.add_assign(-1);
        label.borrow_mut().set_text(format!("Counter: {count}"));
    })).shared();
    app.add_widget(label);
    let mut box_ = WBox::new(Orientation::Horizontal);
    box_.add_widget(button1.clone());
    box_.add_widget(button2.clone());
    app.add_widget(box_.shared());
    app.run();
}
