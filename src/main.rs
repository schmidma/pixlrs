use clap::Parser;
use color_eyre::{eyre::Context, Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
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
        .write_all(b"SIZE\n")
        .await
        .wrap_err("failed to write data")?;
    let ans = stream.read_u8().await?;
    dbg!(ans);
    Ok(())
}
