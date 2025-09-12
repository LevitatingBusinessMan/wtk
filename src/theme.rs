use std::sync::LazyLock;
use crate::prelude::*;

pub static THEME: LazyLock<Theme> = LazyLock::new(|| DEFAULT.clone());

pub const DEFAULT: &'static Theme = &LIGHT;

pub const LIGHT: Theme = Theme {
    background: Color::WHITE,
    primary: Color::BLACK,
    interactive: Color::RGB(0xcc, 0xcc, 0xcc),
};

pub const DARK: Theme = Theme {
    background: Color::RGB(0x40, 0x40, 0x50),
    primary: Color::WHITE,
    interactive: Color::RGB(0xcc, 0xcc, 0xcc),
};

#[derive(Debug, Clone)]
pub struct Theme {
    /// the window background
    pub background: Color,
    /// for text and lines (except that text doesn't work quite yet)
    pub primary: Color,
    /// for buttons and the likes
    pub interactive: Color,
}
