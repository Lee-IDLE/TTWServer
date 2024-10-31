use serde_json::Value;

pub fn category_processing(value: Value){
    match value.get("category").and_then(|c| c.as_str()).unwrap() {
        "login" => {
            if let Some(data) = value.get("data").and_then(|d| d.as_array()){
                for item in data {
                    let id = item.get("id").and_then(|id| id.as_str()).unwrap();
                    let password = item.get("password").and_then(|password| password.as_str()).unwrap();

                    
                }
            }
        }
        _ => {
            println!("What is that..?");
        }
    };
}