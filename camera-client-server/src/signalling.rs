use super::client::Client;
use std::{
    collections::HashMap,
    net::{IpAddr, TcpListener, TcpStream},
};

const ENDPOINT: &'static str = "127.0.0.1:8000";
const CLIENT_LIMIT: i32 = 8;

/** https://github.com/mdn/samples-server/blob/master/s/webrtc-from-chat/chatserver.js */
pub struct SignallingServer {
    next_id: i32,
    clients: HashMap<IpAddr, Client>,
}

fn do_close(server: &SignallingServer, thing: i32) {}

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

            // Can we do the handshake outside of the main thread?
            self.add_client(stream);
            println!("Capacity: {}/{}", self.clients.len(), CLIENT_LIMIT);
        }
    }

    pub fn close_client(&self) {}

    fn add_client(&mut self, stream: TcpStream) {
        let peer_ip = stream.peer_addr().unwrap().ip();
        let local_ip = stream.peer_addr().unwrap().ip();

        let id = self.next_id;
        self.next_id += 1;

        let on_close = |ip_addr: IpAddr| {
            let clients = &self.clients;
        };

        let mut client = Client::new(id, peer_ip);

        println!("{:?}", peer_ip);
        println!("{:?}", local_ip);

        client.start(stream);
        // self.clients.push(client);
        if !self.clients.contains_key(&peer_ip) {
            println!("Existing client: {:?}", &peer_ip);
            self.clients.insert(peer_ip, client);
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
