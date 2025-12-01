//! TOON Value type and conversions.
//!
//! This module re-exports the [`Value`] type from `serde_toon2` and provides
//! additional helper functions for working with TOON values.

pub use serde_toon2::Map;
pub use serde_toon2::Number;
pub use serde_toon2::Value;

/// Extension trait for constructing Value from additional types.
///
/// This provides `into_value()` methods for types not covered by serde_toon2's
/// built-in `From` implementations.
pub trait IntoValue {
    /// Convert this value into a TOON [`Value`].
    fn into_value(self) -> Value;
}

impl IntoValue for i8 {
    #[inline]
    fn into_value(self) -> Value {
        Value::Number(Number::I64(self as i64))
    }
}

impl IntoValue for i16 {
    #[inline]
    fn into_value(self) -> Value {
        Value::Number(Number::I64(self as i64))
    }
}

impl IntoValue for i32 {
    #[inline]
    fn into_value(self) -> Value {
        Value::Number(Number::I64(self as i64))
    }
}

impl IntoValue for isize {
    #[inline]
    fn into_value(self) -> Value {
        Value::Number(Number::I64(self as i64))
    }
}

impl IntoValue for u8 {
    #[inline]
    fn into_value(self) -> Value {
        Value::Number(Number::U64(self as u64))
    }
}

impl IntoValue for u16 {
    #[inline]
    fn into_value(self) -> Value {
        Value::Number(Number::U64(self as u64))
    }
}

impl IntoValue for u32 {
    #[inline]
    fn into_value(self) -> Value {
        Value::Number(Number::U64(self as u64))
    }
}

impl IntoValue for usize {
    #[inline]
    fn into_value(self) -> Value {
        Value::Number(Number::U64(self as u64))
    }
}

impl IntoValue for f32 {
    #[inline]
    fn into_value(self) -> Value {
        Value::Number(Number::F64(self as f64))
    }
}

impl<T: Into<Value>> IntoValue for Option<T> {
    #[inline]
    fn into_value(self) -> Value {
        match self {
            Some(v) => v.into(),
            None => Value::Null,
        }
    }
}

impl<T: Into<Value> + Clone, const N: usize> IntoValue for [T; N] {
    #[inline]
    fn into_value(self) -> Value {
        Value::Array(self.into_iter().map(Into::into).collect())
    }
}

/// Convert any serializable type to a TOON [`Value`].
///
/// # Example
///
/// ```
/// use toon_macro::value::to_value;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let user = User { name: "Alice".into(), age: 30 };
/// let value = to_value(&user).unwrap();
/// ```
#[cfg(feature = "serde")]
pub fn to_value<T: serde::Serialize>(value: &T) -> Result<Value, serde_toon2::Error> {
    // Serialize to TOON string then parse back to Value
    let s = serde_toon2::to_string(value)?;
    serde_toon2::from_str(&s)
}

/// Convert a TOON [`Value`] to any deserializable type.
///
/// # Example
///
/// ```
/// use toon_macro::{toon, value::from_value};
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug, PartialEq)]
/// struct Point {
///     x: i64,
///     y: i64,
/// }
///
/// let value = toon!({ x: 10, y: 20 });
/// let point: Point = from_value(&value).unwrap();
/// assert_eq!(point, Point { x: 10, y: 20 });
/// ```
#[cfg(feature = "serde")]
pub fn from_value<T: serde::de::DeserializeOwned>(value: &Value) -> Result<T, serde_toon2::Error> {
    let s = serde_toon2::to_string(value)?;
    serde_toon2::from_str(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_conversion() {
        assert_eq!(Value::from(true), Value::Bool(true));
        assert_eq!(Value::from(false), Value::Bool(false));
    }

    #[test]
    fn test_integer_conversions() {
        let v: Value = 42i64.into();
        assert!(matches!(v, Value::Number(_)));

        let v: Value = 42u64.into();
        assert!(matches!(v, Value::Number(_)));
    }

    #[test]
    fn test_into_value_smaller_ints() {
        let v = 42i32.into_value();
        assert!(matches!(v, Value::Number(_)));

        let v = 42u32.into_value();
        assert!(matches!(v, Value::Number(_)));
    }

    #[test]
    fn test_string_conversion() {
        assert_eq!(Value::from("hello"), Value::String("hello".to_string()));
        assert_eq!(
            Value::from("world".to_string()),
            Value::String("world".to_string())
        );
    }

    #[test]
    fn test_option_conversion() {
        let some: Value = Some(42i64).into_value();
        assert!(matches!(some, Value::Number(_)));

        let none: Value = Option::<i64>::None.into_value();
        assert_eq!(none, Value::Null);
    }

    #[test]
    fn test_vec_conversion() {
        let v: Value = vec![1i64, 2, 3].into();
        assert!(matches!(v, Value::Array(_)));
    }
}
