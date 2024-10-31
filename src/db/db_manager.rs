use hyper::client;
use mongodb::{bson::doc, error::Error, options::ClientOptions, Client, Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    UserId: String,
    UserPassword: String
}

pub struct Db_Manager {
    dbUri: String
}

pub enum DbError {
    MongoError,
    DocumentNotFound,
}

impl Db_Manager{
    pub fn new() -> Self {
        Self { dbUri: "mongodb://localhost:27017/ttwDB".to_string()}
    }

    pub async fn login_process(self: Self, userId: String, userPassword: String) -> mongodb::error::Result<()> {
        let client = Client::with_uri_str(self.dbUri).await?;
        let db = client.database("ttwDB");
        let collection: Collection<User> = db.collection("Users");

        // 쿼리 필터
        let filter = doc! { "UserId": &userId, "UserPassword": &userPassword };
        // 문서 조회
        let document = collection.find_one(filter).await?;
        match document {
            Some(_) => println!("Login Success!!"),
            None => println!("Login Faile - Id: {}, Password: {}", &userId, &userPassword),
        }
        Ok(())
    }

    async fn connection(self: Self) -> mongodb::error::Result<()> {
        let client = Client::with_uri_str(self.dbUri).await?;
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