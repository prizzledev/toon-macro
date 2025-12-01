//! # toon-macro
//!
//! Ergonomic macros for constructing and parsing TOON (Token-Oriented Object Notation) values.
//!
//! TOON is a compact data format designed to convey the same information as JSON
//! with 30-60% fewer tokens, making it ideal for LLM prompts and responses.
//!
//! ## Features
//!
//! - **`toon!` macro**: JSON-like Rust DSL for constructing TOON values
//! - **`toon_str!` macro**: Parse TOON-format strings at runtime
//! - **`ToonTable` trait**: Encode/decode tabular data efficiently
//! - **`#[derive(ToonTable)]`**: Automatic table serialization (requires `derive` feature)
//!
//! ## Quick Start
//!
//! ### Using `toon!` (Rust-DSL)
//!
//! The `toon!` macro provides a JSON-like syntax for constructing TOON values:
//!
//! ```
//! use toon_macro::{toon, Value};
//!
//! // Simple object
//! let user = toon!({
//!     name: "Alice",
//!     age: 30,
//!     active: true
//! });
//!
//! // Nested structures
//! let data = toon!({
//!     config: {
//!         host: "localhost",
//!         port: 8080
//!     },
//!     users: [
//!         { id: 1, name: "Alice" },
//!         { id: 2, name: "Bob" }
//!     ]
//! });
//!
//! // Using variables
//! let name = "Charlie";
//! let score = 95i64;
//! let result = toon!({
//!     name: name,
//!     score: score
//! });
//! ```
//!
//! ### Using `toon_str!` (TOON syntax)
//!
//! The `toon_str!` macro parses TOON-format text at runtime:
//!
//! ```
//! use toon_macro::toon_str;
//!
//! let config = toon_str!(r#"
//! host: "localhost"
//! port: 5432
//! active: true
//! "#);
//! ```
//!
//! ### Using `from_toon_str` (with error handling)
//!
//! For fallible parsing, use `from_toon_str` directly:
//!
//! ```
//! use toon_macro::from_toon_str;
//!
//! let input = r#"name: "Alice""#;
//! match from_toon_str(input) {
//!     Ok(value) => println!("Parsed: {:?}", value),
//!     Err(e) => eprintln!("Parse error: {}", e),
//! }
//! ```
//!
//! ### Using `ToonTable` (requires `derive` feature)
//!
//! ```ignore
//! use toon_macro::ToonTable;
//!
//! #[derive(ToonTable)]
//! struct User {
//!     id: u64,
//!     name: String,
//!     #[toon(rename = "user_role")]
//!     role: String,
//! }
//!
//! let users = vec![
//!     User { id: 1, name: "Alice".into(), role: "admin".into() },
//!     User { id: 2, name: "Bob".into(), role: "user".into() },
//! ];
//!
//! // Encode to compact table format
//! let table = User::to_toon_table(&users);
//!
//! // Decode back to structs
//! let decoded = User::from_toon_table(&table).unwrap();
//! ```
//!
//! ## Feature Flags
//!
//! - `serde` (default): Enable serde integration for serializing arbitrary types
//! - `derive`: Enable `#[derive(ToonTable)]` macro
//! - `pretty`: Enable pretty-printing functions
//!
//! ## Minimum Supported Rust Version
//!
//! This crate requires Rust 1.70 or later.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(unsafe_code)]

// Declare modules - macros must come first so they can be used in other modules
#[macro_use]
pub mod internal;
#[macro_use]
pub mod macros;

pub mod error;
pub mod ser;
pub mod table;
pub mod value;

// Re-export core types
pub use error::{Error, Result};
pub use ser::{from_toon_str, to_toon_string};
pub use value::Value;

// Re-export the ToonTable trait (always available)
// When the derive feature is enabled, the derive macro is also re-exported
// with the same name, which is the standard pattern for derive macros.
#[cfg(not(feature = "derive"))]
pub use table::ToonTable;

#[cfg(feature = "derive")]
pub use table::ToonTable;

// Conditionally re-export derive macro
#[cfg(feature = "derive")]
pub use toon_macro_derive::ToonTable;

// Re-export serde_toon2 types that users might need
pub use serde_toon2::{Map, Number};

// Re-export pretty printing if enabled
#[cfg(feature = "pretty")]
pub use ser::to_toon_string_pretty;

// Re-export serde helpers if serde feature is enabled
#[cfg(feature = "serde")]
pub use ser::{deserialize, serialize};

#[cfg(feature = "serde")]
pub use value::{from_value, to_value};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toon_macro_basic() {
        let v = toon!({
            name: "test",
            value: 42
        });

        assert!(matches!(v, Value::Object(_)));
    }

    #[test]
    fn test_toon_str_basic() {
        let v = toon_str!(r#"name: "test""#);
        assert!(matches!(v, Value::Object(_)));
    }

    #[test]
    fn test_from_toon_str_error_handling() {
        // Test that we can handle errors gracefully
        let result = from_toon_str("name: \"valid\"");
        assert!(result.is_ok());
    }

    #[test]
    fn test_to_toon_string() {
        let v = toon!({ key: "value" });
        let s = to_toon_string(&v).unwrap();
        assert!(s.contains("key"));
        assert!(s.contains("value"));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_roundtrip() {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Point {
            x: i64,
            y: i64,
        }

        let point = Point { x: 10, y: 20 };
        let serialized = serialize(&point).unwrap();
        let deserialized: Point = deserialize(&serialized).unwrap();
        assert_eq!(point, deserialized);
    }
}
