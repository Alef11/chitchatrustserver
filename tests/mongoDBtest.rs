use chitchatrustserver::db::connect_to_db;

#[tokio::test]
async fn test_connect_to_db() {
    let client = connect_to_db().await;
    assert!(client.is_ok(), "Failed to connect to MongoDB");
    println!("Connected to MongoDB successfully!");
}
