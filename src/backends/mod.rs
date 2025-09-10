mod sdl;
pub use sdl::SDLBackend;

use crate::prelude::*;

pub trait Backend {
    fn init(title: &str) -> Self;
    fn poll_event(&mut self) -> Option<Event>;
}

pub trait DrawBackend {
    fn draw_rect(&mut self, rect: Rect);
}
