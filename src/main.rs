use std::fs;

use anyhow::Ok;
// rcli csv -i input.csv -o output.json
use clap::Parser;
// use std::path::Path;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde[rename_all = "PascalCase"]]
struct Player {
    //#[serde(rename = "Name")]
    name: String,
    //#[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    //#[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    ket: u8,
}

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show Csv or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    //#[arg(short, long, value_parser = verify_input_file)]
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            let mut ret = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let record: Player = result?;
                ret.push(record)
            }
            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(opts.output, json)?;
        }
    }

    Ok(())
}

// fn verify_input_file(filename: &str) -> Result<String, &'static str> {
//     if Path::new(filename).exists() {
//         Ok(filename.into())
//     }else{
//         Err("file does not exits")
//     }
// }
