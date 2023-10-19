use poem::{listener::TcpListener, endpoint::StaticFilesEndpoint, Route};
use poem_openapi::{payload::Json, OpenApi, OpenApiService, Object};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use bcrypt::{hash, verify};

struct Api;

#[derive(Object)]
struct RegisterRequest {
    email: String,
    username: String,
    password: String
}
struct User {
    email: String,
    username: String,
    password_hash: String
}

#[OpenApi]
impl Api {
    #[oai(path = "/register", method = "post")]
    async fn register(&self, request: Json<RegisterRequest>) -> Json<String> {
        let pool = database_connection().await;

        let password_hash = hash(request.0.password, 15).expect("Hashing failed");

        sqlx::query_as!(
            User,
            "insert into users(email, username, password_hash) values ($1, $2, $3)",
            request.0.email, request.0.username, password_hash
            )
            .fetch_all(&pool)
            .await.expect("Unable to insert a user");

            Json(format!("hello, {}!", request.0.username))
    }
}

async fn database_connection() -> Pool<Postgres> {
    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
  
    PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres")

}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    dotenv::dotenv().expect("Unable to load environment variables from .env file");
    
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("/api");
    let front_end = StaticFilesEndpoint::new("../front_end/dist/")
        .index_file("index.html");
    let app = Route::new()
        .nest("/", front_end)
        .nest("/api", api_service);


    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}

