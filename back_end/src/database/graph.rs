use std::error::Error;

use async_recursion::async_recursion;
use poem_openapi::{payload::Json, Object};
use sqlx::types::Uuid;

use super::Proposition;

#[derive(Clone, Object)]
pub struct Graph {
    node: Proposition,
    premises: Vec<Box<Graph>>,
    conclusions: Vec<Box<Graph>>
}


#[async_recursion]
async fn generate_graph(mut graph: Graph, depth: u8) -> Result<Graph, Box<dyn Error>>{
    let premises = super::get_premises(graph.node.id).await?;
    let conclusions = super::get_conclusions(graph.node.id).await?;

    println!("Depth: {}, premises: {:?}", depth, premises);
    
    if depth > 0 {
        for premise in premises {
            graph.premises.push(
                Box::new(generate_graph(Graph { node: premise, premises: vec![], conclusions: vec![] }, depth-1).await?)
            );
        }
        for conclusion in conclusions {
            graph.conclusions.push(
                Box::new(generate_graph(Graph { node: conclusion, premises: vec![], conclusions: vec![] }, depth-1).await?)
            );
        }
    }

    Ok(graph)
}

pub async fn get_graph(center_node_id: Option<Uuid>, depth: u8) -> Result<Json<Graph>, Box<dyn Error>> {
    println!("||||||||||||||||||||||");
    let center_node: Proposition;
    let graph: Graph;
    if center_node_id.is_some() {
        center_node = super::get_proposition(center_node_id.unwrap()).await?;
    }
    else {
        center_node = super::get_random_proposition().await?;
    }
    graph = Graph {
        node: center_node,
        premises: vec![],
        conclusions: vec![],
    };

    Ok(Json(generate_graph(graph, depth).await?))

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

pub async fn get_proposition_search_result(search_query: String) -> Result<Vec<Proposition>, Box<dyn Error>> {
    Ok(super::get_proposition_search_result(search_query).await?)
}

pub async fn add_relation(premise_id: Uuid, conclusion_id: Uuid) -> Result<(), Box<dyn Error>> {
    super:: add_relation(premise_id, conclusion_id).await?;
    Ok(())
}