use poem::session::Session;
use poem_openapi::{payload::Json,OpenApi, Object};

use crate::database::auth;

pub struct Api;

#[derive(Object)]
struct RegisterRequest {
    email: String,
    username: String,
    password: String
}

#[derive(Object)]
struct LoginRequest {
    email: String,
    password: String
}
#[derive(Object)]
struct LogoutRequest {
    test: String
}


#[OpenApi]
impl Api {
    #[oai(path = "/register", method = "post")]
    async fn register(&self, request: Json<RegisterRequest>, session: &Session) -> Json<String> {

        auth::register(request.0.email, &request.0.username, request.0.password, session).await;
        session.set("username", &request.0.username);

        Json(format!("hello, {}, session: {:?}!", request.0.username, session.status()))
    }

    #[oai(path = "/login", method = "post")]
    async fn login(&self, request: Json<LoginRequest>, session: &Session) -> Json<String> {
        auth::login(request.0.email, request.0.password, session).await
    }

    #[oai(path = "/logout", method = "post")]
    async fn logout(&self, session: &Session) -> Json<String> {
        auth::logout(session.get("username")).await;
        session.purge();

        Json(format!("logged out, {:?}", session.status()))
    }

    #[oai(path = "/auth-status", method = "post")]
    async fn auth_status(&self, session: &Session) -> Json<bool> {
        Json(auth::is_logged_in(session.get("username"), session.get("session_id")).await)
    }
}