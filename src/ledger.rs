use crate::transaction::{Transaction, TransactionType};

pub fn parse_transactions(transactions: &Vec<Transaction>) {
    transactions
        .iter()
        .for_each(|transaction| match transaction.tx_type {
            TransactionType::Deposit => println!("Depositing"),
            TransactionType::Withdrawal => println!("Withdrawal"),
            TransactionType::Dispute => println!("Dispute"),
            TransactionType::Resolve => println!("Resolve"),
            TransactionType::Chargeback => println!("Chargeback"),
        })
}
