use std::{ env, process };
use std::io::{self, Write};

pub mod account;
pub mod currency;
pub mod error;
pub mod ledger;
pub mod option;
pub mod reader;
pub mod transaction;

fn main() {
    env::args().collect::<Vec<String>>().get(1).map_or_else(
        || eprintln!("Please provide a path to the CSV file with transactions"),
        |filepath| match reader::read_file_from_path(filepath) {
            Ok(transactions) => {
                let (parsed_transactions, failed_transactions) =
                    ledger::parse_transactions(&transactions);

                let mut writer = csv::Writer::from_writer(vec![]);
                parsed_transactions.iter().for_each(|x| {
                    let _ = writer.serialize(x);
                });

                let data = writer.into_inner().unwrap_or(vec![]);
                let _ = io::stdout().write_all(&data);

                if failed_transactions.len() > 0 {
                    eprintln!("Some transactions could not be handled.");
                    failed_transactions
                        .iter()
                        .for_each(|e| eprintln!("{:?}", e));
                    process::exit(1)
                }
            }
            Err(errors) => {
                eprintln!("Failed to parse CSV input");
                eprintln!("{:?}", errors);
            }
        },
    )
}
