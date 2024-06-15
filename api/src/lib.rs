mod application;
mod domain;

use account_api::account_service_server::{AccountService, AccountServiceServer};
use account_api::{SignInRequest, SignInResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod account_api {
    tonic::include_proto!("account");
}

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
    let addr = "0.0.0.0:50051".parse()?;

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
