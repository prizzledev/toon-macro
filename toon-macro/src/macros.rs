//! TOON construction macros.
//!
//! This module provides two macro families:
//!
//! - `toon!` - A Rust-DSL for constructing TOON values with JSON-like syntax
//! - `toon_str!` - Parse TOON-format text at runtime
//!
//! # Examples
//!
//! ## Using `toon!` (Rust-DSL)
//!
//! ```
//! use toon_macro::toon;
//!
//! // Simple object
//! let obj = toon!({
//!     name: "Alice",
//!     age: 30,
//!     active: true
//! });
//!
//! // Nested structures
//! let nested = toon!({
//!     user: {
//!         id: 1,
//!         profile: {
//!             name: "Bob",
//!             scores: [95, 87, 92]
//!         }
//!     }
//! });
//!
//! // Arrays
//! let arr = toon!([1, 2, 3, 4, 5]);
//! ```
//!
//! ## Using `toon_str!` (TOON syntax)
//!
//! ```
//! use toon_macro::toon_str;
//!
//! let value = toon_str!(r#"
//! title: "Example"
//! count: 42
//! "#);
//! ```

/// Construct a TOON [`Value`] using a JSON-like DSL.
///
/// This macro provides an ergonomic way to construct TOON values directly
/// in Rust code, similar to `serde_json::json!`.
///
/// # Syntax
///
/// - **Objects**: `toon!({ key: value, ... })`
/// - **Arrays**: `toon!([value, ...])`
/// - **Null**: `toon!(null)`
/// - **Booleans**: `toon!(true)`, `toon!(false)`
/// - **Numbers**: `toon!(42)`, `toon!(3.14)`
/// - **Strings**: `toon!("hello")`
/// - **Variables**: `toon!(my_var)`
///
/// # Examples
///
/// ## Basic Object
///
/// ```
/// use toon_macro::toon;
///
/// let obj = toon!({
///     name: "Alice",
///     age: 30
/// });
/// ```
///
/// ## Using Variables
///
/// ```
/// use toon_macro::toon;
///
/// let name = "Bob";
/// let score = 95;
///
/// let obj = toon!({
///     name: name,
///     score: score,
///     passed: true
/// });
/// ```
///
/// ## Nested Structures
///
/// ```
/// use toon_macro::toon;
///
/// let data = toon!({
///     users: [
///         { id: 1, name: "Alice" },
///         { id: 2, name: "Bob" }
///     ],
///     metadata: {
///         version: "1.0",
///         count: 2
///     }
/// });
/// ```
///
/// ## String Keys with Special Characters
///
/// ```
/// use toon_macro::toon;
///
/// let obj = toon!({
///     "kebab-key": "value",
///     "key with spaces": 42
/// });
/// ```
///
/// [`Value`]: crate::Value
#[macro_export]
macro_rules! toon {
    //
    // === NULL ===
    //
    (null) => {
        $crate::Value::Null
    };

    //
    // === BOOLEANS ===
    //
    (true) => {
        $crate::Value::Bool(true)
    };
    (false) => {
        $crate::Value::Bool(false)
    };

    //
    // === OBJECTS ===
    //
    // Empty object
    ({}) => {{
        $crate::Value::Object($crate::internal::new_map())
    }};

    // Object with key-value pairs
    ({ $($key:tt : $value:tt),+ $(,)? }) => {{
        let mut map = $crate::internal::new_map();
        $(
            $crate::internal::map_insert(
                &mut map,
                $crate::__toon_key_to_string!($key),
                $crate::__toon_value!($value),
            );
        )+
        $crate::Value::Object(map)
    }};

    //
    // === ARRAYS ===
    //
    // Empty array
    ([]) => {
        $crate::Value::Array(::std::vec::Vec::new())
    };

    // Array with elements
    ([ $($value:tt),+ $(,)? ]) => {{
        let vec: ::std::vec::Vec<$crate::Value> = ::std::vec![
            $( $crate::__toon_value!($value) ),+
        ];
        $crate::Value::Array(vec)
    }};

    //
    // === EXPRESSIONS ===
    //
    // Any other expression (string literals, numbers, variables)
    ($other:expr) => {
        $crate::internal::into_value($other)
    };
}

/// Parse a TOON-format string literal at runtime.
///
/// This macro accepts a string containing TOON syntax and parses it
/// at runtime into a [`Value`]. If parsing fails, it will panic with
/// an error message.
///
/// For fallible parsing, use [`from_toon_str`] directly.
///
/// # Examples
///
/// ```
/// use toon_macro::toon_str;
///
/// let value = toon_str!(r#"
/// title: "My Document"
/// version: 1
/// "#);
/// ```
///
/// # Panics
///
/// Panics if the input string is not valid TOON syntax.
///
/// [`Value`]: crate::Value
/// [`from_toon_str`]: crate::from_toon_str
#[macro_export]
macro_rules! toon_str {
    ($s:expr) => {
        match $crate::from_toon_str($s) {
            ::std::result::Result::Ok(value) => value,
            ::std::result::Result::Err(err) => {
                panic!("Invalid TOON in toon_str!: {}", err)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::Value;

    #[test]
    fn test_toon_null() {
        let v = toon!(null);
        assert_eq!(v, Value::Null);
    }

    #[test]
    fn test_toon_booleans() {
        assert_eq!(toon!(true), Value::Bool(true));
        assert_eq!(toon!(false), Value::Bool(false));
    }

    #[test]
    fn test_toon_numbers() {
        let v = toon!(42);
        assert!(matches!(v, Value::Number(_)));

        let v = toon!(3.14);
        assert!(matches!(v, Value::Number(_)));
    }

    #[test]
    fn test_toon_strings() {
        let v = toon!("hello");
        assert_eq!(v, Value::String("hello".to_string()));
    }

    #[test]
    fn test_toon_empty_object() {
        let v = toon!({});
        assert!(matches!(v, Value::Object(_)));
    }

    #[test]
    fn test_toon_simple_object() {
        let v = toon!({
            name: "Alice",
            age: 30
        });

        if let Value::Object(map) = v {
            assert_eq!(map.get("name"), Some(&Value::String("Alice".to_string())));
            assert!(matches!(map.get("age"), Some(Value::Number(_))));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_toon_nested_object() {
        let v = toon!({
            user: {
                id: 1,
                name: "Bob"
            }
        });

        if let Value::Object(map) = v {
            assert!(matches!(map.get("user"), Some(Value::Object(_))));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_toon_array() {
        let v = toon!([1, 2, 3]);
        if let Value::Array(arr) = v {
            assert_eq!(arr.len(), 3);
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_toon_empty_array() {
        let v = toon!([]);
        if let Value::Array(arr) = v {
            assert!(arr.is_empty());
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_toon_array_of_objects() {
        let v = toon!([
            { id: 1, name: "Alice" },
            { id: 2, name: "Bob" }
        ]);

        if let Value::Array(arr) = v {
            assert_eq!(arr.len(), 2);
            assert!(matches!(&arr[0], Value::Object(_)));
            assert!(matches!(&arr[1], Value::Object(_)));
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_toon_with_variables() {
        let name = "Charlie";
        let age = 25i64;

        let v = toon!({
            name: name,
            age: age
        });

        if let Value::Object(map) = v {
            assert_eq!(map.get("name"), Some(&Value::String("Charlie".to_string())));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_toon_string_keys() {
        let v = toon!({
            "kebab-key": "value",
            "key with spaces": 42
        });

        if let Value::Object(map) = v {
            assert_eq!(map.get("kebab-key"), Some(&Value::String("value".to_string())));
            assert!(map.contains_key("key with spaces"));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_toon_trailing_comma() {
        let v = toon!({
            a: 1,
            b: 2,
        });

        if let Value::Object(map) = v {
            assert_eq!(map.len(), 2);
        } else {
            panic!("Expected object");
        }

        let arr = toon!([1, 2, 3,]);
        if let Value::Array(arr) = arr {
            assert_eq!(arr.len(), 3);
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_toon_complex_nested() {
        let v = toon!({
            config: {
                database: {
                    host: "localhost",
                    port: 5432
                },
                features: ["auth", "logging", "metrics"]
            },
            users: [
                { id: 1, role: "admin" },
                { id: 2, role: "user" }
            ]
        });

        assert!(matches!(v, Value::Object(_)));
    }
}
