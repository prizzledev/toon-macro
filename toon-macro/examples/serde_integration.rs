//! Serde integration examples for toon-macro.
//!
//! Run with: cargo run --example serde_integration

use serde::{Deserialize, Serialize};
use toon_macro::{serialize, deserialize, toon, value::{to_value, from_value}};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    id: u64,
    name: String,
    email: String,
    active: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Config {
    database: DatabaseConfig,
    server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct DatabaseConfig {
    host: String,
    port: u16,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ServerConfig {
    host: String,
    port: u16,
}

fn main() {
    println!("=== Serde Integration Examples ===\n");

    // Example 1: Serialize a struct to TOON string
    println!("1. Serializing structs to TOON:");
    let user = User {
        id: 1,
        name: "Alice".into(),
        email: "alice@example.com".into(),
        active: true,
    };

    match serialize(&user) {
        Ok(toon_str) => {
            println!("   User as TOON:");
            for line in toon_str.lines() {
                println!("   {}", line);
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 2: Deserialize from TOON string
    println!("2. Deserializing from TOON string:");
    let toon_input = r#"
id: 2
name: "Bob"
email: "bob@example.com"
active: false
"#;

    match deserialize::<User>(toon_input) {
        Ok(user) => println!("   Deserialized user: {:?}", user),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 3: Complex nested structures
    println!("3. Complex nested structures:");
    let config = Config {
        database: DatabaseConfig {
            host: "localhost".into(),
            port: 5432,
            name: "myapp".into(),
        },
        server: ServerConfig {
            host: "0.0.0.0".into(),
            port: 8080,
        },
    };

    match serialize(&config) {
        Ok(toon_str) => {
            println!("   Config as TOON:");
            for line in toon_str.lines() {
                println!("   {}", line);
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 4: Roundtrip serialization
    println!("4. Roundtrip serialization:");
    let original = User {
        id: 42,
        name: "Charlie".into(),
        email: "charlie@example.com".into(),
        active: true,
    };

    let serialized = serialize(&original).unwrap();
    let deserialized: User = deserialize(&serialized).unwrap();

    println!("   Original:     {:?}", original);
    println!("   Deserialized: {:?}", deserialized);
    println!("   Match: {}", original == deserialized);
    println!();

    // Example 5: Convert between Value and typed structs
    println!("5. Converting between Value and structs:");
    let user = User {
        id: 100,
        name: "Diana".into(),
        email: "diana@example.com".into(),
        active: true,
    };

    // Convert struct to Value
    match to_value(&user) {
        Ok(value) => {
            println!("   As Value: {:?}", value);

            // Convert Value back to struct
            match from_value::<User>(&value) {
                Ok(recovered) => println!("   Recovered: {:?}", recovered),
                Err(e) => println!("   Error: {}", e),
            }
        }
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 6: Build Value with toon! then convert to struct
    println!("6. Build Value with toon! then deserialize:");
    let value = toon!({
        id: 200,
        name: "Eve",
        email: "eve@example.com",
        active: true
    });

    match from_value::<User>(&value) {
        Ok(user) => println!("   User from toon!: {:?}", user),
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== Serde Examples Complete ===");
}
