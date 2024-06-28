use std::sync::Arc;

use dotenv::dotenv;
use tonic::transport::Server;

use crate::application::account::account_use_case::AccountUseCaseImpl;
use crate::infrastructure::repository::account_orm_repository::AccountOrmRepository;
use crate::infrastructure::repository::connection::Connection;
use crate::presentation::account::account_resolver::account_api::account_service_server::AccountServiceServer;
use crate::presentation::account::account_resolver::AccountResolver;

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    let connection = Connection::new(std::env::var("DATABASE_URL").unwrap()).await;
    let account_repository = Arc::new(AccountOrmRepository::new(connection));
    let account_use_case = Arc::new(AccountUseCaseImpl::new(account_repository.clone()));
    let account = AccountServiceServer::new(AccountResolver::new(account_use_case));

    Server::builder().add_service(account).serve(addr).await?;

    Ok(())
}

pub fn main() {
    dotenv().ok();

    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
