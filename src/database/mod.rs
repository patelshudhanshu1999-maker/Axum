use mongodb::Client;

pub async fn connect() -> mongodb::error::Result<Client> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;

    let db = client.database("UserDB");

    // Any type of data in the database
    // let collection = db.collection("users");

    let collection = db.collection::<crate::models::user::User>("users");

    // create unique index on email field
    let index_model = mongodb::IndexModel::builder()
        .keys(mongodb::bson::doc! {"email": 1})
        .options(
            mongodb::options::IndexOptions::builder()
                .unique(true)
                .build(),
        )
        .build();

    collection.create_index(index_model).await?;

    Ok(client)
}
