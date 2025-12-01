//! Procedural macros for the toon-macro crate.
//!
//! This crate provides derive macros for TOON table serialization.
//! It is not intended to be used directly; instead, use the `toon-macro` crate
//! with the `derive` feature enabled.
//!
//! # Derive Macros
//!
//! ## `ToonTable`
//!
//! Derives the `ToonTable` trait for a struct, enabling efficient
//! table-based serialization.
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
//!     #[toon(skip)]
//!     internal_data: Vec<u8>,
//!     #[toon(default)]
//!     optional_field: Option<String>,
//! }
//! ```
//!
//! ## Attributes
//!
//! - `#[toon(rename = "...")]` - Use a different column name
//! - `#[toon(skip)]` - Skip this field in table encoding/decoding
//! - `#[toon(default)]` - Use `Default::default()` if column is missing
//! - `#[toon(order = N)]` - Explicit column order (0-based)

extern crate proc_macro;

mod table_derive;
mod utils;

use proc_macro::TokenStream;

/// Derive the `ToonTable` trait for a struct.
///
/// This enables efficient table-based serialization where column names
/// are specified once, significantly reducing token count for arrays
/// of similar objects.
///
/// # Example
///
/// ```ignore
/// use toon_macro::ToonTable;
///
/// #[derive(ToonTable)]
/// struct User {
///     id: u64,
///     name: String,
///     role: String,
/// }
///
/// let users = vec![
///     User { id: 1, name: "Alice".into(), role: "admin".into() },
///     User { id: 2, name: "Bob".into(), role: "user".into() },
/// ];
///
/// // Encodes to:
/// // columns: ["id", "name", "role"]
/// // rows:
/// //   - [1, "Alice", "admin"]
/// //   - [2, "Bob", "user"]
/// let table = User::to_toon_table(&users);
/// ```
///
/// # Attributes
///
/// ## Field Attributes
///
/// - `#[toon(rename = "column_name")]` - Use a custom column name
/// - `#[toon(skip)]` - Exclude this field from the table
/// - `#[toon(default)]` - Use `Default::default()` when the column is missing
/// - `#[toon(order = N)]` - Specify explicit column ordering (0-based)
///
/// # Supported Types
///
/// The following types are supported for table fields:
///
/// - `String`, `&str`
/// - `i32`, `i64`, `u32`, `u64`
/// - `f64`
/// - `bool`
/// - `Option<T>` where `T` is a supported type
///
/// For other types, implement `FromToonValue` and `IntoToonValue` manually.
#[proc_macro_derive(ToonTable, attributes(toon))]
pub fn derive_toon_table(input: TokenStream) -> TokenStream {
    table_derive::derive_toon_table(input)
}
