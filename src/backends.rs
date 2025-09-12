#[cfg(feature = "sdl2")]
mod sdl;
#[cfg(feature = "sdl2")]
pub use sdl::SDLBackend;

use crate::prelude::*;

/// A [Backend] generates events and provides a [DrawBackend].
pub trait Backend {
    fn init(title: &str) -> Self;
    fn poll_event(&mut self) -> Option<Event>;
    fn draw_backend(&mut self) -> &mut impl DrawBackend;
    fn resize(&mut self, size: Size); 
}

/// Trait for the windows drawing surface
pub trait DrawBackend {
    /// Draw a [Rect]
    fn draw_rect(&mut self, rect: Rect);
    /// Use a specific color
    fn set_color(&mut self, color: Color);
    /// Clear the surface
    fn clear(&mut self);
    /// Present drawings
    fn present(&mut self);
    fn draw_text(&mut self, text: &str, pos: Point);
}
