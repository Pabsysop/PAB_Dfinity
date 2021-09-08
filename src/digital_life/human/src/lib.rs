use assets::Assets;
use ic_types::PrincipalError;
use record::Record;
use Ontology::Ontology;
use std::collections::HashMap;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use candid::{CandidType, Principal};

mod assets;
mod record;

type Tag = ();
type Personality = ();
type Honor = ();
type PhysicalBody = Principal;

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub enum Mood {
    Clear,
    Dizzy,
    Smart,
    Sleepwalking
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Human {
    pub body: PhysicalBody,
    pub ontology: Ontology,
    pub mood: Mood,
    pub tag: HashMap<String, Tag>,
    pub personality: HashMap<String, Personality>,
    pub honor: HashMap<String, Honor>,
    pub assets: HashMap<String, Assets>,
    pub record: HashMap<String, Record>,
}

impl Default for Human {
    fn default() -> Self {
        Self { 
            body: Principal::anonymous(), 
            ontology: Default::default(), 
            mood: Mood::Clear,
            tag: Default::default(), 
            personality: Default::default(), 
            honor: Default::default(), 
            assets: Default::default(), 
            record: Default::default() 
        }
    }
}

impl Human {
    pub fn born(&mut self, ident: String, lifeno: u64) -> Result<(), PrincipalError> {
        let b = Principal::from_str(ident.as_str());
        match b {
            Err(e) => Err(e),
            Ok(id) => {
                self.body = id;
                self.ontology = Ontology::born(lifeno);
                self.mood = Mood::Clear;

                Ok(())
            }
        }
    }
}
