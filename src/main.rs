use clap::Parser;
use color_eyre::{eyre::Context, Result};
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
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    const WHITE: Self = Self {
        red: 255,
        green: 255,
        blue: 255,
    };

    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
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

    async fn set_pixel(&mut self, position: Position, color: Color) -> Result<()> {
        let command_string = format!(
            "PX {} {} {:02x}{:02x}{:02x}\n",
            position.x, position.y, color.red, color.green, color.blue
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

    loop {
        for x in 100..200 {
            for y in 100..200 {
                info!("setting");
                connection
                    .set_pixel(Position { x, y }, Color::WHITE)
                    .await?;
            }
        }
    }
}
