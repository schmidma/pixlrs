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
    fn set_offset(&mut self, offset: Vec2) -> Result<(), Self::Error>;
}

pub fn to_ascii_command(position: Vec2, color: &Rgb<u8>) -> String {
    format!(
        "PX {} {} {:02x}{:02x}{:02x}\n",
        position.x, position.y, color[0], color[1], color[2],
    )
}

pub fn parse_color_from_ascii(buffer: &str) -> Result<Rgb<u8>, std::num::ParseIntError> {
    let color: Vec<_> = buffer.split_whitespace().collect();
    let r = color[3].parse()?;
    let g = color[4].parse()?;
    let b = color[5].parse()?;
    Ok(Rgb([r, g, b]))
}

pub fn parse_size_from_ascii(buffer: &str) -> Result<Vec2, std::num::ParseIntError> {
    let size: Vec<_> = buffer.split_whitespace().collect();
    let x = size[1].parse()?;
    let y = size[2].parse()?;
    Ok(Vec2 { x, y })
}
