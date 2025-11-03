#!/usr/bin/env python3
"""
Example Python client for RustVecDB REST API
Requires: pip install requests
Run: python examples/api_client.py
"""

import requests
import json
import uuid

BASE_URL = "http://localhost:6333"

def print_response(title, response):
    """Pretty print API response"""
    print(f"\n{'='*60}")
    print(f"{title}")
    print(f"{'='*60}")
    print(f"Status: {response.status_code}")
    try:
        print(json.dumps(response.json(), indent=2))
    except:
        print(response.text)

def main():
    print("RustVecDB Python Client Example")
    
    # 1. Health check
    response = requests.get(f"{BASE_URL}/health")
    print_response("1. Health Check", response)
    
    # 2. Create a collection
    response = requests.put(
        f"{BASE_URL}/collections/products",
        json={"name": "products", "distance": "Cosine"}
    )
    print_response("2. Create Collection", response)
    
    # 3. Insert products with embeddings
    products = [
        {
            "vector": [0.1, 0.2, 0.3, 0.4],
            "payload": {
                "name": "Laptop",
                "category": "electronics",
                "price": 999.99
            }
        },
        {
            "vector": [0.15, 0.25, 0.35, 0.45],
            "payload": {
                "name": "Smartphone",
                "category": "electronics",
                "price": 699.99
            }
        },
        {
            "vector": [0.8, 0.1, 0.2, 0.3],
            "payload": {
                "name": "Novel",
                "category": "books",
                "price": 14.99
            }
        },
        {
            "vector": [0.12, 0.22, 0.32, 0.42],
            "payload": {
                "name": "Tablet",
                "category": "electronics",
                "price": 499.99
            }
        }
    ]
    
    response = requests.post(
        f"{BASE_URL}/collections/products/points",
        json=products
    )
    print_response("3. Batch Insert Products", response)
    
    # 4. Get collection info
    response = requests.get(f"{BASE_URL}/collections/products")
    print_response("4. Collection Info", response)
    
    # 5. Search for similar products
    response = requests.post(
        f"{BASE_URL}/collections/products/points/search",
        json={
            "vector": [0.1, 0.2, 0.3, 0.4],  # Similar to Laptop
            "limit": 3
        }
    )
    print_response("5. Search Similar Products", response)
    
    # 6. Search with filter (only electronics)
    response = requests.post(
        f"{BASE_URL}/collections/products/points/search",
        json={
            "vector": [0.1, 0.2, 0.3, 0.4],
            "limit": 3,
            "filter": {"category": "electronics"}
        }
    )
    print_response("6. Search Electronics Only", response)
    
    # 7. List all collections
    response = requests.get(f"{BASE_URL}/collections")
    print_response("7. List Collections", response)
    
    # 8. Get storage stats
    response = requests.get(f"{BASE_URL}/stats")
    print_response("8. Storage Stats", response)
    
    # 9. Get all points in collection
    response = requests.get(f"{BASE_URL}/collections/products/points")
    print_response("9. All Points in Collection", response)
    
    print("\n" + "="*60)
    print("Example completed successfully!")
    print("="*60)

if __name__ == "__main__":
    try:
        main()
    except requests.exceptions.ConnectionError:
        print("\n❌ Error: Could not connect to RustVecDB server")
        print("Make sure the server is running: cargo run")
    except Exception as e:
        print(f"\n❌ Error: {e}")

