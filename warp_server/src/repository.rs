use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}
#[derive(Serialize, Deserialize)]
struct TodoMap(HashMap<u64, Todo>);
#[derive(Clone)]
pub struct InMemoryDB {
    items: Arc<Mutex<TodoMap>>,
}

impl InMemoryDB {
    pub fn new() -> Self {
        InMemoryDB {
            items: Arc::new(Mutex::new(TodoMap(HashMap::new()))),
        }
    }

    pub fn create_item(&self, title: String, completed: bool) -> Todo {
        let item = Todo {
            id: rand::random::<u64>(),
            title,
            completed,
        };

        self.items.lock().unwrap().0.insert(item.id, item.clone());
        item
    }

    pub fn read_item(&self, id: u64) -> Option<Todo> {
        self.items.lock().unwrap().0.get(&id).cloned()
    }

    pub fn read_all_items(&self) -> Vec<Todo> {
        self.items.lock().unwrap().0.values().cloned().collect()
    }

    pub fn update_item(&self, id: u64, title: String, completed: bool) -> Option<Todo> {
        let mut items = self.items.lock().unwrap();
        if let Some(item) = items.0.get_mut(&id) {
            item.title = title;
            item.completed = completed;
            return Some(item.clone());
        }
        None
    }

    pub fn delete_item(&self, id: u64) -> bool {
        self.items.lock().unwrap().0.remove(&id).is_some()
    }
}
