# Quick Start Guide

This guide will help you get started with RustVecDB in 5 minutes.

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd rustvecdb

# Build the project
cargo build --release
```

## Start the Server

```bash
# Start with default settings (in-memory storage)
cargo run

# OR start with persistent storage
STORAGE_PATH=./data/storage.json cargo run
```

The server will be available at `http://127.0.0.1:6333`

## Test the Server

Open a new terminal and run:

```bash
# Health check
curl http://localhost:6333/health

# You should see: {"status":"ok","service":"rustvecdb"}
```

## Create Your First Collection

```bash
curl -X PUT http://localhost:6333/collections/test \
  -H "Content-Type: application/json" \
  -d '{
    "name": "test",
    "distance": "Cosine"
  }'
```

## Add Some Vectors

```bash
# Add a single vector
curl -X POST http://localhost:6333/collections/test/points \
  -H "Content-Type: application/json" \
  -d '[
    {
      "vector": [1.0, 2.0, 3.0, 4.0],
      "payload": {"name": "Vector 1"}
    },
    {
      "vector": [1.1, 2.1, 3.1, 4.1],
      "payload": {"name": "Vector 2"}
    },
    {
      "vector": [5.0, 6.0, 7.0, 8.0],
      "payload": {"name": "Vector 3"}
    }
  ]'
```

## Search for Similar Vectors

```bash
curl -X POST http://localhost:6333/collections/test/points/search \
  -H "Content-Type: application/json" \
  -d '{
    "vector": [1.0, 2.0, 3.0, 4.0],
    "limit": 2
  }'
```

You'll get results showing the most similar vectors!

## Run the Examples

### Rust Example

```bash
cargo run --example basic_usage
```

### Python Example

First, make sure the server is running, then:

```bash
pip install requests
python examples/api_client.py
```

## Next Steps

- Check out the [README.md](README.md) for full API documentation
- Explore more examples in the `examples/` directory
- Build your own recommendation system, search engine, or ML application!

## Common Use Cases

### Semantic Search
Store document embeddings and find similar documents based on meaning.

### Recommendation Systems
Store user/item embeddings and recommend similar items.

### Image Search
Store image embeddings from a vision model and find similar images.

### Anomaly Detection
Store normal data points and detect outliers by finding distant points.

## Troubleshooting

### Port Already in Use

```bash
# Use a different port
PORT=8080 cargo run
```

### Storage File Not Found

The data directory is created automatically. Make sure you have write permissions:

```bash
mkdir -p ./data
STORAGE_PATH=./data/storage.json cargo run
```

### Dimension Mismatch

All vectors in a collection must have the same dimension. The dimension is set by the first inserted vector.

## Need Help?

- Check the [README.md](README.md) for detailed documentation
- Look at the examples in `examples/`
- Open an issue on GitHub

