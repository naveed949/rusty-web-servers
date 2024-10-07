use std::{collections::HashMap, sync::{Arc, Mutex}};

pub struct Repository {
    data: HashMap<String, Model>,
}
pub type SharedRepository = Arc<Mutex<Repository>>;
#[derive(serde::Deserialize)]
pub struct Model {
    key: String,
    value: String,
}

impl Repository {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key.clone(), Model { key, value });
    }

    pub fn get(&self, key: &str) -> Option<&Model> {
        self.data.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Model> {
        self.data.remove(key)
    }
}

impl Model {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
    
}