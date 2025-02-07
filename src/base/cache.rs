use std::{any::Any, collections::HashMap, sync::RwLock};

pub struct Cache {
    store: RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            store: RwLock::new(HashMap::new()),
        }
    }

    pub fn set<V>(&self, k: &str, v: V)
    where
        V: Any + Send + Sync + Clone,
    {
        if let Ok(ref mut map) = self.store.write() {
            map.insert(k.to_string(), Box::new(v));
        }
    }

    pub fn get<V>(&self, k: &str) -> Option<V>
    where
        V: Any + Send + Sync + Clone,
    {
        if let Ok(ref map) = self.store.read() {
            if let Some(ref x) = map.get(k) {
                return x.downcast_ref::<V>().cloned();
            }
        }
        None
    }
}
