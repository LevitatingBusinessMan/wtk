use std::cell::UnsafeCell;
use crate::prelude::*;

pub struct ThemeCell(UnsafeCell<Theme>);

unsafe impl Sync for ThemeCell {}

/// Update the theme. This may cause race conditions.
pub fn set_theme(theme: Theme) {
    unsafe { *THEME.0.get() = theme; }
}

impl std::ops::Deref for ThemeCell {
    type Target = Theme;
    
    fn deref(&self) -> &Theme {
        unsafe { &*self.0.get() }
    }
}

pub static THEME: ThemeCell = ThemeCell(UnsafeCell::new(DEFAULT));

pub const DEFAULT: Theme = LIGHT;

pub const LIGHT: Theme = Theme {
    background: Color::WHITE,
    primary: Color::BLACK,
    interactive: Color::rgb(0xcc, 0xcc, 0xcc),
};

pub const DARK: Theme = Theme {
    background: Color::rgb(0x40, 0x40, 0x50),
    primary: Color::WHITE,
    interactive: Color::rgb(0xcc, 0xcc, 0xcc),
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
