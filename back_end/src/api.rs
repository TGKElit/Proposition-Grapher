

use poem::session::Session;
use poem_openapi::{payload::Json,OpenApi, Object, param::Query};
use sqlx::types::Uuid;

use crate::database::{auth, graph, Proposition, Votee};

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
struct PostPropositionResquest {
    lexical_description: String,
}

#[derive(Object)]
struct GetPropositionRequest {
    proposition_id: String
}

#[derive(Object)]
struct VoteRequest {
    votee_id: Uuid,
    vote: bool
}

#[derive(Object)]
struct PostFormalizationRequest {
    formalization_string: String,
    proposition_id: Uuid
}




#[OpenApi]
impl Api {
    //Authentication
    #[oai(path = "/register", method = "post")]
    async fn register(&self, request: Json<RegisterRequest>, session: &Session) -> Json<String> {

        auth::register(request.0.email, &request.0.username, request.0.password, session).await.unwrap();
        session.set("username", &request.0.username);

        Json(format!("hello, {}, session: {:?}!", request.0.username, session.status()))
    }

    #[oai(path = "/login", method = "post")]
    async fn login(&self, request: Json<LoginRequest>, session: &Session) -> Json<String> {
        auth::login(request.0.email, request.0.password, session).await.unwrap()
    }

    #[oai(path = "/logout", method = "post")]
    async fn logout(&self, session: &Session) -> Json<String> {
        auth::logout(session.get("username")).await;
        session.purge();

        Json(format!("logged out, {:?}", session.status()))
    }

    #[oai(path = "/auth-status", method = "get")]
    async fn auth_status(&self, session: &Session) -> Json<bool> {
        Json(auth::is_logged_in(session.get("username"), session.get("session_id")).await.unwrap())
    }

    //TBD
    #[oai(path = "/proposition", method = "post")]
    async fn post_proposition(&self, request: Json<PostPropositionResquest>, session: &Session) -> Json<String> {
        let username = session.get("username").unwrap();
        if auth::is_logged_in(session.get("username"), session.get("session_id")).await.unwrap() {
            let profile_id = graph::get_profile_id(username).await.unwrap();
            let id = graph::post_proposition(profile_id, request.0.lexical_description).await.unwrap();
            Json(id.to_string())
        } else {
            Json("Post failed: Not logged in".to_string())
        }
    }
    #[oai(path = "/proposition", method = "get")]
    async fn get_proposition(&self, id: Query<Option<String>>) -> Json<Proposition> {
        let id = Uuid::parse_str(id.as_ref().unwrap()).unwrap();
        graph::get_proposition(id).await.unwrap()
        //Json("ddd".to_string())
    }

    #[oai(path = "/truth", method = "post")]
    async fn post_truth(&self, request: Json<VoteRequest>, session: &Session) -> Json<String> {
        let username = session.get("username").unwrap();
        if auth::is_logged_in(session.get("username"), session.get("session_id")).await.unwrap() {
            let profile_id = graph::get_profile_id(username).await.unwrap();
            graph::vote(profile_id, request.0.votee_id, request.0.vote, Votee::Proposition).await.unwrap();
            Json("Vote recorded".to_string())
        } else {
            Json("Vote failed: Not logged in".to_string())
        }
    }

    #[oai(path = "/graph", method = "get")]
    async fn get_graph(&self) -> Json<Proposition> {
        let center_node_id: Option<Uuid> = None;
        //let center_node_id = Uuid::parse_str(proposition_id.0.unwrap().as_str()).ok();
        graph::get_graph(center_node_id, 1).await.unwrap()
        //Json("ddd".to_string())
    }

    #[oai(path = "/formalization", method = "post")]
    async fn post_formalization(&self, request: Json<PostFormalizationRequest>, session: &Session) -> Json<String> {
        let username = session.get("username").unwrap();
        if auth::is_logged_in(session.get("username"), session.get("session_id")).await.unwrap() {
            let profile_id = graph::get_profile_id(username).await.unwrap();
            graph::post_formalization(profile_id, request.0.proposition_id, request.0.formalization_string).await.unwrap();
            Json("Formalization successfully published".to_string())
        } else {
            Json("Publishing failed: Not logged in".to_string())
        }
    }
}