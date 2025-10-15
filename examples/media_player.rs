use mpris;
use mpris::Player;
use mpris::PlayerFinder;
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
    player_list: Rc<RefCell<Hider<WBox>>>,
    player_label: Rc<RefCell<Label>>,
    player_list_button: Rc<RefCell<Button>>,
    player_finder: PlayerFinder,
}

#[derive(Debug)]
pub enum MediaPlayerMessage {
    Previous,
    PlayPause,
    Next,
    ChangePlayer(String),
    UpdateStatus,
    SetProgress(f32),
    TogglePlayerList,
}


impl MediaPlayer {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(sender);

        let mut player_box = WBox::new(Orientation::Horizontal);
        let player_label = Label::new(String::new()).shared();
        let player_list_button = Button::new("+", elm_cb!(sender, _b => MediaPlayerMessage::TogglePlayerList)).padding(2).shared();
        player_box.add_widget(player_label.clone()).add_widget(player_list_button.clone());

        let playing_label = Label::new("Playing: error").shared();
        let previous_button = Button::new("<", elm_cb!(sender, _b => MediaPlayerMessage::Previous)).padding(5);
        let play_pause_button = Button::new("||", elm_cb!(sender, _b => MediaPlayerMessage::PlayPause)).padding(5);
        let next_button = Button::new(">", elm_cb!(sender, _b => MediaPlayerMessage::Next)).padding(5);
        let bar = Bar::new(300, elm_cb!(sender, progress => MediaPlayerMessage::SetProgress(progress))).shared();

        let mut inner_box = WBox::new(Orientation::Vertical);
        let mut buttons = WBox::new(Orientation::Horizontal);
        buttons.add_widget(previous_button.shared());
        buttons.add_widget(play_pause_button.shared());
        buttons.add_widget(next_button.shared());

        let player_list = WBox::new(Orientation::Vertical);
        let player_list = Hider::<WBox>::new(player_list, true).shared();

        inner_box.add_widget(player_box.shared());
        inner_box.add_widget(player_list.clone());
        inner_box.add_widget(playing_label.clone());
        inner_box.add_widget(bar.clone());
        inner_box.add_widget(buttons.shared());

        let player_finder = PlayerFinder::new().unwrap();
        let player = find_player(&player_finder).ok();

        // I would prefer listening to player events externally but Player is not thread-safe
        std::thread::Builder::new().spawn(enclose!((sender) move || {
            loop {
                sender.send(MediaPlayerMessage::UpdateStatus).unwrap();
                std::thread::sleep(Duration::new(1, 0));
            }
        })).unwrap();

        sender.send(MediaPlayerMessage::UpdateStatus).unwrap();

        MediaPlayer {
            player,
            sender,
            receiver,
            inner_box,
            playing_label,
            bar,
            player_list,
            player_label,
            player_list_button,
            player_finder,
        }
    }
}

impl ElmModel for MediaPlayer {
    type Message = MediaPlayerMessage;

    fn receiver(&mut self) -> &mut mpsc::Receiver<Self::Message> {
        &mut self.receiver
    }

    fn update<B>(&mut self, app: &mut App<B>, msg: Self::Message) where B: Backend {
        println!("{msg:?}");
        match msg {
                MediaPlayerMessage::Previous => {
                if let Some(player) = &self.player {
                    let _ = player.previous();
                    self.sender.send(MediaPlayerMessage::UpdateStatus).unwrap();
                }
            },
            MediaPlayerMessage::PlayPause => {
                if let Some(player) = &self.player {
                    let _ = player.play_pause();
                    self.sender.send(MediaPlayerMessage::UpdateStatus).unwrap();
                }
            },
            MediaPlayerMessage::Next => {
                if let Some(player) = &self.player {
                    let _ = player.next();
                    self.sender.send(MediaPlayerMessage::UpdateStatus).unwrap();
                }
            },
            MediaPlayerMessage::ChangePlayer(player_name) => {
                self.player = self.player_finder.find_by_name(&player_name).ok();
                self.player_list.borrow_mut().hide();
                self.player_list_button.borrow_mut().set_text("+");
                self.sender.send(MediaPlayerMessage::UpdateStatus).unwrap();
            },
            MediaPlayerMessage::UpdateStatus => {
                if let Some(player) = &self.player {
                    let metadata = player.get_metadata().unwrap();
                    let position = player.get_position().unwrap();
                    let artists = metadata.artists().map_or("?".to_string(), |arts| arts.join(", ").to_string());
                    let title = metadata.title().unwrap_or("?");
                    self.playing_label.borrow_mut().set_text(format!("{} - {}", artists, title));
                    self.player_label.borrow_mut().set_text(format!("Controlling {} ", player.identity()));
                    self.bar.borrow_mut().progress = metadata.length().map_or(0.0, |length| position.as_secs_f32() / length.as_secs_f32());
                    if !self.player_list.borrow().hidden() {
                        self.player_list.replace(create_player_list(&self.player_finder, self.sender.clone()));
                    }
                } else {
                    self.player_label.borrow_mut().set_text("No player selected");
                    self.playing_label.borrow_mut().set_text("Playing: nothing");
                }
            },
            MediaPlayerMessage::SetProgress(progress) => {
                if let Some(player) = &self.player {
                    let metadata = player.get_metadata().unwrap();
                    if let Some(length) = metadata.length() && let Some(track_id) = metadata.track_id() {
                        let position = Duration::from_secs_f32(length.as_secs_f32() * progress);
                        player.set_position(track_id, &position).unwrap();
                        self.sender.send(MediaPlayerMessage::UpdateStatus).unwrap();
                    }
                }
            },
            MediaPlayerMessage::TogglePlayerList => {
                let mut player_list = self.player_list.borrow_mut();
                if player_list.hidden() {
                    player_list.show();
                    self.player_list_button.borrow_mut().set_text("-");
                    *player_list = create_player_list(&self.player_finder, self.sender.clone());
                } else {
                    player_list.hide();
                    self.player_list_button.borrow_mut().set_text("+");
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

fn create_player_list(player_finder: &PlayerFinder, sender: Arc<mpsc::Sender<MediaPlayerMessage>>) -> Hider<WBox> {
    let mut new_box = WBox::new(Orientation::Vertical);
    for player in player_finder.iter_players().unwrap() {
        if let Ok(player) = player {
            let button = Button::new(player.identity().to_string(), elm_cb!(sender, _b => MediaPlayerMessage::ChangePlayer(player.identity().to_string())));
            new_box.add_widget(button.shared());
        }
    }
    Hider::new(new_box, false)
}

fn main() {
    let mut app = App::<SDLBackend>::new("WTK Media Player");
    let media_player = MediaPlayer::new().shared();
    app.add_widget(media_player.clone());
    app.elm_run(media_player);
}

fn find_player(player_finder: &PlayerFinder) -> Result<mpris::Player, String> {
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
    cb: Rc<dyn Fn(f32)>,
    pub margin: u32,
}

impl Widget for Bar {
    fn draw(&self, ctx: &mut DrawContext) {
        let thickness = 2;
        let margin = 5;
        ctx.set_color(Color::rgb(0xdd, 0xdd, 0xdd));
        let background = Rect::new(0, margin, self.size, thickness);
        ctx.draw_rect(background);
        ctx.set_color(THEME.primary);
        ctx.draw_rect(Rect::new(0, margin, (self.size as f32 * self.progress) as u32, thickness));
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
                    let cb = self.cb.clone();
                    cb(click_progress);
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
    fn new<F>(size: u32, cb: F) -> Self where F: Fn(f32) + 'static {
        Self {
            progress: 0.0,
            size,
            bounds: Rect::zero(),
            cb: Rc::new(cb),
            margin: 10,
        }
    }
}
