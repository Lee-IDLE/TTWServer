use std::sync::Arc;
use serde_json::Value;
use crate::communication::communication_manager::db::db_manager::Db_Manager;

#[path = "../db/mod.rs"]
mod db;
use db::db_instance;

pub async fn category_processing(value: Value, db: &Arc<Db_Manager>) -> String{
    let mut result = String::new();

    match value.get("Category").and_then(|c| c.as_str()).unwrap() {
        "login" => {
            if let Some(data) = value.get("Data").and_then(|d| d.as_array()){
                for item in data {
                    let id = item.get("UserId").and_then(|id| id.as_str()).unwrap();
                    let password = item.get("UserPassword").and_then(|password| password.as_str()).unwrap();
                    println!("Wow!!!");
                    let db_manager: Arc<db::db_manager::Db_Manager> = Arc::clone(&db_instance::get_db_instance().await);
                    match db_manager.login_process(id.to_string(), password.to_string()).await {
                        Ok(_) => {
                            result = "success".to_string();
                        }
                        Err(e) => {
                            result = e.to_string();
                        }
                    };
                }
            }
        }
        _ => {
            println!("What is that..?");
            result = "Error".to_string();
        }
    };

    result
}