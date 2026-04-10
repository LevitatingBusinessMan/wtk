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
        count.borrow_mut().add_assign(1);
        label.borrow_mut().set_text(format!("Counter: {}", count.borrow()));
    })).shared();
    let button2 = Button::new("-", enclose!((count, label) move |_b| {
        count.borrow_mut().add_assign(-1);
        label.borrow_mut().set_text(format!("Counter: {}", count.borrow()));
    })).shared();
    app.add_widget(label);
    app.add_widget(
        WBox::horizontal().with(vec![
            button1,
            button2
        ]).shared()
    );
    app.run();
}
