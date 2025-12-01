//! Implementation of the `#[derive(ToonTable)]` macro.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Result};

use crate::utils::FieldAttrs;

/// Main entry point for the ToonTable derive macro.
pub fn derive_toon_table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive_toon_table_impl(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Internal implementation that can return errors.
fn derive_toon_table_impl(input: &DeriveInput) -> Result<TokenStream2> {
    let name = &input.ident;

    // Only support structs with named fields
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(Error::new_spanned(
                    input,
                    "ToonTable can only be derived for structs with named fields",
                ))
            }
        },
        _ => {
            return Err(Error::new_spanned(
                input,
                "ToonTable can only be derived for structs",
            ))
        }
    };

    // Parse field attributes and collect field info
    let mut field_infos = Vec::new();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let attrs = FieldAttrs::from_attrs(&field.attrs)?;

        if attrs.skip {
            continue;
        }

        let column_name = attrs
            .rename
            .clone()
            .unwrap_or_else(|| field_name.to_string());

        field_infos.push(FieldInfo {
            name: field_name.clone(),
            ty: field_type.clone(),
            column_name,
            default: attrs.default,
            order: attrs.order,
        });
    }

    // Sort by explicit order if provided
    field_infos.sort_by(|a, b| match (a.order, b.order) {
        (Some(a_ord), Some(b_ord)) => a_ord.cmp(&b_ord),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });

    // Generate COLUMNS array
    let column_names: Vec<_> = field_infos.iter().map(|f| &f.column_name).collect();
    let _columns_len = column_names.len();

    // Generate to_toon_table implementation
    let to_table_fields: Vec<_> = field_infos
        .iter()
        .map(|f| {
            let field_name = &f.name;
            quote! {
                ::toon_macro::table::IntoToonValue::to_toon_value(&row.#field_name)
            }
        })
        .collect();

    // Generate from_toon_table implementation
    let from_table_fields: Vec<_> = field_infos
        .iter()
        .map(|f| {
            let field_name = &f.name;
            let column_name = &f.column_name;

            if f.default {
                quote! {
                    #field_name: {
                        let col_idx = column_map.get(#column_name).copied();
                        match col_idx {
                            Some(idx) => {
                                let cell = ::toon_macro::table::get_cell(row, idx)?;
                                ::toon_macro::table::FromToonValue::from_toon_value(cell)?
                            }
                            None => Default::default()
                        }
                    }
                }
            } else {
                quote! {
                    #field_name: {
                        let col_idx = column_map.get(#column_name).copied()
                            .ok_or_else(|| ::toon_macro::Error::MissingColumn(#column_name))?;
                        let cell = ::toon_macro::table::get_cell(row, col_idx)?;
                        ::toon_macro::table::FromToonValue::from_toon_value(cell)?
                    }
                }
            }
        })
        .collect();

    let expanded = quote! {
        impl ::toon_macro::ToonTable for #name {
            const COLUMNS: &'static [&'static str] = &[#(#column_names),*];

            fn to_toon_table(rows: &[Self]) -> ::toon_macro::Value {
                let columns: Vec<::toon_macro::Value> = Self::COLUMNS
                    .iter()
                    .map(|&s| ::toon_macro::Value::String(s.to_string()))
                    .collect();

                let data_rows: Vec<::toon_macro::Value> = rows
                    .iter()
                    .map(|row| {
                        let cells: Vec<::toon_macro::Value> = vec![
                            #(#to_table_fields),*
                        ];
                        ::toon_macro::Value::Array(cells)
                    })
                    .collect();

                let mut map = ::toon_macro::internal::new_map();
                ::toon_macro::internal::map_insert(
                    &mut map,
                    "columns".to_string(),
                    ::toon_macro::Value::Array(columns)
                );
                ::toon_macro::internal::map_insert(
                    &mut map,
                    "rows".to_string(),
                    ::toon_macro::Value::Array(data_rows)
                );
                ::toon_macro::Value::Object(map)
            }

            fn from_toon_table(value: &::toon_macro::Value) -> ::toon_macro::Result<Vec<Self>> {
                use ::std::collections::HashMap;

                // Extract and validate columns
                let columns = ::toon_macro::table::extract_columns(value)?;
                let mut column_map: HashMap<&str, usize> = HashMap::new();
                for (idx, col) in columns.iter().enumerate() {
                    column_map.insert(col.as_str(), idx);
                }

                // Extract rows and decode each one
                let rows = ::toon_macro::table::extract_rows(value)?;
                let mut result = Vec::with_capacity(rows.len());

                for row in rows {
                    let item = Self {
                        #(#from_table_fields),*
                    };
                    result.push(item);
                }

                Ok(result)
            }
        }
    };

    Ok(expanded)
}

/// Information about a single field.
struct FieldInfo {
    name: syn::Ident,
    #[allow(dead_code)]
    ty: syn::Type,
    column_name: String,
    default: bool,
    order: Option<usize>,
}
