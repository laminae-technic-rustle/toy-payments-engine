#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::io::{self, Write};
use std::{env, process};

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

                /* --------------------- */
                /* Write correct results */
                /* --------------------- */
                let mut writer = csv::Writer::from_writer(vec![]);
                parsed_transactions.iter().for_each(|x| {
                    /*
                     * I'm ignoring the error's here, rely'ing on the soundness of my types,
                     * ideally, this should be handled correctly.
                     * */
                    let _ = writer.serialize(x);
                });

                /* This is rather unreasonable, should handle errs properly */
                let std_output = &writer.into_inner().unwrap_or(vec![]);
                let _ = io::stdout().write_all(std_output);

                /* --------------------- */
                /* Write errors if any   */
                /* --------------------- */
                if failed_transactions.len() > 0 {
                    eprintln!("Some transactions could not be handled. See output below:");
                    failed_transactions
                        .iter()
                        .for_each(|e| eprintln!("- {:?}", e));
                    process::exit(1)
                } else {
                    process::exit(0)
                }
            }
            Err(errors) => {
                eprintln!("Failed to parse CSV input");
                eprintln!("{:?}", errors);
            }
        },
    )
}
