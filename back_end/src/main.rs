mod api;
mod database;

use poem::{
    listener::TcpListener,
    endpoint::StaticFilesEndpoint,
    Route,
    session::{CookieConfig, ServerSession, MemoryStorage},
    EndpointExt,
    web::cookie::SameSite
};
use poem_openapi::OpenApiService;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    dotenv::dotenv().expect("Unable to load environment variables from .env file");
    
    let api_service =
        OpenApiService::new(api::Api, "Hello World", "1.0").server("/api");
    macro_rules! front_end {
        () => {
            StaticFilesEndpoint::new("../front_end/dist/")
                .index_file("index.html")
        };
    } 
    let app = Route::new()
        .nest("/", front_end!())
        .nest("/login", front_end!())
        .nest("/new-proposition", front_end!())
        .nest("/proposition", front_end!())
        .nest("/api", api_service)
        .with(ServerSession::new(
            CookieConfig::new().same_site(SameSite::Strict),
            MemoryStorage::new()
        ));


    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await

    
}

