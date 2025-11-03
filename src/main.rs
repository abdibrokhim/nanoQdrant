use actix_web::{middleware, web, App, HttpServer};
use rustvecdb::api::*;
use rustvecdb::VectorStorage;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get configuration from environment
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "6333".to_string());
    let storage_path = env::var("STORAGE_PATH").ok();

    log::info!("Initializing RustVecDB...");
    
    // Create storage
    let storage = VectorStorage::new(storage_path.clone());
    let storage_data = web::Data::new(storage);

    if let Some(path) = storage_path {
        log::info!("Using persistent storage at: {}", path);
    } else {
        log::info!("Using in-memory storage (no persistence)");
    }

    log::info!("Starting server at http://{}:{}", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(storage_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            // Health and stats
            .service(health_check)
            .service(get_stats)
            // Collection management
            .service(create_collection)
            .service(list_collections)
            .service(get_collection)
            .service(delete_collection)
            // Point operations
            .service(upsert_point)
            .service(batch_upsert_points)
            .service(get_point)
            .service(delete_point)
            .service(search_points)
            .service(get_all_points)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

