use clap::Parser;
use color_eyre::{eyre::Context, Result};
use image::io::Reader as ImageReader;
use tokio::{
    io::AsyncWriteExt,
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

#[derive(Clone, Copy, Default)]
struct Position {
    x: usize,
    y: usize,
}

struct PixelFlut {
    stream: TcpStream,
}

impl PixelFlut {
    async fn new(address: impl ToSocketAddrs) -> Result<Self> {
        let stream = TcpStream::connect(address)
            .await
            .wrap_err("failed to connect socket")?;
        Ok(Self { stream })
    }

    async fn set_pixel(&mut self, position: Position, color: &image::Rgb<u8>) -> Result<()> {
        let command_string = format!(
            "PX {} {} {:02x}{:02x}{:02x}\n",
            position.x, position.y, color[0], color[1], color[2]
        );
        self.stream.write_all(command_string.as_bytes()).await?;
        Ok(())
    }

    // async fn get_size(&mut self) -> Result<Position> {
    //     let resonse = self.tcp_stream.write_all(b"SIZE\n").await?;
    // }
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

    loop {
        for (x, y, pixel) in img.enumerate_pixels() {
            connection
                .set_pixel(
                    Position {
                        x: x as usize,
                        y: y as usize,
                    },
                    pixel,
                )
                .await?;
        }
    }
}
