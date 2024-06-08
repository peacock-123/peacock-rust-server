use tonic::{Request, Response, Status};
use tonic::transport::Server;

pub mod account_api {
    tonic::include_proto!("account");
}

use account_api::account_service_server::{AccountService, AccountServiceServer};
use account_api::{SignInRequest, SignInResponse};

#[derive(Debug, Default)]
struct AccountServiceImpl;

#[tonic::async_trait]
impl AccountService for AccountServiceImpl {
    async fn sign_in(&self, _: Request<SignInRequest>) -> Result<Response<SignInResponse>, Status> {
        let response = SignInResponse { id: 1 };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    let account = AccountServiceServer::new(AccountServiceImpl);

    Server::builder().add_service(account).serve(addr).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}