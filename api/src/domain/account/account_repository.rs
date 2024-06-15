use crate::domain::account::account::Account;

pub trait AccountRepository {
    fn save(&self, account: Account);
    fn next_identity(&self) -> String;
}
