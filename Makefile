.PHONY: help build run test clean dev release example

# Default target
help:
	@echo "RustVecDB - Makefile Commands"
	@echo ""
	@echo "Available targets:"
	@echo "  make build      - Build the project in debug mode"
	@echo "  make release    - Build the project in release mode"
	@echo "  make run        - Run the server in debug mode"
	@echo "  make dev        - Run the server with persistent storage"
	@echo "  make test       - Run all tests"
	@echo "  make example    - Run the basic usage example"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make check      - Run cargo check and clippy"
	@echo ""

# Build in debug mode
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Run the server
run:
	RUST_LOG=info cargo run

# Run with persistent storage
dev:
	mkdir -p ./data
	RUST_LOG=info STORAGE_PATH=./data/storage.json cargo run

# Run tests
test:
	cargo test

# Run example
example:
	cargo run --example basic_usage

# Clean build artifacts
clean:
	cargo clean
	rm -rf ./data

# Check code quality
check:
	cargo check
	cargo clippy -- -D warnings

# Format code
fmt:
	cargo fmt

# Run the Python example (requires server to be running)
py-example:
	python3 examples/api_client.py

