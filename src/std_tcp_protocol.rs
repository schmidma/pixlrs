use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpStream, ToSocketAddrs},
};

use image::Rgb;

use crate::pixelflut::{
    parse_color_from_ascii, parse_size_from_ascii, to_ascii_command, PixelFlut, Vec2,
};

pub struct StdTcpProtocol {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
    offset: Vec2,
}

impl StdTcpProtocol {
    pub fn new(address: impl ToSocketAddrs) -> std::io::Result<Self> {
        let stream = TcpStream::connect(address)?;
        let reader = BufReader::new(stream.try_clone()?);
        let writer = BufWriter::new(stream);
        Ok(Self {
            reader,
            writer,
            offset: Vec2::default(),
        })
    }
}

impl PixelFlut for StdTcpProtocol {
    type Error = color_eyre::eyre::Error;

    fn get_pixel(&mut self, position: Vec2) -> Result<Rgb<u8>, Self::Error> {
        let command_string = format!("PX {} {}\n", position.x, position.y);
        self.writer.write_all(command_string.as_bytes())?;
        self.writer.flush()?;
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        Ok(parse_color_from_ascii(&buffer)?)
    }

    fn set_pixel(&mut self, position: Vec2, color: &Rgb<u8>) -> Result<(), Self::Error> {
        let position = position + self.offset;
        let command_string = to_ascii_command(position, color);
        self.writer.write_all(command_string.as_bytes())?;
        Ok(())
    }

    fn get_size(&mut self) -> Result<Vec2, Self::Error> {
        self.writer.write_all(b"SIZE\n")?;
        self.writer.flush()?;
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        Ok(parse_size_from_ascii(&buffer)?)
    }

    fn set_offset(&mut self, offset: Vec2) -> Result<(), Self::Error> {
        self.offset = offset;
        Ok(())
    }
}
