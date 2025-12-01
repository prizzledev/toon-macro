//! TOON table encoding and decoding.
//!
//! This module provides the [`ToonTable`] trait for converting between
//! slices of Rust structs and TOON table representations.
//!
//! TOON tables are a compact, columnar representation of tabular data
//! that significantly reduces token usage compared to arrays of objects.
//!
//! # Example
//!
//! ```ignore
//! use toon_macro::ToonTable;
//!
//! #[derive(ToonTable)]
//! struct User {
//!     id: u64,
//!     name: String,
//!     role: String,
//! }
//!
//! let users = vec![
//!     User { id: 1, name: "Alice".into(), role: "admin".into() },
//!     User { id: 2, name: "Bob".into(), role: "user".into() },
//! ];
//!
//! // Encode to TOON table format
//! let table_value = User::to_toon_table(&users);
//!
//! // Decode back to structs
//! let decoded: Vec<User> = User::from_toon_table(&table_value).unwrap();
//! ```

use crate::{Error, Result, Value};

/// A trait for types that can be encoded as TOON tables.
///
/// TOON tables provide a compact, columnar representation for arrays
/// of similar objects. Instead of repeating keys for each object,
/// the keys are specified once as column headers.
///
/// # Implementing Manually
///
/// While you can implement this trait manually, it's recommended to use
/// the `#[derive(ToonTable)]` macro (requires the `derive` feature).
///
/// # Table Format
///
/// A TOON table is represented as an object with:
/// - `columns`: An array of column names
/// - `rows`: An array of row arrays, where each row contains values
///   in the same order as the columns
///
/// ```text
/// columns: ["id", "name", "role"]
/// rows:
///   - [1, "Alice", "admin"]
///   - [2, "Bob", "user"]
/// ```
pub trait ToonTable: Sized {
    /// The column names for this table type.
    const COLUMNS: &'static [&'static str];

    /// Encode a slice of structs into a TOON table value.
    ///
    /// # Arguments
    ///
    /// * `rows` - The structs to encode
    ///
    /// # Returns
    ///
    /// A [`Value`] representing the table in TOON format.
    fn to_toon_table(rows: &[Self]) -> Value;

    /// Decode a TOON table value into a vector of structs.
    ///
    /// # Arguments
    ///
    /// * `value` - The TOON table value to decode
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The value is not a valid table structure
    /// - Required columns are missing
    /// - Values cannot be converted to the expected types
    fn from_toon_table(value: &Value) -> Result<Vec<Self>>;

    /// Get a single row from a TOON table by index.
    ///
    /// # Arguments
    ///
    /// * `value` - The TOON table value
    /// * `index` - The row index (0-based)
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds or decoding fails.
    fn get_row(value: &Value, index: usize) -> Result<Self> {
        let rows = Self::from_toon_table(value)?;
        let len = rows.len();
        rows.into_iter()
            .nth(index)
            .ok_or(Error::RowOutOfBounds { index, len })
    }
}

/// Encode a slice of [`ToonTable`] items into a TOON table value.
///
/// This is a convenience function that calls [`ToonTable::to_toon_table`].
///
/// # Examples
///
/// ```ignore
/// use toon_macro::{ToonTable, table::encode_table};
///
/// #[derive(ToonTable)]
/// struct Item { id: u64, name: String }
///
/// let items = vec![
///     Item { id: 1, name: "One".into() },
///     Item { id: 2, name: "Two".into() },
/// ];
///
/// let table = encode_table(&items);
/// ```
pub fn encode_table<T: ToonTable>(rows: &[T]) -> Value {
    T::to_toon_table(rows)
}

/// Decode a TOON table value into a vector of [`ToonTable`] items.
///
/// This is a convenience function that calls [`ToonTable::from_toon_table`].
///
/// # Examples
///
/// ```ignore
/// use toon_macro::{ToonTable, table::decode_table};
///
/// #[derive(ToonTable)]
/// struct Item { id: u64, name: String }
///
/// // Assuming `table_value` is a valid TOON table
/// let items: Vec<Item> = decode_table(&table_value).unwrap();
/// ```
pub fn decode_table<T: ToonTable>(value: &Value) -> Result<Vec<T>> {
    T::from_toon_table(value)
}

/// Helper to extract columns array from a table value.
pub fn extract_columns(value: &Value) -> Result<Vec<String>> {
    match value {
        Value::Object(map) => {
            let columns = map
                .get("columns")
                .ok_or_else(|| Error::InvalidTable("missing 'columns' field".into()))?;

            match columns {
                Value::Array(arr) => arr
                    .iter()
                    .map(|v| match v {
                        Value::String(s) => Ok(s.clone()),
                        _ => Err(Error::InvalidTable("column names must be strings".into())),
                    })
                    .collect(),
                _ => Err(Error::InvalidTable("'columns' must be an array".into())),
            }
        }
        _ => Err(Error::InvalidTable("table must be an object".into())),
    }
}

/// Helper to extract rows array from a table value.
pub fn extract_rows(value: &Value) -> Result<&Vec<Value>> {
    match value {
        Value::Object(map) => {
            let rows = map
                .get("rows")
                .ok_or_else(|| Error::InvalidTable("missing 'rows' field".into()))?;

            match rows {
                Value::Array(arr) => Ok(arr),
                _ => Err(Error::InvalidTable("'rows' must be an array".into())),
            }
        }
        _ => Err(Error::InvalidTable("table must be an object".into())),
    }
}

/// Helper to get a value from a row by column index.
pub fn get_cell(row: &Value, index: usize) -> Result<&Value> {
    match row {
        Value::Array(arr) => {
            let len = arr.len();
            arr.get(index)
                .ok_or(Error::ColumnOutOfBounds { index, len })
        }
        _ => Err(Error::InvalidTable("row must be an array".into())),
    }
}

/// Helper to convert a Value to a specific type.
pub trait FromToonValue: Sized {
    /// Convert a TOON value to this type.
    fn from_toon_value(value: &Value) -> Result<Self>;
}

impl FromToonValue for String {
    fn from_toon_value(value: &Value) -> Result<Self> {
        match value {
            Value::String(s) => Ok(s.clone()),
            Value::Null => Ok(String::new()),
            _ => Err(Error::invalid_type("string", value)),
        }
    }
}

impl FromToonValue for i64 {
    fn from_toon_value(value: &Value) -> Result<Self> {
        match value {
            Value::Number(n) => n
                .as_i64()
                .ok_or_else(|| Error::ConversionError("number is not an i64".into())),
            _ => Err(Error::invalid_type("i64", value)),
        }
    }
}

impl FromToonValue for u64 {
    fn from_toon_value(value: &Value) -> Result<Self> {
        match value {
            Value::Number(n) => n
                .as_u64()
                .ok_or_else(|| Error::ConversionError("number is not a u64".into())),
            _ => Err(Error::invalid_type("u64", value)),
        }
    }
}

impl FromToonValue for f64 {
    fn from_toon_value(value: &Value) -> Result<Self> {
        match value {
            Value::Number(n) => Ok(n.as_f64()),
            _ => Err(Error::invalid_type("f64", value)),
        }
    }
}

impl FromToonValue for bool {
    fn from_toon_value(value: &Value) -> Result<Self> {
        match value {
            Value::Bool(b) => Ok(*b),
            _ => Err(Error::invalid_type("bool", value)),
        }
    }
}

impl<T: FromToonValue> FromToonValue for Option<T> {
    fn from_toon_value(value: &Value) -> Result<Self> {
        match value {
            Value::Null => Ok(None),
            _ => T::from_toon_value(value).map(Some),
        }
    }
}

/// Helper to convert a type to a TOON Value for table cells.
pub trait IntoToonValue {
    /// Convert this value to a TOON Value.
    fn to_toon_value(&self) -> Value;
}

impl IntoToonValue for String {
    fn to_toon_value(&self) -> Value {
        Value::String(self.clone())
    }
}

impl IntoToonValue for &str {
    fn to_toon_value(&self) -> Value {
        Value::String((*self).to_string())
    }
}

impl IntoToonValue for i64 {
    fn to_toon_value(&self) -> Value {
        Value::from(*self)
    }
}

impl IntoToonValue for u64 {
    fn to_toon_value(&self) -> Value {
        Value::from(*self)
    }
}

impl IntoToonValue for i32 {
    fn to_toon_value(&self) -> Value {
        Value::from(*self as i64)
    }
}

impl IntoToonValue for u32 {
    fn to_toon_value(&self) -> Value {
        Value::from(*self as u64)
    }
}

impl IntoToonValue for f64 {
    fn to_toon_value(&self) -> Value {
        Value::from(*self)
    }
}

impl IntoToonValue for bool {
    fn to_toon_value(&self) -> Value {
        Value::Bool(*self)
    }
}

impl<T: IntoToonValue> IntoToonValue for Option<T> {
    fn to_toon_value(&self) -> Value {
        match self {
            Some(v) => v.to_toon_value(),
            None => Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toon;

    #[test]
    fn test_extract_columns() {
        let table = toon!({
            columns: ["id", "name"],
            rows: [[1, "Alice"]]
        });

        let cols = extract_columns(&table).unwrap();
        assert_eq!(cols, vec!["id", "name"]);
    }

    #[test]
    fn test_extract_rows() {
        let table = toon!({
            columns: ["id", "name"],
            rows: [
                [1, "Alice"],
                [2, "Bob"]
            ]
        });

        let rows = extract_rows(&table).unwrap();
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn test_from_toon_value() {
        let v = Value::String("hello".into());
        assert_eq!(String::from_toon_value(&v).unwrap(), "hello");

        let v = Value::Bool(true);
        assert!(bool::from_toon_value(&v).unwrap());

        let v = Value::Null;
        assert_eq!(Option::<String>::from_toon_value(&v).unwrap(), None);
    }

    #[test]
    fn test_to_toon_value() {
        assert_eq!("hello".to_toon_value(), Value::String("hello".into()));
        assert_eq!(true.to_toon_value(), Value::Bool(true));
        assert_eq!(Option::<String>::None.to_toon_value(), Value::Null);
    }
}
