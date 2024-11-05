use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::RecordId;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;



#[derive(Debug, Deserialize, Serialize)]
struct User {
    #[allow(dead_code)]
    id: RecordId,
    username: String,
}

pub async fn open_db_connection() {
    let db = match Surreal::new::<Ws>("localhost:8000").await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to connect to SurrealDB: {:?}", e);
            return;
        }
    };

    match db
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
    {
        Ok(jwt) => jwt,
        Err(e) => {
            eprintln!("Failed jwt: {:?}", e);
            return;
        }
    };
    match db.use_ns("sandbox").use_db("sandbox").await {
        Ok(sandbox) => sandbox,
        Err(e) => {
            eprintln!("Failed sandbox: {:?}", e);
            return;
        }
    };
    let users: Vec<User> = match db.select("user").await {
        Ok(users) => users,
        Err(e) => {
            eprintln!("Failed to select user: {:?}", e);
            return;
        }
    };

    println!("Users: {:?}", users);
}