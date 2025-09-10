extern crate wtk;
use wtk::prelude::*;
use std::cell::RefCell;
use std::ops::AddAssign;
use std::rc::Rc;

fn main() {
    let mut app = App::<SDLBackend>::new("Counter go brrr");
    let count = Rc::new(RefCell::new(0));
    let label = Label::new(format!("Counter: 0")).shared();
    let labelrc = label.clone();
    let countrc  = count.clone();
    let button1 = Button::new("+", move |_b| {
        let mut count = countrc.borrow_mut();
        count.add_assign(1);
        labelrc.borrow_mut().set_text(format!("Counter: {count}"));
    }).shared();
    let countrc = count.clone();
    let labelrc = label.clone();    
    let button2 = Button::new("-", move |_b| {
        let mut count = countrc.borrow_mut();
        count.add_assign(-1);
        labelrc.borrow_mut().set_text(format!("Counter: {count}"));
    }).shared();
    app.add_widget(label);
    app.add_widget(button1);
    app.add_widget(button2);
    app.run();
}
