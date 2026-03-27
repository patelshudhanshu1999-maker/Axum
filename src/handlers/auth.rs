use crate::models::user::{LoginUser, RegisterResponse, RegisterUser, User};
use crate::utils::password::{hash_password, verify_password};
use ::axum::http::StatusCode;
use ::axum::{Extension, Json};
use ::mongodb::Client;
use ::mongodb::bson::doc;

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

    let password = payload.password.clone();
    let hashed_password = tokio::task::spawn_blocking(move || hash_password(&password))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = User {
        id: mongodb::bson::oid::ObjectId::new(),
        name: payload.name,
        email: payload.email,
        password: hashed_password,
        age: payload.age.unwrap_or(18),
    };

    match collection.insert_one(&user).await {
        Ok(_) => Ok(Json(RegisterResponse {
            user: user.name,
            id: user.id,
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn login(
    Extension(client): Extension<Client>,
    Json(payload): Json<LoginUser>,
) -> Result<Json<RegisterResponse>, (StatusCode, String)> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Email and password are required".to_string(),
        ));
    }

    let db = client.database("UserDB");
    let collection = db.collection::<User>("users");

    let filter = doc! {"email": payload.email};
    let user = collection
        .find_one(filter)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Invalid email or password".to_string(),
        ))?;

    let password = payload.password.clone();
    let hashed_password = user.password.clone();
    let is_valid =
        tokio::task::spawn_blocking(move || verify_password(&password, &hashed_password))
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if is_valid {
        Ok(Json(RegisterResponse {
            user: user.name,
            id: user.id,
        }))
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            "Invalid email or password".to_string(),
        ))
    }
}
