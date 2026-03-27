use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: mongodb::bson::oid::ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub age: u8,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub age: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub user: String,
    pub id: mongodb::bson::oid::ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
