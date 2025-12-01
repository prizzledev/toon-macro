//! Utility functions for the derive macros.

use syn::{Attribute, Result};

/// Parsed field attributes from #[toon(...)]
#[derive(Default, Debug)]
pub struct FieldAttrs {
    /// Rename the column (e.g., #[toon(rename = "userId")])
    pub rename: Option<String>,
    /// Skip this field in table encoding/decoding
    pub skip: bool,
    /// Use default value if column is missing
    pub default: bool,
    /// Explicit column order (0-based)
    pub order: Option<usize>,
}

impl FieldAttrs {
    /// Parse #[toon(...)] attributes from a field.
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = FieldAttrs::default();

        for attr in attrs {
            if !attr.path().is_ident("toon") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    let value: syn::LitStr = meta.value()?.parse()?;
                    result.rename = Some(value.value());
                    Ok(())
                } else if meta.path.is_ident("skip") {
                    result.skip = true;
                    Ok(())
                } else if meta.path.is_ident("default") {
                    result.default = true;
                    Ok(())
                } else if meta.path.is_ident("order") {
                    let value: syn::LitInt = meta.value()?.parse()?;
                    result.order = Some(value.base10_parse()?);
                    Ok(())
                } else {
                    Err(meta.error("expected `rename`, `skip`, `default`, or `order`"))
                }
            })?;
        }

        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_field_attrs() {
        let attrs = FieldAttrs::default();
        assert!(attrs.rename.is_none());
        assert!(!attrs.skip);
        assert!(!attrs.default);
        assert!(attrs.order.is_none());
    }
}
