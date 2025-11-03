# Project Structure

```
rustvecdb/
├── Cargo.toml                 # Project dependencies and metadata
├── Cargo.lock                 # Locked dependency versions
├── .gitignore                 # Git ignore rules
├── Makefile                   # Build and run commands
│
├── README.md                  # Main documentation
├── QUICK_START.md            # Quick start guide
├── ARCHITECTURE.md           # Architecture documentation
├── PROJECT_STRUCTURE.md      # This file
│
├── src/
│   ├── main.rs               # Application entry point
│   ├── lib.rs                # Library exports
│   ├── models.rs             # Data structures and types
│   ├── vector.rs             # Vector similarity operations
│   ├── collection.rs         # Collection management
│   ├── storage.rs            # Storage layer
│   └── api.rs                # REST API handlers
│
└── examples/
    ├── basic_usage.rs        # Rust example
    └── api_client.py         # Python API client example
```

## File Descriptions

### Root Files

- **Cargo.toml**: Rust package manifest with dependencies
- **Cargo.lock**: Locked versions for reproducible builds
- **.gitignore**: Ignores target/, data/, *.log files
- **Makefile**: Convenient commands (make run, make test, etc.)

### Documentation

- **README.md**: Comprehensive guide with API documentation and examples
- **QUICK_START.md**: Get started in 5 minutes
- **ARCHITECTURE.md**: Detailed architecture and design decisions
- **PROJECT_STRUCTURE.md**: This file describing the project layout

### Source Code (`src/`)

#### `main.rs` - Application Entry Point
- Initializes the HTTP server
- Configures logging
- Sets up routes
- Reads environment variables

#### `lib.rs` - Library Exports
- Public API for the library
- Module declarations
- Re-exports main types

#### `models.rs` - Data Structures
Contains:
- `Point`: Vector with ID and payload
- `DistanceMetric`: Cosine, Euclidean, DotProduct
- `SearchRequest`: Search query parameters
- `SearchResult`: Search results with scores
- `ApiResponse<T>`: Generic API response wrapper
- `CollectionInfo`: Collection metadata

#### `vector.rs` - Vector Operations
Functions:
- `cosine_similarity()`: Angle-based similarity
- `euclidean_distance()`: Straight-line distance
- `dot_product()`: Element-wise multiplication sum
- `magnitude()`: Vector norm
- `normalize()`: Convert to unit vector
- `calculate_similarity()`: Dispatch based on metric

Includes unit tests for all functions.

#### `collection.rs` - Collection Management
- `Collection` struct manages vectors in a collection
- Methods:
  - `upsert_point()`: Insert or update vector
  - `get_point()`: Retrieve by ID
  - `delete_point()`: Remove vector
  - `search()`: Find similar vectors
  - `search_with_filter()`: Search with payload filters

Includes unit tests.

#### `storage.rs` - Storage Layer
- `VectorStorage` struct manages multiple collections
- Thread-safe with `Arc<RwLock<>>`
- Methods:
  - `create_collection()`: Create new collection
  - `get_collection()`: Retrieve collection
  - `delete_collection()`: Remove collection
  - `update_collection()`: Modify collection
  - `persist_to_disk()`: Save to JSON file
  - `load_from_disk()`: Load from JSON file

#### `api.rs` - REST API Handlers
Actix-web handlers:
- `health_check()`: GET /health
- `get_stats()`: GET /stats
- `create_collection()`: PUT /collections/{name}
- `list_collections()`: GET /collections
- `get_collection()`: GET /collections/{name}
- `delete_collection()`: DELETE /collections/{name}
- `upsert_point()`: PUT /collections/{name}/points/{id}
- `batch_upsert_points()`: POST /collections/{name}/points
- `get_point()`: GET /collections/{name}/points/{id}
- `delete_point()`: DELETE /collections/{name}/points/{id}
- `search_points()`: POST /collections/{name}/points/search
- `get_all_points()`: GET /collections/{name}/points

### Examples (`examples/`)

#### `basic_usage.rs`
Demonstrates:
- Creating a collection
- Inserting vectors with payloads
- Searching for similar vectors
- Filtering by payload

Run with: `cargo run --example basic_usage`

#### `api_client.py`
Python script demonstrating REST API usage:
- Health check
- Creating collections
- Batch inserting points
- Searching with and without filters
- Getting statistics

Run with: `python examples/api_client.py` (server must be running)

## Build Artifacts (not in repo)

```
target/
├── debug/              # Debug builds
│   └── rustvecdb      # Debug binary
└── release/            # Release builds
    └── rustvecdb      # Optimized binary
```

## Data Files (not in repo)

```
data/
└── storage.json       # Persistent storage file
```

Created when using `STORAGE_PATH=./data/storage.json`

## Dependencies Summary

### Production
- **actix-web**: Web framework
- **tokio**: Async runtime
- **serde**: Serialization
- **uuid**: ID generation
- **nalgebra**: Linear algebra
- **env_logger**: Logging

### Development
- **reqwest**: HTTP client for tests

## Lines of Code

Approximate breakdown:
- `models.rs`: ~80 lines
- `vector.rs`: ~120 lines
- `collection.rs`: ~180 lines
- `storage.rs`: ~150 lines
- `api.rs`: ~320 lines
- `main.rs`: ~60 lines
- `lib.rs`: ~10 lines

**Total**: ~920 lines of Rust code (excluding tests and comments)

## Key Design Patterns

1. **Repository Pattern**: `VectorStorage` acts as repository
2. **Builder Pattern**: Actix-web app configuration
3. **Strategy Pattern**: Different distance metrics
4. **Wrapper Pattern**: `ApiResponse<T>` wraps all responses

## API Design Philosophy

- **RESTful**: HTTP methods map to CRUD operations
- **JSON**: All data exchanged as JSON
- **Consistent**: Same response format for all endpoints
- **Idempotent**: PUT operations are idempotent

## Testing Strategy

- Unit tests in `vector.rs` and `collection.rs`
- Integration tests via examples
- Manual testing with Python client
- Run with: `cargo test`

