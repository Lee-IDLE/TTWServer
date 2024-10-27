use std::convert::Infallible;
use std::net::{SocketAddr, TcpStream};

use tokio::net::{TcpListener, TcpStream as TokioTCPStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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

// mut socket: tokio::net::TcpStream
pub async fn handle_client(temp: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(
        Response::new(Full::new(Bytes::from("success")))
    )
    /*
    let mut buffer = [0; 1024];
    let mut n = 0;
    // 클라이언트로부터 비동기로 데이터 읽기
    match socket.read(&mut buffer).await {
        Ok(n) => {
            println!("연결 성공 및 데이터 전송!");
            socket.write_all(b"success").await.unwrap();
        },
        Err(e) => println!("Data receive fail: {}", e)
    }
    */
}

#[derive(Clone)]
// An Executor that uses the tokio runtime.
pub struct TokioExecutor;

// Implement the `hyper::rt::Executor` trait for `TokioExecutor` so that it can be used to spawn
// tasks in the hyper runtime.
// An Executor allows us to manage execution of tasks which can help us improve the efficiency and
// scalability of the server.
impl<F> hyper::rt::Executor<F> for TokioExecutor
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, fut: F) {
        tokio::task::spawn(fut);
    }
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
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let listener = TcpListener::bind(addr).await.unwrap();
        
        loop {
            let (stream, _) = listener.accept().await.unwrap();

            let io = TokioIo::new(stream);

            tokio::task::spawn(async move {
                match http2::Builder::new(TokioExecutor)
                    .serve_connection(io, service_fn(handle_client))
                    .await
                {
                    Ok(_) => { println!("connection success") },
                    Err(err) => {eprintln!("Error serving connection: {}", err )}
                }
            });
        }
        /*
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        println!("Server is running on 127.0.0.1:8080");
    
        loop {
            println!("연결 대기!");
            let (socket, _) = listener.accept().await.unwrap();
            println!("연결 성공!");
            // 비동기로 클라이언트 처리
            tokio::spawn(async move {
                Self::handle_client(socket).await
            });
        }
        */
    }
}

impl Drop for Communication_Manager{
    fn drop(&mut self) {
        println!("Drop the bit!");
    }
}