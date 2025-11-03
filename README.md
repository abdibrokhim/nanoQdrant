# RustVecDB

A high-performance, lightweight vector database written in Rust, inspired by [Qdrant](https://github.com/qdrant/qdrant). This is a proof-of-concept implementation featuring core vector search capabilities.

## Features

- ✨ **Vector Similarity Search** - Fast similarity search using multiple distance metrics
- 📊 **Multiple Distance Metrics** - Cosine, Euclidean, and Dot Product
- 🗂️ **Collection Management** - Organize vectors into collections
- 🏷️ **Payload Support** - Attach JSON metadata to vectors
- 🔍 **Filtering** - Search with payload-based filters
- 💾 **Persistence** - Optional disk persistence
- 🚀 **REST API** - Full-featured HTTP API
- ⚡ **In-Memory Storage** - Lightning-fast operations

## Quick Start

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd rustvecdb

# Build the project
cargo build --release
```

### Running the Server

```bash
# Run with in-memory storage (default)
cargo run

# Run with persistent storage
STORAGE_PATH=./data/storage.json cargo run

# Custom host and port
HOST=0.0.0.0 PORT=8080 cargo run
```

The server will start at `http://127.0.0.1:6333` by default.

## API Usage

### Health Check

```bash
curl http://localhost:6333/health
```

### Create a Collection

```bash
curl -X PUT http://localhost:6333/collections/my_collection \
  -H "Content-Type: application/json" \
  -d '{
    "name": "my_collection",
    "distance": "Cosine"
  }'
```

Distance options: `Cosine`, `Euclidean`, `DotProduct`

### Insert Points (Vectors)

#### Single Point

```bash
curl -X PUT http://localhost:6333/collections/my_collection/points/550e8400-e29b-41d4-a716-446655440000 \
  -H "Content-Type: application/json" \
  -d '{
    "vector": [0.1, 0.2, 0.3, 0.4],
    "payload": {
      "name": "Item 1",
      "category": "electronics"
    }
  }'
```

#### Batch Insert

```bash
curl -X POST http://localhost:6333/collections/my_collection/points \
  -H "Content-Type: application/json" \
  -d '[
    {
      "vector": [0.1, 0.2, 0.3, 0.4],
      "payload": {"name": "Item 1", "category": "electronics"}
    },
    {
      "vector": [0.5, 0.6, 0.7, 0.8],
      "payload": {"name": "Item 2", "category": "books"}
    }
  ]'
```

### Search for Similar Vectors

```bash
curl -X POST http://localhost:6333/collections/my_collection/points/search \
  -H "Content-Type: application/json" \
  -d '{
    "vector": [0.1, 0.2, 0.3, 0.4],
    "limit": 10
  }'
```

#### Search with Filter

```bash
curl -X POST http://localhost:6333/collections/my_collection/points/search \
  -H "Content-Type: application/json" \
  -d '{
    "vector": [0.1, 0.2, 0.3, 0.4],
    "limit": 10,
    "filter": {
      "category": "electronics"
    }
  }'
```

### Get a Point

```bash
curl http://localhost:6333/collections/my_collection/points/550e8400-e29b-41d4-a716-446655440000
```

### List All Points

```bash
curl http://localhost:6333/collections/my_collection/points
```

### Delete a Point

```bash
curl -X DELETE http://localhost:6333/collections/my_collection/points/550e8400-e29b-41d4-a716-446655440000
```

### List Collections

```bash
curl http://localhost:6333/collections
```

### Get Collection Info

```bash
curl http://localhost:6333/collections/my_collection
```

### Delete Collection

```bash
curl -X DELETE http://localhost:6333/collections/my_collection
```

### Get Storage Stats

```bash
curl http://localhost:6333/stats
```

## Architecture

### Core Components

- **Vector Operations** (`src/vector.rs`) - Similarity calculations (cosine, euclidean, dot product)
- **Collection** (`src/collection.rs`) - Vector storage and search within a collection
- **Storage** (`src/storage.rs`) - Collection management and persistence
- **Models** (`src/models.rs`) - Data structures and types
- **API** (`src/api.rs`) - REST API endpoints
- **Main** (`src/main.rs`) - Server initialization and configuration

### Distance Metrics

#### Cosine Similarity
Measures the cosine of the angle between vectors. Range: [-1, 1], where 1 means identical direction.

```rust
similarity = (A · B) / (||A|| * ||B||)
```

#### Euclidean Distance
Straight-line distance between vectors. Lower is more similar.

```rust
distance = sqrt(Σ(a_i - b_i)²)
```

#### Dot Product
Sum of element-wise multiplication. Higher is more similar.

```rust
dot_product = Σ(a_i * b_i)
```

## Example: Building a Simple Recommendation System

```python
import requests
import json

BASE_URL = "http://localhost:6333"

# 1. Create a collection
requests.put(f"{BASE_URL}/collections/movies", json={
    "name": "movies",
    "distance": "Cosine"
})

# 2. Add movie embeddings
movies = [
    {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "vector": [0.1, 0.2, 0.3, 0.4],
        "payload": {"title": "The Matrix", "genre": "sci-fi", "year": 1999}
    },
    {
        "id": "550e8400-e29b-41d4-a716-446655440001",
        "vector": [0.15, 0.25, 0.35, 0.45],
        "payload": {"title": "Inception", "genre": "sci-fi", "year": 2010}
    },
    {
        "id": "550e8400-e29b-41d4-a716-446655440002",
        "vector": [0.8, 0.1, 0.2, 0.3],
        "payload": {"title": "The Notebook", "genre": "romance", "year": 2004}
    }
]

for movie in movies:
    requests.put(
        f"{BASE_URL}/collections/movies/points/{movie['id']}",
        json={"vector": movie["vector"], "payload": movie["payload"]}
    )

# 3. Search for similar movies
response = requests.post(f"{BASE_URL}/collections/movies/points/search", json={
    "vector": [0.1, 0.2, 0.3, 0.4],  # The Matrix embedding
    "limit": 3
})

results = response.json()
print("Similar movies:", json.dumps(results, indent=2))

# 4. Search with genre filter
response = requests.post(f"{BASE_URL}/collections/movies/points/search", json={
    "vector": [0.1, 0.2, 0.3, 0.4],
    "limit": 3,
    "filter": {"genre": "sci-fi"}
})

filtered_results = response.json()
print("Sci-fi movies:", json.dumps(filtered_results, indent=2))
```

## Development

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
cargo build --release
./target/release/rustvecdb
```

### Environment Variables

- `HOST` - Server host (default: `127.0.0.1`)
- `PORT` - Server port (default: `6333`)
- `STORAGE_PATH` - Path to persistence file (optional)
- `RUST_LOG` - Log level (`info`, `debug`, `warn`, `error`)

## Performance Considerations

- **In-Memory Storage**: All data is kept in RAM for fast access
- **Linear Search**: Current implementation uses brute-force search (O(n) complexity)
- **Persistence**: Writes entire dataset to disk on updates

### Future Optimizations

- [ ] HNSW (Hierarchical Navigable Small World) index for faster search
- [ ] Product Quantization for memory efficiency
- [ ] Sharding for horizontal scaling
- [ ] Write-Ahead Log (WAL) for safer persistence
- [ ] gRPC API for better performance
- [ ] Sparse vector support
- [ ] Advanced filtering with query DSL

## Comparison with Qdrant

This is a **proof-of-concept** implementation. Qdrant offers many more features:

| Feature | RustVecDB | Qdrant |
|---------|-----------|--------|
| Vector Search | ✅ | ✅ |
| Multiple Metrics | ✅ | ✅ |
| Payload Filtering | ✅ (Basic) | ✅ (Advanced) |
| Persistence | ✅ (JSON) | ✅ (Custom) |
| HNSW Index | ❌ | ✅ |
| Distributed | ❌ | ✅ |
| gRPC | ❌ | ✅ |
| Quantization | ❌ | ✅ |
| Sparse Vectors | ❌ | ✅ |

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

Inspired by [Qdrant](https://github.com/qdrant/qdrant) - the production-ready vector database.

