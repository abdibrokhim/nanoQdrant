# Understanding Text Embeddings: From Words to Vectors

This guide explains how words and text are converted into numerical vectors (embeddings) that can be stored and searched in RustVecDB.

## Table of Contents

- [What are Embeddings?](#what-are-embeddings)
- [Why Do We Need Embeddings?](#why-do-we-need-embeddings)
- [The Conversion Process](#the-conversion-process)
- [How to Generate Embeddings](#how-to-generate-embeddings)
- [Using Embeddings with RustVecDB](#using-embeddings-with-rustvecdb)
- [Best Practices](#best-practices)

## What are Embeddings?

**Embeddings** are numerical representations of text (words, sentences, or documents) as vectors of floating-point numbers. They capture the semantic meaning of text in a way that computers can understand and process.

### Example

The word **"cat"** might be represented as:
```python
[0.2, -0.5, 0.8, 0.3, ...]  # A vector with hundreds of dimensions
```

The word **"dog"** (which has similar meaning) might be:
```python
[0.25, -0.48, 0.75, 0.28, ...]  # Similar values to "cat"
```

The word **"car"** (which has different meaning) might be:
```python
[-0.6, 0.3, -0.2, 0.9, ...]  # Very different values
```

### Key Properties

1. **Similar meanings = Similar vectors**: Words with similar meanings have vectors that are close together in the vector space
2. **Fixed length**: All embeddings from the same model have the same number of dimensions
3. **Capture semantic meaning**: They understand context, not just literal word matching

## Why Do We Need Embeddings?

Computers can't understand text directly. Embeddings solve this by:

1. **Enabling Semantic Search**: Find documents by meaning, not just keywords
   - Query: "How to fix a broken computer"
   - Matches: "repair laptop", "troubleshoot PC" (even without the exact words!)

2. **Enabling Similarity Comparisons**: Measure how similar two pieces of text are
   - "I love pizza" vs "Pizza is great" → High similarity
   - "I love pizza" vs "The weather is cold" → Low similarity

3. **Supporting Machine Learning**: Neural networks need numerical inputs, not text

## The Conversion Process

Converting text to vectors happens in **three main steps**:

### Step 1: Tokenization (Text → Tokens)

Text is broken down into smaller pieces called **tokens** (usually words or subwords).

```
Input:  "The cat sits on the mat"
Tokens: ["The", "cat", "sits", "on", "the", "mat"]
```

For subword tokenization (used by modern models):
```
Input:  "unhappiness"
Tokens: ["un", "happiness"]
```

### Step 2: Token to ID Mapping (Tokens → Numbers)

Each token is converted to a unique integer ID from a fixed vocabulary.

```
Vocabulary: {"The": 1, "cat": 45, "sits": 123, "on": 7, "mat": 89, ...}

Tokens: ["The", "cat", "sits", "on", "the", "mat"]
IDs:    [1,     45,    123,    7,   1,     89]
```

### Step 3: Embedding Lookup (IDs → Vectors)

Each ID is mapped to a learned vector (embedding) that captures meaning.

```
ID 45 ("cat") → [0.2, -0.5, 0.8, 0.3, -0.1, ...]  (384 dimensions)
ID 89 ("mat") → [0.1, 0.3, -0.2, 0.4, 0.6, ...]   (384 dimensions)
```

For sentences, the individual word vectors are combined (averaged, max-pooled, or using attention mechanisms) into a single sentence vector.

```
"The cat sits on the mat" → [0.15, -0.2, 0.3, 0.35, 0.1, ...]
```

## How to Generate Embeddings

There are several ways to generate embeddings for your text:

### Method 1: Using Sentence Transformers (Recommended for Local Use)

**Sentence Transformers** is a Python library that makes it easy to generate high-quality embeddings locally.

#### Installation

```bash
pip install sentence-transformers
```

#### Basic Example

```python
from sentence_transformers import SentenceTransformer

# Load a pre-trained model
model = SentenceTransformer('all-MiniLM-L6-v2')

# Generate embeddings
sentences = [
    "The cat sits on the mat",
    "A dog plays in the park",
    "I love machine learning"
]

embeddings = model.encode(sentences)

# embeddings is a numpy array of shape (3, 384)
# Each sentence is now a 384-dimensional vector
print(f"Shape: {embeddings.shape}")
print(f"First embedding: {embeddings[0][:5]}...")  # Show first 5 values
```

#### Popular Models

| Model | Dimensions | Speed | Quality | Use Case |
|-------|-----------|-------|---------|----------|
| `all-MiniLM-L6-v2` | 384 | Fast | Good | General purpose, balanced |
| `all-mpnet-base-v2` | 768 | Medium | Better | Higher quality needed |
| `paraphrase-multilingual-MiniLM-L12-v2` | 384 | Fast | Good | Multiple languages |
| `multi-qa-MiniLM-L6-cos-v1` | 384 | Fast | Good | Question-answering |

### Method 2: Using OpenAI API (Powerful, Cloud-based)

**OpenAI's API** provides state-of-the-art embeddings via their `text-embedding-3-small` and `text-embedding-3-large` models.

#### Installation

```bash
pip install openai
```

#### Example

```python
from openai import OpenAI

client = OpenAI(api_key="your-api-key-here")

# Generate embeddings
response = client.embeddings.create(
    model="text-embedding-3-small",
    input="The cat sits on the mat"
)

embedding = response.data[0].embedding

# embedding is a list of 1536 floats
print(f"Dimensions: {len(embedding)}")
print(f"First few values: {embedding[:5]}")
```

**Cost**: Small (~$0.02 per 1M tokens), Large (~$0.13 per 1M tokens)

### Method 3: Using Hugging Face Transformers

**Transformers** library gives you access to thousands of models.

#### Installation

```bash
pip install transformers torch
```

#### Example

```python
from transformers import AutoTokenizer, AutoModel
import torch

# Load model and tokenizer
model_name = "sentence-transformers/all-MiniLM-L6-v2"
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModel.from_pretrained(model_name)

# Encode text
text = "The cat sits on the mat"
tokens = tokenizer(text, return_tensors="pt", padding=True, truncation=True)

# Generate embeddings
with torch.no_grad():
    outputs = model(**tokens)
    # Use mean pooling
    embedding = outputs.last_hidden_state.mean(dim=1).squeeze().numpy()

print(f"Dimensions: {len(embedding)}")
print(f"Embedding: {embedding[:5]}...")
```

### Method 4: Using Cohere API

**Cohere** provides multilingual embeddings with excellent quality.

#### Installation

```bash
pip install cohere
```

#### Example

```python
import cohere

co = cohere.Client("your-api-key-here")

# Generate embeddings
response = co.embed(
    texts=["The cat sits on the mat", "A dog plays in the park"],
    model="embed-english-v3.0",
    input_type="search_document"
)

embeddings = response.embeddings
print(f"Number of embeddings: {len(embeddings)}")
print(f"Dimensions: {len(embeddings[0])}")
```

## Using Embeddings with RustVecDB

Once you have embeddings, you can store and search them in RustVecDB.

### Complete Example: Semantic Search System

```python
#!/usr/bin/env python3
"""
Complete example: Building a semantic search system with embeddings
"""

import requests
from sentence_transformers import SentenceTransformer

# Configuration
BASE_URL = "http://localhost:6333"
COLLECTION_NAME = "documents"

# Step 1: Load the embedding model
print("Loading embedding model...")
model = SentenceTransformer('all-MiniLM-L6-v2')

# Step 2: Create a collection in RustVecDB
print("Creating collection...")
requests.put(
    f"{BASE_URL}/collections/{COLLECTION_NAME}",
    json={"name": COLLECTION_NAME, "distance": "Cosine"}
)

# Step 3: Prepare your documents
documents = [
    "Python is a high-level programming language",
    "Machine learning enables computers to learn from data",
    "The cat sleeps peacefully on the warm couch",
    "Neural networks are inspired by the human brain",
    "Dogs are loyal and friendly pets"
]

# Step 4: Convert documents to embeddings
print("Generating embeddings...")
embeddings = model.encode(documents)

# Step 5: Store embeddings in RustVecDB
print("Storing embeddings in database...")
points = []
for i, (doc, embedding) in enumerate(zip(documents, embeddings)):
    points.append({
        "vector": embedding.tolist(),  # Convert numpy array to list
        "payload": {
            "text": doc,
            "id": i
        }
    })

response = requests.post(
    f"{BASE_URL}/collections/{COLLECTION_NAME}/points",
    json=points
)
print(f"Stored {len(points)} documents")

# Step 6: Search with a query
query = "What is artificial intelligence?"
print(f"\nSearching for: '{query}'")

# Convert query to embedding
query_embedding = model.encode(query)

# Search in RustVecDB
response = requests.post(
    f"{BASE_URL}/collections/{COLLECTION_NAME}/points/search",
    json={
        "vector": query_embedding.tolist(),
        "limit": 3
    }
)

# Display results
print("\nTop 3 similar documents:")
results = response.json()
for i, result in enumerate(results, 1):
    print(f"\n{i}. Score: {result['score']:.4f}")
    print(f"   Text: {result['payload']['text']}")
```

### Expected Output

```
Loading embedding model...
Creating collection...
Generating embeddings...
Storing embeddings in database...
Stored 5 documents

Searching for: 'What is artificial intelligence?'

Top 3 similar documents:

1. Score: 0.8234
   Text: Machine learning enables computers to learn from data

2. Score: 0.7891
   Text: Neural networks are inspired by the human brain

3. Score: 0.4123
   Text: Python is a high-level programming language
```

Notice how the search found relevant documents about AI/ML, even though the exact words "artificial intelligence" weren't in the documents!

## Best Practices

### 1. Choose the Right Model

- **For speed**: Use smaller models like `all-MiniLM-L6-v2` (384 dimensions)
- **For quality**: Use larger models like `all-mpnet-base-v2` (768 dimensions)
- **For multiple languages**: Use multilingual models
- **For specific domains**: Use domain-specific models (legal, medical, code, etc.)

### 2. Consistent Model Usage

⚠️ **Important**: Always use the same model for generating embeddings that you use for queries!

```python
# WRONG: Using different models
model1 = SentenceTransformer('all-MiniLM-L6-v2')
embeddings = model1.encode(documents)  # 384 dimensions

model2 = SentenceTransformer('all-mpnet-base-v2')
query_embedding = model2.encode(query)  # 768 dimensions - Won't match!

# RIGHT: Using the same model
model = SentenceTransformer('all-MiniLM-L6-v2')
embeddings = model.encode(documents)  # 384 dimensions
query_embedding = model.encode(query)  # 384 dimensions - Perfect!
```

### 3. Batch Processing

Generate embeddings in batches for better performance:

```python
# Good: Process in batches
documents = ["doc1", "doc2", ..., "doc10000"]
embeddings = model.encode(documents, batch_size=32)

# Less efficient: One at a time
embeddings = [model.encode(doc) for doc in documents]
```

### 4. Normalize Vectors for Cosine Similarity

When using cosine similarity, normalized vectors give the same results but compute faster:

```python
import numpy as np

# Normalize embeddings
embeddings = model.encode(documents)
normalized = embeddings / np.linalg.norm(embeddings, axis=1, keepdims=True)
```

### 5. Choose the Right Distance Metric

| Metric | Best For | Notes |
|--------|----------|-------|
| **Cosine** | Text embeddings | Most common, handles different vector magnitudes |
| **Euclidean** | Spatial data, normalized vectors | Measures absolute distance |
| **Dot Product** | Pre-normalized vectors | Fastest, equivalent to cosine for normalized vectors |

### 6. Cache Embeddings

Generating embeddings can be slow. Cache them to avoid recomputation:

```python
import pickle

# Save embeddings
with open('embeddings_cache.pkl', 'wb') as f:
    pickle.dump(embeddings, f)

# Load embeddings
with open('embeddings_cache.pkl', 'rb') as f:
    embeddings = pickle.load(f)
```

## Common Questions

### Q: How do I choose the embedding dimension?

**A**: The dimension is determined by the model you choose. Common sizes:
- 384 dimensions: Fast, good for most use cases
- 768 dimensions: Better quality, slower
- 1536 dimensions: Very high quality (OpenAI), slowest

### Q: Can I reduce the dimension of embeddings?

**A**: Yes, using dimensionality reduction (PCA, UMAP), but you'll lose some semantic information:

```python
from sklearn.decomposition import PCA

# Reduce from 384 to 128 dimensions
pca = PCA(n_components=128)
reduced_embeddings = pca.fit_transform(embeddings)
```

### Q: How do I handle long documents?

**A**: Most models have a maximum token limit (usually 512 tokens). For long documents:
1. Split into chunks and embed separately
2. Use models with longer context windows
3. Summarize first, then embed the summary

### Q: Which embedding model is best?

**A**: It depends on your use case:
- **General text**: `all-mpnet-base-v2` or `all-MiniLM-L6-v2`
- **Questions & Answers**: `multi-qa-MiniLM-L6-cos-v1`
- **Code search**: `huggingface/CodeBERTa-small-v1`
- **Multilingual**: `paraphrase-multilingual-MiniLM-L12-v2`

Test with your specific data to find the best fit!

## Additional Resources

### Libraries and Tools
- [Sentence Transformers](https://www.sbert.net/) - Easy-to-use embeddings
- [Hugging Face Hub](https://huggingface.co/models) - Browse thousands of models
- [OpenAI Embeddings](https://platform.openai.com/docs/guides/embeddings) - API documentation
- [Cohere Embeddings](https://docs.cohere.com/docs/embeddings) - Multilingual embeddings

### Learning Resources
- [What are Word Embeddings?](https://colah.github.io/posts/2014-07-NLP-RNNs-Representations/)
- [Sentence Embeddings Explained](https://www.sbert.net/examples/applications/computing-embeddings/README.html)
- [OpenAI Embeddings Guide](https://platform.openai.com/docs/guides/embeddings/what-are-embeddings)

### Benchmarks
- [MTEB Leaderboard](https://huggingface.co/spaces/mteb/leaderboard) - Compare embedding models

## Summary

1. **Embeddings** convert text into numerical vectors that capture semantic meaning
2. **The process**: Text → Tokens → IDs → Vectors
3. **Tools**: Use Sentence Transformers (local) or OpenAI/Cohere (API) to generate embeddings
4. **Usage**: Generate embeddings for your data, store in RustVecDB, search by converting queries to embeddings
5. **Best practices**: Use the same model consistently, batch process, and choose the right distance metric

With embeddings, you can build powerful semantic search systems, recommendation engines, and more! 🚀
