use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub UserId: String,
    pub UserPassword: String
}