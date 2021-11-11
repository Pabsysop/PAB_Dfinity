use candid::{CandidType};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Record(pub Vec<u8>);

#[derive(Clone, Debug, Deserialize, Serialize, CandidType, PartialEq)]
pub enum RecordContentType {
    Intro(String),
}
#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct RecordDetail{
    pub content_type: RecordContentType,
    pub content: Vec<u8>
}
