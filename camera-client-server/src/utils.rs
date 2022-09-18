use std::net::TcpStream;
use tungstenite::{accept, WebSocket};

pub fn create_websocket(stream: TcpStream) -> WebSocket<TcpStream> {
    accept(stream).unwrap()
}

// trait ReadMessage: Send + Sync {
//     fn read(&mut self) -> Result<WebSocketMessage, WebSocketError>;
//     fn write(&mut self, message: WebSocketMessage) -> WebSocketResult<()>;
// }

// impl ReadMessage for WebSocket<TcpStream> {
//     fn read(&mut self) -> Result<WebSocketMessage, WebSocketError> {
//         self.read_message()
//     }
//     fn write(&mut self, message: WebSocketMessage) -> WebSocketResult<()> {
//         self.write_message(message)
//     }
// }
