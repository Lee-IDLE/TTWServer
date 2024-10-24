mod communication;
use communication::communication_manager::Communication_Manager;

#[tokio::main]
async fn main() {
    let comm_manager = Communication_Manager::default();
    comm_manager.test().await;
}
