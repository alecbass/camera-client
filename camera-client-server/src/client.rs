use crate::utils::create_websocket;
use std::net::{SocketAddr, TcpStream};
use tungstenite::{Message, WebSocket};

pub struct Client {
    /** ID of this client */
    pub id: i32,
    /** Websocket connection of this client */
    pub websocket: Option<WebSocket<TcpStream>>,
}

impl Client {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            websocket: None,
        }
    }

    pub fn start(&mut self, stream: TcpStream) {
        println!("Starting client stream for {}", self.id);
        let websocket = create_websocket(stream);
        self.websocket = Some(websocket);

        // // Create the lock and start the websocket thread
        // let ip_addr = Arc::new(self.ip_addr);
        // let websocket_lock = RwLock::new(self.websocket.unwrap());
        // let is_running_lock = Arc::new(RwLock::new(self.is_running));
        // spawn(move || {
        //     let mut websocket = websocket_lock.write().unwrap();

        //     loop {
        //         let msg = match websocket.read_message() {
        //             Ok(msg) => msg,
        //             Err(_e) => continue,
        //         };
        //         println!("{:?}", &msg);

        //         if let Message::Close(_close_frame) = msg {
        //             let mut is_running = is_running_lock.write().unwrap();
        //             *is_running = false;
        //             break;
        //         }
        //         // We do not want to send back ping/pong messages.
        //         // if msg.is_binary() || msg.is_text() {
        //         //     self.websocket.write_message(msg).unwrap();
        //         // }
        //     }
        // });
    }

    /** Alerts this client of other clients' addresses */
    pub fn alert(&mut self, addresses: &Vec<&SocketAddr>) {
        let ip_strings = addresses
            .iter()
            .map(|ip| ip.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        println!("Alerting: {}", ip_strings);

        let bytes = ip_strings.as_bytes().to_vec();

        let websocket = self.websocket.as_mut();

        if let Some(websocket) = websocket {
            if let Err(e) = websocket.write_message(Message::Binary(bytes)) {
                println!("Client {} websocket error: {:?}", self.id, e);
            }
        }
    }
}
