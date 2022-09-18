use crate::utils::create_websocket;
use std::{
    net::{IpAddr, TcpStream},
    sync::{Arc, RwLock},
    thread::spawn,
};
use tungstenite::Message;

pub struct Client {
    /** ID of this client */
    pub id: i32,
    /** IP address of this client's websocket */
    pub ip_addr: IpAddr,
    /** Does this client have an open websocket? */
    pub is_running: bool,
}

impl Client {
    pub fn new(id: i32, ip_addr: IpAddr) -> Self {
        Self {
            id,
            ip_addr,
            is_running: false,
        }
    }

    pub fn start(&mut self, stream: TcpStream) {
        println!("Starting client stream for {}", self.id);
        let websocket = create_websocket(stream);
        self.is_running = true;

        // Create the lock and start the websocket thread
        let ip_addr = Arc::new(self.ip_addr);
        let websocket_lock = Arc::new(RwLock::new(websocket));
        let is_running_lock = Arc::new(RwLock::new(self.is_running));
        spawn(move || {
            let mut websocket = websocket_lock.write().unwrap();

            loop {
                let msg = match websocket.read_message() {
                    Ok(msg) => msg,
                    Err(_e) => continue,
                };
                println!("{:?}", &msg);

                if let Message::Close(_close_frame) = msg {
                    let mut is_running = is_running_lock.write().unwrap();
                    *is_running = false;
                    break;
                }
                // We do not want to send back ping/pong messages.
                // if msg.is_binary() || msg.is_text() {
                //     self.websocket.write_message(msg).unwrap();
                // }
            }
        });
    }
}
