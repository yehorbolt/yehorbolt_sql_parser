# SQL Parser

This is a simple SQL parser written in Rust using the `pest` library. The parser supports parsing `CREATE TABLE` statements with column definitions.

## Installation

To use this parser, you need to have Rust installed. Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
anyhow = "1.0.75"
clap = "4.4.8"
pest = "2.7.5"
pest_derive = "2.7.5"
thiserror = "1.0.50"
```

Then, include the provided sql_grammar.pest file in your project. You can customize this grammar file to support more SQL statements or modify the existing rules.

## Usage
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

## Structure

### SqlParseError
This enum represents parsing errors and includes a variant for invalid data.

### SqlType
An enum representing SQL data types (Int, Text, Bool). Implements FromStr for parsing from strings and PartialEq for equality comparison.

### ColumnInfo
A struct representing information about a database column, including the column name and type.

### ColumnInfoOption
A variant of ColumnInfo with options for column name and column type.

### CreateTable
A struct representing the structure of a SQL CREATE TABLE statement, including the table name and column information.

### CreateTableOption
A variant of CreateTable with options for table name and column definitions.

### Parsed
An enum representing parsed SQL statements, currently supporting only CreateTable.

### fn parse_sql
The main function that takes a SQL query as input and returns a parsed result. It uses the pest parser to handle the grammar defined in sql_grammar.pest.

### fn parse_create_table
A function that parses a CREATE TABLE statement, extracting the table name, column names, and types to construct a CreateTable struct.

### fn unwrap_column_info
A helper function that recursively unwraps and extracts column information from a vector of optional ColumnInfoOption instances, returning a vector of ColumnInfo.

## Usage
Include the sql_parser crate in your Rust project and use the provided functions to parse SQL `CREATE TABLE` statements. Refer to the example for usage details.
