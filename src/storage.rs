use crate::collection::Collection;
use crate::models::{CollectionInfo, DistanceMetric};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};

/// In-memory storage with persistence support
#[derive(Clone)]
pub struct VectorStorage {
    collections: Arc<RwLock<HashMap<String, Collection>>>,
    storage_path: Option<String>,
}

impl VectorStorage {
    /// Create a new storage instance
    pub fn new(storage_path: Option<String>) -> Self {
        let mut storage = Self {
            collections: Arc::new(RwLock::new(HashMap::new())),
            storage_path: storage_path.clone(),
        };

        // Try to load from disk if path is provided
        if let Some(path) = storage_path {
            if let Err(e) = storage.load_from_disk(&path) {
                log::warn!("Failed to load from disk: {}", e);
            }
        }

        storage
    }

    /// Create a new collection
    pub fn create_collection(
        &self,
        name: String,
        distance_metric: DistanceMetric,
    ) -> Result<(), String> {
        let mut collections = self.collections.write().unwrap();
        
        if collections.contains_key(&name) {
            return Err(format!("Collection '{}' already exists", name));
        }

        collections.insert(name.clone(), Collection::new(name, distance_metric));
        drop(collections);

        // Persist to disk
        self.persist_to_disk()?;
        
        Ok(())
    }

    /// Get a collection by name
    pub fn get_collection(&self, name: &str) -> Option<Collection> {
        let collections = self.collections.read().unwrap();
        collections.get(name).cloned()
    }

    /// Delete a collection
    pub fn delete_collection(&self, name: &str) -> Result<bool, String> {
        let mut collections = self.collections.write().unwrap();
        let existed = collections.remove(name).is_some();
        drop(collections);

        if existed {
            self.persist_to_disk()?;
        }

        Ok(existed)
    }

    /// List all collections
    pub fn list_collections(&self) -> Vec<CollectionInfo> {
        let collections = self.collections.read().unwrap();
        collections
            .values()
            .map(|c| CollectionInfo {
                name: c.name.clone(),
                vectors_count: c.count(),
                dimension: c.dimension(),
            })
            .collect()
    }

    /// Update a collection
    pub fn update_collection<F>(&self, name: &str, update_fn: F) -> Result<(), String>
    where
        F: FnOnce(&mut Collection) -> Result<(), String>,
    {
        let mut collections = self.collections.write().unwrap();
        
        let collection = collections
            .get_mut(name)
            .ok_or_else(|| format!("Collection '{}' not found", name))?;
        
        update_fn(collection)?;
        drop(collections);

        // Persist to disk
        self.persist_to_disk()?;
        
        Ok(())
    }

    /// Persist all collections to disk
    fn persist_to_disk(&self) -> Result<(), String> {
        if let Some(ref path) = self.storage_path {
            let collections = self.collections.read().unwrap();
            let json = serde_json::to_string_pretty(&*collections)
                .map_err(|e| format!("Failed to serialize: {}", e))?;
            
            fs::write(path, json)
                .map_err(|e| format!("Failed to write to disk: {}", e))?;
        }
        Ok(())
    }

    /// Load collections from disk
    fn load_from_disk(&mut self, path: &str) -> Result<(), String> {
        if !Path::new(path).exists() {
            return Ok(());
        }

        let json = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read from disk: {}", e))?;
        
        let collections: HashMap<String, Collection> = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to deserialize: {}", e))?;
        
        *self.collections.write().unwrap() = collections;
        
        Ok(())
    }

    /// Get storage stats
    pub fn get_stats(&self) -> StorageStats {
        let collections = self.collections.read().unwrap();
        let total_vectors = collections.values().map(|c| c.count()).sum();
        let collection_count = collections.len();

        StorageStats {
            total_collections: collection_count,
            total_vectors,
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct StorageStats {
    pub total_collections: usize,
    pub total_vectors: usize,
}

impl Default for VectorStorage {
    fn default() -> Self {
        Self::new(None)
    }
}

