//! # SQL parser
//! This is a simple SQL parser written in Rust using the pest library.
//! The parser supports parsing CREATE TABLE statements with column definitions.
//!
//! # Example
//! ```
//! extern crate yehorbolt_sql_parser;
//! use yehorbolt_sql_parser::parse_sql;
//!
//! fn main() {
//!    let table = "CREATE TABLE financial_report
//!    {
//!        id INT,
//!        currency_name TEXT,
//!        is_usable BOOL
//!    }";
//!    let res = parse_sql(table);
//!    println!("Parsed: {:#?}", res);
//! }
//! ```
//!
//! In this example, the `parse_sql` function is used to parse the provided SQL `CREATE TABLE` statement,
//! and the parsed result is printed using `println!` (with {:?}).

use std::str::FromStr;
extern crate pest;
extern crate thiserror;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "sql_grammar.pest"]
/// SQL parser struct
pub struct SQLParser;

#[derive(Debug, Error)]
/// Using thiserror::Error for error handling
pub enum SqlParseError {
    /// Invalid data
    #[error("Invalid data")]
    InvalidData,
}

#[derive(Debug)]
/// SQL data types.
pub enum SqlType {
    /// Integer type
    Int,
    /// Text type
    Text,
    /// Boolean type
    Bool,
}

/// FromStr implementation for SqlType
impl FromStr for SqlType {
    type Err = SqlParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INT" => Ok(SqlType::Int),
            "TEXT" => Ok(SqlType::Text),
            "BOOL" => Ok(SqlType::Bool),
            _ => Err(SqlParseError::InvalidData),
        }
    }
}

/// Implemented PartialEq for SqlType
impl PartialEq for SqlType {
    fn eq(&self, other: &SqlType) -> bool {
        match (&self, other) {
            (SqlType::Int, SqlType::Int) => true,
            (SqlType::Text, SqlType::Text) => true,
            (SqlType::Bool, SqlType::Bool) => true,
            (_, _) => false,
        }
    }
}

/// Represents information about a database column.
#[derive(Debug)]
pub struct ColumnInfo {
    /// Column name
    pub column_name: String,
    /// Column type
    pub column_type: SqlType,
}

/// A variant of `ColumnInfo` with options for column name and column type.
#[derive(Debug)]
pub struct ColumnInfoOption {
    pub column_name: Option<String>,
    pub column_type: Option<SqlType>,
}

/// Represents the structure of a SQL `CREATE TABLE` statement.
#[derive(Debug)]
pub struct CreateTable {
    /// Table name
    pub table_name: String,
    /// Columns info
    pub column_info: Vec<ColumnInfo>,
}

/// A variant of `CreateTable` with options for table name and column definitions.
#[derive(Debug)]
pub struct CreateTableOption {
    pub table_name: Option<String>,
    pub column_info: Vec<Option<ColumnInfoOption>>,
}

/// Represents parsed SQL statements.
#[derive(Debug)]
pub enum Parsed {
    CreateTable(CreateTable),
}

/// This function, unwrap_column_info, recursively unwraps and extracts column information from a vector
/// of optional ColumnInfoOption instances, returning a vector of ColumnInfo.
pub fn unwrap_column_info(column_info: &mut Vec<Option<ColumnInfoOption>>) -> Vec<ColumnInfo> {
    if column_info.is_empty() {
        return Vec::<ColumnInfo>::new();
    }

    let cdef = column_info.pop().unwrap().unwrap();
    let cd = ColumnInfo {
        column_name: cdef.column_name.unwrap(),
        column_type: cdef.column_type.unwrap(),
    };

    let mut result = unwrap_column_info(column_info);
    result.push(cd);
    result
}

/// This function parses a create table statement, extracting the table name, column names, and types to construct a
/// CreateTable struct.
pub fn parse_create_table(pairs: pest::iterators::FlatPairs<'_, Rule>) -> CreateTable {
    let mut table_names = CreateTableOption {
        table_name: None,
        column_info: Vec::new(),
    };

    let mut column_name: Option<String> = None;

    for child in pairs {
        match child.as_rule() {
            Rule::table_name => {
                let table_name = child.as_str();
                table_names.table_name = Some(String::from(table_name))
            }
            Rule::column_name => {
                column_name = Some(String::from(child.as_str()));
            }
            Rule::column_type => {
                // ordering ensures column_name is set
                table_names.column_info.push(Some(ColumnInfoOption {
                    column_name: column_name.clone(),
                    column_type: Some(SqlType::from_str(child.as_str()).unwrap()),
                }));
            }
            _ => (),
        }
    }
    let column_info = unwrap_column_info(&mut table_names.column_info);

    CreateTable {
        table_name: table_names.table_name.unwrap(),
        column_info,
    }
}

/// This function takes a SQL query as input and returns a parsed result.
pub fn parse_sql(query: &str) -> Result<Parsed, SqlParseError> {
    let parsed = SQLParser::parse(Rule::sql_grammar, query)
        .map_err(|_e| SqlParseError::InvalidData)?
        .next()
        .ok_or(SqlParseError::InvalidData)?;

    let pairs = parsed.into_inner();
    if let Some(_child) = pairs.clone().flatten().next() {
        let create_table = parse_create_table(pairs.clone().flatten());
        return Ok(Parsed::CreateTable(create_table));
    }

    Err(SqlParseError::InvalidData)
}
