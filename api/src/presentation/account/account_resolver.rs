use std::sync::Arc;

use tonic::{Request, Response, Status};

use account_api::account_service_server::AccountService;
use account_api::{SignInRequest, SignInResponse};

use crate::application::account::account_use_case::AccountUseCase;
use crate::application::account::dto::create_account_dto::CreateAccountDto;
use crate::presentation::account::account_resolver::account_api::AuthProvider;

pub mod account_api {
    tonic::include_proto!("account");
}

pub struct AccountResolver {
    account_use_case: Arc<AccountUseCase>,
}

impl AccountResolver {
    pub fn new(account_use_case: Arc<AccountUseCase>) -> AccountResolver {
        AccountResolver { account_use_case }
    }
}

#[tonic::async_trait]
impl AccountService for AccountResolver {
    async fn sign_in(
        &self,
        req: Request<SignInRequest>,
    ) -> Result<Response<SignInResponse>, Status> {
        let message = req.get_ref();

        self.account_use_case
            .create_account(CreateAccountDto::new(
                message.code.to_string(),
                AuthProvider::try_from(message.provider).unwrap(),
            ))
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SignInResponse {
            id: "1".to_string(),
        }))
    }
}
