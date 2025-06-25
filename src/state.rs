use crate::steam::OwnedGames;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

// Cache shared across requests
#[derive(Debug, Clone)]
pub struct State<T> {
    pub cache: Arc<Mutex<HashMap<String, T>>>,
}

impl<T> State<T> {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub type GamesState = State<OwnedGames>;
pub type WinsState = State<u32>;
