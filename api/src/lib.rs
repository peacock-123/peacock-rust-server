use std::sync::Arc;

use dotenv::dotenv;
use tonic::transport::Server;

use crate::application::account::account_use_case::{AccountUseCase, AuthEnv};
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
    let http_client = Arc::new(reqwest::Client::new());
    let auth_env = AuthEnv {
        kakao_client_id: std::env::var("KAKAO_CLIENT_ID").unwrap(),
        kakao_redirect_dir: std::env::var("KAKAO_REDIRECT_DIR").unwrap(),
    };

    let account_repository = Arc::new(AccountOrmRepository::new(connection));
    let account_use_case = Arc::new(AccountUseCase::new(
        account_repository.clone(),
        http_client.clone(),
        auth_env,
    ));
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
