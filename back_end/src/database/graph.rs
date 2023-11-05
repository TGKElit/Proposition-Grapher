use std::error::Error;

use poem::session::Session;
use poem_openapi::payload::Json;
use sqlx::types::Uuid;

use super::Proposition;

pub async fn get_graph(center_node_id: Option<Uuid>, depth: i8) -> Result<Json<Proposition>, Box<dyn Error>> {
    let center_node: Proposition;
    let graph: (Proposition, Vec<(Vec<Proposition>, Vec<Proposition>)>);
    if center_node_id.is_some() {
        center_node = super::get_proposition(center_node_id.unwrap()).await?;
    }
    else {
        center_node = super::get_random_proposition().await?;
    }

    Ok(Json(center_node))

}

pub async fn get_proposition(proposition_id: Uuid) -> Result<Json<Proposition>, Box<dyn Error>> {
    Ok(Json(super::get_proposition(proposition_id).await?))
}

pub async fn post_proposition(profile_id: Uuid, lexical_description: String) -> Result<Uuid, Box<dyn Error>> {
    Ok(super::add_proposition(lexical_description, profile_id).await?)
}

pub async fn get_profile_id(username: String) -> Result<Uuid, Box<dyn Error>> {
    Ok(super::get_profile(username).await?.id)
}

pub async fn vote(profile_id: Uuid, subject_id: Uuid, vote: bool, votee: super::Votee) -> Result<(), Box<dyn Error>> {
    match votee {
        super::Votee::Proposition => super::set_proposition_truth(profile_id, subject_id, vote).await,
        super::Votee::Relation | super::Votee::PropositionalFormalization => Ok(())
    }
}

pub async fn post_formalization(profile_id: Uuid, proposition_id: Uuid, formalization_string: String) -> Result<(), Box<dyn Error>> {
    super::add_propositional_formalization(profile_id, proposition_id, formalization_string).await?;
    Ok(())
}