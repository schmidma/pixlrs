use image::Rgb;
use kanal::{AsyncReceiver, OneshotAsyncSender};
use tokio::{
    net::ToSocketAddrs,
    runtime::{Builder, Runtime},
};

use crate::{
    async_protocol::AsyncProtocol,
    pixelflut::{PixelFlut, Vec2},
};

enum Command {
    GetPixel(Vec2, OneshotAsyncSender<Rgb<u8>>),
    SetPixel(Vec2, Rgb<u8>),
    GetSize(OneshotAsyncSender<Vec2>),
    SetOffset(Vec2),
}

pub struct AsyncMultiThreadedProtocol {
    runtime: Runtime,
    sender: kanal::AsyncSender<Command>,
    tasks: Vec<tokio::task::JoinHandle<color_eyre::Result<()>>>,
}

impl AsyncMultiThreadedProtocol {
    pub fn new(
        server_address: impl ToSocketAddrs + Clone,
        num_endpoints: usize,
    ) -> color_eyre::Result<Self> {
        let (sender, receiver) = kanal::bounded_async(100);
        let mut tasks = Vec::with_capacity(num_endpoints);
        let runtime = Builder::new_multi_thread().enable_all().build()?;
        for _ in 0..num_endpoints {
            let protocol = runtime.block_on(AsyncProtocol::new(server_address.clone()))?;
            let receiver = receiver.clone();
            let task = runtime.spawn(async move { run_protocol(receiver, protocol).await });
            tasks.push(task);
        }
        Ok(Self {
            runtime,
            sender,
            tasks,
        })
    }
}

impl PixelFlut for AsyncMultiThreadedProtocol {
    type Error = color_eyre::eyre::Error;

    fn get_pixel(&mut self, position: Vec2) -> Result<Rgb<u8>, Self::Error> {
        let (sender, receiver) = kanal::oneshot_async();
        self.runtime
            .block_on(self.sender.send(Command::GetPixel(position, sender)))
            .unwrap();
        Ok(self.runtime.block_on(receiver.recv()).unwrap())
    }

    fn set_pixel(&mut self, position: Vec2, color: &Rgb<u8>) -> Result<(), Self::Error> {
        self.runtime
            .block_on(self.sender.send(Command::SetPixel(position, *color)))
            .unwrap();
        Ok(())
    }

    fn get_size(&mut self) -> Result<Vec2, Self::Error> {
        let (sender, receiver) = kanal::oneshot_async();
        self.runtime
            .block_on(self.sender.send(Command::GetSize(sender)))
            .unwrap();
        Ok(self.runtime.block_on(receiver.recv()).unwrap())
    }

    fn set_offset(&mut self, offset: Vec2) -> Result<(), Self::Error> {
        self.runtime
            .block_on(self.sender.send(Command::SetOffset(offset)))
            .unwrap();
        Ok(())
    }
}

async fn run_protocol(
    receiver: AsyncReceiver<Command>,
    mut protocol: AsyncProtocol,
) -> color_eyre::Result<()> {
    while let Ok(command) = receiver.recv().await {
        match command {
            Command::GetPixel(position, sender) => {
                let _ = sender.send(protocol.get_pixel(position).await?).await;
            }
            Command::SetPixel(position, color) => {
                protocol.set_pixel(position, &color).await?;
            }
            Command::GetSize(sender) => {
                let _ = sender.send(protocol.get_size().await?).await;
            }
            Command::SetOffset(offset) => {
                protocol.set_offset(offset).await?;
            }
        }
    }
    Ok(())
}
