#[readonly::make]
pub struct CreateAccountDto {
    pub email: String,
}

impl CreateAccountDto {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}
