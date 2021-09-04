use candid::{CandidType, Principal};
use ic_cdk::api::id;
use serde::{Serialize, Deserialize};

mod profile;
mod gene;
mod view;

static COMMON_NAME: &str = "@pab#";

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Ontology {
    pub name: String,
    pub id: Principal,
    pub gene: Vec<u32>,
}

impl Default for Ontology {
    fn default() -> Self {
        Self { 
            name: Default::default(), 
            id: Principal::anonymous(), 
            gene: Default::default() 
        }
    }
}

impl Ontology {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn born(sn: u64) -> Ontology {
        let name = COMMON_NAME.to_string() + "#" + sn.to_string().as_str();
        let id = id();
        Ontology {
            name,
            gene: gene::generate_random_dna(id.to_text()),
            id
        }
    }
}
