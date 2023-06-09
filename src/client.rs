use clap::Parser;
use service::{models::NewTask, WorldClient};
use std::{net::SocketAddr, time::Duration};
use tarpc::{client, context, tokio_serde::formats::Json};
use tokio::time::sleep;

#[derive(Parser)]
struct Flags {
    /// Sets the server address to connect to.
    #[clap(long)]
    server_addr: SocketAddr,
    /// Sets the name to say hello to.
    #[clap(long)]
    name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let flags = Flags::parse();

    let mut transport = tarpc::serde_transport::tcp::connect(flags.server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    // WorldClient is generated by the service attribute. It has a constructor `new` that takes a
    // config and any Transport as input.
    let client = WorldClient::new(client::Config::default(), transport.await?).spawn();

    let hello: Result<_, _> = async move {
        // Send the request twice, just to be safe! ;)
        tokio::select! {
            hello1 = client.new_task(context::current(),  NewTask{name: flags.name.clone()}) => { hello1 }
            hello2 = client.new_task(context::current(), NewTask{name: flags.name}) => { hello2 }
        }
    }
    .await;

    match hello {
        Ok(hello) => println!("{hello:?}"),
        Err(e) => println!("{:?}", anyhow::Error::from(e)),
    }

    // Let the background span processor finish.
    sleep(Duration::from_micros(1)).await;

    Ok(())
}
