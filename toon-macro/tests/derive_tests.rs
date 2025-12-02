//! Integration tests for the ToonTable derive macro.

#![cfg(feature = "derive")]

use toon_macro::{ToonTable, Value, toon};

#[derive(Debug, Clone, PartialEq, ToonTable)]
struct User {
    id: u64,
    name: String,
    role: String,
}

#[test]
fn test_toon_table_columns() {
    assert_eq!(User::COLUMNS, &["id", "name", "role"]);
}

#[test]
fn test_toon_table_encode() {
    let users = vec![
        User {
            id: 1,
            name: "Alice".into(),
            role: "admin".into(),
        },
        User {
            id: 2,
            name: "Bob".into(),
            role: "user".into(),
        },
    ];

    let table = User::to_toon_table(&users);

    // Verify structure
    if let Value::Object(map) = &table {
        assert!(map.contains_key("columns"));
        assert!(map.contains_key("rows"));

        if let Some(Value::Array(cols)) = map.get("columns") {
            assert_eq!(cols.len(), 3);
            assert_eq!(cols[0], Value::String("id".into()));
            assert_eq!(cols[1], Value::String("name".into()));
            assert_eq!(cols[2], Value::String("role".into()));
        } else {
            panic!("Expected columns to be an array");
        }

        if let Some(Value::Array(rows)) = map.get("rows") {
            assert_eq!(rows.len(), 2);
        } else {
            panic!("Expected rows to be an array");
        }
    } else {
        panic!("Expected table to be an object");
    }
}

#[test]
fn test_toon_table_decode() {
    let table = toon!({
        columns: ["id", "name", "role"],
        rows: [
            [1, "Alice", "admin"],
            [2, "Bob", "user"]
        ]
    });

    let users = User::from_toon_table(&table).unwrap();
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].id, 1);
    assert_eq!(users[0].name, "Alice");
    assert_eq!(users[0].role, "admin");
    assert_eq!(users[1].id, 2);
    assert_eq!(users[1].name, "Bob");
    assert_eq!(users[1].role, "user");
}

#[test]
fn test_toon_table_roundtrip() {
    let original = vec![
        User {
            id: 1,
            name: "Alice".into(),
            role: "admin".into(),
        },
        User {
            id: 2,
            name: "Bob".into(),
            role: "user".into(),
        },
    ];

    let table = User::to_toon_table(&original);
    let decoded = User::from_toon_table(&table).unwrap();

    assert_eq!(original, decoded);
}

#[derive(Debug, Clone, PartialEq, ToonTable)]
struct RenamedFields {
    #[toon(rename = "userId")]
    id: u64,
    #[toon(rename = "userName")]
    name: String,
}

#[test]
fn test_toon_table_rename() {
    assert_eq!(RenamedFields::COLUMNS, &["userId", "userName"]);

    let items = vec![RenamedFields {
        id: 1,
        name: "Test".into(),
    }];

    let table = RenamedFields::to_toon_table(&items);

    if let Value::Object(map) = &table {
        if let Some(Value::Array(cols)) = map.get("columns") {
            assert_eq!(cols[0], Value::String("userId".into()));
            assert_eq!(cols[1], Value::String("userName".into()));
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToonTable)]
struct DefaultFields {
    id: u64,
    name: String,
    #[toon(default)]
    optional: String,
}

#[test]
fn test_toon_table_default() {
    // Table without the optional column
    let table = toon!({
        columns: ["id", "name"],
        rows: [[1, "Alice"]]
    });

    let items = DefaultFields::from_toon_table(&table).unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, 1);
    assert_eq!(items[0].name, "Alice");
    assert_eq!(items[0].optional, ""); // Default value for String
}

#[derive(Debug, Clone, PartialEq, ToonTable)]
struct MixedTypes {
    id: i64,
    count: u64,
    score: f64,
    active: bool,
    name: String,
}

#[test]
fn test_toon_table_mixed_types() {
    let items = vec![MixedTypes {
        id: -1,
        count: 100,
        score: 95.5,
        active: true,
        name: "Test".into(),
    }];

    let table = MixedTypes::to_toon_table(&items);
    let decoded = MixedTypes::from_toon_table(&table).unwrap();

    assert_eq!(items, decoded);
}
