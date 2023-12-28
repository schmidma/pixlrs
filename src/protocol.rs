use std::ops::Add;

use image::Rgb;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2 {
    pub x: u32,
    pub y: u32,
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub trait PixelFlut {
    type Error;

    fn get_pixel(&mut self, position: Vec2) -> Result<Rgb<u8>, Self::Error>;
    fn set_pixel(&mut self, position: Vec2, color: &Rgb<u8>) -> Result<(), Self::Error>;
    fn get_size(&mut self) -> Result<Vec2, Self::Error>;
}

pub fn to_ascii_command(position: Vec2, color: &Rgb<u8>) -> String {
    format!(
        "PX {} {} {:02x}{:02x}{:02x}\n",
        position.x, position.y, color[0], color[1], color[2],
    )
}
