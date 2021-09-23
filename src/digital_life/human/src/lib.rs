mod assets;

use assets::Assets;
use Ontology::Ontology;
use serde::{Serialize, Deserialize};
use candid::{CandidType, Principal};
use ic_cdk::api::id;
use nft::NFT;

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Tag(Vec<NFT>);

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Honor(Vec<NFT>);

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Personality(Vec<NFT>);

static COMMON_NAME: &str = "@pab#";

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub enum Mood {
    Clear,
    Dizzy,
    Smart,
    Sleepwalking
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Connections {
    pub followers: Vec<(Principal, u64)>,
    pub followings: Vec<(Principal, u64)>,
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Human {
    pub name: String,
    pub ontology: Ontology,
    pub mood: Mood,
    pub tag: Option<Tag>,
    pub personality: Option<Personality>,
    pub honor: Option<Honor>,
    pub assets: Vec<Assets>,
    pub connections: Option<Connections>,
    pub value: f64
}

impl Default for Human {
    fn default() -> Self {
        Self { 
            name: Default::default(), 
            ontology: Default::default(), 
            mood: Mood::Clear,
            tag: None, 
            personality: None, 
            honor: None, 
            assets: vec![],
            connections: None,
            value: 0.0, 
        }
    }
}

impl Human {
    pub fn born(&mut self, lifeno: u64) {
        self.name = COMMON_NAME.to_string() + "#" + lifeno.to_string().as_str();
        self.ontology = Ontology::born(id().to_text());
        self.mood = Mood::Clear;

        self.ontology.set_gene(vec![0,0,1]);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_following(&mut self, f: Principal){
        let conns = self.connections.clone();
        match conns {
            Some(mut c) => {
                c.followings.push((f, 0));
                self.connections = Some(c);
            }
            None => {
                let conns = Connections {
                    followers: vec![],
                    followings: vec![(f, 0)],
                };
                self.connections = Some(conns);
            }
        }
    }

    pub fn add_followers(&mut self, f: Principal){
        let conns = self.connections.clone();
        match conns {
            Some(mut c) => {
                c.followers.push((f, 0));
                self.connections = Some(c);
            }
            None => {
                let conns = Connections {
                    followers: vec![(f, 0)],
                    followings: vec![],
                };
                self.connections = Some(conns);
            }
        }
    }

}
