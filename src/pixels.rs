#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Into<Color> for (u8,u8,u8,u8) {
    fn into(self) -> Color {
        Color { r: self.0, g: self.1, b: self.2, a: self.3 }
    }
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255}
    }
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(0xff, 0xff, 0xff);
    pub const RED: Color = Color::rgb(0xff, 0x00, 0x00);
    pub const GREEN: Color = Color::rgb(0x00, 0xff, 0x00);
    pub const BLUE: Color = Color::rgb(0x00, 0x00, 0xff);
}
