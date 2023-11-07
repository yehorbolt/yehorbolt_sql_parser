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

Then, include the provided sql_grammar.pest file in your project. You can customize this grammar file to support more SQL statements or modify the existing rules.

## Usage
To parse SQL statements, use the parse_sql function provided in the code. It takes a SQL statement as a string and returns a ParsedStmnt enum variant.


use sql_parser::parse_sql;

fn main() {
    let sql_statement = "CREATE TABLE users (id INT, name TEXT);";
    match parse_sql(sql_statement) {
        ParsedStmnt::CreateTable(create_table) => {
            println!("Table Name: {}", create_table.table_name);
            for column_def in create_table.column_defs {
                println!("Column Name: {}, Type: {:?}", column_def.column_name, column_def.column_type);
            }
        }
    }
}


In this example, the parse_sql function is used to parse a CREATE TABLE statement, and the parsed result is printed to the console.

## Supported SQL Types
The parser supports three SQL types: INT, TEXT, and BOOL. You can extend the SqlType enum in the code to add more supported types if needed.

## Error Handling
The parser does not handle errors related to invalid SQL syntax. You can enhance error handling by extending the code to handle specific error cases and provide meaningful error messages to the users.

Feel free to customize and extend the parser to fit your specific use case or add more SQL features.