//! # SQL parser
//! This is a simple SQL parser written in Rust using the pest library. 
//! The parser supports parsing CREATE TABLE statements with column definitions.
//!
//! # Example
//! ``` rust
//!    let table = "CREATE TABLE financial_report 
//!    { 
//!        id INT, 
//!        currency_name TEXT, 
//!        is_usable BOOL
//!    }";
//!    let res = parse_sql(table);
//!    println!("Parsed: {:?}", res);
//! ```

use std::str::FromStr;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "sql_grammar.pest"]
pub struct SQLParser;

#[derive(Debug)]
pub enum SqlParseError {
    InvalidSqlType,
}

impl std::fmt::Display for SqlParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlParseError::InvalidSqlType => write!(f, "Invalid SQL type"),
        }
    }
}

impl std::error::Error for SqlParseError {}



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
            _ => Err(SqlParseError::InvalidSqlType),
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
    pub column_name: String,
    pub column_type: SqlType,
}

/// A variant of `ColumnInfo` with options for column name and column type.
#[derive(Debug)]
struct ColumnInfoOption {
    column_name: Option<String>,
    column_type: Option<SqlType>,
}

/// Represents the structure of a SQL `CREATE TABLE` statement.
#[derive(Debug)]
pub struct CreateTable {
    pub table_name: String,
    pub column_defs: Vec<ColumnInfo>,
}

/// A variant of `CreateTable` with options for table name and column definitions.
#[derive(Debug)]
struct CreateTableOption {
    table_name: Option<String>,
    column_defs: Vec<Option<ColumnInfoOption>>,
}

/// Represents parsed SQL statements.
#[derive(Debug)]
pub enum Parsed {
    CreateTable(CreateTable),
}


/// This function, unwrap_column_defs, recursively unwraps and extracts column information from a vector
/// of optional ColumnInfoOption instances, returning a vector of ColumnInfo. If the input vector is empty, it
/// returns an empty vector.
fn unwrap_column_defs(column_info: &mut Vec<Option<ColumnInfoOption>>) -> Vec<ColumnInfo> {
    if column_info.is_empty() {
        return Vec::<ColumnInfo>::new();
    }

    let cdef = column_info.pop().unwrap().unwrap();
    let cd = ColumnInfo {
        column_name: cdef.column_name.unwrap(),
        column_type: cdef.column_type.unwrap(),
    };

    let mut result = unwrap_column_defs(column_info);
    result.push(cd);
    result
}

/// This function parses a create table statement, extracting the table name, column names, and types to construct a
/// CreateTable struct. It uses a CreateTableOption for temporary storage and iterates through parsed pairs to populate the struct.
/// The result is a CreateTable with the table name and column definitions.
fn parse_create_table(pairs: pest::iterators::FlatPairs<'_, Rule>) -> CreateTable {
    let mut table_names = CreateTableOption {
        table_name: None,
        column_defs: Vec::new(),
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
                table_names.column_defs.push(Some(ColumnInfoOption {
                    column_name: column_name.clone(),
                    column_type: Some(SqlType::from_str(child.as_str()).unwrap()),
                }));
            }
            _ => (),
        }
    }
    let column_defs = unwrap_column_defs(&mut table_names.column_defs);

    CreateTable {
        table_name: table_names.table_name.unwrap(),
        column_defs,
    }
}


/// This function takes a SQL query as input and returns a parsed result. It uses an SQL parser (SQLParser)
/// with a specified grammar rule (Rule::sql_grammar) to parse the query. The function then iterates through the parsed result, specifically
/// looking for create_table instances using the parse_create_table function. The parsed result is stored in the Parsed::CreateTable variant,
/// and the final result is returned.
pub fn parse_sql(query: &str) -> Parsed {
    let parsed = SQLParser::parse(Rule::sql_grammar, query)
        .expect("Parsing error")
        .next()
        .unwrap();

    let mut result: Option<Parsed> = None;

    let pairs = parsed.into_inner();
    for _child in pairs.clone().flatten() {
        let create_table = parse_create_table(pairs.clone().flatten());
        result = Some(Parsed::CreateTable(create_table))
    }
    result.unwrap()
}
