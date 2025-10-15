#[cfg(feature = "xrdb")]
use std::sync::LazyLock;

use crate::prelude::*;
#[cfg(feature = "xrdb")]
use xrdb;

pub(crate) static MONOGRAM_PNG: &'static [u8] = include_bytes!("fonts/monogram.png");
static FONT_SIZE: Size = Size::new(96, 96);
pub(crate) static GLYPH_SIZE: Size = Size::new(6, 12);
pub(crate) static DEFAULT_SCALE: f64 = 2.0;

/// Get a [Rect] describing where a character can be found
pub(crate) fn source_char(c: char) -> Rect {
    let i = c as u32 - ' ' as u32;
    let row = i / (FONT_SIZE.width/GLYPH_SIZE.width);
    let col = i % (FONT_SIZE.width/GLYPH_SIZE.width);
    Point::new(col * GLYPH_SIZE.width, row * GLYPH_SIZE.height).with_size(GLYPH_SIZE)
}

/// calculate size of a text string, does not yet take into account newlines
pub fn text_size(str: &str) -> Size {
    let mut glyph = GLYPH_SIZE * scale();
    glyph.width = glyph.width * str.len() as u32;
    glyph
}

#[cfg(feature = "xrdb")]
static XFT_DPI: LazyLock<Option<f64>> = LazyLock::new(|| xrdb::Xft::new().dpi.map(|dpi| dpi as f64 / 100.0));

static WTK_TEXT_SCALE: LazyLock<Option<f64>> = LazyLock::new(||
    std::env::var("WTK_TEXT_SCALE")
        .map(|e| e.parse().unwrap())
        .ok()
);

/// Get font scale
pub fn scale() -> f64 {
    #[cfg(feature = "xrdb")]
    if let Some(scale) = WTK_TEXT_SCALE.or(*XFT_DPI) {
        return scale;
    }

    WTK_TEXT_SCALE.unwrap_or(DEFAULT_SCALE)
}

// #[test]
// fn source_char_test() {
//     assert!(source_char('!') == Point::new(0, 6).with_size(GLYPH_SIZE));
//     assert!(source_char('a') == Point::new(6, 48).with_size(GLYPH_SIZE));
// }
