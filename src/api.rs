use crate::models::*;
use crate::storage::VectorStorage;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use uuid::Uuid;

/// Health check endpoint
#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "rustvecdb"
    }))
}

/// Get storage stats
#[get("/stats")]
pub async fn get_stats(storage: web::Data<VectorStorage>) -> impl Responder {
    let stats = storage.get_stats();
    HttpResponse::Ok().json(ApiResponse::success(stats))
}

/// Create a new collection
#[put("/collections/{collection_name}")]
pub async fn create_collection(
    path: web::Path<String>,
    body: web::Json<CreateCollectionRequest>,
    storage: web::Data<VectorStorage>,
) -> impl Responder {
    let collection_name = path.into_inner();
    let distance_metric = body.distance.unwrap_or(DistanceMetric::Cosine);

    match storage.create_collection(collection_name.clone(), distance_metric) {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
            "collection": collection_name,
            "status": "created"
        }))),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

/// List all collections
#[get("/collections")]
pub async fn list_collections(storage: web::Data<VectorStorage>) -> impl Responder {
    let collections = storage.list_collections();
    HttpResponse::Ok().json(ApiResponse::success(collections))
}

/// Get collection info
#[get("/collections/{collection_name}")]
pub async fn get_collection(
    path: web::Path<String>,
    storage: web::Data<VectorStorage>,
) -> impl Responder {
    let collection_name = path.into_inner();

    match storage.get_collection(&collection_name) {
        Some(collection) => {
            let info = CollectionInfo {
                name: collection.name.clone(),
                vectors_count: collection.count(),
                dimension: collection.dimension(),
            };
            HttpResponse::Ok().json(ApiResponse::success(info))
        }
        None => HttpResponse::NotFound().json(ApiResponse::<()>::error(format!(
            "Collection '{}' not found",
            collection_name
        ))),
    }
}

/// Delete a collection
#[delete("/collections/{collection_name}")]
pub async fn delete_collection(
    path: web::Path<String>,
    storage: web::Data<VectorStorage>,
) -> impl Responder {
    let collection_name = path.into_inner();

    match storage.delete_collection(&collection_name) {
        Ok(true) => HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
            "collection": collection_name,
            "status": "deleted"
        }))),
        Ok(false) => HttpResponse::NotFound().json(ApiResponse::<()>::error(format!(
            "Collection '{}' not found",
            collection_name
        ))),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e)),
    }
}

/// Insert or update a point
#[put("/collections/{collection_name}/points/{point_id}")]
pub async fn upsert_point(
    path: web::Path<(String, String)>,
    body: web::Json<CreatePointRequest>,
    storage: web::Data<VectorStorage>,
) -> impl Responder {
    let (collection_name, point_id_str) = path.into_inner();

    let point_id = match Uuid::parse_str(&point_id_str) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest()
                .json(ApiResponse::<()>::error("Invalid UUID format".to_string()))
        }
    };

    let point = Point {
        id: point_id,
        vector: body.vector.clone(),
        payload: body.payload.clone(),
    };

    match storage.update_collection(&collection_name, |collection| {
        collection.upsert_point(point)?;
        Ok(())
    }) {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
            "id": point_id,
            "status": "upserted"
        }))),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

/// Batch upsert points
#[post("/collections/{collection_name}/points")]
pub async fn batch_upsert_points(
    path: web::Path<String>,
    body: web::Json<Vec<CreatePointRequest>>,
    storage: web::Data<VectorStorage>,
) -> impl Responder {
    let collection_name = path.into_inner();
    let mut inserted_ids = Vec::new();

    let result = storage.update_collection(&collection_name, |collection| {
        for req in body.iter() {
            let point_id = req.id.unwrap_or_else(Uuid::new_v4);
            let point = Point {
                id: point_id,
                vector: req.vector.clone(),
                payload: req.payload.clone(),
            };

            collection.upsert_point(point)?;
            inserted_ids.push(point_id);
        }
        Ok(())
    });

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
            "ids": inserted_ids,
            "count": inserted_ids.len()
        }))),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

/// Get a point by ID
#[get("/collections/{collection_name}/points/{point_id}")]
pub async fn get_point(
    path: web::Path<(String, String)>,
    storage: web::Data<VectorStorage>,
) -> impl Responder {
    let (collection_name, point_id_str) = path.into_inner();

    let point_id = match Uuid::parse_str(&point_id_str) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest()
                .json(ApiResponse::<()>::error("Invalid UUID format".to_string()))
        }
    };

    match storage.get_collection(&collection_name) {
        Some(collection) => match collection.get_point(&point_id) {
            Some(point) => HttpResponse::Ok().json(ApiResponse::success(point)),
            None => HttpResponse::NotFound()
                .json(ApiResponse::<()>::error("Point not found".to_string())),
        },
        None => HttpResponse::NotFound()
            .json(ApiResponse::<()>::error("Collection not found".to_string())),
    }
}

/// Delete a point
#[delete("/collections/{collection_name}/points/{point_id}")]
pub async fn delete_point(
    path: web::Path<(String, String)>,
    storage: web::Data<VectorStorage>,
) -> impl Responder {
    let (collection_name, point_id_str) = path.into_inner();

    let point_id = match Uuid::parse_str(&point_id_str) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest()
                .json(ApiResponse::<()>::error("Invalid UUID format".to_string()))
        }
    };

    match storage.update_collection(&collection_name, |collection| {
        if collection.delete_point(&point_id) {
            Ok(())
        } else {
            Err("Point not found".to_string())
        }
    }) {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({
            "id": point_id,
            "status": "deleted"
        }))),
        Err(e) => HttpResponse::NotFound().json(ApiResponse::<()>::error(e)),
    }
}

/// Search for similar vectors
#[post("/collections/{collection_name}/points/search")]
pub async fn search_points(
    path: web::Path<String>,
    body: web::Json<SearchRequest>,
    storage: web::Data<VectorStorage>,
) -> impl Responder {
    let collection_name = path.into_inner();

    match storage.get_collection(&collection_name) {
        Some(collection) => {
            let results = if let Some(filter) = &body.filter {
                collection.search_with_filter(&body.vector, body.limit, filter)
            } else {
                collection.search(&body.vector, body.limit)
            };

            HttpResponse::Ok().json(ApiResponse::success(results))
        }
        None => HttpResponse::NotFound()
            .json(ApiResponse::<()>::error("Collection not found".to_string())),
    }
}

/// Get all points in a collection
#[get("/collections/{collection_name}/points")]
pub async fn get_all_points(
    path: web::Path<String>,
    storage: web::Data<VectorStorage>,
) -> impl Responder {
    let collection_name = path.into_inner();

    match storage.get_collection(&collection_name) {
        Some(collection) => {
            let points = collection.get_all_points();
            HttpResponse::Ok().json(ApiResponse::success(points))
        }
        None => HttpResponse::NotFound()
            .json(ApiResponse::<()>::error("Collection not found".to_string())),
    }
}

