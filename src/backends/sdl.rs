use sdl3::{self, event::WindowEvent, image::LoadTexture, mouse::MouseButton, render::Canvas, EventPump, Sdl, VideoSubsystem};
use crate::{font::{self}, prelude::*};

use super::DrawBackend;

pub struct SDLBackend {
    ctx: Sdl,
    video: VideoSubsystem,
    canvas: Canvas<sdl3::video::Window>,
    event_pump: EventPump,
}

impl Backend for SDLBackend {
    fn init(title: &str) -> Self {
        let ctx = sdl3::init().unwrap();
        let video = ctx.video().unwrap();
        let win = video.window(title, 300, 200)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = win.into_canvas();
        let event_pump = ctx.event_pump().unwrap();

        DrawBackend::clear(&mut canvas);
        canvas.present();

        video.text_input().start(canvas.window());

        Self {
            ctx,
            video,
            canvas,
            event_pump,
        }
    }
    fn poll_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event().map(Into::<Event>::into)
    }
    fn draw_backend(&mut self) -> &mut impl DrawBackend {
        &mut self.canvas
    }
    fn resize(&mut self, size: Size) {
        self.canvas.window_mut().set_size(size.width, size.height).unwrap();
    }
}

impl Into<Event> for sdl3::event::Event {
    fn into(self) -> Event {
        match self {
            Self::Quit{ timestamp: _ } => Event::Quit,
            Self::MouseButtonDown{ timestamp: _, window_id: _, which: _, mouse_btn, clicks: _, x, y }  => {
                Event::MouseButtonDown { button: mouse_btn.into(), pos: Point::new(x as u32, y as u32)}
            },
            Self::Window { timestamp, window_id, win_event: WindowEvent::Resized(w, h) } => {
                Event::Resized(Size::new(w as u32, h as u32))
            },
            Self::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => {
                Event::MouseMove(Point::new(x as u32, y as u32))
            },
            Self::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks: _, x, y } => {
                Event::MouseButtonUp { button: mouse_btn.into(), pos: Point::new(x as u32, y as u32)}
            },
            Self::TextInput { timestamp, window_id, text } => {
                Event::TextInput(text)
            },
            // Self::Window { timestamp, window_id, win_event: WindowEvent::Resized(w, h) } => {
            //     Event::Resized(Size::new(w as u32, h as u32))
            // },
            Self::Unknown { timestamp: _, type_: _ } => { Event::Unsupported },
            _ => {
                eprintln!("Unknown SDL event: {self:?}");
                Event::Unsupported
            }
        }
    }
}

impl Into<crate::event::input::MouseButton> for sdl3::mouse::MouseButton {
    fn into(self) -> crate::event::input::MouseButton {
        match self {
            MouseButton::Unknown => crate::event::input::MouseButton::Left,
            MouseButton::Left => crate::event::input::MouseButton::Left,
            MouseButton::Middle => crate::event::input::MouseButton::Middle,
            MouseButton::Right => crate::event::input::MouseButton::Right,
            MouseButton::X1 => crate::event::input::MouseButton::Left,
            MouseButton::X2 => crate::event::input::MouseButton::Right,
        }
    }
}

impl Into<Rect> for &sdl3::rect::Rect {
    fn into(self) -> Rect {
        Rect {
            x: self.x as u32,
            y: self.y as u32,
            width: self.w as u32,
            height: self.h as u32,
        }
    }
}

impl Into<sdl3::rect::Rect> for &Rect {
    fn into(self) -> sdl3::rect::Rect {
        sdl3::rect::Rect::new(self.x as i32, self.y as i32, self.width, self.height)
    }
}

impl Into<sdl3::rect::Rect> for Rect {
    fn into(self) -> sdl3::rect::Rect {
        sdl3::rect::Rect::new(self.x as i32, self.y as i32, self.width, self.height)
    }
}

impl Into<sdl3::render::FRect> for &Rect {
    fn into(self) -> sdl3::render::FRect {
        sdl3::render::FRect::new(self.x as f32, self.y as f32, self.width as f32, self.height as f32)
    }
}

impl Into<sdl3::render::FRect> for Rect {
    fn into(self) -> sdl3::render::FRect {
        sdl3::render::FRect::new(self.x as f32, self.y as f32, self.width as f32, self.height as f32)
    }
}

impl DrawBackend for Canvas<sdl3::video::Window> {
    fn draw_rect(&mut self, rect: Rect) {
        self.draw_rect(rect.into()).unwrap();
    }
    fn clear(&mut self) {
        self.set_draw_color(Color::WHITE);
        self.clear();
    }
    fn present(&mut self) {
        self.present();
    }
    fn set_color(&mut self, color: Color) {
        self.set_draw_color(color);
    }
    fn draw_text(&mut self, text: &str, mut pos: Point) {
        let texture_creator = self.texture_creator();
        let texture = texture_creator.load_texture_bytes(crate::font::MONOGRAM_PNG).unwrap();
        for c in text.chars() {
            self.copy(
                &texture,
                Some((&crate::font::source_char(c)).into()),
                Some((&pos.with_size(font::GLYPH_SIZE * font::scale())).into())
            ).unwrap();
            pos.x += (font::GLYPH_SIZE * font::scale()).width;
        }
    }
}

impl Into<sdl3::pixels::Color> for Color {
    fn into(self) -> sdl3::pixels::Color {
        sdl3::pixels::Color::RGBA(self.r, self.g, self.b, self.a)
    }
}
