use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration, Instant};

pub struct ExpiryValue {
    value: String,
    expires_at: Option<Instant>,
}

pub struct ServerState {
    data: Mutex<HashMap<String, ExpiryValue>>,
}

impl ServerState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Mutex::new(HashMap::new()),
        })
    }

    pub async fn set(&self, key: String, value: String, ttl: Option<Duration>) {
        let mut data = self.data.lock().await;
        let expires_at = ttl.map(|duration| Instant::now() + duration);
        let expiry_value = ExpiryValue { value, expires_at };
        data.insert(key, expiry_value);
    }
    pub async fn get(&self, key: String) -> Option<String> {
        let mut data = self.data.lock().await;

        if let Some(expiry_value) = data.get(&key) {
            if let Some(expires_at) = expiry_value.expires_at {
                if Instant::now() >= expires_at {
                    data.remove(&key);
                    return None;
                }
            }
            return Some(expiry_value.value.clone());
        }
        None
    }
}
