use crate::domain::account::account::Account;

pub trait AccountRepository: Send + Sync {
    fn save(&self, account: Account);
    fn next_identity(&self) -> String;
}
