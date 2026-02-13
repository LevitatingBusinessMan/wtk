//! # Rust Widget Toolkit
//! 
//! Wtk is a simple widget toolkit.
//! 
//! ```no_run
//! use wtk::prelude::*;
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
//! ## Embedding
//! 
//! You don't need to use [App], you may embed wtk by directly utilizing a [DrawBackend]. See the `embedded.rs` example.
//! 
//! [App]: app:App
//! [DrawBackend]: draw:DrawBackend
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
