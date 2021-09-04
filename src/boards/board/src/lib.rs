use ic_cdk::export::Principal;
use url::Url;
use std::str::FromStr;
use std::collections::HashMap;
use consensus::Consensus;
use ic_cdk::storage;
use std::ops::Index;
use room::Room;
use event::Event;
use workshop::Workshop;
use visa::Visa;

const BASE_FEE: u32 = 1;
static COMMON_TITLE: &str = "Anonymous board";
static COMMON_VIEW: &str = "https://partyboard.org/media/blog/blog_2.jpg";
static DEPOSIT: i32 = 0;

type LifeCanisterId = Principal;

struct Committee {
    pub chairman: Vec<LifeCanisterId>,
    pub member: Vec<LifeCanisterId>
}
type Point = ();
type Population = Vec<LifeCanisterId>;

pub struct Board {
    pub id: String,
    pub title: String,
    pub cover: String,
    pub map: Vec<Point>,
    pub rooms: Vec<Room>,
    pub workshop: Vec<Workshop>,
    pub events: Vec<Event>,
    consensuses: Consensus,
    visas: Vec<Visa>,
    committee: Committee,
    population: Population,
    invites: Vec<(String,String)>
}

impl Board {

    pub fn default(ident: String) -> Board{
        Board::build(String::from(COMMON_TITLE), String::from(COMMON_VIEW), ident)
    }

    pub fn build(title: String, cover: String, id: String) -> Board {

        let board = Board {
            id,
            title,
            cover,
            map: vec![],
            rooms: Default::default(),
            workshop: vec![],
            events: Default::default(),
            consensuses: Consensus::POW,
            visas: Default::default(),
            committee: Committee {chairman: vec![], member: vec![]},
            population: vec![],
            invites: vec![]
        };

        board
    }

    pub fn set_consensus(&mut self, con: Consensus){
        self.consensuses = con;
    }
    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }
    pub fn add_workshop(&mut self, w: Workshop) {
        self.workshop.push(w);
    }
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }
    pub fn add_chairman(&mut self, man: Principal) {
        self.committee.chairman.push(man);
    }
    pub fn add_committee_members(&mut self, man: LifeCanisterId) {
        self.committee.member.push(man);
    }
    pub fn add_members(&mut self, man: LifeCanisterId) {
        self.population.push(man);
    }
    pub fn in_committee_chairman(self, person: &String) -> bool {
        let p = &Principal::from_text(person);
        match p {
            Err(e) => false,
            Ok(pstr) => self.committee.chairman.contains(pstr)
        }

    }
    pub fn in_committee(&self, person: &String) -> bool {
        let p = &Principal::from_text(person);
        match p {
            Err(e) => false,
            Ok(pstr) => self.committee.member.contains(pstr)
                || self.committee.chairman.contains(pstr)
        }

    }
    pub fn in_population(&self, person: &String) -> bool {
        let p = &Principal::from_text(person);
        match p {
            Err(e) => false,
            Ok(pstr) => self.population.contains(pstr)
        }
    }
    pub fn in_invite_list(&self, person: &String, code: &String) -> bool {
        self.invites.iter().any(|e| e.0 == person.to_string() && e.1 == code.to_string())
    }
    pub fn invited(&mut self,  person: &String, code: &String) {
        let mut find: i32 = -1;
        for i in 0..self.invites.len() {
            let pair = self.invites.get(i);
            match pair {
                Some(p) => {
                    if p.0 == person.to_string() && p.1 == code.to_string() {
                        find = i as i32;
                        break
                    }
                }
                _ => {}
            }
        }
        if find >= 0 {
            self.invites.remove(find as usize);
        }
    }
    pub fn add_invites(&mut self,  pair: (String, String)) {
        self.invites.push(pair);
    }
    pub fn increase_population(&mut self, person: String){
        let p = Principal::from_text(person);
        match p {
            Err(e) => { },
            Ok(pstr) => self.population.push(pstr)
        }
    }
}
