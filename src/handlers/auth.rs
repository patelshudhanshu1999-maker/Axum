use crate::models::user::{
    User,
    RegisterUser,
    RegisterResponse,
};
use ::axum::http::StatusCode;
use ::axum::{Extension, Json};
use ::mongodb::Client;


pub async fn hello() -> String {
    "Hello, World2!".to_string()
}


pub async fn register(
    Extension(client): Extension<Client>,
    Json(payload): Json<RegisterUser>,
) -> Result<Json<RegisterResponse>, (StatusCode, String)> {
    if payload.email.is_empty() || payload.password.is_empty() || payload.name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "All fields are required".to_string(),
        ));
    }

    let db = client.database("UserDB");
    let collection = db.collection::<User>("users");

    let user = User {
        id: mongodb::bson::oid::ObjectId::new(),
        name: payload.name,
        email: payload.email,
        password: payload.password,
        age: 0,
    };

    match collection.insert_one(&user).await {
        Ok(_) => Ok(Json(RegisterResponse {
            user: user.name,
            id: user.id,
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }

    // match collection.insert_one(&user).await {
    //     Ok(_) => Ok(Json(RegisterResponse {
    //         user: user.name,
    //         id: user.id,
    //     })),
    //     Err(e) => {
    //         let error_string = e.to_string();
    //         // E11000 is the MongoDB error code for duplicate key
    //         if error_string.contains("E11000 duplicate key error") {
    //             Err((
    //                 StatusCode::CONFLICT,
    //                 "User with this email already exists".to_string(),
    //             ))
    //         } else {
    //             Err((StatusCode::INTERNAL_SERVER_ERROR, error_string))
    //         }
    //     }
    // }
}