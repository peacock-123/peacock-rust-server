use std::sync::Arc;

use reqwest::Error;
use serde::Deserialize;
use thiserror::Error;

use crate::application::account::dto::create_account_dto::CreateAccountDto;
use crate::domain::account::account::Account;
use crate::domain::account::account_repository::AccountRepository;

pub struct AccountUseCase {
    account_repository: Arc<dyn AccountRepository>,
    http_client: Arc<reqwest::Client>,
    kakao_client_id: String,
    kakao_redirect_dir: String,
    kakao_client_secret: String,
}

pub struct AuthEnv {
    pub kakao_client_id: String,
    pub kakao_redirect_dir: String,
    pub kakao_client_secret: String,
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
            kakao_client_secret: auth_env.kakao_client_secret,
        }
    }
}

#[derive(Error, Debug)]
pub enum CreateAccountError {
    #[error("Provider error: {0}")]
    ProviderError(#[source] Error),
}

#[derive(Deserialize)]
pub struct KakakoTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
pub struct KakakoProfileResponse {
    kakao_account: KakaoAccount,
}

#[derive(Deserialize)]
pub struct KakaoAccount {
    email: String,
    is_email_verified: bool,
}

impl AccountUseCase {
    pub async fn create_account(
        &self,
        request: CreateAccountDto,
    ) -> Result<(), CreateAccountError> {
        let response: KakakoTokenResponse = self
            .http_client
            .post("https://kauth.kakao.com/oauth/token")
            .form(&[
                ("grant_type", "authorization_code"),
                ("client_id", self.kakao_client_id.as_str()),
                ("redirect_uri", self.kakao_redirect_dir.as_str()),
                ("code", &request.code),
                ("client_secret", self.kakao_client_secret.as_str()),
            ])
            .send()
            .await
            .map_err(CreateAccountError::ProviderError)?
            .json()
            .await
            .map_err(CreateAccountError::ProviderError)?;

        let profile: KakakoProfileResponse = self
            .http_client
            .post("https://kapi.kakao.com/v2/user/me")
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", response.access_token),
            )
            .send()
            .await
            .map_err(CreateAccountError::ProviderError)?
            .json()
            .await
            .map_err(CreateAccountError::ProviderError)?;

        Account::new(, profile.kakao_account.email);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::account::account_repository::MockAccountRepository;
    use crate::presentation::account::account_resolver::account_api::AuthProvider;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_create_account() {
        // given
        let auth_env = AuthEnv {
            kakao_client_id: "".to_string(),
            kakao_redirect_dir: "".to_string(),
            kakao_client_secret: "".to_string(),
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
            .await;

        // then
        assert!(result.is_ok());
    }
}
