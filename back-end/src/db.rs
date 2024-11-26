use crate::config::{db_name, db_url};
use mongodb::{Client, Database};

pub async fn get_database() -> Database {
    let client = Client::with_uri_str(db_url())
        .await
        .expect("Failed to initialize client.");
    client.database(db_name().as_str())
}

