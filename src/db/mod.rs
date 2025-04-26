use mongodb::{Client, error::Result};

pub async fn connect_to_db() -> Result<Client> {
    // Replace this with your actual MongoDB URI
    let uri = "mongodb://localhost:27017";

    // Create a client
    let client = Client::with_uri_str(uri).await?;

    println!("Successfully connected to MongoDB!");

    Ok(client)
}
