use serde::{Serialize, Deserialize};
use candid::CandidType;

mod profile;
mod gene;
mod view;

#[derive(Clone, Debug, Deserialize, Serialize, Default, CandidType)]
pub struct Ontology {
    pub id: String,
    pub gene: Vec<u32>,
}

impl Ontology {
    pub fn born(id: String) -> Ontology {
        Ontology {
            gene: gene::generate_random_dna(id.clone()),
            id
        }
    }

    pub fn set_gene(&mut self, v: Vec<u32>){
        self.gene = v;
    }
}
