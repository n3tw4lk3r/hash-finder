# hash-finder

A high-performance Rust application that finds SHA-256 hashes of consecutive integers ending with a specified number of zeros. Utilizes parallel processing for optimal performance.

## Features

- **Parallel Processing**: Automatically uses all available CPU cores
- **Real-time Results**: Outputs found hashes immediately as they are discovered
- **Configurable**: Specify number of zeros and required results

## Requirements

- Rust 1.70 or higher
- Cargo package manager

## Installation

1. Clone the repository:
```bash
git clone https://github.com/n3tw4lk3r/hash-finder
cd hash-finder
```

2. Build the project
```bash
cargo build --release
```

The optimized binary will be created at target/release/hash_finder

## Usage examples

### Find 5 hashes ending with 3 zeros
```bash
./hash_finder -N 3 -F 5
```

### Find 10 hashes ending with 4 zeros
```bash
./hash_finder -N 4 -F 10
```
