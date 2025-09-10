mod sdl;
pub use sdl::SDLBackend;

use crate::prelude::*;

/// A [Backend] generates events and provides a [DrawBackend].
pub trait Backend {
    fn init(title: &str) -> Self;
    fn poll_event(&mut self) -> Option<Event>;
    fn draw_backend(&mut self) -> &mut impl DrawBackend;
}

/// Trait for the windows drawing surface
pub trait DrawBackend {
    /// Draw a [Rect]
    fn draw_rect(&mut self, rect: &Rect);
    /// Use a specific color
    fn set_color(&mut self, color: sdl2::pixels::Color);
    /// Clear the surface
    fn clear(&mut self);
    /// Present drawings
    fn present(&mut self);
}
