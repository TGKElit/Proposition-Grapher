

use poem::session::Session;
use poem_openapi::{payload::Json, OpenApi, Object, param::Query};
use sqlx::types::Uuid;

use crate::database::{auth, graph::{self, Graph}, Proposition, Votee, PropositionalFormalization};

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
    proposition_id: Uuid,
    formalization_string: String
}

#[derive(Object)]
struct PostRelationRequest {
    premise_id: Uuid,
    conclusion_id: Uuid
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
        auth::logout(session.get("username")).await.unwrap();
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
    async fn get_proposition(&self, id: Query<String>) -> Json<Proposition> {
        let id = Uuid::parse_str(id.as_ref()).unwrap();
        graph::get_proposition(id).await.unwrap()
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

    #[oai(path = "/correctness", method = "post")]
    async fn post_correctness(&self, request: Json<VoteRequest>, session: &Session) -> Json<String> {
        let username = session.get("username").unwrap();
        if auth::is_logged_in(session.get("username"), session.get("session_id")).await.unwrap() {
            let profile_id = graph::get_profile_id(username).await.unwrap();
            graph::vote(profile_id, request.0.votee_id, request.0.vote, Votee::PropositionalFormalization).await.unwrap();
            Json("Vote recorded".to_string())
        } else {
            Json("Vote failed: Not logged in".to_string())
        }
    }

    #[oai(path = "/graph", method = "get")]
    async fn get_graph(&self, depth: Query<u8>, id: Query<Option<String>>) -> Json<Graph> {
        println!("Graph endpoint called");
        let mut center_node_id: Option<Uuid> = None;
        if id.is_some() {
            center_node_id = Some(Uuid::parse_str(id.as_ref().unwrap()).unwrap());
        }
        graph::get_graph(center_node_id, *depth).await.unwrap()
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

    #[oai(path = "/formalization-list", method = "get")]
    async fn get_formalizations(&self, id:Query<Uuid>) -> Json<Vec<PropositionalFormalization>> {
        Json(graph::get_propositional_formalizations(id.0).await.unwrap())
    }

    #[oai(path = "/relation", method = "post")]
    async fn post_relation(&self, request: Json<PostRelationRequest>, session: &Session) -> Json<String> {
        if auth::is_logged_in(session.get("username"), session.get("session_id")).await.unwrap() {
            graph::add_relation(request.0.premise_id, request.0.conclusion_id).await.unwrap();
            Json("Relation added".to_string())
        } else {
            Json("Publishing failed: Not logged in".to_string())
        } 
    }

    #[oai(path = "/search", method = "get")]
    async fn search(&self, query: Query<String>) -> Json<Vec<Proposition>> {
        Json(graph::get_proposition_search_result(query.0).await.unwrap())
    }
}