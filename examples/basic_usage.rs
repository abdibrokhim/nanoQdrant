/// Example demonstrating basic usage of RustVecDB
/// Run with: cargo run --example basic_usage

use rustvecdb::models::DistanceMetric;
use rustvecdb::VectorStorage;
use std::collections::HashMap;
use uuid::Uuid;

fn main() {
    println!("=== RustVecDB Basic Usage Example ===\n");

    // Create storage instance (in-memory)
    let storage = VectorStorage::new(None);

    // 1. Create a collection
    println!("1. Creating collection 'movies'...");
    storage
        .create_collection("movies".to_string(), DistanceMetric::Cosine)
        .expect("Failed to create collection");
    println!("   ✓ Collection created\n");

    // 2. Insert some movie vectors
    println!("2. Inserting movie vectors...");
    
    let movies = vec![
        (
            "The Matrix",
            vec![0.1, 0.2, 0.3, 0.4, 0.5],
            "sci-fi",
            1999,
        ),
        (
            "Inception",
            vec![0.15, 0.25, 0.35, 0.45, 0.55],
            "sci-fi",
            2010,
        ),
        (
            "Interstellar",
            vec![0.12, 0.22, 0.32, 0.42, 0.52],
            "sci-fi",
            2014,
        ),
        (
            "The Notebook",
            vec![0.8, 0.1, 0.2, 0.3, 0.1],
            "romance",
            2004,
        ),
        (
            "Titanic",
            vec![0.75, 0.15, 0.25, 0.35, 0.15],
            "romance",
            1997,
        ),
    ];

    for (title, vector, genre, year) in movies {
        let mut payload = HashMap::new();
        payload.insert("title".to_string(), serde_json::json!(title));
        payload.insert("genre".to_string(), serde_json::json!(genre));
        payload.insert("year".to_string(), serde_json::json!(year));

        storage
            .update_collection("movies", |collection| {
                let point = rustvecdb::models::Point {
                    id: Uuid::new_v4(),
                    vector,
                    payload: Some(payload),
                };
                collection.upsert_point(point)?;
                Ok(())
            })
            .expect("Failed to insert point");
        
        println!("   ✓ Added: {}", title);
    }
    println!();

    // 3. Get collection info
    println!("3. Collection information:");
    if let Some(collection) = storage.get_collection("movies") {
        println!("   - Name: {}", collection.name);
        println!("   - Vectors count: {}", collection.count());
        println!("   - Dimension: {:?}", collection.dimension());
    }
    println!();

    // 4. Search for similar movies
    println!("4. Searching for movies similar to 'The Matrix'...");
    let query_vector = vec![0.1, 0.2, 0.3, 0.4, 0.5]; // The Matrix vector

    if let Some(collection) = storage.get_collection("movies") {
        let results = collection.search(&query_vector, 3);
        
        println!("   Top 3 similar movies:");
        for (i, result) in results.iter().enumerate() {
            if let Some(payload) = &result.payload {
                let title = payload.get("title").unwrap();
                let year = payload.get("year").unwrap();
                println!(
                    "   {}. {} ({}) - Score: {:.4}",
                    i + 1,
                    title.as_str().unwrap(),
                    year,
                    result.score
                );
            }
        }
    }
    println!();

    // 5. Search with filter (only sci-fi movies)
    println!("5. Searching for similar sci-fi movies...");
    let mut filter = HashMap::new();
    filter.insert("genre".to_string(), serde_json::json!("sci-fi"));

    if let Some(collection) = storage.get_collection("movies") {
        let results = collection.search_with_filter(&query_vector, 3, &filter);
        
        println!("   Sci-fi movies:");
        for (i, result) in results.iter().enumerate() {
            if let Some(payload) = &result.payload {
                let title = payload.get("title").unwrap();
                let year = payload.get("year").unwrap();
                println!(
                    "   {}. {} ({}) - Score: {:.4}",
                    i + 1,
                    title.as_str().unwrap(),
                    year,
                    result.score
                );
            }
        }
    }
    println!();

    // 6. Get storage stats
    println!("6. Storage statistics:");
    let stats = storage.get_stats();
    println!("   - Total collections: {}", stats.total_collections);
    println!("   - Total vectors: {}", stats.total_vectors);
    println!();

    // 7. List all collections
    println!("7. All collections:");
    let collections = storage.list_collections();
    for collection_info in collections {
        println!(
            "   - {} ({} vectors, dimension: {:?})",
            collection_info.name, collection_info.vectors_count, collection_info.dimension
        );
    }
    println!();

    println!("=== Example completed successfully! ===");
}

