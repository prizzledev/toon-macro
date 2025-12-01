//! ToonTable examples for toon-macro.
//!
//! Run with: cargo run --example tables --features derive

use toon_macro::{toon, to_toon_string, ToonTable, Value};

#[derive(Debug, Clone, PartialEq, ToonTable)]
struct User {
    id: u64,
    name: String,
    role: String,
}

#[derive(Debug, Clone, PartialEq, ToonTable)]
struct Product {
    #[toon(rename = "productId")]
    id: u64,
    #[toon(rename = "productName")]
    name: String,
    price: f64,
    #[toon(default)]
    category: String,
}

#[derive(Debug, Clone, PartialEq, ToonTable)]
struct Employee {
    id: i64,
    name: String,
    department: String,
    salary: f64,
    active: bool,
}

fn main() {
    println!("=== ToonTable Examples ===\n");

    // Example 1: Basic table encoding
    println!("1. Basic table encoding:");
    let users = vec![
        User { id: 1, name: "Alice".into(), role: "admin".into() },
        User { id: 2, name: "Bob".into(), role: "user".into() },
        User { id: 3, name: "Charlie".into(), role: "user".into() },
    ];

    println!("   Columns: {:?}", User::COLUMNS);
    let table = User::to_toon_table(&users);
    println!("   Table Value: {:?}", table);
    println!();

    // Example 2: Table decoding
    println!("2. Table decoding:");
    let table_data = toon!({
        columns: ["id", "name", "role"],
        rows: [
            [10, "Diana", "manager"],
            [11, "Eve", "developer"],
            [12, "Frank", "designer"]
        ]
    });

    match User::from_toon_table(&table_data) {
        Ok(users) => {
            for user in &users {
                println!("   {:?}", user);
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 3: Roundtrip encoding/decoding
    println!("3. Roundtrip encoding/decoding:");
    let original = vec![
        User { id: 100, name: "Test1".into(), role: "role1".into() },
        User { id: 101, name: "Test2".into(), role: "role2".into() },
    ];

    let encoded = User::to_toon_table(&original);
    let decoded = User::from_toon_table(&encoded).unwrap();

    println!("   Original: {:?}", original);
    println!("   Decoded:  {:?}", decoded);
    println!("   Match: {}", original == decoded);
    println!();

    // Example 4: Renamed columns
    println!("4. Renamed columns with #[toon(rename)]:");
    let products = vec![
        Product { id: 1, name: "Widget".into(), price: 9.99, category: "Tools".into() },
        Product { id: 2, name: "Gadget".into(), price: 19.99, category: "Electronics".into() },
    ];

    println!("   Columns: {:?}", Product::COLUMNS);
    let table = Product::to_toon_table(&products);

    if let Value::Object(map) = &table {
        if let Some(Value::Array(cols)) = map.get("columns") {
            print!("   Column names in table: ");
            for (i, col) in cols.iter().enumerate() {
                if i > 0 { print!(", "); }
                if let Value::String(s) = col {
                    print!("{}", s);
                }
            }
            println!();
        }
    }
    println!();

    // Example 5: Default values
    println!("5. Default values with #[toon(default)]:");
    let table_missing_category = toon!({
        columns: ["productId", "productName", "price"],
        rows: [
            [100, "NoCategory", 5.99]
        ]
    });

    match Product::from_toon_table(&table_missing_category) {
        Ok(products) => {
            for product in &products {
                println!("   {:?}", product);
                println!("   (category defaulted to empty string)");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 6: Mixed types
    println!("6. Mixed types (i64, u64, f64, bool, String):");
    let employees = vec![
        Employee {
            id: -1,  // negative i64
            name: "Temp Worker".into(),
            department: "Temp".into(),
            salary: 0.0,
            active: false,
        },
        Employee {
            id: 1000,
            name: "John Doe".into(),
            department: "Engineering".into(),
            salary: 75000.50,
            active: true,
        },
    ];

    println!("   Columns: {:?}", Employee::COLUMNS);
    let table = Employee::to_toon_table(&employees);
    let decoded = Employee::from_toon_table(&table).unwrap();

    println!("   Original employees:");
    for emp in &employees {
        println!("      {:?}", emp);
    }
    println!("   Decoded employees:");
    for emp in &decoded {
        println!("      {:?}", emp);
    }
    println!("   Match: {}", employees == decoded);
    println!();

    // Example 7: Serialize table to TOON string
    println!("7. Table as TOON string:");
    let users = vec![
        User { id: 1, name: "Alice".into(), role: "admin".into() },
        User { id: 2, name: "Bob".into(), role: "user".into() },
    ];
    let table = User::to_toon_table(&users);

    match to_toon_string(&table) {
        Ok(s) => {
            println!("   TOON output:");
            for line in s.lines() {
                println!("   {}", line);
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== ToonTable Examples Complete ===");
}
