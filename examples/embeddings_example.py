#!/usr/bin/env python3
"""
Complete example: Text to Vectors - Semantic Search System

This example demonstrates:
1. How to convert text (words/sentences) into numerical vectors (embeddings)
2. How to store these embeddings in RustVecDB
3. How to perform semantic search

Requirements:
    pip install sentence-transformers requests

Usage:
    # Start the RustVecDB server first:
    cargo run

    # Then run this script:
    python examples/embeddings_example.py
"""

import requests
from sentence_transformers import SentenceTransformer
import sys
from numpy import dot
from numpy.linalg import norm

# Configuration
BASE_URL = "http://localhost:6333"
COLLECTION_NAME = "semantic_search"

def print_section(title):
    """Print a formatted section header"""
    print(f"\n{'='*70}")
    print(f"  {title}")
    print(f"{'='*70}\n")

def check_server():
    """Check if RustVecDB server is running"""
    try:
        response = requests.get(f"{BASE_URL}/health", timeout=2)
        return response.status_code == 200
    except Exception:
        return False

def main():
    print("🚀 Text to Vectors: Complete Example")
    print("This demonstrates how words are converted to numbers and vectors\n")

    # Check server
    if not check_server():
        print("❌ Error: RustVecDB server is not running!")
        print("Please start it first: cargo run")
        sys.exit(1)
    print("✅ RustVecDB server is running\n")

    # ============================================================================
    # STEP 1: Understanding the Model
    # ============================================================================
    print_section("STEP 1: Loading the Embedding Model")
    
    print("We're loading 'all-MiniLM-L6-v2' - a neural network trained to")
    print("convert text into 384-dimensional vectors that capture meaning.\n")
    
    model = SentenceTransformer('all-MiniLM-L6-v2')
    print("✅ Model loaded successfully!")
    print(f"   - Model name: all-MiniLM-L6-v2")
    print(f"   - Output dimensions: 384")
    print(f"   - Max sequence length: 256 tokens")

    # ============================================================================
    # STEP 2: Converting Text to Vectors
    # ============================================================================
    print_section("STEP 2: Converting Text to Vectors (Embeddings)")
    
    # Example texts
    example_texts = [
        "The cat sits on the mat",
        "A dog plays in the park"
    ]
    
    print("Let's convert these two sentences into vectors:\n")
    for i, text in enumerate(example_texts, 1):
        print(f"{i}. \"{text}\"")
    
    # Generate embeddings
    print("\nConverting text to embeddings...")
    embeddings = model.encode(example_texts)
    
    print(f"\n✅ Conversion complete!")
    print(f"   - Input: {len(example_texts)} sentences (text)")
    print(f"   - Output: {embeddings.shape[0]} vectors")
    print(f"   - Each vector has {embeddings.shape[1]} dimensions")
    
    print(f"\nExample - First sentence as a vector:")
    print(f"   First 10 values: {embeddings[0][:10]}")
    print(f"   ... (374 more values) ...")
    print(f"\n   This 384-dimensional vector now represents the meaning of:")
    print(f"   \"{example_texts[0]}\"")

    # ============================================================================
    # STEP 3: Understanding How Similar Words Have Similar Vectors
    # ============================================================================
    print_section("STEP 3: Similar Meanings → Similar Vectors")
    
    test_words = [
        "cat",      # Animal
        "dog",      # Animal (similar to cat)
        "kitten",   # Baby cat (very similar to cat)
        "car",      # Vehicle (different from cat)
        "computer"  # Electronics (very different from cat)
    ]
    
    print("Let's see how similarity works. Embedding these words:")
    for word in test_words:
        print(f"  - {word}")
    
    word_embeddings = model.encode(test_words)
    
    # Calculate similarity with "cat"
    def cosine_similarity(a, b):
        return dot(a, b) / (norm(a) * norm(b))
    
    cat_embedding = word_embeddings[0]
    
    print(f"\nSimilarity scores compared to 'cat':")
    print(f"{'Word':<12} {'Similarity Score':<20} {'Interpretation'}")
    print("-" * 60)
    
    for word, embedding in zip(test_words, word_embeddings):
        similarity = cosine_similarity(cat_embedding, embedding)
        if word == "cat":
            interpretation = "(itself - perfect match!)"
        elif similarity > 0.6:
            interpretation = "(very similar meaning)"
        elif similarity > 0.3:
            interpretation = "(somewhat related)"
        else:
            interpretation = "(different meaning)"
        print(f"{word:<12} {similarity:<20.4f} {interpretation}")
    
    print("\n💡 Notice: 'dog' and 'kitten' have high similarity scores because")
    print("   they share semantic meaning with 'cat' (animals, pets)")

    # ============================================================================
    # STEP 4: Creating a Collection
    # ============================================================================
    print_section("STEP 4: Creating a Collection in RustVecDB")
    
    print("Creating a collection to store our document embeddings...\n")
    
    response = requests.put(
        f"{BASE_URL}/collections/{COLLECTION_NAME}",
        json={
            "name": COLLECTION_NAME,
            "distance": "Cosine"  # Best for text embeddings
        }
    )
    
    if response.status_code == 200:
        print(f"✅ Collection '{COLLECTION_NAME}' created")
        print(f"   - Distance metric: Cosine similarity")
        print(f"   - Ready to store 384-dimensional vectors")
    else:
        print(f"⚠️  Collection might already exist, continuing...")

    # ============================================================================
    # STEP 5: Storing Documents with Embeddings
    # ============================================================================
    print_section("STEP 5: Storing Documents with Embeddings")
    
    # Sample documents about different topics
    documents = [
        # Tech/Programming
        "Python is a high-level programming language used for web development",
        "JavaScript is essential for building interactive web applications",
        "Machine learning algorithms learn patterns from data automatically",
        
        # Animals/Pets
        "The golden retriever is a friendly and loyal dog breed",
        "Cats are independent pets that enjoy lounging in sunny spots",
        "Parrots are intelligent birds that can mimic human speech",
        
        # Food
        "Pizza is an Italian dish with tomato sauce and cheese",
        "Sushi is a Japanese cuisine featuring raw fish and rice",
        "Tacos are Mexican food with meat and vegetables in tortillas",
        
        # Science
        "The solar system contains eight planets orbiting the sun",
        "DNA carries genetic information in all living organisms",
        "Photosynthesis converts sunlight into chemical energy in plants"
    ]
    
    print(f"Documents to store: {len(documents)}")
    print("\nSample documents:")
    for i, doc in enumerate(documents[:3], 1):
        print(f"  {i}. {doc[:60]}...")
    print(f"  ... and {len(documents) - 3} more")
    
    # Convert all documents to embeddings
    print("\nConverting all documents to embeddings...")
    doc_embeddings = model.encode(documents, show_progress_bar=False)
    print(f"✅ Generated {len(doc_embeddings)} embeddings")
    
    # Prepare points for RustVecDB
    points = []
    for i, (doc, embedding) in enumerate(zip(documents, doc_embeddings)):
        points.append({
            "vector": embedding.tolist(),  # Convert numpy array to list
            "payload": {
                "text": doc,
                "id": i,
                # Automatically detect category for demo purposes
                "category": (
                    "tech" if any(word in doc.lower() for word in ["python", "javascript", "machine", "programming"]) else
                    "animals" if any(word in doc.lower() for word in ["dog", "cat", "bird", "parrot", "retriever"]) else
                    "food" if any(word in doc.lower() for word in ["pizza", "sushi", "taco"]) else
                    "science"
                )
            }
        })
    
    # Store in database
    print("\nStoring embeddings in RustVecDB...")
    response = requests.post(
        f"{BASE_URL}/collections/{COLLECTION_NAME}/points",
        json=points
    )
    
    if response.status_code == 200:
        print(f"✅ Successfully stored {len(points)} documents with embeddings")
    else:
        print(f"❌ Error storing documents: {response.text}")
        sys.exit(1)

    # ============================================================================
    # STEP 6: Semantic Search - The Magic!
    # ============================================================================
    print_section("STEP 6: Semantic Search - Finding Similar Documents")
    
    # Different search queries
    queries = [
        "How do I learn to code?",
        "What are good pets for families?",
        "Tell me about space exploration"
    ]
    
    for query_num, query in enumerate(queries, 1):
        print(f"\n{'─'*70}")
        print(f"Query {query_num}: \"{query}\"")
        print(f"{'─'*70}")
        
        # Convert query to embedding
        print("\n1. Converting query to embedding...")
        query_embedding = model.encode(query)
        print(f"   ✅ Query converted to 384-dimensional vector")
        
        # Search in database
        print("2. Searching for similar documents in database...")
        response = requests.post(
            f"{BASE_URL}/collections/{COLLECTION_NAME}/points/search",
            json={
                "vector": query_embedding.tolist(),
                "limit": 3
            }
        )
        
        if response.status_code == 200:
            results = response.json()
            print(f"   ✅ Found {len(results)} most similar documents\n")
            
            print("3. Results:")
            for i, result in enumerate(results, 1):
                score = result['score']
                text = result['payload']['text']
                category = result['payload']['category']
                
                print(f"\n   Rank {i}: [Score: {score:.4f}] [{category.upper()}]")
                print(f"   {text}")
                
                # Explain the score
                if score > 0.7:
                    print(f"   → Very relevant! High semantic similarity")
                elif score > 0.5:
                    print(f"   → Moderately relevant")
                else:
                    print(f"   → Somewhat related")
        else:
            print(f"   ❌ Search failed: {response.text}")
        
        print()

    # ============================================================================
    # STEP 7: Filtered Search
    # ============================================================================
    print_section("STEP 7: Filtered Search - Combining Vectors with Metadata")
    
    query = "What should I eat today?"
    category_filter = "food"
    
    print(f"Query: \"{query}\"")
    print(f"Filter: Only show results from category '{category_filter}'\n")
    
    query_embedding = model.encode(query)
    
    response = requests.post(
        f"{BASE_URL}/collections/{COLLECTION_NAME}/points/search",
        json={
            "vector": query_embedding.tolist(),
            "limit": 3,
            "filter": {"category": category_filter}
        }
    )
    
    if response.status_code == 200:
        results = response.json()
        print(f"✅ Found {len(results)} results (filtered to '{category_filter}' only):\n")
        
        for i, result in enumerate(results, 1):
            print(f"{i}. [Score: {result['score']:.4f}]")
            print(f"   {result['payload']['text']}")
            print()

    # ============================================================================
    # Summary
    # ============================================================================
    print_section("🎉 Summary: How Words Become Vectors")
    
    print("""
The complete process we just demonstrated:

1. TEXT INPUT (Words)
   ↓
   "Python is a programming language"

2. TOKENIZATION (Breaking into pieces)
   ↓
   ["Python", "is", "a", "programming", "language"]

3. MODEL ENCODING (Neural network processing)
   ↓
   [0.234, -0.567, 0.891, 0.123, ..., -0.445]  ← 384 numbers!

4. STORAGE (Save in vector database)
   ↓
   RustVecDB stores the vector + metadata

5. SEARCH (Find similar meanings)
   ↓
   Query: "How to code?" → Vector → Find similar vectors → Results!

KEY INSIGHTS:
✅ Words with similar meanings have similar vectors
✅ Vectors enable semantic search (meaning-based, not keyword-based)
✅ Same model must be used for both storage and search
✅ Distance metrics (Cosine) measure similarity between vectors

This is the foundation of:
- Semantic search engines
- Recommendation systems
- Question answering systems
- Document similarity
- And much more!
""")

    print("="*70)
    print("Example completed successfully! 🎉")
    print("="*70)
    
    # Cleanup option
    print("\n💡 Tip: To clean up, delete the collection:")
    print(f"   curl -X DELETE {BASE_URL}/collections/{COLLECTION_NAME}")

if __name__ == "__main__":
    try:
        main()
    except ImportError as e:
        print("\n❌ Error: Missing required package")
        print("Please install: pip install sentence-transformers requests")
        print(f"\nDetails: {e}")
    except requests.exceptions.ConnectionError:
        print("\n❌ Error: Cannot connect to RustVecDB server")
        print("Please start the server first: cargo run")
    except KeyboardInterrupt:
        print("\n\n👋 Interrupted by user")
    except Exception as e:
        print(f"\n❌ Unexpected error: {e}")
        import traceback
        traceback.print_exc()
