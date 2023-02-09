use std::time::Duration;

use tracing::{info, span, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tokio::join!(receive_data(), send_data());
}

async fn receive_data() {
    let span = span!(Level::TRACE, "receiving_data");
    let _guard = span.enter();
    info!("receiving");
    tokio::time::sleep(Duration::from_secs(3)).await;
    info!("received");
}

async fn send_data() {
    let span = span!(Level::TRACE, "sending_data");
    let _guard = span.enter();
    info!("sending");
    tokio::time::sleep(Duration::from_secs(1)).await;
    info!("sent");
}
