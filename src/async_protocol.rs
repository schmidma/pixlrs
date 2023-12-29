use image::Rgb;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream, ToSocketAddrs,
    },
};

use crate::pixelflut::{parse_color_from_ascii, parse_size_from_ascii, to_ascii_command, Vec2};

pub struct AsyncProtocol {
    reader: BufReader<OwnedReadHalf>,
    writer: BufWriter<OwnedWriteHalf>,
    offset: Vec2,
}

impl AsyncProtocol {
    pub async fn new(address: impl ToSocketAddrs) -> std::io::Result<Self> {
        let stream = TcpStream::connect(address).await?;
        let (reader, writer) = stream.into_split();
        Ok(Self {
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
            offset: Vec2 { x: 0, y: 0 },
        })
    }

    pub async fn get_pixel(&mut self, position: Vec2) -> color_eyre::Result<Rgb<u8>> {
        let command_string = format!("PX {} {}\n", position.x, position.y);
        self.writer.write_all(command_string.as_bytes()).await?;
        self.writer.flush().await?;
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer).await?;
        Ok(parse_color_from_ascii(&buffer)?)
    }

    pub async fn set_pixel(&mut self, position: Vec2, color: &Rgb<u8>) -> std::io::Result<()> {
        let position = position + self.offset;
        let command_string = to_ascii_command(position, color);
        self.writer.write_all(command_string.as_bytes()).await
    }

    pub async fn get_size(&mut self) -> color_eyre::Result<Vec2> {
        self.writer.write_all(b"SIZE\n").await?;
        self.writer.flush().await?;
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer).await?;
        Ok(parse_size_from_ascii(&buffer)?)
    }

    pub async fn set_offset(&mut self, offset: Vec2) -> std::io::Result<()> {
        self.offset = offset;
        Ok(())
    }
}
