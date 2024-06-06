use std::env;

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "0.0.0.0:50051".parse()?;
    // 
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // 
    // // establish database connection
    // let connection = Database::connect(&database_url).await?;
    // Migrator::up(&connection, None).await?;
    // 
    // let hello_server = MyServer { connection };
    // Server::builder()
    //     .add_service(BlogpostServer::new(hello_server))
    //     .serve(addr)
    //     .await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}