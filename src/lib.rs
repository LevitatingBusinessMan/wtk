//! # Rust Widget Toolkit
//! 
//! Wtk is a simple widget toolkit.
//! 
//! ```
//! fn main() {
//!     let mut app = App::<SDLBackend>::new("WTK button example");
//!     let button = Button::new("clickme", |b| {
//!         b.set_text("clicked");
//!     });
//!     app.add_widget(button.shared());
//!     app.run();
//! }
//! ```
//! 
pub mod widgets;
pub mod rect;
pub mod event;
pub mod app;
pub mod prelude;
pub mod backends;
pub mod draw;
pub mod pixels;
pub mod font;
#[macro_use]
pub mod macros;
pub mod theme;
#[cfg(feature = "elm")]
pub mod elm;
mod ffi;
