use crate::account::Account;
use crate::currency;
use crate::transaction::{Transaction, TransactionType};
use std::collections::HashMap;

pub fn parse_transactions(
    mut ledger_map: HashMap<u16, Account>,
    transactions: &Vec<Transaction>,
) -> HashMap<u16, Account> {
    transactions.iter().for_each(|transaction| {
        // Fetch Account First
        let client = ledger_map.get_mut(&transaction.client);

        match (&transaction.tx_type, client) {
            (TransactionType::Deposit, Some(client)) => {
                client.available = currency::add(client.available, transaction.amount)
            }
            (TransactionType::Withdrawal, Some(client)) => {
                client.available =
                    currency::safe_subtract_silent(client.available, transaction.amount)
            }
            (TransactionType::Deposit, None) => {
                ledger_map.insert(
                    transaction.client,
                    Account {
                        client: transaction.client,
                        available: transaction.amount,
                        held: currency::from_float(0.0),
                        total: currency::from_float(0.0),
                        locked: false,
                    },
                );
            }
            (TransactionType::Withdrawal, None) => {
                ledger_map.insert(
                    transaction.client,
                    Account {
                        client: transaction.client,
                        available: currency::from_float(0.0),
                        held: currency::from_float(0.0),
                        total: currency::from_float(0.0),
                        locked: false,
                    },
                );
            }
            (TransactionType::Dispute, _) => println!("Dispute"),
            (TransactionType::Resolve, _) => println!("Resolve"),
            (TransactionType::Chargeback, _) => println!("Chargeback"),
        }
    });

    ledger_map
}
