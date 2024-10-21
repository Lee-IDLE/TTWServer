use tokio::netTcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

struct CommunicationManager{
    ip: String,
    port: int,
}

impl CommunicationManager{
    fn new(ip: String, port: int) -> Slef {
        Self { ip, port}
    }

    fn new() -> Self {
        Self { 
            ip: "127.0.0.1".toString(),
            port: 8080
         }
    }

    async fn handle_client(mut socket: tokio::net::TcpStream) {
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
    
    fn test() {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        println!("Server is running on 127.0.0.1:8080");
    
        loop {
            let (socket, _) = listener.accept().await.unwrap();
    
            // 비동기로 클라이언트 처리
            tokio::spawn(async move {
                handle_client(socket).await;
            })
        }
    }
}

impl Drop for CommunicationManager{
    fn drop(&mut self) {
        println!("Drop the bit!");
    }
}