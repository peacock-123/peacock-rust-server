use std::sync::Arc;

use crate::application::account::dto::create_account_dto::CreateAccountDto;
use crate::domain::account::account::Account;
use crate::domain::account::account_repository::AccountRepository;

pub trait AccountUseCase: Send + Sync {
    fn create_account(&self, request: CreateAccountDto);
}

pub struct AccountUseCaseImpl {
    account_repository: Arc<dyn AccountRepository>,
}

impl AccountUseCaseImpl {
    pub fn new(account_repository: Arc<dyn AccountRepository>) -> AccountUseCaseImpl {
        AccountUseCaseImpl { account_repository }
    }
}

impl AccountUseCase for AccountUseCaseImpl {
    fn create_account(&self, request: CreateAccountDto) {
        let id = self.account_repository.next_identity();
        let account = Account::new(id, request.email.to_string());

        let a = &request.email;

        self.account_repository.save(account);
    }
}
