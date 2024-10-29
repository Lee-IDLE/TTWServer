use std::convert::Infallible;
use std::net::{SocketAddr, TcpStream};

use tokio::net::{TcpListener, TcpStream as TokioTCPStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{
    accept_async, 
    tungstenite::protocol::Message,
    tungstenite::Message::*
};
use serde_json::{Result, value};
use futures::{StreamExt, SinkExt};
use http_body_util::Full;
use http_body::Body;
use hyper::{
    service::service_fn,
    body,
    body::Bytes,
    Request,
    Response,
    server::conn::http2,
    Uri,
    rt,
    Version,
};

#[path="../support/mod.rs"]
mod support;
use support::TokioIo;

pub struct Communication_Manager{
    ip: String,
    port: u16,
}

fn handle_json_message(json_str: &str) -> Result<()> {
    // 정규 표현식을 사용하여 JSON 데이터에서 이상한 문자열 제거
    /*
    let re = regex::Regex::new(r#"[^{}\[\]:,"a-zA-Z0-9\s]"#).unwrap();
    let cleaned_json_str = re.replace_all(json_str, "");
    let cleaned_json_str = cleaned_json_str.as_ref();
    */

    let v = match serde_json::from_str(json_str){
        Ok(s) => s,
        Err(e) => { println!("parsing err: {:?}", e) }
    };//cleaned_json_str
    println!("Received Json Message: {:?}", v);
    Ok(())
}

pub async fn handle_client(stream: TokioTCPStream, addr: SocketAddr){
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake occureed");
    println!("Websocekt connection established: {}", addr);
    
    let (mut write, mut read) = ws_stream.split();

    loop {
        tokio::select! {
            Some(msg) = read.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        // 텍스트가 JSON형식인지 확인
                        if let Ok(()) = handle_json_message(&text) {
                            println!("Receive Data: {}", text);
                        } else {
                            // JSON 메시지가 아님
                            println!("What the fucking is that?");
                        }
                    }
                    Ok(Message::Binary(bin)) => {
                        println!("Received a binary message from {}: {:?}", addr, bin)
                    }
                    Ok(Message::Ping(ping)) => {
                        let _ = write.send(Message::Pong(ping)).await;
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
    write.close().await;
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