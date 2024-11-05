use crate::db::open_db_connection;

pub async fn get_user() {
    open_db_connection().await; // NEW
}