extern crate clap;
extern crate yehorbolt_sql_parser;

use clap::Parser;
use std::fs;
use std::path::PathBuf;
use yehorbolt_sql_parser::parse_sql;

#[derive(Parser)]
#[command(name = "Cli for SQL parser")]
#[command(author = "Bolotov Y. <bolotov.yehor@gmail.com>")]
#[command(version = "0.1.7")]
#[command(about = "A simple SQL parser written in Rust", long_about = None)]
struct Cli {
    #[arg(short = 'p', long)]
    /// Parse SQL CREATE TABLE statement from file
    parse: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(_file) = cli.parse.as_deref() {
        let file = cli.parse.unwrap();
        let file_content = fs::read_to_string(file).unwrap();
        let res = parse_sql(&file_content);

        println!("Parsed: {:#?}", res.unwrap())
    }

    Ok(())
}
