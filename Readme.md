# SQL Parser

This is a simple SQL parser written in Rust using the `pest` library. The parser supports parsing `CREATE TABLE` statements with column definitions.

## Installation

To use this parser, you need to have Rust installed. Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
anyhow = "1.0.75"
pest = "2.7.5"
pest_derive = "2.7.5"
thiserror = "1.0.50"
```


Then, include the provided sql_grammar.pest file in your project. You can customize this grammar file to support more SQL statements or modify the existing rules.

## Usage
To parse SQL statements, use the parse_sql function provided in the code. It takes a SQL statement as a string and returns a ParsedStmnt enum variant.


```rust
extern crate yehorbolt_sql_parser;
use yehorbolt_sql_parser::parse_sql;

fn main() {
    let table = "CREATE TABLE financial_report { id INT, currency_name TEXT, is_usable BOOL}";
    let res = parse_sql(table);
    println!("Parsed: {:?}", res);
}

```

In this example, the parse_sql function is used to parse a CREATE TABLE statement, and the parsed result is printed to the console.

## Supported SQL Types
The parser supports three SQL types: INT, TEXT, and BOOL. You can extend the SqlType enum in the code to add more supported types if needed.

