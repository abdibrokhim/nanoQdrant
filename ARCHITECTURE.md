# RustVecDB Architecture

This document describes the architecture and design decisions of RustVecDB.

## Overview

RustVecDB is a lightweight, in-memory vector database with optional persistence, built in Rust. It's designed as a proof-of-concept implementation of core vector database features inspired by Qdrant.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────┐
│                   REST API Layer                     │
│              (Actix-web Handlers)                    │
│  /collections  /points  /search  /health             │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│                 Storage Layer                        │
│           (VectorStorage + RwLock)                   │
│  - Collection Management                             │
│  - Thread-safe access                                │
│  - Persistence (JSON)                                │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│              Collection Layer                        │
│            (HashMap<UUID, Point>)                    │
│  - Vector storage                                    │
│  - Dimension validation                              │
│  - Search operations                                 │
└──────────────────────┬──────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│              Vector Operations                       │
│  - Cosine similarity                                 │
│  - Euclidean distance                                │
│  - Dot product                                       │
└─────────────────────────────────────────────────────┘
```

## Core Components

### 1. Models (`src/models.rs`)

Defines the data structures used throughout the application:

- **Point**: Represents a vector with an ID and optional payload
- **SearchRequest/SearchResult**: Request and response for search operations
- **CollectionInfo**: Metadata about a collection
- **DistanceMetric**: Enum for different similarity metrics

**Design Choice**: Using `serde` for easy serialization to JSON for both API responses and persistence.

### 2. Vector Operations (`src/vector.rs`)

Implements mathematical operations for vector similarity:

- **Cosine Similarity**: Best for text embeddings, angle-based comparison
- **Euclidean Distance**: Good for spatial data, measures straight-line distance
- **Dot Product**: Fast, useful when vectors are normalized

**Design Choice**: Pure Rust implementation without SIMD for simplicity. Could be optimized with SIMD instructions or GPU acceleration.

**Time Complexity**: O(d) where d is the vector dimension.

### 3. Collection (`src/collection.rs`)

Manages vectors within a single collection:

```rust
pub struct Collection {
    pub name: String,
    pub distance_metric: DistanceMetric,
    points: HashMap<Uuid, Point>,
    dimension: Option<usize>,
}
```

**Key Features**:
- Automatic dimension validation
- Linear search through all vectors
- Payload filtering support

**Design Choices**:
- **HashMap** for O(1) point lookup by ID
- **Linear search** for similarity (O(n) complexity)
- Dimension is optional until first vector is inserted

**Performance**:
- Insert/Update: O(1)
- Get: O(1)
- Delete: O(1)
- Search: O(n * d) where n is number of vectors, d is dimension

### 4. Storage (`src/storage.rs`)

Thread-safe storage layer managing multiple collections:

```rust
pub struct VectorStorage {
    collections: Arc<RwLock<HashMap<String, Collection>>>,
    storage_path: Option<String>,
}
```

**Design Choices**:
- **Arc<RwLock>**: Thread-safe, allows multiple readers or single writer
- **HashMap**: O(1) collection lookup
- **JSON Persistence**: Simple, human-readable, but not optimized for large datasets

**Thread Safety**:
- Read operations (search, get) acquire read lock - can happen concurrently
- Write operations (upsert, delete) acquire write lock - exclusive access

### 5. API Layer (`src/api.rs`)

RESTful API using Actix-web:

**Endpoints**:
- `GET /health` - Health check
- `GET /stats` - Storage statistics
- `PUT /collections/{name}` - Create collection
- `GET /collections` - List collections
- `GET /collections/{name}` - Get collection info
- `DELETE /collections/{name}` - Delete collection
- `PUT /collections/{name}/points/{id}` - Upsert point
- `POST /collections/{name}/points` - Batch upsert
- `GET /collections/{name}/points/{id}` - Get point
- `DELETE /collections/{name}/points/{id}` - Delete point
- `POST /collections/{name}/points/search` - Search vectors
- `GET /collections/{name}/points` - Get all points

**Design Choice**: RESTful design following HTTP semantics (GET, POST, PUT, DELETE).

## Data Flow

### Insert Operation

```
Client → API Handler → Storage → Collection → HashMap
                                        ↓
                                  Persistence Layer
```

1. Client sends PUT request with vector and payload
2. API handler validates request
3. Storage acquires write lock
4. Collection validates dimension
5. Point inserted into HashMap
6. Data persisted to disk (if enabled)

### Search Operation

```
Client → API Handler → Storage → Collection → Linear Scan
                                                    ↓
                                         Calculate Similarities
                                                    ↓
                                              Sort Results
                                                    ↓
                                             Return Top-K
```

1. Client sends POST request with query vector
2. API handler validates request
3. Storage acquires read lock
4. Collection performs linear scan
5. Calculate similarity for each vector
6. Sort by similarity score
7. Apply filters if provided
8. Return top-k results

## Performance Characteristics

### Time Complexity

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Insert | O(1) | HashMap insertion |
| Get | O(1) | HashMap lookup |
| Delete | O(1) | HashMap removal |
| Search | O(n * d) | Linear scan, d = dimension |
| Filtered Search | O(n * d) | Same as search + filter |

### Space Complexity

| Component | Complexity | Notes |
|-----------|-----------|-------|
| Storage | O(n * d) | n vectors, d dimensions |
| Index | None | No indexing structure |
| Payload | O(n * p) | p = average payload size |

## Limitations

### Current Implementation

1. **Linear Search**: O(n) complexity, slow for large datasets
2. **In-Memory Only**: All data must fit in RAM
3. **No Indexing**: No HNSW, IVF, or other indexing structures
4. **Single-Node**: No distributed capabilities
5. **JSON Persistence**: Inefficient for large datasets
6. **No Versioning**: No MVCC or transaction support

### Scalability Limits

- **Vectors**: Thousands to tens of thousands (depending on dimension)
- **Dimension**: No hard limit, but higher dimensions slow search
- **Payload**: Any JSON, but large payloads increase memory usage
- **Throughput**: Limited by single-node capacity

## Future Improvements

### High Priority

1. **HNSW Index**: Reduce search complexity to O(log n)
2. **Product Quantization**: Reduce memory usage by 4-32x
3. **Batch Operations**: More efficient bulk inserts
4. **Better Persistence**: Binary format, write-ahead log

### Medium Priority

5. **Async Search**: Don't block other operations
6. **gRPC API**: Better performance than REST
7. **Sparse Vectors**: BM25-style keyword search
8. **Advanced Filtering**: Complex queries with AND/OR/NOT

### Low Priority

9. **Distributed Mode**: Sharding and replication
10. **GPU Acceleration**: CUDA/OpenCL for vector operations
11. **Compression**: LZ4/Zstd for stored data
12. **Monitoring**: Prometheus metrics

## Design Principles

1. **Simplicity**: Easy to understand and modify
2. **Safety**: Leverage Rust's type system and ownership
3. **Correctness**: Comprehensive tests
4. **Performance**: Optimize hot paths
5. **Extensibility**: Easy to add features

## Comparison with Qdrant

| Feature | RustVecDB | Qdrant |
|---------|-----------|--------|
| Language | Rust | Rust |
| Storage | In-memory + JSON | Custom + WAL |
| Search | Linear O(n) | HNSW O(log n) |
| API | REST | REST + gRPC |
| Distributed | ❌ | ✅ |
| Quantization | ❌ | ✅ |
| Sparse Vectors | ❌ | ✅ |
| Production Ready | ❌ | ✅ |

## Testing Strategy

- **Unit Tests**: Vector operations, collection logic
- **Integration Tests**: API endpoints (via examples)
- **Manual Tests**: Python client script

## Dependencies

Key dependencies and their purposes:

- **actix-web**: HTTP server and routing
- **serde**: Serialization/deserialization
- **uuid**: Unique identifiers
- **tokio**: Async runtime
- **nalgebra**: Linear algebra operations (minimal use)

## Conclusion

RustVecDB demonstrates core vector database concepts in a clean, understandable implementation. While not suitable for production use, it serves as an excellent learning resource and proof-of-concept for vector search systems.

