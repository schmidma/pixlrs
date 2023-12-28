use crate::protocol::{PixelFlut, Vec2};

pub struct Canvas<Protocol>
where
    Protocol: PixelFlut,
{
    pub offset: Vec2,
    protocol: Protocol,
}

impl<Protocol> Canvas<Protocol>
where
    Protocol: PixelFlut,
{
    pub fn new(protocol: Protocol, offset: Vec2) -> Result<Self, Protocol::Error> {
        Ok(Self { offset, protocol })
    }

    pub fn set_pixel(
        &mut self,
        position: Vec2,
        color: &image::Rgb<u8>,
    ) -> Result<(), Protocol::Error> {
        self.protocol.set_pixel(position + self.offset, color)
    }

    pub fn size(&mut self) -> Result<Vec2, Protocol::Error> {
        self.protocol.get_size()
    }
}
