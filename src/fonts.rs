//! For bitmap fonts
mod hex;
pub mod monogram;

pub use hex::UNSCII_8;
pub use hex::UNSCII_FANTASY_8;
pub use hex::UNSCII_16;

use crate::rect;
use crate::rect::Size;

pub const DEFAULT_FONT: &'static BitmapFont = &UNSCII_8;
const FONT_SCALING: f64 = 2.0;

/// A bitmap font.
/// They are currently required to be 8 bits in width.
#[derive(Debug)]
pub struct BitmapFont {
    /// how many rows
    pub height: u8,
    /// pointer to the flattened array of glyphs
    inner: &'static [u8],
    /// how a char should be mapped to an index into the array
    mapping: fn(char) -> usize,
}

impl BitmapFont {
    /// get a glyph for a char
    /// a glyph is one byte per row (1 bit per pixel)
    pub fn get(&self, c: char) -> &'static [u8] {
        let index = (self.mapping)(c) * self.height as usize;
        &self.inner[index..index + self.height as usize]
    }
    pub fn rendered_text_size(&self, text: &str) -> Size {
        rect::Size::new(text.len() as u32 * 8, self.height as u32) * FONT_SCALING
    }
}
