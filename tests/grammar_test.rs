extern crate anyhow;
extern crate yehorbolt_sql_parser;

use anyhow::*;
use yehorbolt_sql_parser::*;

#[test]
fn test_parse_create_table() -> anyhow::Result<()> {
    let test_create = "CREATE TABLE financial_report { 
            id INT, 
            currency_name TEXT, 
            is_usable BOOL 
        }";
    let Parsed::CreateTable(res) = parse_sql(test_create)?;

    assert_eq!(res.table_name, "financial_report");
    assert_eq!(res.column_info[0].column_name, "id");
    assert_eq!(res.column_info[0].column_type, SqlType::Int);
    assert_eq!(res.column_info[1].column_name, "currency_name");
    assert_eq!(res.column_info[1].column_type, SqlType::Text);
    assert_eq!(res.column_info[2].column_name, "is_usable");
    assert_eq!(res.column_info[2].column_type, SqlType::Bool);

    Ok(())
}

#[test]
fn test_parse_syntax_error() {
    let invalid_table = "ABOBA TABLE financial_report { 
        id INT, 
        currency_name TEXT, 
        is_usable BOOL 
    }";
    assert!(matches!(
        parse_sql(invalid_table),
        Err(SqlParseError::InvalidData)
    ));
}

#[test]
fn test_parse_create_table_name_error() {
    let invalid_table_name = "CREATE TABLE financial_report ABOBA { 
        id INT, 
        currency_name TEXT, 
        is_usable BOOL 
    }";
    assert!(matches!(
        parse_sql(invalid_table_name),
        Err(SqlParseError::InvalidData)
    ));
}

#[test]
fn test_parse_column_name_error() {
    let invalid_column_name = "CREATE TABLE financial_report { id ABOBA INT }";
    assert!(matches!(
        parse_sql(invalid_column_name),
        Err(SqlParseError::InvalidData)
    ));
}

#[test]
fn test_parse_column_type_error() {
    let invalid_column_type = "CREATE TABLE financial_report { id ABOBA }";
    assert!(matches!(
        parse_sql(invalid_column_type),
        Err(SqlParseError::InvalidData)
    ));
}

#[test]
fn test_parse_punctuation_error() {
    let invalid_punctuation = "CREATE TABLE financial_report , { id INT }";
    assert!(matches!(
        parse_sql(invalid_punctuation),
        Err(SqlParseError::InvalidData)
    ));
}
