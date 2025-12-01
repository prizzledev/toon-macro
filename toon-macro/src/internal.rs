//! Internal helper macros and utilities.
//!
//! This module contains implementation details used by the public macros.
//! These are not part of the public API and may change without notice.

/// Convert a key (identifier or literal) to a String.
///
/// This macro is used internally by the `toon!` macro to handle both
/// identifier keys (like `name`) and string literal keys (like `"weird-key"`).
#[doc(hidden)]
#[macro_export]
macro_rules! __toon_key_to_string {
    ($k:ident) => {
        stringify!($k).to_string()
    };
    ($k:literal) => {
        $k.to_string()
    };
}

/// Internal macro for constructing TOON values.
///
/// This handles the recursive value construction in the `toon!` macro.
#[doc(hidden)]
#[macro_export]
macro_rules! __toon_value {
    // Null literal
    (null) => {
        $crate::Value::Null
    };

    // Boolean literals
    (true) => {
        $crate::Value::Bool(true)
    };
    (false) => {
        $crate::Value::Bool(false)
    };

    // Nested object
    ({ $($key:tt : $value:tt),* $(,)? }) => {
        $crate::toon!({ $($key : $value),* })
    };

    // Array
    ([ $($value:tt),* $(,)? ]) => {
        $crate::toon!([ $($value),* ])
    };

    // Any other expression (numbers, strings, variables)
    // We use a helper function to convert types properly
    ($other:expr) => {
        $crate::internal::into_value($other)
    };
}

/// Internal helper to create a TOON Map.
#[doc(hidden)]
#[inline]
pub fn new_map() -> serde_toon2::Map<String, serde_toon2::Value> {
    serde_toon2::Map::new()
}

/// Internal helper to insert into a TOON Map.
#[doc(hidden)]
#[inline]
pub fn map_insert(
    map: &mut serde_toon2::Map<String, serde_toon2::Value>,
    key: String,
    value: serde_toon2::Value,
) {
    map.insert(key, value);
}

/// Trait for converting values to TOON Value type.
///
/// This exists to provide conversions for types not covered by
/// serde_toon2's From implementations (like i32, u32, etc.)
#[doc(hidden)]
pub trait IntoToonValueInternal {
    fn into_toon_value(self) -> serde_toon2::Value;
}

// Implement for types that already have From<T> for Value
impl IntoToonValueInternal for bool {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::Bool(self)
    }
}

impl IntoToonValueInternal for i64 {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::from(self)
    }
}

impl IntoToonValueInternal for u64 {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::from(self)
    }
}

impl IntoToonValueInternal for f64 {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::from(self)
    }
}

impl IntoToonValueInternal for String {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::String(self)
    }
}

impl IntoToonValueInternal for &str {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::String(self.to_string())
    }
}

impl IntoToonValueInternal for &String {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::String(self.clone())
    }
}

// Implement for smaller integer types (convert to i64/u64)
impl IntoToonValueInternal for i8 {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::Number(serde_toon2::Number::I64(self as i64))
    }
}

impl IntoToonValueInternal for i16 {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::Number(serde_toon2::Number::I64(self as i64))
    }
}

impl IntoToonValueInternal for i32 {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::Number(serde_toon2::Number::I64(self as i64))
    }
}

impl IntoToonValueInternal for isize {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::Number(serde_toon2::Number::I64(self as i64))
    }
}

impl IntoToonValueInternal for u8 {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::Number(serde_toon2::Number::U64(self as u64))
    }
}

impl IntoToonValueInternal for u16 {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::Number(serde_toon2::Number::U64(self as u64))
    }
}

impl IntoToonValueInternal for u32 {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::Number(serde_toon2::Number::U64(self as u64))
    }
}

impl IntoToonValueInternal for usize {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::Number(serde_toon2::Number::U64(self as u64))
    }
}

impl IntoToonValueInternal for f32 {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        serde_toon2::Value::Number(serde_toon2::Number::F64(self as f64))
    }
}

// Implement for Value itself (passthrough)
impl IntoToonValueInternal for serde_toon2::Value {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        self
    }
}

// Implement for &Value
impl IntoToonValueInternal for &serde_toon2::Value {
    #[inline]
    fn into_toon_value(self) -> serde_toon2::Value {
        self.clone()
    }
}

/// Helper function to convert any supported type to a TOON Value.
#[doc(hidden)]
#[inline]
pub fn into_value<T: IntoToonValueInternal>(value: T) -> serde_toon2::Value {
    value.into_toon_value()
}
