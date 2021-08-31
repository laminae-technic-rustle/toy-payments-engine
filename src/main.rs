pub mod error;
pub mod ledger;
pub mod reader;
pub mod transaction;

use std::env;

fn main() {
    env::args().collect::<Vec<String>>().get(1).map_or_else(
        || println!("Please provide a path to the CSV file with transactions"),
        |filepath| match reader::read_file_from_path(filepath) {
            Ok(transactions) => {
                ledger::parse_transactions(&transactions);

                let mut writer = csv::Writer::from_writer(vec![]);
                let _ = writer.serialize(transactions);
                let data = String::from_utf8(writer.into_inner().unwrap_or(vec![0]))
                    .unwrap_or(String::from("b"));
                println!("{:?}", data);
            }
            Err(errors) => {
                println!("Failed to parse CSV input");
                println!("{:?}", errors);
            }
        },
    )
}
