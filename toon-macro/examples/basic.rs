//! Basic usage examples for toon-macro.
//!
//! Run with: cargo run --example basic

use toon_macro::{toon, toon_str, from_toon_str, to_toon_string};

fn main() {
    println!("=== toon-macro Basic Examples ===\n");

    // Example 1: Using toon! macro to create objects
    println!("1. Creating objects with toon! macro:");
    let user = toon!({
        name: "Alice",
        age: 30,
        active: true,
        email: "alice@example.com"
    });
    println!("   User: {:?}\n", user);

    // Example 2: Nested structures
    println!("2. Nested structures:");
    let config = toon!({
        database: {
            host: "localhost",
            port: 5432,
            name: "myapp"
        },
        server: {
            host: "0.0.0.0",
            port: 8080
        }
    });
    println!("   Config: {:?}\n", config);

    // Example 3: Arrays
    println!("3. Arrays:");
    let numbers = toon!([1, 2, 3, 4, 5]);
    println!("   Numbers: {:?}", numbers);

    let users = toon!([
        { id: 1, name: "Alice" },
        { id: 2, name: "Bob" },
        { id: 3, name: "Charlie" }
    ]);
    println!("   Users: {:?}\n", users);

    // Example 4: Using variables
    println!("4. Using variables:");
    let name = "Dynamic Name";
    let score = 95i64;
    let is_valid = true;

    let result = toon!({
        name: name,
        score: score,
        valid: is_valid
    });
    println!("   Result: {:?}\n", result);

    // Example 5: Special keys with string literals
    println!("5. String literal keys:");
    let with_special_keys = toon!({
        "kebab-key": "value1",
        "key with spaces": "value2",
        "123numeric": "value3"
    });
    println!("   Special keys: {:?}\n", with_special_keys);

    // Example 6: Using toon_str! for TOON format parsing
    println!("6. Parsing TOON format with toon_str!:");
    let parsed = toon_str!(r#"
title: "My Document"
version: 1
author: "John Doe"
"#);
    println!("   Parsed: {:?}\n", parsed);

    // Example 7: Error handling with from_toon_str
    println!("7. Error handling with from_toon_str:");
    let valid_input = r#"status: "ok""#;
    match from_toon_str(valid_input) {
        Ok(value) => println!("   Valid: {:?}", value),
        Err(e) => println!("   Error: {}", e),
    }

    let invalid_input = ":::invalid:::";
    match from_toon_str(invalid_input) {
        Ok(value) => println!("   Parsed: {:?}", value),
        Err(e) => println!("   Expected error: {}\n", e),
    }

    // Example 8: Serializing to TOON string
    println!("8. Serializing to TOON string:");
    let data = toon!({
        message: "Hello, TOON!",
        count: 42
    });
    match to_toon_string(&data) {
        Ok(s) => println!("   Serialized:\n{}", s),
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== Examples Complete ===");
}
