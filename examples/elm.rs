use mpris;
use mpris::Player;
use wtk::enclose;
use wtk::prelude::*;
use std::rc::Rc;
//use std::sync::mpmc::Sender;
use std::sync::mpsc;

// pub struct ElmApp<W> where W: Widget + ElmModel {
//     widget: W,
// }

// impl<W> ElmApp<W> where W: Widget + ElmModel {
//     pub fn new(widget: W) -> Self {
//         Self {
//             widget
//         }
//     }
// }

pub trait ElmModel {
    type Message;
    /// Send a message to the model
    fn receiver(&mut self) -> &mut mpsc::Receiver<Self::Message>;
    /// Process all events. Returns true if a draw is required.
    fn update_all(&mut self) -> bool {
        let mut draw = false;
        loop {
            match self.receiver().try_recv() {
                Ok(msg) => {
                    draw = self.update(msg);
                },
                Err(e) => match e {
                    mpsc::TryRecvError::Empty => break,
                    mpsc::TryRecvError::Disconnected => panic!(),
                },
            }
        }
        draw
    }
    /// Process a single event. Returns true if a draw is required.
    fn update(&mut self, msg: Self::Message) -> bool;
}

pub struct MediaPlayer {
    player: Option<Player>,
    sender: Rc<mpsc::Sender<MediaPlayerMessage>>,
    receiver: mpsc::Receiver<MediaPlayerMessage>,
    inner_box: WBox,
}

pub enum MediaPlayerMessage {
    Previous,
    PlayPause,
    Next,
    ChangePlayer,
}

/// A shorthand for creating a closure that sends a message over a reference counted channel
macro_rules! elm_cb {
    ($sender:ident, $($param:pat),+ => $message:expr) => {
        enclose!(($sender) move |$($param),+| $sender.send($message).unwrap())
    };
}

impl MediaPlayer {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let sender = Rc::new(sender);
        
        let player = find_player().ok();

        let player_label = Label::new(format!("Player: {}", player.as_ref().map_or( "none".to_owned(), |p| p.bus_name_player_name_part().to_string())));
        let previous_button = Button::new("<", elm_cb!(sender, _b => MediaPlayerMessage::Previous));
        let play_pause_button = Button::new("||", elm_cb!(sender, _b => MediaPlayerMessage::PlayPause));
        let next_button = Button::new(">", elm_cb!(sender, _b => MediaPlayerMessage::Next));

        let mut inner_box = WBox::new(Orientation::Vertical);
        let mut player_controls_box = WBox::new(Orientation::Horizontal);
        player_controls_box.set_padding(4);
        player_controls_box.set_margin(5);
        player_controls_box.set_border(true);
        player_controls_box.add_widget(previous_button.shared());
        player_controls_box.add_widget(play_pause_button.shared());
        player_controls_box.add_widget(next_button.shared());

        inner_box.add_widget(player_label.shared());
        inner_box.add_widget(player_controls_box.shared());

        MediaPlayer {
            player,
            sender,
            receiver,
            inner_box,
        }
    }
}

impl ElmModel for MediaPlayer {
    type Message = MediaPlayerMessage;

    fn receiver(&mut self) -> &mut mpsc::Receiver<Self::Message> {
        &mut self.receiver
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            MediaPlayerMessage::Previous => {
                if let Some(player) = &self.player {
                    player.previous();
                    return true
                }
            },
            MediaPlayerMessage::PlayPause => {
                if let Some(player) = &self.player {
                    player.play_pause();
                    return true
                }
            },
            MediaPlayerMessage::Next => {
                if let Some(player) = &self.player {
                    player.next();
                    return true
                }
            },
            MediaPlayerMessage::ChangePlayer => {
                todo!()
            },
        };
        return false
    }
}

impl Widget for MediaPlayer {
    fn draw(&self, ctx: &mut DrawContext) {
        self.inner_box.draw(ctx);
    }
    fn process_event(&mut self, e: &Event) -> bool {
        self.inner_box.process_event(e)
    }
}

fn main() {
    let mut app = App::<SDLBackend>::new("WTK Media Player");
    let media_player = MediaPlayer::new().shared();
    app.add_widget(media_player.clone());
    //let elm = ElmApp::<MediaPlayer>::new(media_player);
    app.draw();
    while !app.quit {
        let draw = app.poll_and_process_event();
        let draw = media_player.borrow_mut().update_all() || draw;
        if draw { app.draw(); }
    }
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
