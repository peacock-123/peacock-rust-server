use crate::domain::account::account::Account;
use mockall::*;

#[automock]
pub trait AccountRepository: Send + Sync {
    fn save(&self, account: Account);
    fn next_identity(&self) -> String;
}
