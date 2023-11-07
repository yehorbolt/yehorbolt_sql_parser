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
pub enum SqlType {
    Int,
    Text,
    Bool,
}

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

#[derive(Debug)]
pub struct ColumnInfo {
    pub column_name: String,
    pub column_type: SqlType,
}

#[derive(Debug)]
struct ColumnInfoOption {
    column_name: Option<String>,
    column_type: Option<SqlType>,
}

#[derive(Debug)]
pub struct CreateTable {
    pub table_name: String,
    pub column_defs: Vec<ColumnInfo>,
}

#[derive(Debug)]
struct CreateTableOption {
    table_name: Option<String>,
    column_defs: Vec<Option<ColumnInfoOption>>,
}

#[derive(Debug)]
pub enum Parsed {
    CreateTable(CreateTable),
}

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
