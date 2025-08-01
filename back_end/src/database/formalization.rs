use std::collections::{HashSet, HashMap};
use itertools::{repeat_n, Itertools};


use serde::{Deserialize, Serialize};
use serde_json::Result;

use super::Validity;

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
struct Formalization {
    uuid: Option<String>,
    negated: bool,
    formula: Formula,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(untagged)]
enum Formula {
    Atom(String),
    Composite {
        left_operand: Box<Formalization>,
        main_connective: Connective,
        right_operand: Box<Formalization>,
    },
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
enum Connective {
    And,
    Or,
    Implies,
    Iff,
}

fn get_atoms(proposition: Formalization) -> Result<HashSet<(Formula, Option<String>)>> {
    let formula = proposition.formula;
    let uuid = proposition.uuid;
    let mut atoms = HashSet::new();
    match formula {
        Formula::Atom(_) => {
            atoms.insert((formula, uuid));
        }
        Formula::Composite { left_operand, main_connective: _, right_operand } => {
            let left_atoms = get_atoms(*left_operand)?;
            let right_atoms = get_atoms(*right_operand)?;
            atoms.extend(left_atoms);
            atoms.extend(right_atoms);
        }
    }
    Ok(atoms)
}

fn check_truth(proposition: Formalization, truth_assignments: &HashMap<(Formula, Option<String>), bool>) -> Result<bool> {
    let formula = proposition.formula;
    let uuid = proposition.uuid;
    let negation = proposition.negated;
    match formula {
        Formula::Atom(_) => {
            Ok(negation ^ *truth_assignments.get(&(formula, uuid)).unwrap())
        }
        Formula::Composite { left_operand, main_connective, right_operand } => {
            let is_left_true = check_truth(*left_operand, &truth_assignments)?;
            let is_right_true = check_truth(*right_operand, &truth_assignments)?;
            match main_connective {
                Connective::And => {
                    Ok(negation ^ (is_left_true && is_right_true))
                },
                Connective::Or => {
                    Ok(negation ^ (is_left_true || is_right_true))
                },
                Connective::Implies => {
                    Ok(negation ^ (!is_left_true || is_right_true))
                },
                Connective::Iff => {
                    Ok(negation ^ ((is_left_true && is_right_true) || (!is_left_true && !is_right_true)))
                },
            }
            
            
        }
    }
}

fn generate_assignment_matrix(atoms: HashSet<(Formula, Option<String>)>) -> Result<Vec<HashMap<(Formula, Option<String>), bool>>> {
    let mut assignment_matrix: Vec<HashMap<(Formula, Option<String>), bool>> = vec![];
    let atom_length = (&atoms).len();
    let assignment_vectors: Vec<Vec<bool>> = repeat_n([true, false], (&atoms).len()).multi_cartesian_product().collect();//[true, false].into_iter().combinations_with_replacement((&atoms).len()).collect();
    let atoms_vec: Vec<(Formula, Option<String>)> = atoms.into_iter().collect();
    for truth_vec in assignment_vectors {
        let mut truth_map: HashMap<(Formula, Option<String>), bool> = HashMap::new();
        for index in 0..atom_length {
            truth_map.insert(atoms_vec[index].clone(), truth_vec[index]);
        } 
        assignment_matrix.push(truth_map);
    }

    Ok(assignment_matrix)
}

fn check_validity(premise: Formalization, conclusion: Formalization) -> Result<Validity> {
    let mut atoms = get_atoms(premise.clone())?;
    atoms.extend(get_atoms(conclusion.clone())?);
    let assignment_matrix = generate_assignment_matrix(atoms)?;
    let mut possibly_valid = true;
    let mut possibly_antivalid = true;
    
    for truth_assignments in assignment_matrix {
        let premise_truth = check_truth(premise.clone(), &truth_assignments)?;
        let conclusion_truth = check_truth(conclusion.clone(), &truth_assignments)?;
        
        if premise_truth && !conclusion_truth {
            possibly_valid = false;
        } else if premise_truth && conclusion_truth {
            possibly_antivalid = false;
        }
    }
    

    if possibly_valid {
        Ok(Validity::Valid)
    } else if possibly_antivalid {
        Ok(Validity::Antivalid)
    } else {
        Ok(Validity::Invalid)
    }
}




#[cfg(test)]
mod tests {
    use std::collections::{HashSet, HashMap};

    use crate::database::{formalization::{Formalization, check_validity, get_atoms, Formula, check_truth}, Validity};

    #[test]
    fn atom_getter() {
        let prop_str = r#" {
            "uuid":null,
            "negated":false,
            "formula":{
                "left_operand":{
                    "uuid":null,
                    "negated":false,
                    "formula":"P"
                },
                "main_connective":"And",
                "right_operand":{
                    "uuid":null,
                    "negated":false,
                    "formula":"Q"
                }
            }
        }"#;
        let prop: Formalization = serde_json::from_str(prop_str).unwrap();

        assert_eq!(get_atoms(prop).unwrap(), HashSet::from([(Formula::Atom("P".to_string()), None), (Formula::Atom("Q".to_string()), None)]))
    }

    #[test]
    fn truth() {
        let atom_str = r#" {
            "uuid":null,
            "negated":false,
            "formula":"P"
        }"#;
        let atom: Formalization = serde_json::from_str(atom_str).unwrap();
        let neg_atom_str = r#" {
            "uuid":null,
            "negated":true,
            "formula":"P"
        }"#;
        let neg_atom: Formalization = serde_json::from_str(neg_atom_str).unwrap();
        let prop_str = r#" {
            "uuid":null,
            "negated":false,
            "formula":{
                "left_operand":{
                    "uuid":null,
                    "negated":false,
                    "formula":"P"
                },
                "main_connective":"And",
                "right_operand":{
                    "uuid":null,
                    "negated":false,
                    "formula":"Q"
                }
            }
        }"#;
        let prop: Formalization = serde_json::from_str(prop_str).unwrap();
        let p = (Formula::Atom("P".to_string()), None);
        let q = (Formula::Atom("Q".to_string()), None);

        assert_eq!(check_truth(atom.clone(), &HashMap::from([(p.clone(), true),(q.clone(), true)])).unwrap(), true);
        assert_eq!(check_truth(atom.clone(), &HashMap::from([(p.clone(), true),(q.clone(), false)])).unwrap(), true);
        assert_eq!(check_truth(atom.clone(), &HashMap::from([(p.clone(), false),(q.clone(), true)])).unwrap(), false);
        assert_eq!(check_truth(atom.clone(), &HashMap::from([(p.clone(), false),(q.clone(), false)])).unwrap(), false);

        assert_eq!(check_truth(neg_atom.clone(), &HashMap::from([(p.clone(), true),(q.clone(), true)])).unwrap(), false);
        assert_eq!(check_truth(neg_atom.clone(), &HashMap::from([(p.clone(), true),(q.clone(), false)])).unwrap(), false);
        assert_eq!(check_truth(neg_atom.clone(), &HashMap::from([(p.clone(), false),(q.clone(), true)])).unwrap(), true);
        assert_eq!(check_truth(neg_atom.clone(), &HashMap::from([(p.clone(), false),(q.clone(), false)])).unwrap(), true);

        assert_eq!(check_truth(prop.clone(), &HashMap::from([(p.clone(), true),(q.clone(), true)])).unwrap(), true);
        assert_eq!(check_truth(prop.clone(), &HashMap::from([(p.clone(), true),(q.clone(), false)])).unwrap(), false);
        assert_eq!(check_truth(prop.clone(), &HashMap::from([(p.clone(), false),(q.clone(), true)])).unwrap(), false);
        assert_eq!(check_truth(prop.clone(), &HashMap::from([(p.clone(), false),(q.clone(), false)])).unwrap(), false);
    }

    #[test]
    fn validity() {
        let premise_str = r#" 
            "uuid":null,
            "negated":false,
            "formula":{
                "left_operand":{
                    "uuid":"baaa93af-02a1-4528-aa77-bd95df4ae22c",
                    "negated":false,
                    "formula":"P"
                },
                "main_connective":"And",
                "right_operand":{
                    "uuid":null,
                    "negated":false,
                    "formula":"Q"
                }
            }
        }"#;
        let conclusion_str = r#"{"uuid":"baaa93af-02a1-4528-aa77-bd95df4ae22c","negated":false,"formula":"P"}"#;
        let negated_conclusion_str =r#"{"uuid":"baaa93af-02a1-4528-aa77-bd95df4ae22c","negated":true,"formula":"P"}"#;

        let premise: Formalization = serde_json::from_str(premise_str).unwrap();
        let conclusion: Formalization = serde_json::from_str(conclusion_str).unwrap();
        let negated_conclusion: Formalization = serde_json::from_str(negated_conclusion_str).unwrap();
        
        assert_eq!(check_validity(premise.clone(), conclusion.clone()).unwrap(), Validity::Valid);
        assert_eq!(check_validity(conclusion.clone(), premise.clone()).unwrap(), Validity::Invalid);
        assert_eq!(check_validity(negated_conclusion.clone(), conclusion.clone()).unwrap(), Validity::Antivalid);


    }

    
}