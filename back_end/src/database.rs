
pub mod auth;

use sqlx::{postgres::PgPoolOptions, Postgres, Pool};

#[derive(Clone)]
struct User {
    email: String,
    username: String,
    password_hash: String,
    session_id_hash: Option<String>
}


async fn database_connection() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
  
    PgPoolOptions::new()
        .max_connections(100)
        .connect(&database_url)
        .await.expect("Unable to connect to Postgres")

}

async fn get_user(username: String) -> User {
    let pool = database_connection().await;
    sqlx::query_as!(
        User,
        "SELECT *
        FROM users
        WHERE username = $1 OR email = $1",
        username
    ).fetch_all(&pool)
    .await.expect("Unable to retrieve hash")[0].clone()
}

async fn add_user(email: String, username: &String, password_hash: String, session_id_hash: String) {
    let pool = database_connection().await;
    sqlx::query_as!(
        User,
        "INSERT INTO users(email, username, password_hash, session_id_hash)
        VALUES ($1, $2, $3, $4)",
        email, username, password_hash, session_id_hash
    )
    .fetch_all(&pool)
    .await.expect("Unable to insert a user");
}

async fn set_session_id_hash (session_id_hash: String, email: String) {
    let pool = database_connection().await;

    sqlx::query_as!(
        User,
        "UPDATE users
        SET session_id_hash = $1
        WHERE email = $2",
        session_id_hash, email
    )
    .fetch_all(&pool)
    .await.expect("Unable to update session_id");
}

async fn remove_session_id_hash (username: String) {
    let pool = database_connection().await;
    
    sqlx::query_as!(
        User,
        "UPDATE users
        SET session_id_hash = NULL
        WHERE username = $1",
        username
    )
    .fetch_all(&pool)
    .await.expect("Unable to insert a user");
}