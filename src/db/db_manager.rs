use mongodb::{Client, options::ClientOptions};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    UserId: String,
    UserPassword: String
}

pub struct Db_Manager {
    Uri: String
}

impl Db_Manager{
    pub fn new() -> Self {
        Self { Uri : "mongodb://localhost:27017".to_string() }
    }

    pub async fn CreateTest(self: Self) -> mongodb::error::Result<()> {
        let client = Client::with_uri_str(self.Uri).await?;
        let db = client.database("ttwDB");
        let collection = db.collection("Users");

        let new_user = User {
            UserId: "Nidus0526".to_string(),
            UserPassword: "48E8BC2FB09A492D0C80B04CAEE9E907B603E1EA59D46E18EC13430B96F8AB77".to_string()
        };

        //let doc = bson::doc!["UserId":"Nidus0526", "UserPassword":"48E8BC2FB09A492D0C80B04CAEE9E907B603E1EA59D46E18EC13430B96F8AB77"];
        let test = collection.insert_one(new_user).await?;   

        Ok(())
    }
}