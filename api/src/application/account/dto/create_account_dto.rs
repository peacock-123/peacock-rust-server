use crate::presentation::account::account_resolver::account_api::AuthProvider;

#[readonly::make]
pub struct CreateAccountDto {
    pub code: String,
    pub provider: AuthProvider,
}

impl CreateAccountDto {
    pub fn new(code: String, provider: AuthProvider) -> Self {
        Self { code, provider }
    }
}
