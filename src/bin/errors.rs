use std::error::Error;
use std::net::TcpStream;
use std::sync::mpsc;

use tracing::{trace, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let rx = mpsc::channel::<i32>().1;
    let err = rx.recv().unwrap_err();
    log_error(&err);

    let err = TcpStream::connect("127.0.0.1:55888").unwrap_err();
    log_error(&err);
}

fn log_error(e: &impl Error) {
    trace!("Error happened: {e}")
}
