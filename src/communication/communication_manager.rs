use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Communication_Manager{
    ip: String,
    port: u16,
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

    pub async fn handle_client(mut socket: tokio::net::TcpStream) {
        let mut buffer = [0; 1024];
        let mut n = 0;
        // 클라이언트로부터 비동기로 데이터 읽기
        match socket.read(&mut buffer).await {
            Ok(n) => {
                socket.write_all(b"success").await.unwrap();
            },
            Err(e) => println!("Data receive fail: {}", e)
        }
    }
    
    pub async fn test(self: Self) {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        println!("Server is running on 127.0.0.1:8080");
    
        loop {
            let (socket, _) = listener.accept().await.unwrap();
    
            // 비동기로 클라이언트 처리
            tokio::spawn(async move {
                Self::handle_client(socket).await
            });
        }
    }
}

impl Drop for Communication_Manager{
    fn drop(&mut self) {
        println!("Drop the bit!");
    }
}