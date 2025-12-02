# toon-macro

[![Crates.io](https://img.shields.io/crates/v/toon-macro.svg)](https://crates.io/crates/toon-macro)
[![Documentation](https://docs.rs/toon-macro/badge.svg)](https://docs.rs/toon-macro)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

Ergonomic macros for constructing and parsing TOON (Token-Oriented Object Notation) values in Rust.

TOON is a compact data format designed to convey the same information as JSON with 30-60% fewer tokens, making it ideal for LLM prompts and responses.

## Features

- **`toon!` macro**: JSON-like Rust DSL for constructing TOON values
- **`toon_str!` macro**: Parse TOON-format strings at runtime
- **`ToonTable` trait**: Encode/decode tabular data efficiently
- **`#[derive(ToonTable)]`**: Automatic table serialization (with `derive` feature)
- **Full serde integration**: Serialize any serde type to TOON

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
toon-macro = "0.1"
```

With derive macro support:

```toml
[dependencies]
toon-macro = { version = "0.1", features = ["derive"] }
```

## Quick Start

### Using `toon!` (Rust-DSL)

The `toon!` macro provides a JSON-like syntax for constructing TOON values:

```rust
use toon_macro::{toon, Value};

// Simple object
let user = toon!({
    name: "Alice",
    age: 30,
    active: true
});

// Nested structures
let data = toon!({
    config: {
        host: "localhost",
        port: 8080
    },
    users: [
        { id: 1, name: "Alice" },
        { id: 2, name: "Bob" }
    ]
});

// Using variables
let name = "Charlie";
let score = 95i64;
let result = toon!({
    name: name,
    score: score
});
```

### Using `toon_str!` (TOON syntax)

Parse TOON-format text at runtime:

```rust
use toon_macro::toon_str;

let config = toon_str!(r#"
host: "localhost"
port: 5432
active: true
"#);
```

### Error Handling with `from_toon_str`

For fallible parsing, use `from_toon_str` directly:

```rust
use toon_macro::from_toon_str;

let input = r#"name: "Alice""#;
match from_toon_str(input) {
    Ok(value) => println!("Parsed: {:?}", value),
    Err(e) => eprintln!("Parse error: {}", e),
}
```

### Using `ToonTable` for Tabular Data

With the `derive` feature, you can efficiently encode/decode collections of structs:

```rust
use toon_macro::{ToonTable, toon};

#[derive(ToonTable)]
struct User {
    id: u64,
    name: String,
    #[toon(rename = "user_role")]
    role: String,
}

let users = vec![
    User { id: 1, name: "Alice".into(), role: "admin".into() },
    User { id: 2, name: "Bob".into(), role: "user".into() },
];

// Encode to compact table format
let table = User::to_toon_table(&users);

// Decode back to structs
let decoded = User::from_toon_table(&table).unwrap();
```

## ToonTable Attributes

When using `#[derive(ToonTable)]`, you can customize field behavior:

- `#[toon(rename = "column_name")]` - Use a custom column name
- `#[toon(skip)]` - Exclude this field from the table
- `#[toon(default)]` - Use `Default::default()` when the column is missing
- `#[toon(order = N)]` - Specify explicit column ordering (0-based)

## Serde Integration

Serialize and deserialize any serde-compatible type:

```rust
use toon_macro::{serialize, deserialize};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 10, y: 20 };
let toon_string = serialize(&point).unwrap();
let decoded: Point = deserialize(&toon_string).unwrap();
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `serde` | Yes | Enable serde integration |
| `derive` | No | Enable `#[derive(ToonTable)]` macro |
| `pretty` | No | Enable pretty-printing functions |

## Why TOON?

TOON (Token-Oriented Object Notation) is designed to be:

- **Token-efficient**: 30-60% fewer tokens than JSON
- **Human-readable**: Easy to read and write
- **JSON-compatible**: Same data model as JSON
- **LLM-friendly**: Optimized for AI model input/output

## Minimum Supported Rust Version

This crate requires Rust 1.85 or later (due to `serde_toon2` using Rust 2024 edition).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

Built on top of [serde_toon2](https://crates.io/crates/serde_toon2) for TOON parsing and serialization.
