use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use canvas::Canvas;
use clap::Parser;
use color_eyre::{eyre::Context, Result};
use image::io::Reader as ImageReader;
use protocol::Vec2;
use rand::{seq::SliceRandom, thread_rng};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::{fps_counter::AveragingFpsCounter, std_tcp_protocol::StdTcpProtocol};

mod canvas;
mod fps_counter;
mod protocol;
mod std_tcp_protocol;

#[derive(Parser, Debug)]
struct Arguments {
    #[clap(long, default_value = "151.217.15.90")]
    host: String,
    #[clap(long, default_value = "1337")]
    port: u16,
    #[clap(long)]
    image: PathBuf,
    #[clap(long, default_value = "3")]
    fps_log_interval: u64,
}

fn main() -> Result<()> {
    let arguments = Arguments::parse();
    FmtSubscriber::builder().with_max_level(Level::DEBUG).init();

    let image = ImageReader::open(arguments.image)
        .wrap_err("failed to open image")?
        .decode()
        .wrap_err("failed to decode image")?
        .to_rgb8();

    let mut rng = thread_rng();
    let mut indices: Vec<_> = image
        .enumerate_pixels()
        .filter(|(_, _, color)| color[1] > 0)
        .collect();
    indices.shuffle(&mut rng);

    let server_address = format!("{}:{}", arguments.host, arguments.port);
    info!("connecting to pixelflut server at {} ...", server_address);
    let protocol =
        StdTcpProtocol::new(server_address).wrap_err("failed to connect to pixelflut server")?;
    info!("connected to pixelflut server");

    let offset = Vec2::default();
    let mut canvas = Canvas::new(protocol, offset).wrap_err("failed to create canvas")?;

    let size = canvas.size().wrap_err("failed to get canvas size")?;
    info!("canvas size is {:?}", size);
    let offset = Vec2 {
        x: size.x.saturating_sub(image.width()),
        y: size.y.saturating_sub(image.height()),
    };
    canvas.offset = offset;
    info!("set canvas offset to {:?}", offset);

    let fps_log_interval = Duration::from_secs(arguments.fps_log_interval);
    let mut fps_counter = AveragingFpsCounter::new(fps_log_interval);
    let mut last_time_log = Instant::now();

    loop {
        for (x, y, color) in indices.iter() {
            canvas.set_pixel(Vec2 { x: *x, y: *y }, color)?;
        }

        let fps = fps_counter.tick().unwrap_or_default();
        if last_time_log.elapsed() > fps_log_interval {
            info!("fps: {:.2}", fps);
            last_time_log = Instant::now();
        }
    }
}
