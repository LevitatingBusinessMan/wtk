extern crate wtk;
use std::cell::RefCell;
use std::rc::Rc;
use mpris;

use wtk::prelude::*;

struct MediaPlayerButtons {
    inner_box: Rc<RefCell<WBox>>,
}

impl MediaPlayerButtons {
    pub fn new() -> Self {
        let left = Button::new("<", |_| mediaplayer_controls(MediaPlayerAction::Previous));
        let right: Button = Button::new("||", |_| mediaplayer_controls(MediaPlayerAction::Pause));
        let middle = Button::new(">", |_| mediaplayer_controls(MediaPlayerAction::Next));
        
        let mut inner_box = WBox::new(Orientation::Vertical);
        inner_box.set_padding(4);
        inner_box.set_margin(5);
        inner_box.set_border(true);
        inner_box.add_widget(left.shared());
        inner_box.add_widget(right.shared());
        inner_box.add_widget(middle.shared());
        
        Self { inner_box: inner_box.shared() }
    }
}

impl Widget for MediaPlayerButtons {
    fn draw(&self, ctx: &mut DrawContext) {
        self.inner_box.borrow().draw(ctx);
    }
    fn process_event(&mut self, event: &Event) -> bool {
        self.inner_box.borrow_mut().process_event(event)
    }
}

fn main() {
    let mut app = App::<SDLBackend>::new("WTK button example");
    app.add_widget(MediaPlayerButtons::new().shared());
    app.run();
}


#[derive(Debug)]
enum MediaPlayerAction {
    Previous,
    Pause,
    Next,
}

fn mediaplayer_controls(action: MediaPlayerAction) {
    eprintln!("{:?}", action);
    return;
    match action {
        MediaPlayerAction::Previous => todo!(),
        MediaPlayerAction::Pause => todo!(),
        MediaPlayerAction::Next => todo!(),
    }
}
