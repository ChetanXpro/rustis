use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ServerState {
    store: Mutex<HashMap<String, String>>,
}

impl ServerState {
    pub fn new() -> Arc<Self> {
        Arc::new(ServerState {
            store: Mutex::new(HashMap::new()),
        })
    }

    pub async fn get(&self, key: String) -> Option<String> {
        let store = self.store.lock().await;
        store.get(&key).cloned()
    }

    pub async fn set(&self, key: String, value: String) {
        let mut store = self.store.lock().await;
        store.insert(key, value);
    }
}
