# SQL Parser

This is a simple SQL parser written in Rust using the `pest` library. The parser supports parsing `CREATE TABLE` statements with column definitions.

[![Crates.io](https://img.shields.io/crates/v/yehorbolt_sql_parser)](https://crates.io/crates/yehorbolt_sql_parser)
[![Documentation](https://docs.rs/yehorbolt_sql_parser/badge.svg)](https://docs.rs/yehorbolt_sql_parser/0.1.6/yehorbolt_sql_parser/)

## Installation

To use this parser, you need to have Rust installed. Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
anyhow = "1.0.75"
clap = { version = "4.4.8", features = ["derive"] }
pest = "2.7.5"
pest_derive = "2.7.5"
thiserror = "1.0.50"
```

Then, include the provided sql_grammar.pest file in your project. You can customize this grammar file to support more SQL statements or modify the existing rules.

## Example
To parse SQL statements, use the parse_sql function provided in the code. It takes a SQL statement as a string and returns a Parsed enum variant.

```rust
extern crate yehorbolt_sql_parser;
use yehorbolt_sql_parser::parse_sql;

fn main() {
    let table = "CREATE TABLE financial_report 
    { 
        id INT, 
        currency_name TEXT, 
        is_usable BOOL
    }";
    let res = parse_sql(table);
    println!("Parsed: {:?}", res);
}

```

In this example, the parse_sql function is used to parse a CREATE TABLE statement, and the parsed result is printed to the console.

## SQL data types
The parser supports the following SQL data types:

INT: Integer type
TEXT: Text type
BOOL: Boolean type

## Usage
Include the sql_parser crate in your Rust project and use the provided functions to parse SQL `CREATE TABLE` statements. Refer to the example for usage details.

## CLI usage
-p, --parse <PARSE>: Parse SQL CREATE TABLE statement from the specified file.
-h, --help : Prints help.
-V, --version : Prints version.

## CLI example
cargo run -- -p ./examples/txtfile.txt
cargo run -- -p ./examples/sqlfile.sql

(Examples included)