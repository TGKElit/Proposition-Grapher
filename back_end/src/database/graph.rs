use std::error::Error;

use async_recursion::async_recursion;
use poem_openapi::{payload::Json, Object};
use sqlx::types::Uuid;

use super::{Proposition, Relation, PropositionalFormalization};

#[derive(Clone, Object, Debug)]
pub struct Nodes {
    node: Proposition,
    premises: Vec<Box<Nodes>>,
    conclusions: Vec<Box<Nodes>>

}

#[derive(Clone, Object, Debug)]
pub struct Graph {
    nodes: Nodes,
    relations: Vec<Relation>
}

#[async_recursion]
async fn generate_graph(mut graph: Graph, depth: u8) -> Result<Graph, Box<dyn Error>>{
    
    if depth > 0 {
        let premises = super::get_premises(graph.nodes.node.id).await?;
        let conclusions = super::get_conclusions(graph.nodes.node.id).await?;
        let relations = super::get_relations(graph.nodes.node.id).await?;

        for relation in relations {
            if !graph.relations.contains(&relation) {
                graph.relations.push(relation);
            }
        }
    
        println!("Depth: {}, premises: {:?}, conclusions: {:?}", depth, premises.len(), conclusions.len());
        

        for premise in premises {
            let sub_graph = generate_graph(Graph { nodes: Nodes { node: premise, premises: vec![], conclusions: vec![] }, relations: graph.relations.clone() }, depth-1).await?;
            graph.nodes.premises.push(
                Box::new(sub_graph.nodes)
            );
            for relation in sub_graph.relations {
                if !graph.relations.contains(&relation) {
                    graph.relations.push(relation);
                }
            }
        }
        for conclusion in conclusions {
            let sub_graph = generate_graph(Graph { nodes: Nodes { node: conclusion, premises: vec![], conclusions: vec![] }, relations: graph.relations.clone() }, depth-1).await?;
            graph.nodes.conclusions.push(
                Box::new(sub_graph.nodes)
            );
            for relation in sub_graph.relations {
                if !graph.relations.contains(&relation) {
                    graph.relations.push(relation);
                }
            }
        }
    }

    Ok(graph)
}

pub async fn get_graph(center_node_id: Option<Uuid>, depth: u8) -> Result<Json<Graph>, Box<dyn Error>> {
    println!("||||||||||||||||||||||");
    let center_node: Proposition;
    let nodes: Nodes;
    let graph: Graph;
    if center_node_id.is_some() {
        center_node = super::get_proposition(center_node_id.unwrap()).await?;
    }
    else {
        center_node = super::get_random_proposition().await?;
    }
    nodes = Nodes {
        node: center_node,
        premises: vec![],
        conclusions: vec![],
    };
    graph = Graph {
        nodes: nodes,
        relations: vec![]
    };
    println!("{:?}", graph.relations);

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
        super::Votee::Relation => Ok(()),
        super::Votee::PropositionalFormalization => super::set_propositional_formalization_truth(profile_id, subject_id, vote).await
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

pub async fn get_propositional_formalizations(proposition_id: Uuid) -> Result<Vec<PropositionalFormalization>, Box<dyn Error>> {
    Ok(super::get_propositional_formalizations(proposition_id).await?)
}