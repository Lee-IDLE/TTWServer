use std::sync::Arc;
use serde_json::{Value, json};
use crate::communication::communication_manager::db::db_manager::Db_Manager;

#[path = "../db/mod.rs"]
mod db;
use db::db_instance;

pub async fn category_processing(value: Value, db: &Arc<Db_Manager>) -> String{
    let mut result = String::new();

    match value.get("Category").and_then(|c| c.as_str()).unwrap() {
        "login" => {
            result = login_processing(value, db).await;
        }
        _ => {
            println!("What is that..?");
            result = "Error".to_string();
        }
    };

    result
}

async fn login_processing(value: Value, db: &Arc<Db_Manager>) -> String {
    let mut result = "fail".to_string();
    if let Some(data) = value.get("Data").and_then(|d| d.as_array()){
        for item in data {
            let id = item.get("UserId").and_then(|id| id.as_str()).unwrap();
            let password = item.get("UserPassword").and_then(|password| password.as_str()).unwrap();
            
            let db_manager: Arc<db::db_manager::Db_Manager> = Arc::clone(&db_instance::get_db_instance().await);
            match db_manager.login_search(id.to_string(), password.to_string()).await {
                Ok(_) => {
                    let json_data = json!({
                        "Category": "login",
                        "Data": [
                            {
                                "Result": "success"
                            }
                        ]
                    });
                    result = json_data.to_string();
                }
                Err(e) => {
                    let json_data = json!({
                        "Category": "login",
                        "Data":[
                            {
                                "Result": "fail",
                                "Message": e.to_string()
                            }
                        ]
                    });
                    result = json_data.to_string();
                }
            };
        }
    }

    result
}