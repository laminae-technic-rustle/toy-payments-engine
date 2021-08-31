use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

pub mod account;
pub mod currency;
pub mod error;
pub mod ledger;
pub mod reader;
pub mod transaction;

fn main() {
    let ledger_map = HashMap::new(); // This could be something taken from outside

    env::args().collect::<Vec<String>>().get(1).map_or_else(
        || println!("Please provide a path to the CSV file with transactions"),
        |filepath| match reader::read_file_from_path(filepath) {
            Ok(transactions) => {
                // TODO - Refactor to writer file
                let mut writer = csv::Writer::from_writer(vec![]);

                ledger::parse_transactions(ledger_map, &transactions)
                    .into_values()
                    .collect::<Vec<account::Account>>()
                    .iter()
                    .for_each(|x| {
                        let _ = writer.serialize(x);
                    });

                let data = writer.into_inner().unwrap_or(vec![]);
                let _ = io::stdout().write_all(&data);
            }
            Err(errors) => {
                println!("Failed to parse CSV input");
                println!("{:?}", errors);
            }
        },
    )
}
