extern crate anyhow;
extern crate yehorbolt_sql_parser;

use anyhow::*;
use yehorbolt_sql_parser::*;

#[test]
fn test_parse_create_table() -> anyhow::Result<()> {
    let test_create =
        "CREATE TABLE financial_report { id INT, currency_name TEXT, is_usable BOOL }";
    let Parsed::CreateTable(res) = parse_sql(test_create);

    assert_eq!(res.table_name, "financial_report");
    assert_eq!(res.column_defs[0].column_name, "id");
    assert_eq!(res.column_defs[0].column_type, SqlType::Int);
    assert_eq!(res.column_defs[1].column_name, "currency_name");
    assert_eq!(res.column_defs[1].column_type, SqlType::Text);
    assert_eq!(res.column_defs[2].column_name, "is_usable");
    assert_eq!(res.column_defs[2].column_type, SqlType::Bool);

    Ok(())
}
