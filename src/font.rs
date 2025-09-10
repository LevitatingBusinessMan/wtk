use crate::prelude::*;

pub(crate) static MONOGRAM_PNG: &'static [u8] = include_bytes!("../monogram.png");
pub(crate) static FONT_SIZE: Size = Size::new(96, 96);
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
pub fn text_size(nchars: usize) -> Size {
    let mut glyph = GLYPH_SIZE * DEFAULT_SCALE;
    glyph.width = glyph.width * nchars as u32;
    glyph
}

#[test]
fn source_char_test() {
    assert!(source_char('!') == Point::new(0, 6).with_size(GLYPH_SIZE));
    assert!(source_char('a') == Point::new(6, 48).with_size(GLYPH_SIZE));
}
