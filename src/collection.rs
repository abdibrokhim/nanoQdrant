use crate::models::{DistanceMetric, Point, SearchResult};
use crate::vector::calculate_similarity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A collection stores vectors with the same dimension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub distance_metric: DistanceMetric,
    points: HashMap<Uuid, Point>,
    dimension: Option<usize>,
}

impl Collection {
    /// Create a new collection
    pub fn new(name: String, distance_metric: DistanceMetric) -> Self {
        Self {
            name,
            distance_metric,
            points: HashMap::new(),
            dimension: None,
        }
    }

    /// Insert or update a point
    pub fn upsert_point(&mut self, point: Point) -> Result<Uuid, String> {
        // Check dimension consistency
        if let Some(dim) = self.dimension {
            if point.vector.len() != dim {
                return Err(format!(
                    "Vector dimension mismatch: expected {}, got {}",
                    dim,
                    point.vector.len()
                ));
            }
        } else {
            self.dimension = Some(point.vector.len());
        }

        let id = point.id;
        self.points.insert(id, point);
        Ok(id)
    }

    /// Get a point by ID
    pub fn get_point(&self, id: &Uuid) -> Option<&Point> {
        self.points.get(id)
    }

    /// Delete a point by ID
    pub fn delete_point(&mut self, id: &Uuid) -> bool {
        self.points.remove(id).is_some()
    }

    /// Search for similar vectors
    pub fn search(&self, query_vector: &[f32], limit: usize) -> Vec<SearchResult> {
        // Check dimension
        if let Some(dim) = self.dimension {
            if query_vector.len() != dim {
                return vec![];
            }
        }

        let mut results: Vec<(Uuid, f32, Option<HashMap<String, serde_json::Value>>)> = self
            .points
            .values()
            .map(|point| {
                let score = calculate_similarity(&point.vector, query_vector, self.distance_metric);
                (point.id, score, point.payload.clone())
            })
            .collect();

        // Sort by score descending
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Take top-k and convert to SearchResult
        results
            .into_iter()
            .take(limit)
            .map(|(id, score, payload)| SearchResult { id, score, payload })
            .collect()
    }

    /// Search with payload filter
    pub fn search_with_filter(
        &self,
        query_vector: &[f32],
        limit: usize,
        filter: &HashMap<String, serde_json::Value>,
    ) -> Vec<SearchResult> {
        // Check dimension
        if let Some(dim) = self.dimension {
            if query_vector.len() != dim {
                return vec![];
            }
        }

        let mut results: Vec<(Uuid, f32, Option<HashMap<String, serde_json::Value>>)> = self
            .points
            .values()
            .filter(|point| {
                // Apply filter
                if let Some(payload) = &point.payload {
                    filter.iter().all(|(key, value)| {
                        payload.get(key).map_or(false, |v| v == value)
                    })
                } else {
                    filter.is_empty()
                }
            })
            .map(|point| {
                let score = calculate_similarity(&point.vector, query_vector, self.distance_metric);
                (point.id, score, point.payload.clone())
            })
            .collect();

        // Sort by score descending
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Take top-k
        results
            .into_iter()
            .take(limit)
            .map(|(id, score, payload)| SearchResult { id, score, payload })
            .collect()
    }

    /// Get all points
    pub fn get_all_points(&self) -> Vec<&Point> {
        self.points.values().collect()
    }

    /// Get points count
    pub fn count(&self) -> usize {
        self.points.len()
    }

    /// Get dimension
    pub fn dimension(&self) -> Option<usize> {
        self.dimension
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_upsert() {
        let mut collection = Collection::new("test".to_string(), DistanceMetric::Cosine);
        let point = Point {
            id: Uuid::new_v4(),
            vector: vec![1.0, 0.0, 0.0],
            payload: None,
        };

        let result = collection.upsert_point(point);
        assert!(result.is_ok());
        assert_eq!(collection.count(), 1);
    }

    #[test]
    fn test_collection_search() {
        let mut collection = Collection::new("test".to_string(), DistanceMetric::Cosine);
        
        let point1 = Point {
            id: Uuid::new_v4(),
            vector: vec![1.0, 0.0, 0.0],
            payload: None,
        };
        let point2 = Point {
            id: Uuid::new_v4(),
            vector: vec![0.0, 1.0, 0.0],
            payload: None,
        };

        collection.upsert_point(point1).unwrap();
        collection.upsert_point(point2).unwrap();

        let results = collection.search(&[1.0, 0.0, 0.0], 2);
        assert_eq!(results.len(), 2);
        assert!(results[0].score > results[1].score);
    }
}

