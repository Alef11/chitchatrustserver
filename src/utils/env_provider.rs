use dotenv::dotenv;
use std::env;

fn main() {
    // Load the environment variables from the .env file
    dotenv().ok();

    // Access the environment variables using std::env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    println!("Database URL: {}", database_url);
    println!("Secret Key: {}", secret_key);
}
