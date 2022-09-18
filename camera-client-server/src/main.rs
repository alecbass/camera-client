use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

mod client;
mod signalling;
mod utils;

use signalling::SignallingServer;

/// A WebSocket echo server
fn main() {
    let mut server = SignallingServer::new();
    server.start();
}
