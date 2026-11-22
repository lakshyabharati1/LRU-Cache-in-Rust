# LRU Cache

This is a practice project btw, ik its basic, its js for practice.

## Overview

A simple implementation of a Least Recently Used (LRU) Cache in Rust.

The cache stores key-value pairs and maintains an ordering of recently accessed entries. When the cache reaches its capacity, the least recently used entry is evicted to make room for new entries.

## Features

- Key-value storage using `HashMap`.
- Tracks entry ordering using `LinkedList`.
- Supports insertion and retrieval.
- Supports updating existing keys.
- Automatically evicts entries when capacity is reached.
- Includes unit tests using Rust's built-in testing framework.

## Implementation

The cache is implemented using two data structures:

- **HashMap:** Stores key-value pairs for quick lookups.
- **LinkedList:** Maintains the order of keys based on recency.

## Running the Project

Run the project:

```bash
cargo run
```

Run the tests:

```bash
cargo test
```

## Tests

The project currently includes tests for:

- Basic insertion and retrieval.
- Insertion beyond capacity.
- Updating existing keys.
- Duplicate key insertion.
- Repeated retrieval.

## Future Improvements

- Update recency on every successful retrieval.
- Ensure existing keys don't create duplicate entries in the ordering list.
- Improve eviction and insertion logic.
- Explore more efficient implementations with O(1) operations.
