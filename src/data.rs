use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub projects: BTreeMap<usize, Project>,
    pub events: BTreeMap<usize, Event>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub source: Source,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Source {
    GitHub { link: String },
}

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    pub date: String,
    pub description: String,
}
