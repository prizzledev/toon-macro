//! Unified error types for toon-macro.
//!
//! This module provides a single [`enum@Error`] type that wraps all possible
//! errors from TOON parsing, serialization, and table operations.

use thiserror::Error;

/// A unified error type for all toon-macro operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Error during TOON serialization.
    #[error("TOON serialization error: {0}")]
    Serialize(String),

    /// Error during TOON deserialization/parsing.
    #[error("TOON deserialization error: {0}")]
    Deserialize(String),

    /// Invalid TOON table structure.
    #[error("Invalid TOON table: {0}")]
    InvalidTable(String),

    /// A required column is missing from the table.
    #[error("Missing required column: {0}")]
    MissingColumn(&'static str),

    /// Error converting between types.
    #[error("Conversion error: {0}")]
    ConversionError(String),

    /// Invalid value type encountered.
    #[error("Invalid value type: expected {expected}, got {got}")]
    InvalidType {
        /// The expected type name.
        expected: &'static str,
        /// The actual type name.
        got: String,
    },

    /// Row index out of bounds.
    #[error("Row index {index} out of bounds (table has {len} rows)")]
    RowOutOfBounds {
        /// The requested index.
        index: usize,
        /// The number of rows in the table.
        len: usize,
    },

    /// Column index out of bounds.
    #[error("Column index {index} out of bounds (table has {len} columns)")]
    ColumnOutOfBounds {
        /// The requested index.
        index: usize,
        /// The number of columns in the table.
        len: usize,
    },
}

/// A `Result` type alias using [`enum@Error`].
pub type Result<T> = std::result::Result<T, Error>;

impl From<serde_toon2::Error> for Error {
    fn from(err: serde_toon2::Error) -> Self {
        // Determine if it's a serialization or deserialization error
        // based on the error message content
        let msg = err.to_string();
        if msg.contains("serialize") || msg.contains("Serialize") {
            Error::Serialize(msg)
        } else {
            Error::Deserialize(msg)
        }
    }
}

impl Error {
    /// Create a serialization error from a message.
    pub fn serialize<S: Into<String>>(msg: S) -> Self {
        Error::Serialize(msg.into())
    }

    /// Create a deserialization error from a message.
    pub fn deserialize<S: Into<String>>(msg: S) -> Self {
        Error::Deserialize(msg.into())
    }

    /// Create an invalid table error.
    pub fn invalid_table<S: Into<String>>(msg: S) -> Self {
        Error::InvalidTable(msg.into())
    }

    /// Create a missing column error.
    pub fn missing_column(name: &'static str) -> Self {
        Error::MissingColumn(name)
    }

    /// Create a conversion error.
    pub fn conversion<S: Into<String>>(msg: S) -> Self {
        Error::ConversionError(msg.into())
    }

    /// Create an invalid type error.
    pub fn invalid_type(expected: &'static str, got: impl std::fmt::Debug) -> Self {
        Error::InvalidType {
            expected,
            got: format!("{:?}", got),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::MissingColumn("id");
        assert_eq!(err.to_string(), "Missing required column: id");

        let err = Error::InvalidType {
            expected: "string",
            got: "number".to_string(),
        };
        assert!(err.to_string().contains("expected string"));
    }

    #[test]
    fn test_error_constructors() {
        let err = Error::serialize("failed to write");
        assert!(matches!(err, Error::Serialize(_)));

        let err = Error::invalid_table("missing header");
        assert!(matches!(err, Error::InvalidTable(_)));
    }
}
