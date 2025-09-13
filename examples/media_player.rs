use mpris;
use mpris::Player;
use wtk::elm_cb;
use wtk::enclose;
use wtk::prelude::*;
use std::rc::Rc;
//use std::sync::mpmc::Sender;
use std::sync::mpsc;
use wtk::elm::*;

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
    app.elm_run(media_player);
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
