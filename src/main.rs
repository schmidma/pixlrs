use clap::Parser;
use color_eyre::{eyre::Context, Result};
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
struct Arguments {
    #[clap(long, default_value = "151.217.111.34")]
    host: String,
    #[clap(long, default_value = "1234")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = Arguments::parse();
    FmtSubscriber::builder().with_max_level(Level::DEBUG).init();

    let server_address = format!("{}:{}", arguments.host, arguments.port);
    info!("connecting to {server_address} ...");

    let mut stream = TcpStream::connect(server_address)
        .await
        .wrap_err("failed to connect socket")?;

    info!("connected, sending data ...");
    stream
        .write_all(b"hello world!")
        .await
        .wrap_err("failed to write data")?;
    Ok(())
}
