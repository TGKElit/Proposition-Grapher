use bcrypt::{verify, hash};
use poem::session::Session;
use poem_openapi::payload::Json;
use sha2::{Sha256, Digest};
use rand::{distributions::Alphanumeric, Rng};

fn generate_session_id () -> String {
    let session_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();
    session_id
}

fn generate_session_id_hash(session_id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&session_id);
    format!("{:X}", hasher.finalize())
}

async fn update_session_id (session: &Session) -> String{
    let session_id_option: Option<String> = session.get("session_id");
    let session_id: String;
    if session_id_option.is_some() && !is_logged_in(session.get("username"), session.get("session_id")).await {
        session_id = session_id_option.unwrap();
    } else {
        session_id = generate_session_id();
        session.set("session_id", &session_id);
    }

    generate_session_id_hash(&session_id)
}

pub async fn register (email: String, username: &String, password: String, session: &Session) {
    let password_hash = hash(password, 12).expect("Hashing failed");
    let session_id_hash = update_session_id(session).await;
    super::add_user(email, username, password_hash, session_id_hash).await
}

pub async fn login (email: String, password: String, session: &Session) -> Json<String> {
    let user = super::get_user(email).await;

    if verify(password, &user.password_hash).expect("Verification Error") {
        
        let session_id_hash = update_session_id(session).await;

        super::set_session_id_hash(session_id_hash, user.email).await;

        session.set("username", &user.username);
        
        let username: Option<String> = session.get("username");
        Json(format!("hello, {}! {:?}", username.unwrap(), session.status()))
    } else {
        Json(format!("Wrong password"))
    }
}

pub async fn logout(username: Option<String>) {
    super::remove_session_id_hash(username.unwrap()).await;
}

pub async fn is_logged_in (username: Option<String>, session_id: Option<String>) -> bool {
    if username.is_some() && session_id.is_some() {
        let user = super::get_user(username.unwrap()).await;
        user.session_id_hash.unwrap() == generate_session_id_hash(&session_id.unwrap())
    } else {
        false
    }
}