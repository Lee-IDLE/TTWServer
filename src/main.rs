mod communication;
use communication::communication_manager::Communication_Manager;
mod db;
use db::db_manager;

#[tokio::main]
async fn main() {
    let comm_manager = Communication_Manager::default();
    //comm_manager.test().await;

    let db = db_manager::Db_Manager::new();
    let result = db.CreateTest().await;
}
