use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream as TokioTCPStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use serde_json::Value;

use futures::{StreamExt, SinkExt};

#[path = "../services/mod.rs"]
mod service;
use service::communication_service;

#[path="../support/mod.rs"]
mod support;
use support::TokioIo;

pub struct Communication_Manager{
    ip: String,
    port: u16,
}

enum JsonParseError{
    InvalidJson,
    MissingField,
}
// json 메시지가 맞는지 확인
fn handle_json_message(json_str: &str) -> Result<Value, JsonParseError> {
    println!("Received Json Message: {:?}", json_str);
    match serde_json::from_str(json_str) {
        Ok(value) => {
            Ok(value)
        }
        Err(_) => {
            println!("Parsing Error");
            Err(JsonParseError::InvalidJson)
        }
    }
}

pub async fn handle_client(stream: TokioTCPStream, addr: SocketAddr){
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake occureed");
    println!("Websocekt connection established: {}", addr);
    
    let (mut sender, mut receiver) = ws_stream.split();

    loop {
        tokio::select! {
            Some(msg) = receiver.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        // 텍스트가 JSON형식인지 확인
                        if let Ok(value) = handle_json_message(&text) {
                            communication_service::category_processing(value);
                        } else {
                            // JSON 메시지가 아님
                            println!("What the fucking is that?");
                        }
                    }
                    Ok(Message::Binary(bin)) => {
                        println!("Received a binary message from {}: {:?}", addr, bin)
                    }
                    Ok(Message::Ping(ping)) => {
                        let _ = sender.send(Message::Pong(ping)).await;
                    }
                    Ok(Message::Close(_)) => {
                        println!("Connection closed by {}", addr);
                        break;
                    }
                    Err(e) => {
                        println!("Error receiving message: {}", e);
                        break;
                    }
                    _ => break
                }
            }
            /*
            _ = write.close() => {
                println!("Connection close {}", addr);
                break;
            }
            */
        }
    }
    sender.close().await;
}

impl Communication_Manager{
    pub fn new(ip: String, port: u16) -> Self {
        Self { ip, port}
    }

    pub fn default() -> Self {
        Self { 
            ip: "127.0.0.1".to_string(),
            port: 8080
         }
    }
    
    pub async fn test(self: Self) {
        let mut addr = self.ip.clone();
        addr.push_str(":");
        addr.push_str(self.port.to_string().as_str());

        let tcpSocket = TcpListener::bind(&addr).await;
        let listener = tcpSocket.expect("Fail to bind");
        println!("Listening on: {}", addr);

        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(handle_client(stream, addr));
        }
    }
}

impl Drop for Communication_Manager{
    fn drop(&mut self) {
        println!("Drop the bit!");
    }
}