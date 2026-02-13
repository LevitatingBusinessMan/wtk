extern crate wtk;
use wtk::draw;
use wtk::elm::ElmModel;
use wtk::elm::ElmLoop;
use wtk::elm_cb;
use wtk::enclose;
use wtk::prelude::*;
use wtk::widgets::SharedWidget;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;


pub enum CounterMessage{
    Increment,
    Decrement,
}

pub struct Counter {
    count: i32,
    button_box: Rc<RefCell<WBox>>,
    counter_label: Rc<RefCell<Label>>,
    receiver: mpsc::Receiver<CounterMessage>,
}

impl Counter {
    fn new() -> Counter {
        let (sender, receiver) = mpsc::channel();
        let dec_button = Button::new("-", elm_cb!(sender, _b => CounterMessage::Decrement)).shared();
        let inc_buton = Button::new("+", elm_cb!(sender, _b => CounterMessage::Increment)).shared();
        let button_box = WBox::with(Orientation::Horizontal, vec![dec_button, inc_buton]).shared();
        let counter_label = Label::new("Count: 0").shared();
        Counter {
            count: 0,
            button_box,
            counter_label,
            receiver,
        }
    }
}

impl Widget for Counter {
    fn draw(&self, ctx: &mut DrawContext) {
        ctx.draw_widgets(Orientation::Vertical, 6, None, &vec![
            self.counter_label.clone() as SharedWidget,
            self.button_box.clone() as SharedWidget,
        ]);
    }
    
    fn process_event(&mut self, e: &Event) -> bool { 
        self.button_box.borrow_mut().process_event(e)
    }
}

impl ElmModel for Counter {
    type Message = CounterMessage;

    fn receiver(&mut self) -> &mut std::sync::mpsc::Receiver<Self::Message> {
        &mut self.receiver
    }

    fn update<B>(&mut self, app: &mut App<B>, msg: Self::Message) where B: Backend {
        match msg {
            CounterMessage::Increment => self.count += 1,
            CounterMessage::Decrement => self.count -= 1,
        }
        self.counter_label.borrow_mut().set_text(format!("Count: {}", self.count));
    }
}

fn main() {
    let mut app = App::<SDLBackend>::new("Counter go brrr");
    let counter = Counter::new().shared();
    app.add_widget(counter.clone());
    app.elm_run(counter);    
}
