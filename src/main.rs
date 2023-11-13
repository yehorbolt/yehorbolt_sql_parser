///! # SQL parser
///! This is a simple SQL parser written in Rust using the pest library. 
///! The parser supports parsing CREATE TABLE statements with column definitions.
///!
///! # Example
///! ``` rust
///!    let table = "CREATE TABLE financial_report 
///!    { 
///!        id INT, 
///!        currency_name TEXT, 
///!        is_usable BOOL
///!    }";
///!    let res = parse_sql(table);
///!    println!("Parsed: {:?}", res);
///! ```




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
