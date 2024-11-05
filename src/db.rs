use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Error as SurrealError;

pub async fn open_db_connection() -> Result<Surreal<Client>, SurrealError>  {
    // Connect to the database
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    // Sign in
    db.signin(Root {
        username: "root",
        password: "root",
    })
        .await?;

    // Use namespace and database
    db.use_ns("sandbox").use_db("sandbox").await?;

    // Return the configured database connection
    Ok(db)
}