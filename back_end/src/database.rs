
pub mod auth;
pub mod graph;

use poem_openapi::Object;
use sqlx::{postgres::PgPoolOptions, Postgres, Pool, types::Uuid};
use std::error::Error;

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
#[derive(Object)]
pub struct Proposition {
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
async fn database_connection() -> Result<Pool<Postgres>, Box<dyn Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
  
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

// Adders
async fn add_user(email: &String, username: &String, password_hash: String, session_id_hash: String) -> Result<(), Box<dyn Error>>{
    let pool = database_connection().await?;
    sqlx::query_as!(
        User,
        "INSERT INTO users(email, username, password_hash, session_id_hash)
        VALUES ($1, $2, $3, $4)",
        email, username, password_hash, session_id_hash
    )
    .execute(&pool)
    .await?;
    Ok(())
}

async fn add_profile(email: String) -> Result<(), Box<dyn Error>> {
    let pool = database_connection().await?;
    sqlx::query_as!(
        Profile,
        "INSERT INTO profiles(user_email)
        VALUES ($1)",
        email
    )
    .execute(&pool)
    .await?;
    Ok(())
}

async fn add_proposition (lexical_description: String, profile_id: Uuid) -> Result<Uuid, Box<dyn Error>> {
    let pool = database_connection().await?;
    let proposition_id = sqlx::query_as!(
        Proposition,
        "INSERT INTO propositions(lexical_description, profile_id)
        VALUES ($1, $2)
        RETURNING *, CAST(0 AS FLOAT) AS truth_score",
        lexical_description, profile_id
    )
    .fetch_one(&pool)
    .await?.id;
    Ok((proposition_id)) 
}

async fn add_relation (premise_id: Uuid, conclusion_id: Uuid) -> Result<(), Box<dyn Error>> {
    let pool = database_connection().await?;
    sqlx::query_as!(
        Relation,
        "INSERT INTO relations(premise_id, conclusion_id)
        VALUES ($1, $2)",
        premise_id, conclusion_id
    )
    .execute(&pool)
    .await?;
    Ok(())
}

async fn add_propositional_formalization (profile_id: Uuid, proposition_id: Uuid, formalization_string: String) -> Result<(), Box<dyn Error>> {
    let pool = database_connection().await?;
    sqlx::query_as!(
        PropositionalFormalization,
        "INSERT INTO propositional_formalizations(profile_id, proposition_id, formalization_string)
        VALUES ($1, $2, $3)",
        profile_id, proposition_id, formalization_string
    )
    .execute(&pool)
    .await?;
    Ok(())
}
// Getters
async fn get_user(id: String) -> Result<User, Box<dyn Error>> {
    let pool = database_connection().await?;
    let user = sqlx::query_as!(
        User,
        "SELECT *
        FROM users
        WHERE username = $1 OR email = $1",
        id
    ).fetch_one(&pool)
    .await?;
    Ok(user)
}

async fn get_profile(username: String) -> Result<Profile, Box<dyn Error>> {
    let pool = database_connection().await?;
    let profile = sqlx::query_as!(
        Profile,
        "SELECT profiles.*
        FROM profiles
        INNER JOIN users
        ON profiles.user_email = users.email
        WHERE username = $1",
        username
    ).fetch_one(&pool)
    .await?;
    Ok(profile)
}

async fn get_random_proposition() -> Result<Proposition, Box<dyn Error>> {
    let pool = database_connection().await?;
    let proposition = sqlx::query_as!(
        Proposition,
        "SELECT propositions.*, CAST(truth_scores.truth_score AS FLOAT) AS truth_score
        FROM propositions
        LEFT JOIN (
            SELECT proposition_id , COUNT(CASE WHEN is_true = true THEN 1 END) / COUNT(*) AS truth_score
            FROM profiles_propositions
            GROUP BY proposition_id
        ) AS truth_scores
        ON propositions.id = truth_scores.proposition_id
        ORDER BY RANDOM()
        LIMIT 1"
    ).fetch_one(&pool)
    .await?;
    Ok(proposition)
}

async fn get_proposition(id: Uuid) -> Result<Proposition, Box<dyn Error>> {
    let pool = database_connection().await?;
    let proposition = sqlx::query_as!(
        Proposition,
        "SELECT propositions.*, CAST(truth_scores.truth_score AS FLOAT) AS truth_score
        FROM propositions
        LEFT JOIN (
            SELECT proposition_id , COUNT(CASE WHEN is_true = true THEN 1 END) / COUNT(*) AS truth_score
            FROM profiles_propositions
            GROUP BY proposition_id
        ) AS truth_scores
        ON propositions.id = truth_scores.proposition_id
        WHERE id = $1",
        id
    ).fetch_one(&pool)
    .await?;
    Ok(proposition)
}

async fn get_premises(id: Uuid) -> Result<Vec<Proposition>, Box<dyn Error>> {
    let pool = database_connection().await?;
    let premises = sqlx::query_as!(
        Proposition,
        "SELECT propositions.*, CAST(truth_scores.truth_score AS FLOAT) AS truth_score
        FROM propositions
        INNER JOIN relations
        ON relations.premise_id = propositions.id
        LEFT JOIN (
            SELECT proposition_id , COUNT(CASE WHEN is_true = true THEN 1 END) / COUNT(*) AS truth_score
            FROM profiles_propositions
            GROUP BY proposition_id
        ) AS truth_scores
        ON propositions.id = truth_scores.proposition_id
        WHERE relations.conclusion_id = $1",
        id
    ).fetch_all(&pool)
    .await?;
    Ok(premises)
}

async fn get_conclusions(id: Uuid) -> Result<Vec<Proposition>, Box<dyn Error>> {
    let pool = database_connection().await?;
    let conclusions = sqlx::query_as!(
        Proposition,
        "SELECT propositions.*, CAST(truth_scores.truth_score AS FLOAT) AS truth_score
        FROM propositions
        INNER JOIN relations
        ON relations.conclusion_id = propositions.id
        LEFT JOIN (
            SELECT proposition_id , COUNT(CASE WHEN is_true = true THEN 1 END) / COUNT(*) AS truth_score
            FROM profiles_propositions
            GROUP BY proposition_id
        ) AS truth_scores
        ON propositions.id = truth_scores.proposition_id
        WHERE relations.premise_id = $1",
        id
    ).fetch_all(&pool)
    .await?;
    Ok(conclusions)
}

async fn get_relation(id: Uuid) -> Result<Relation, Box<dyn Error>> {
    let pool = database_connection().await?;
    let relation = sqlx::query_as!(
        Relation,
        r#"SELECT id, premise_id, conclusion_id, propositional_validity as "propositional_validity: Validity", CAST(correlation_scores.correlation_score AS FLOAT) AS correlation_score
        FROM relations
        LEFT JOIN (
            SELECT relation_id , COUNT(CASE WHEN is_correlated = true THEN 1 END) / COUNT(*) AS correlation_score
            FROM profiles_relations
            GROUP BY relation_id
        ) AS correlation_scores
        ON relations.id = correlation_scores.relation_id
        WHERE id = $1"#,
        id
    ).fetch_one(&pool)
    .await?;
    Ok(relation)
}

async fn get_relations(proposition_id: Uuid) -> Result<Vec<Relation>, Box<dyn Error>> {
    let pool = database_connection().await?;
    let relations = sqlx::query_as!(
        Relation,
        r#"SELECT id, premise_id, conclusion_id, propositional_validity as "propositional_validity: Validity", CAST(correlation_scores.correlation_score AS FLOAT) AS correlation_score
        FROM relations
        LEFT JOIN (
            SELECT relation_id , COUNT(CASE WHEN is_correlated = true THEN 1 END) / COUNT(*) AS correlation_score
            FROM profiles_relations
            GROUP BY relation_id
        ) AS correlation_scores
        ON relations.id = correlation_scores.relation_id
        WHERE premise_id = $1 OR conclusion_id = $1"#,
        proposition_id
    ).fetch_all(&pool)
    .await?;
    Ok(relations)
}

async fn get_propositional_formalization(id: Uuid) -> Result<PropositionalFormalization, Box<dyn Error>> {
    let pool = database_connection().await?;
    let propositional_formalization = sqlx::query_as!(
        PropositionalFormalization,
        "SELECT propositional_formalizations.*, CAST(correctness_scores.correctness_score AS FLOAT) AS correctness_score
        FROM propositional_formalizations
        LEFT JOIN (
            SELECT propositional_formalization_id , COUNT(CASE WHEN is_correct = true THEN 1 END) / COUNT(*) AS correctness_score
            FROM profiles__propositional_formalizations
            GROUP BY propositional_formalization_id
        ) AS correctness_scores
        ON propositional_formalizations.id = correctness_scores.propositional_formalization_id
        WHERE id = $1",
        id
    ).fetch_one(&pool)
    .await?;
    Ok(propositional_formalization)
}

async fn get_propositional_formalizations(proposition_id: Uuid) -> Result<Vec<PropositionalFormalization>, Box<dyn Error>> {
    let pool = database_connection().await?;
    let propositional_formalizations = sqlx::query_as!(
        PropositionalFormalization,
        "SELECT propositional_formalizations.*, CAST(correctness_scores.correctness_score AS FLOAT) AS correctness_score
        FROM propositional_formalizations
        LEFT JOIN (
            SELECT propositional_formalization_id , COUNT(CASE WHEN is_correct = true THEN 1 END) / COUNT(*) AS correctness_score
            FROM profiles__propositional_formalizations
            GROUP BY propositional_formalization_id
        ) AS correctness_scores
        ON propositional_formalizations.id = correctness_scores.propositional_formalization_id
        WHERE proposition_id = $1",
        proposition_id
    ).fetch_all(&pool)
    .await?;
    Ok(propositional_formalizations)
}

// Setters
async fn set_session_id_hash (session_id_hash: String, email: String) -> Result<(), Box<dyn Error>>{
    let pool = database_connection().await?;

    sqlx::query_as!(
        User,
        "UPDATE users
        SET session_id_hash = $1
        WHERE email = $2",
        session_id_hash, email
    )
    .execute(&pool)
    .await?;
    Ok(())
}

// Removers
async fn remove_session_id_hash (username: String) -> Result<(), Box<dyn Error>> {
    let pool = database_connection().await?;
    
    sqlx::query_as!(
        User,
        "UPDATE users
        SET session_id_hash = NULL
        WHERE username = $1",
        username
    )
    .execute(&pool)
    .await?;
    Ok(())
}