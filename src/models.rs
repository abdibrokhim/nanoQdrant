use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a vector with optional payload (metadata)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub id: Uuid,
    pub vector: Vec<f32>,
    pub payload: Option<HashMap<String, serde_json::Value>>,
}

/// Distance metric for vector similarity
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DistanceMetric {
    Cosine,
    Euclidean,
    DotProduct,
}

/// Search request
#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub vector: Vec<f32>,
    #[serde(default = "default_limit")]
    pub limit: usize,
    pub filter: Option<HashMap<String, serde_json::Value>>,
}

fn default_limit() -> usize {
    10
}

/// Search result
#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub id: Uuid,
    pub score: f32,
    pub payload: Option<HashMap<String, serde_json::Value>>,
}

/// Request to create a point
#[derive(Debug, Deserialize)]
pub struct CreatePointRequest {
    pub id: Option<Uuid>,
    pub vector: Vec<f32>,
    pub payload: Option<HashMap<String, serde_json::Value>>,
}

/// Response for operations
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// Collection info
#[derive(Debug, Serialize)]
pub struct CollectionInfo {
    pub name: String,
    pub vectors_count: usize,
    pub dimension: Option<usize>,
}

/// Request to create a collection
#[derive(Debug, Deserialize)]
pub struct CreateCollectionRequest {
    pub name: String,
    pub distance: Option<DistanceMetric>,
}

