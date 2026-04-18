# Kache

Kache is a simple in-memory cache system built to understand how real-world caching systems work internally. This project is a minimal implementation of a key-value store that supports basic cache operations. The goal is to explore core system design concepts like data storage, expiration, and eviction strategies.

![](./public/repl.jpg)

## How It Works

- Data is stored in memory using a key-value structure.
- Each entry can optionally have an expiration time.
- Expired entries are removed using a cleanup mechanism.

---

## Goals

- Understand how caching systems like Redis work internally
- Learn about memory management and performance trade-offs
- Implement core system design concepts from scratch

---

## Features
 
- **SET / GET / DELETE / EXISTS** : basic key-value operations
- **TTL expiration** : entries are automatically removed after they expire
- **LRU eviction** : when the cache is full, the least recently used entry is removed
- **CLEAN** : manually purge all expired entries at once

---

## Setup
 
```bash
git clone https://github.com/Angshuman09/kache.git
cd kache
cargo run
```

---

## Notes

This is a learning project and not intended for production use.
