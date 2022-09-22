use super::client::Client;
use std::{
    collections::HashMap,
    net::{SocketAddr, TcpListener, TcpStream},
};

const ENDPOINT: &'static str = "127.0.0.1:8000";
const CLIENT_LIMIT: i32 = 8;

/** https://github.com/mdn/samples-server/blob/master/s/webrtc-from-chat/chatserver.js */
pub struct SignallingServer {
    next_id: i32,
    clients: HashMap<i32, (SocketAddr, Client)>,
}

impl SignallingServer {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            clients: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        println!("Running server");
        let server = TcpListener::bind(ENDPOINT).unwrap();
        for stream in server.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(_e) => continue,
            };
            let peer_addr = stream.peer_addr().unwrap();

            if !self.get_addresses().contains(&peer_addr) {
                self.add_client(stream, peer_addr);
                self.alert_clients();
            }
        }
    }

    /** Returns all currently connected addresses */
    fn get_addresses(&self) -> Vec<SocketAddr> {
        self.clients
            .values()
            .filter(|(_, client)| client.is_running)
            .map(|value| value.0)
            .collect::<Vec<_>>()
    }

    fn add_client(&mut self, stream: TcpStream, peer_addr: SocketAddr) {
        let id = self.next_id;
        self.next_id += 1;

        let mut client = Client::new(id);

        client.start(stream);
        self.clients.insert(id, (peer_addr, client));
    }

    fn alert_clients(&mut self) {
        let mut ips: Vec<&SocketAddr> = Vec::new();
        let mut clients: Vec<&mut Client> = Vec::new();
        for (ip, client) in self.clients.values_mut() {
            if client.is_running {
                ips.push(ip);
                clients.push(client);
            }
        }

        for client in clients {
            if client.is_running {
                println!("Alerting client {}", client.id);
                client.alert(&ips);
            }
        }
    }
}

// A relic of what I tried to pass the socket into threads

// impl<SocketType> IClient<SocketType> for Client<WebSocket<TcpStream>>
// where
//     SocketType: ReadMessage,
// {
//     fn new(stream: TcpStream) -> Self {
//         let websocket = accept(stream).unwrap();

//         Self { websocket }
//     }

//     fn start(&self, socket: SocketType) {
//         let lock = Arc::new(RwLock::new(socket));
//         spawn(move || {
//             let websocket_lock = lock.read().unwrap();
//             let mut websocket = websocket_lock;
//             loop {
//                 let msg = match websocket.read_message() {
//                     Ok(msg) => msg,
//                     Err(_e) => continue,
//                 };
//                 println!("{:?}", &msg);
//                 println!("Binary: {}     Text: {}", msg.is_binary(), msg.is_text());

//                 // We do not want to send back ping/pong messages.
//                 // if msg.is_binary() || msg.is_text() {
//                 //     self.websocket.write_message(msg).unwrap();
//                 // }
//             }
//         });
//     }
// }
