use std::sync::Arc;

use reqwest::Error;
use thiserror::Error;

use crate::application::account::dto::create_account_dto::CreateAccountDto;
use crate::domain::account::account_repository::AccountRepository;

pub struct AccountUseCase {
    account_repository: Arc<dyn AccountRepository>,
    http_client: Arc<reqwest::Client>,
    kakao_client_id: String,
    kakao_redirect_dir: String,
}

pub struct AuthEnv {
    pub kakao_client_id: String,
    pub kakao_redirect_dir: String,
}

impl AccountUseCase {
    pub fn new(
        account_repository: Arc<dyn AccountRepository>,
        http_client: Arc<reqwest::Client>,
        auth_env: AuthEnv,
    ) -> AccountUseCase {
        AccountUseCase {
            account_repository,
            http_client,
            kakao_client_id: auth_env.kakao_client_id,
            kakao_redirect_dir: auth_env.kakao_redirect_dir,
        }
    }
}

#[derive(Error, Debug)]
pub enum CreateAccountError {
    #[error("Provider error: {0}")]
    ProviderError(#[source] Error),
}

impl AccountUseCase {
    pub async fn create_account(
        &self,
        request: CreateAccountDto,
    ) -> Result<(), CreateAccountError> {
        let response = self
            .http_client
            .post("https://kauth.kakao.com/oauth/token")
            .form(&[
                ("grant_type", "authorization_code"),
                ("client_id", self.kakao_client_id.as_str()),
                ("redirect_uri", self.kakao_redirect_dir.as_str()),
                ("code", &request.code),
            ])
            .send()
            .await
            .map_err(CreateAccountError::ProviderError)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::account::account_repository::MockAccountRepository;
    use crate::presentation::account::account_resolver::account_api::AuthProvider;

    #[tokio::test]
    async fn test_create_account() {
        // given
        let auth_env = AuthEnv {
            kakao_client_id: "kakao_client_id".to_string(),
            kakao_redirect_dir: "kakao_redirect_dir".to_string(),
        };
        let account_use_case = AccountUseCase::new(
            Arc::new(MockAccountRepository::new()),
            Arc::new(reqwest::Client::new()),
            auth_env,
        );

        // when
        let result = account_use_case
            .create_account(CreateAccountDto::new(
                "code".to_string(),
                AuthProvider::Kakao,
            ))
            .await
            .unwrap();

        // then
    }
}
