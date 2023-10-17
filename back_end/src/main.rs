use poem::{listener::TcpListener, endpoint::StaticFilesEndpoint, Route};
use poem_openapi::{param::Query, payload::Json, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> Json<String> {
        match name.0 {
            Some(name) => Json(format!("hello, {}!", name)),
            None => Json("hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("/api");
    let app = Route::new().nest(
        "/",
        StaticFilesEndpoint::new("../front_end/dist/")
            .show_files_listing()
            .index_file("index.html"),
    ).nest("/api", api_service);
    
    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await

}