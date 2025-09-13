use mpris;
use mpris::Player;
use wtk::elm_cb;
use wtk::enclose;
use wtk::prelude::*;
use wtk::theme::THEME;
use std::cell::RefCell;
use std::rc::Rc;
//use std::sync::mpmc::Sender;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;
use wtk::elm::*;

pub struct MediaPlayer {
    player: Option<Player>,
    sender: Arc<mpsc::Sender<MediaPlayerMessage>>,
    receiver: mpsc::Receiver<MediaPlayerMessage>,
    playing_label: Rc<RefCell<Label>>,
    inner_box: WBox,
    bar: Rc<RefCell<Bar>>,
}

pub enum MediaPlayerMessage {
    Previous,
    PlayPause,
    Next,
    ChangePlayer,
    UpdateStatus,
    SetProgress(f32),
}


impl MediaPlayer {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(sender);
        
        let player = find_player().ok();

        let player_label = Label::new(format!("Player: {}", player.as_ref().map_or( "none".to_owned(), |p| p.bus_name_player_name_part().to_string())));
        let playing_label = Label::new("Playing: error").shared();
        let previous_button = Button::new("<", elm_cb!(sender, _b => MediaPlayerMessage::Previous));
        let play_pause_button = Button::new("||", elm_cb!(sender, _b => MediaPlayerMessage::PlayPause));
        let next_button = Button::new(">", elm_cb!(sender, _b => MediaPlayerMessage::Next));
        let bar = Bar::new(sender.clone(), 400).shared();

        let mut inner_box = WBox::new(Orientation::Vertical);
        let mut player_controls_box = WBox::new(Orientation::Horizontal);
        player_controls_box.set_padding(4);
        player_controls_box.set_margin(5);
        player_controls_box.set_border(true);
        player_controls_box.add_widget(previous_button.shared());
        player_controls_box.add_widget(play_pause_button.shared());
        player_controls_box.add_widget(next_button.shared());

        inner_box.add_widget(player_label.shared());
        inner_box.add_widget(playing_label.clone());
        inner_box.add_widget(bar.clone());
        inner_box.add_widget(player_controls_box.shared());

        // I would prefer listening to player events externally but Player is not thread-safe
        std::thread::Builder::new().spawn(enclose!((sender) move || {
            loop {
                sender.send(MediaPlayerMessage::UpdateStatus).unwrap();
                std::thread::sleep(Duration::new(1, 0));
            }
        })).unwrap();

        MediaPlayer {
            player,
            sender,
            receiver,
            inner_box,
            playing_label,
            bar,
        }
    }
}

impl ElmModel for MediaPlayer {
    type Message = MediaPlayerMessage;

    fn receiver(&mut self) -> &mut mpsc::Receiver<Self::Message> {
        &mut self.receiver
    }

    fn update<B>(&mut self, app: &mut App<B>, msg: Self::Message) where B: Backend {
        match msg {
            MediaPlayerMessage::Previous => {
                if let Some(player) = &self.player {
                    player.previous().unwrap();
                    self.sender.send(MediaPlayerMessage::UpdateStatus).unwrap();
                }
            },
            MediaPlayerMessage::PlayPause => {
                if let Some(player) = &self.player {
                    player.play_pause().unwrap();
                    self.sender.send(MediaPlayerMessage::UpdateStatus).unwrap();
                }
            },
            MediaPlayerMessage::Next => {
                if let Some(player) = &self.player {
                    player.next().unwrap();
                    self.sender.send(MediaPlayerMessage::UpdateStatus).unwrap();
                }
            },
            MediaPlayerMessage::ChangePlayer => {
                todo!()
            },
            MediaPlayerMessage::UpdateStatus => {
                if let Some(player) = &self.player {
                    let metadata = player.get_metadata().unwrap();
                    let position = player.get_position().unwrap();
                    let artists = metadata.artists().map_or("none".to_string(), |arts| arts.join(", ").to_string());
                    let title = metadata.title().unwrap_or("none");
                    self.playing_label.borrow_mut().set_text(format!("Playing: {} - {}", artists, title));
                    self.bar.borrow_mut().progress = metadata.length().map_or(0.0, |length| position.as_secs_f32() / length.as_secs_f32())
                }
            },
            MediaPlayerMessage::SetProgress(progress) => {
                if let Some(player) = &self.player {
                    let metadata = player.get_metadata().unwrap();
                    if let Some(length) = metadata.length() && let Some(track_id) = metadata.track_id() {
                        println!("{}", progress);
                        let position = Duration::from_secs_f32(length.as_secs_f32() * progress);
                        println!("{} {:?}", track_id, position);
                        player.set_position(track_id, &position).unwrap();
                        self.sender.send(MediaPlayerMessage::UpdateStatus).unwrap();
                    }
                }
            },
        };
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

struct Bar {
    /// precentage
    pub progress: f32,
    pub size: u32,
    bounds: Rect,
    sender: Arc<mpsc::Sender<MediaPlayerMessage>>,
    pub margin: u32,
}

impl Widget for Bar {
    fn draw(&self, ctx: &mut DrawContext) {
        let thickness = 2;
        let margin = 5;
        ctx.set_color(Color::RGB(0xdd, 0xdd, 0xdd));
        let background = Rect::new(0, 0, self.size, thickness);
        ctx.draw_rect(background);
        ctx.set_color(THEME.primary);
        ctx.draw_rect(Rect::new(0, 0, (self.size as f32 * self.progress) as u32, thickness));
        ctx.claim(background + margin);
    }
    
    fn process_event(&mut self, e: &Event) -> bool {
        match e {
            Event::MouseButtonDown { button, pos } => {
                if matches!(button, MouseButton::Left) && pos.is_in(self.bounds) {
                    let start = self.bounds.x + self.margin;
                    let click = pos.x;
                    let end = start + self.size;
                    let click_progress = click as f32 / (end - start) as f32;
                    self.sender.send(MediaPlayerMessage::SetProgress(click_progress)).unwrap();
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }
    
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }
}

impl Bar {
    fn new(sender: Arc<mpsc::Sender<MediaPlayerMessage>>, size: u32) -> Self {
        Self {
            progress: 0.0,
            size,
            bounds: Rect::zero(),
            sender,
            margin: 5,
        }
    }
}
