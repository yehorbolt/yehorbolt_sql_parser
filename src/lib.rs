use std::str::FromStr;
use std::string::ParseError;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "sql_grammar.pest"]
pub struct SQLParser;

#[derive(Debug)]
pub enum SqlType {
    Int,
    Text,
    Bool,
}

impl FromStr for SqlType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "INT" {
            Ok(SqlType::Int)
        } else if s == "TEXT" {
            Ok(SqlType::Text)
        } else if s == "BOOL" {
            Ok(SqlType::Bool)
        } else {
            unreachable!();
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
pub struct ColumnDef {
    pub column_name: String,
    pub column_type: SqlType,
}

#[derive(Debug)]
struct ColumnDefOption {
    column_name: Option<String>,
    column_type: Option<SqlType>,
}

#[derive(Debug)]
pub struct CreateTable {
    pub table_name: String,
    pub column_defs: Vec<ColumnDef>,
}

#[derive(Debug)]
struct CreateTableOption {
    table_name: Option<String>,
    column_defs: Vec<Option<ColumnDefOption>>,
}

#[derive(Debug)]
pub enum ParsedStmnt {
    CreateTable(CreateTable),
}

fn unwrap_column_defs(column_defs: &mut Vec<Option<ColumnDefOption>>) -> Vec<ColumnDef> {
    if column_defs.is_empty() {
        return Vec::<ColumnDef>::new();
    }
    let cdef = column_defs.pop().unwrap().unwrap();
    let cd = ColumnDef {
        column_name: cdef.column_name.unwrap(),
        column_type: cdef.column_type.unwrap(),
    };
    let mut result = unwrap_column_defs(column_defs);
    result.push(cd);
    result
}

fn parse_create_table(pairs: pest::iterators::FlatPairs<'_, Rule>) -> CreateTable {
    let mut ct_opt = CreateTableOption {
        table_name: None,
        column_defs: Vec::new(),
    };
    let mut column_name: Option<String> = None;

    for child in pairs {
        match child.as_rule() {
            Rule::table_name => {
                let table_name = child.as_str();
                ct_opt.table_name = Some(String::from(table_name))
            }
            Rule::column_name => {
                column_name = Some(String::from(child.as_str()));
            }
            Rule::column_type => {
                // ordering ensures column_name is set
                ct_opt.column_defs.push(Some(ColumnDefOption {
                    column_name: column_name.clone(),
                    column_type: Some(SqlType::from_str(child.as_str()).unwrap()),
                }));
            }
            _ => (),
        }
    }
    let column_defs = unwrap_column_defs(&mut ct_opt.column_defs);
    CreateTable {
        table_name: ct_opt.table_name.unwrap(),
        column_defs,
    }
}

pub fn parse_sql(stmnt: &str) -> ParsedStmnt {
    let parsed_stmnt = SQLParser::parse(Rule::sql_grammar, stmnt)
        .expect("grammar parse failed")
        .next()
        .unwrap();

    let mut result: Option<ParsedStmnt> = None;

    let pairs = parsed_stmnt.into_inner();
    for _child in pairs.clone().flatten() {
        let create_table = parse_create_table(pairs.clone().flatten());
        result = Some(ParsedStmnt::CreateTable(create_table))
    }
    result.unwrap()
}
