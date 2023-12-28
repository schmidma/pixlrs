use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpStream, ToSocketAddrs},
};

use image::Rgb;

use crate::protocol::{to_ascii_command, PixelFlut, Vec2};

pub struct StdTcpProtocol {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl StdTcpProtocol {
    pub fn new(address: impl ToSocketAddrs) -> std::io::Result<Self> {
        let stream = TcpStream::connect(address)?;
        let reader = BufReader::new(stream.try_clone()?);
        let writer = BufWriter::new(stream);
        Ok(Self { reader, writer })
    }
}

impl PixelFlut for StdTcpProtocol {
    type Error = std::io::Error;

    fn get_pixel(&mut self, position: Vec2) -> Result<Rgb<u8>, Self::Error> {
        let command_string = format!("PX {} {}\n", position.x, position.y);
        self.writer.write_all(command_string.as_bytes())?;
        self.writer.flush()?;
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        let color: Vec<_> = buffer.split_whitespace().collect();
        let r = color[3].parse().unwrap();
        let g = color[4].parse().unwrap();
        let b = color[5].parse().unwrap();
        Ok(Rgb([r, g, b]))
    }

    fn set_pixel(&mut self, position: Vec2, color: &Rgb<u8>) -> Result<(), Self::Error> {
        let command_string = to_ascii_command(position, color);
        self.writer.write_all(command_string.as_bytes())
    }

    fn get_size(&mut self) -> Result<Vec2, Self::Error> {
        self.writer.write_all(b"SIZE\n")?;
        self.writer.flush()?;
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        let size: Vec<_> = buffer.split_whitespace().collect();
        let x = size[1].parse().unwrap();
        let y = size[2].parse().unwrap();
        Ok(Vec2 { x, y })
    }
}
