use sdl2::{self, mouse::MouseButton, render::Canvas, sys::SDL_Event, video::{self, Window}, EventPump, Sdl, VideoSubsystem};
use crate::prelude::*;

use super::DrawBackend;

pub struct SDLBackend {
    ctx: Sdl,
    video: VideoSubsystem,
    canvas: Canvas<sdl2::video::Window>,
    event_pump: EventPump,
    ttf: sdl2::ttf::Sdl2TtfContext,
}

impl Backend for SDLBackend {
    fn init(title: &str) -> Self {
        let ctx = sdl2::init().unwrap();
        let video = ctx.video().unwrap();
        let win = video.window(title, 300, 200)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = win.into_canvas().build().unwrap();
        let event_pump = ctx.event_pump().unwrap();
        let ttf = sdl2::ttf::init().unwrap();

        DrawBackend::clear(&mut canvas);
        canvas.present();

        Self {
            ctx,
            video,
            canvas,
            event_pump,
            ttf,
        }
    }
    fn poll_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event().map(Into::<Event>::into)
    }
    fn draw_backend(&mut self) -> &mut impl DrawBackend {
        &mut self.canvas
    }
}

impl Into<Event> for sdl2::event::Event {
    fn into(self) -> Event {
        match self {
            Self::Quit{ timestamp: _ } => Event::Quit,
            Self::MouseButtonDown{ timestamp: _, window_id: _, which: _, mouse_btn, clicks, x, y }  => {
                Event::MouseButtonDown { button: mouse_btn.into(), clicks, x: x as u32, y: y as u32 }
            }
            _ => {
                eprintln!("Unknown SDL event: {self:?}");
                Event::Unsupported
            }
        }
    }
}

impl Into<crate::event::input::MouseButton> for sdl2::mouse::MouseButton {
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

impl Into<Rect> for &sdl2::rect::Rect {
    fn into(self) -> Rect {
        Rect {
            x: self.x as u32,
            y: self.y as u32,
            w: self.w as u32,
            h: self.h as u32,
        }
    }
}

impl Into<sdl2::rect::Rect> for &Rect {
    fn into(self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x as i32, self.y as i32, self.w, self.h)
    }
}

impl DrawBackend for Canvas<sdl2::video::Window> {
    fn draw_rect(&mut self, rect: &Rect) {
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
}

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGBA(self.r, self.g, self.b, self.a)
    }
}
