
pub mod auth;
pub mod graph;

use sqlx::{postgres::PgPoolOptions, Postgres, Pool, types::Uuid};

#[derive(sqlx::Type)]
#[sqlx(type_name = "validity", rename_all = "lowercase")]
enum Validity {
    Valid,
    Invalid,
    Antivalid
}


struct User {
    email: String,
    username: String,
    password_hash: String,
    session_id_hash: Option<String>
}


struct Profile {
    id: Uuid,
    user_email: String,
    propositional_access: bool
}

struct Proposition {
    id: Uuid,
    profile_id: Option<Uuid>,
    lexical_description: String,
    truth_score: Option<f64>
}

struct Relation {
    id: Uuid,
    premise_id: Uuid,
    conclusion_id: Uuid,
    propositional_validity: Option<Validity>,
    correlation_score: Option<f64>
}

struct PropositionalFormalization {
    id: Uuid,
    profile_id: Option<Uuid>,
    proposition_id: Uuid,
    formalization_string: String,
    correctness_score: Option<f64>
}



// General database
async fn database_connection() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
  
    PgPoolOptions::new()
        .max_connections(100)
        .connect(&database_url)
        .await.expect("Unable to connect to Postgres")

}

// Adders
async fn add_user(email: &String, username: &String, password_hash: String, session_id_hash: String) {
    let pool = database_connection().await;
    sqlx::query_as!(
        User,
        "INSERT INTO users(email, username, password_hash, session_id_hash)
        VALUES ($1, $2, $3, $4)",
        email, username, password_hash, session_id_hash
    )
    .execute(&pool)
    .await.expect("Unable to insert a user");
}

async fn add_profile(email: String) {
    let pool = database_connection().await;
    sqlx::query_as!(
        Profile,
        "INSERT INTO profiles(user_email)
        VALUES ($1)",
        email
    )
    .execute(&pool)
    .await.expect("Unable to insert a profile");
}

async fn add_proposition (lexical_description: String, profile_id: Uuid) {
    let pool = database_connection().await;
    sqlx::query_as!(
        Proposition,
        "INSERT INTO propositions(lexical_description, profile_id)
        VALUES ($1, $2)",
        lexical_description, profile_id
    )
    .execute(&pool)
    .await.expect("Unable to inser proposition");
}

async fn add_relation (premise_id: Uuid, conclusion_id: Uuid) {
    let pool = database_connection().await;
    sqlx::query_as!(
        Relation,
        "INSERT INTO relations(premise_id, conclusion_id)
        VALUES ($1, $2)",
        premise_id, conclusion_id
    )
    .execute(&pool)
    .await.expect("Unable to insert relation");
}

async fn add_propositional_formalization (profile_id: Uuid, proposition_id: Uuid, formalization_string: String) {
    let pool = database_connection().await;
    sqlx::query_as!(
        PropositionalFormalization,
        "INSERT INTO propositional_formalizations(profile_id, proposition_id, formalization_string)
        VALUES ($1, $2, $3)",
        profile_id, proposition_id, formalization_string
    )
    .execute(&pool)
    .await.expect("Unable to insert relation");
}
// Getters
async fn get_user(id: String) -> User {
    let pool = database_connection().await;
    sqlx::query_as!(
        User,
        "SELECT *
        FROM users
        WHERE username = $1 OR email = $1",
        id
    ).fetch_one(&pool)
    .await.expect("Unable to retrieve user")
}

async fn get_profile(username: String) -> Profile {
    let pool = database_connection().await;
    sqlx::query_as!(
        Profile,
        "SELECT profiles.*
        FROM profiles
        INNER JOIN users
        ON profiles.user_email = users.email
        WHERE username = $1",
        username
    ).fetch_one(&pool)
    .await.expect("Unable to retrieve user")
}

async fn get_proposition(id: Uuid) -> Proposition {
    let pool = database_connection().await;
    sqlx::query_as!(
        Proposition,
        "SELECT propositions.*, CAST(truth_scores.truth_score AS FLOAT) AS truth_score
        FROM propositions
        LEFT JOIN (SELECT proposition_id , COUNT(CASE WHEN is_true = true THEN 1 END) / COUNT(*) AS truth_score
        FROM profiles_propositions
        GROUP BY proposition_id) AS truth_scores
        ON propositions.id = truth_scores.proposition_id
        WHERE id = $1",
        id
    ).fetch_one(&pool)
    .await.expect("Unable to retrive proposition")
}

async fn get_premises(id: Uuid) -> Vec<Proposition> {
    let pool = database_connection().await;
    sqlx::query_as!(
        Proposition,
        "SELECT propositions.*, CAST(truth_scores.truth_score AS FLOAT) AS truth_score
        FROM propositions
        INNER JOIN relations
        ON relations.premise_id = propositions.id
        LEFT JOIN (SELECT proposition_id , COUNT(CASE WHEN is_true = true THEN 1 END) / COUNT(*) AS truth_score
        FROM profiles_propositions
        GROUP BY proposition_id) AS truth_scores
        ON propositions.id = truth_scores.proposition_id
        WHERE relations.conclusion_id = $1",
        id
    ).fetch_all(&pool)
    .await.expect("Unable to retrive premises")
}

async fn get_conclusions(id: Uuid) -> Vec<Proposition> {
    let pool = database_connection().await;
    sqlx::query_as!(
        Proposition,
        "SELECT propositions.*, CAST(truth_scores.truth_score AS FLOAT) AS truth_score
        FROM propositions
        INNER JOIN relations
        ON relations.conclusion_id = propositions.id
        LEFT JOIN (SELECT proposition_id , COUNT(CASE WHEN is_true = true THEN 1 END) / COUNT(*) AS truth_score
        FROM profiles_propositions
        GROUP BY proposition_id) AS truth_scores
        ON propositions.id = truth_scores.proposition_id
        WHERE relations.premise_id = $1",
        id
    ).fetch_all(&pool)
    .await.expect("Unable to retrive conclusions")
}

async fn get_relation(id: Uuid) -> Relation {
    let pool = database_connection().await;
    sqlx::query_as!(
        Relation,
        r#"SELECT id, premise_id, conclusion_id, propositional_validity as "propositional_validity: Validity", CAST(correlation_scores.correlation_score AS FLOAT) AS correlation_score
        FROM relations
        LEFT JOIN (SELECT relation_id , COUNT(CASE WHEN is_correlated = true THEN 1 END) / COUNT(*) AS correlation_score
        FROM profiles_relations
        GROUP BY relation_id) AS correlation_scores
        ON relations.id = correlation_scores.relation_id
        WHERE id = $1"#,
        id
    ).fetch_one(&pool)
    .await.expect("Unable to retrive relation")
}

async fn get_relations(proposition_id: Uuid) -> Vec<Relation> {
    let pool = database_connection().await;
    sqlx::query_as!(
        Relation,
        r#"SELECT id, premise_id, conclusion_id, propositional_validity as "propositional_validity: Validity", CAST(correlation_scores.correlation_score AS FLOAT) AS correlation_score
        FROM relations
        LEFT JOIN (SELECT relation_id , COUNT(CASE WHEN is_correlated = true THEN 1 END) / COUNT(*) AS correlation_score
        FROM profiles_relations
        GROUP BY relation_id) AS correlation_scores
        ON relations.id = correlation_scores.relation_id
        WHERE premise_id = $1 OR conclusion_id = $1"#,
        proposition_id
    ).fetch_all(&pool)
    .await.expect("Unable to retrieve relations")
}

async fn get_propositional_formalization(id: Uuid) -> PropositionalFormalization {
    let pool = database_connection().await;
    sqlx::query_as!(
        PropositionalFormalization,
        "SELECT propositional_formalizations.*, CAST(correctness_scores.correctness_score AS FLOAT) AS correctness_score
        FROM propositional_formalizations
        LEFT JOIN (SELECT propositional_formalization_id , COUNT(CASE WHEN is_correct = true THEN 1 END) / COUNT(*) AS correctness_score
        FROM profiles__propositional_formalizations
        GROUP BY propositional_formalization_id) AS correctness_scores
        ON propositional_formalizations.id = correctness_scores.propositional_formalization_id
        WHERE id = $1",
        id
    ).fetch_one(&pool)
    .await.expect("Unable to retrieve formalization")
}

async fn get_propositional_formalizations(proposition_id: Uuid) -> Vec<PropositionalFormalization> {
    let pool = database_connection().await;
    sqlx::query_as!(
        PropositionalFormalization,
        "SELECT propositional_formalizations.*, CAST(correctness_scores.correctness_score AS FLOAT) AS correctness_score
        FROM propositional_formalizations
        LEFT JOIN (SELECT propositional_formalization_id , COUNT(CASE WHEN is_correct = true THEN 1 END) / COUNT(*) AS correctness_score
        FROM profiles__propositional_formalizations
        GROUP BY propositional_formalization_id) AS correctness_scores
        ON propositional_formalizations.id = correctness_scores.propositional_formalization_id
        WHERE proposition_id = $1",
        proposition_id
    ).fetch_all(&pool)
    .await.expect("Unable to retrieve formalization")
}

// Setters
async fn set_session_id_hash (session_id_hash: String, email: String) {
    let pool = database_connection().await;

    sqlx::query_as!(
        User,
        "UPDATE users
        SET session_id_hash = $1
        WHERE email = $2",
        session_id_hash, email
    )
    .execute(&pool)
    .await.expect("Unable to update session_id");
}

// Removers
async fn remove_session_id_hash (username: String) {
    let pool = database_connection().await;
    
    sqlx::query_as!(
        User,
        "UPDATE users
        SET session_id_hash = NULL
        WHERE username = $1",
        username
    )
    .execute(&pool)
    .await.expect("Unable to insert a user");
}