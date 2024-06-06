use std::env;
use entity::{sea_orm::{Database}};

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = Database::connect(&database_url).await?;

    connection.ping().await?;
   
    connection.close().await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}