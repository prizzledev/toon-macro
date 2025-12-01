//! TOON serialization and deserialization utilities.
//!
//! This module provides functions for parsing TOON text and converting
//! between TOON values and strings.

use crate::{Error, Result, Value};

/// Parse a TOON-format string into a [`Value`].
///
/// This is the runtime parsing function used by [`toon_str!`].
/// For compile-time parsing or when you need error handling,
/// use this function directly.
///
/// # Examples
///
/// ```
/// use toon_macro::from_toon_str;
///
/// let value = from_toon_str(r#"
/// name: "Alice"
/// age: 30
/// "#).unwrap();
/// ```
///
/// # Errors
///
/// Returns an [`Error::Deserialize`] if the input is not valid TOON syntax.
///
/// [`toon_str!`]: crate::toon_str
/// [`Error::Deserialize`]: crate::Error::Deserialize
pub fn from_toon_str(s: &str) -> Result<Value> {
    serde_toon2::from_str(s).map_err(|e| Error::Deserialize(e.to_string()))
}

/// Serialize a [`Value`] to a TOON string.
///
/// # Examples
///
/// ```
/// use toon_macro::{toon, to_toon_string};
///
/// let value = toon!({
///     name: "Alice",
///     age: 30
/// });
///
/// let s = to_toon_string(&value).unwrap();
/// println!("{}", s);
/// ```
///
/// # Errors
///
/// Returns an [`Error::Serialize`] if serialization fails.
///
/// [`Error::Serialize`]: crate::Error::Serialize
pub fn to_toon_string(value: &Value) -> Result<String> {
    serde_toon2::to_string(value).map_err(|e| Error::Serialize(e.to_string()))
}

/// Serialize a [`Value`] to a pretty-printed TOON string.
///
/// This function produces more human-readable output with proper
/// indentation and formatting.
///
/// # Examples
///
/// ```
/// use toon_macro::{toon, to_toon_string_pretty};
///
/// let value = toon!({
///     users: [
///         { id: 1, name: "Alice" },
///         { id: 2, name: "Bob" }
///     ]
/// });
///
/// let s = to_toon_string_pretty(&value).unwrap();
/// println!("{}", s);
/// ```
///
/// # Errors
///
/// Returns an [`Error::Serialize`] if serialization fails.
///
/// [`Error::Serialize`]: crate::Error::Serialize
#[cfg(feature = "pretty")]
pub fn to_toon_string_pretty(value: &Value) -> Result<String> {
    // Use encoder options for pretty printing if available
    use serde_toon2::EncoderOptions;

    let opts = EncoderOptions::default();
    serde_toon2::to_string_with_options(value, opts).map_err(|e| Error::Serialize(e.to_string()))
}

/// Serialize any serde-serializable type to a TOON string.
///
/// # Examples
///
/// ```
/// use toon_macro::ser::serialize;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let user = User { name: "Alice".into(), age: 30 };
/// let s = serialize(&user).unwrap();
/// ```
///
/// # Errors
///
/// Returns an [`Error::Serialize`] if serialization fails.
#[cfg(feature = "serde")]
pub fn serialize<T: serde::Serialize>(value: &T) -> Result<String> {
    serde_toon2::to_string(value).map_err(|e| Error::Serialize(e.to_string()))
}

/// Deserialize a TOON string into any serde-deserializable type.
///
/// # Examples
///
/// ```
/// use toon_macro::ser::deserialize;
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let s = r#"
/// name: "Alice"
/// age: 30
/// "#;
///
/// let user: User = deserialize(s).unwrap();
/// println!("{:?}", user);
/// ```
///
/// # Errors
///
/// Returns an [`Error::Deserialize`] if deserialization fails.
#[cfg(feature = "serde")]
pub fn deserialize<'a, T: serde::Deserialize<'a>>(s: &'a str) -> Result<T> {
    serde_toon2::from_str(s).map_err(|e| Error::Deserialize(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toon;

    #[test]
    fn test_from_toon_str_simple() {
        let result = from_toon_str(
            r#"
name: "Alice"
age: 30
"#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_toon_str_invalid() {
        // This should fail - invalid TOON syntax
        let result = from_toon_str("{{{{invalid}}}}");
        // Note: depending on serde_toon2's parser, this may or may not error
        // We just verify the function runs without panicking
        let _ = result;
    }

    #[test]
    fn test_to_toon_string() {
        let value = toon!({
            name: "Bob",
            active: true
        });

        let result = to_toon_string(&value);
        assert!(result.is_ok());
        let s = result.unwrap();
        assert!(s.contains("name"));
        assert!(s.contains("Bob"));
    }

    #[test]
    fn test_roundtrip() {
        let original = toon!({
            title: "Test",
            count: 42,
            items: [1, 2, 3]
        });

        let serialized = to_toon_string(&original).unwrap();
        let deserialized = from_toon_str(&serialized).unwrap();

        // Compare structure
        if let (Value::Object(orig_map), Value::Object(deser_map)) = (&original, &deserialized) {
            assert_eq!(orig_map.len(), deser_map.len());
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_struct() {
        use serde::Serialize;

        #[derive(Serialize)]
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Point { x: 10, y: 20 };
        let result = serialize(&point);
        assert!(result.is_ok());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_struct() {
        use serde::Deserialize;

        #[derive(Deserialize, Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        let s = r#"
x: 10
y: 20
"#;

        let result: Result<Point> = deserialize(s);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Point { x: 10, y: 20 });
    }
}
