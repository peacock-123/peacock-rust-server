use crate::domain::account::account::Account;
use crate::domain::account::account_repository::AccountRepository;
use crate::infrastructure::repository::connection::Connection;

pub struct AccountOrmRepository {
    connection: Connection,
}

impl AccountOrmRepository {
    pub fn new(connection: Connection) -> AccountOrmRepository {
        AccountOrmRepository { connection }
    }
}

impl AccountRepository for AccountOrmRepository {
    fn save(&self, account: Account) {
        unimplemented!()
    }

    fn next_identity(&self) -> String {
        unimplemented!()
    }
}
