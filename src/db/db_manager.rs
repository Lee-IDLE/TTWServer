use hyper::client;
use mongodb::{bson::doc, error::Error, options::ClientOptions, Client, Collection, Database};
use serde::{Deserialize, Serialize};

#[path = "../models/mod.rs"]
mod models;
use models::user;

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

    pub async fn login_process(self: &Self, userId: String, userPassword: String) -> Result<(), mongodb::error::Error> {
        let client = Client::with_uri_str(&self.dbUri).await?;
        let db = client.database("ttwDB");
        let collection: Collection<user::User> = db.collection("Users");

        // 쿼리 필터
        let filter = doc! { "UserId": &userId, "UserPassword": &userPassword };
        // 문서 조회
        let document = collection.find_one(filter).await?;
        match document {
            Some(_) => {
                println!("Login Success!!");
                Ok(())
            },
            None => {
                println!("Login Faile - Id: {}, Password: {}", &userId, &userPassword);
                Err(Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "아이디 또는 비밀번호가 틀렸습니다.")))
            },
        }
    }

    async fn connection(self: Self) -> mongodb::error::Result<()> {
        let client = Client::with_uri_str(self.dbUri).await?;
        let db = client.database("ttwDB");
        let collection = db.collection("Users");

        let new_user = user::User {
            UserId: "Nidus0526".to_string(),
            UserPassword: "48E8BC2FB09A492D0C80B04CAEE9E907B603E1EA59D46E18EC13430B96F8AB77".to_string()
        };

        //let doc = bson::doc!["UserId":"Nidus0526", "UserPassword":"48E8BC2FB09A492D0C80B04CAEE9E907B603E1EA59D46E18EC13430B96F8AB77"];
        let test = collection.insert_one(new_user).await?;   

        Ok(())
    }
}

