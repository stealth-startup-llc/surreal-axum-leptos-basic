use axum::Json;
use serde::{Deserialize, Serialize};
use crate::db::open_db_connection;
use surrealdb::{RecordId};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[allow(dead_code)]
    id: RecordId,
    username: String,
}

// Added 'pub' to make ApiResponse public
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub async fn get_user() -> Json<ApiResponse<Vec<User>>> {
    let db = match open_db_connection().await {
        Ok(db) => db,
        Err(e) => {
            let error_msg = format!("Database connection error: {}", e);
            return Json(ApiResponse {
                success: false,
                data: None,
                error: Some(error_msg),
            });
        }
    };

    match db.select("user").await {
        Ok(users) => Json(ApiResponse {
            success: true,
            data: Some(users),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Failed to select users: {:?}", e)),
        }),
    }
}