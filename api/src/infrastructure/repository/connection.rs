use entity::sea_orm::{Database, DatabaseConnection};

pub struct Connection {
    db: DatabaseConnection,
}

impl Connection {
    pub async fn new(option: String) -> Self {
        let db = Database::connect(option).await.unwrap();

        Connection { db }
    }
}
