use clap::Parser;
use color_eyre::{
    eyre::{Context, ContextCompat},
    Result,
};
use image::io::Reader as ImageReader;
use rand::{seq::SliceRandom, thread_rng};
use tokio::{
    io::AsyncBufReadExt,
    io::{AsyncWriteExt, BufReader},
    net::{TcpStream, ToSocketAddrs},
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
struct Arguments {
    #[clap(long, default_value = "151.217.15.90")]
    host: String,
    #[clap(long, default_value = "1337")]
    port: u16,
}

#[derive(Clone, Copy, Default, Debug)]
struct Position {
    x: u32,
    y: u32,
}

struct PixelFlut {
    stream: BufReader<TcpStream>,
}

impl PixelFlut {
    async fn new(address: impl ToSocketAddrs) -> Result<Self> {
        let stream = TcpStream::connect(address)
            .await
            .wrap_err("failed to connect socket")?;
        Ok(Self {
            stream: BufReader::new(stream),
        })
    }

    async fn set_pixel(&mut self, position: Position, color: &image::Rgb<u8>) -> Result<()> {
        let command_string = format!(
            "PX {} {} {:02x}{:02x}{:02x}\n",
            position.x, position.y, color[0], color[1], color[2]
        );
        self.stream.write_all(command_string.as_bytes()).await?;
        Ok(())
    }

    async fn set_offset(&mut self, position: Position) -> Result<()> {
        let command_string = format!("OFFSET {} {}\n", position.x, position.y);
        self.stream.write_all(command_string.as_bytes()).await?;
        Ok(())
    }

    async fn get_size(&mut self) -> Result<Position> {
        self.stream.write_all(b"SIZE\n").await?;
        let mut response = String::with_capacity(15);
        self.stream.read_line(&mut response).await?;
        let mut iter = response.split_whitespace();
        iter.next();
        let x_string = iter.next().wrap_err("cannot split x string")?;
        let y_string = iter.next().wrap_err("cannot split x string")?;
        Ok(Position {
            x: x_string.parse()?,
            y: y_string.parse()?,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::parse();
    FmtSubscriber::builder().with_max_level(Level::DEBUG).init();

    let server_address = format!("{}:{}", arguments.host, arguments.port);

    info!("connecting to {server_address} ...");
    let mut connection = PixelFlut::new(server_address).await?;
    info!("connected");
    let img = ImageReader::open("img/hulks.png")?.decode()?.to_rgb8();

    let mut rng = thread_rng();

    let mut indices: Vec<_> = img
        .enumerate_pixels()
        .filter(|(_, _, color)| color[1] > 0)
        .collect();
    indices.shuffle(&mut rng);

    let size = connection.get_size().await?;
    connection.set_offset(Position {
        x: 0,
        y: size.y / 2 - img.height() / 2,
    }).await?;
    loop {
        for (x, y, color) in &indices {
            connection
                .set_pixel(Position { x: *x, y: *y }, color)
                .await?;
        }
    }
}
