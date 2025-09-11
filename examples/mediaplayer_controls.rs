extern crate wtk;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::OnceLock;
use mpris;

use wtk::prelude::*;
use wtk::enclose;

struct MediaPlayerButtons {
    inner_box: Rc<RefCell<WBox>>,
}

impl MediaPlayerButtons {
    pub fn new(player: Rc<mpris::Player>) -> Self {
        let left = Button::new("<", enclose!((player) move |_| mediaplayer_controls(player.clone(), MediaPlayerAction::Previous)));
        let right: Button = Button::new("||", enclose!{(player) move |_| mediaplayer_controls(player.clone(), MediaPlayerAction::PlayPause)});
        let middle = Button::new(">", enclose!{(player) move |_| mediaplayer_controls(player.clone(), MediaPlayerAction::Next)});
        
        let mut inner_box = WBox::new(Orientation::Horizontal);
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

fn main() -> Result<(), String>{
    let player = find_player()?;
    let mut app = App::<SDLBackend>::new(&format!("{} controls", player.bus_name_player_name_part()));
    app.add_widget(MediaPlayerButtons::new(Rc::new(player)).shared());
    app.run();
    Ok(())
}


#[derive(Debug)]
enum MediaPlayerAction {
    Previous,
    PlayPause,
    Next,
}

fn mediaplayer_controls(player: Rc<mpris::Player>, action: MediaPlayerAction) {
    println!("{action:?}");
    let _ = match action {
        MediaPlayerAction::Previous => player.previous(),
        MediaPlayerAction::PlayPause => player.play_pause(),
        MediaPlayerAction::Next => player.next(),
    }.inspect_err(|e| eprintln!("D-Bus error: {e}"));
}

fn find_player() -> Result<mpris::Player, String> {
    let player_finder = mpris::PlayerFinder::new()
        .map_err(|e| e.to_string())?;

    match player_finder.find_active() {
        Ok(player) => Ok(player),
        Err(e) => match e {
            mpris::FindingError::NoPlayerFound => {
                match player_finder.find_first() {
                    Ok(p) => Ok(p),
                    Err(e) => Err(e.to_string()),
                }
            },
            mpris::FindingError::DBusError(e) => Err(e.to_string()),
        },
    }
}
