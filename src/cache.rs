use crate::steam::OwnedGames;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, broadcast};

#[derive(Debug, Clone)]
pub struct Cache<T> {
    wrapper: Arc<Mutex<HashMap<String, T>>>,
}

impl<T> Cache<T> {
    pub fn new() -> Self {
        Self {
            wrapper: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, key: String, value: T) {
        let mut map = self.wrapper.lock().await;
        map.insert(key, value);
    }

    pub async fn get(&self, key: &str) -> Option<T>
    where
        T: Clone,
    {
        let map = self.wrapper.lock().await;
        map.get(key).cloned()
    }
}

pub type GamesState = Cache<OwnedGames>;
pub struct WinsState {
    pub cache: Cache<u32>,
    pub sender: broadcast::Sender<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinsMessage {
    pub wins: u32,
}
