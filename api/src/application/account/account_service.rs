use crate::application::account::dto::create_account_dto::CreateAccountDto;
use crate::domain::account::account::Account;
use crate::domain::account::account_repository::AccountRepository;
use std::rc::Rc;

pub struct AccountService {
    account_repository: Rc<dyn AccountRepository>,
}

impl AccountService {
    pub fn new(account_repository: Rc<dyn AccountRepository>) -> AccountService {
        AccountService { account_repository }
    }

    pub fn create_account(&self, request: CreateAccountDto) {
        let id = self.account_repository.next_identity();
        let account = Account::new(id, request.email.to_string());

        let a = &request.email;

        self.account_repository.save(account);
    }
}
