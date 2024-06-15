use crate::domain::account::account_id::AccountId;

#[derive(Clone, PartialEq, Eq)]
#[readonly::make]
pub struct Account {
    pub id: AccountId,
    pub email: String,
}

impl Account {
    pub fn new(id: String, email: String) -> Self {
        Self {
            id: AccountId { value: id },
            email,
        }
    }
}
