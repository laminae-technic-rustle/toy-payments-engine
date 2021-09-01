use crate::transaction;

#[derive(Debug, Eq, PartialEq)]
pub enum Csv {
    FileReadError(String),
    ParseError(Vec<String>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum UnsettledReason {
    InsufficientFunds,
    TransactionOrAccountNotFound,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TransactionError {
    AccountlessAction(transaction::Transaction),
    UnsettledWithdrawal(transaction::Transaction, UnsettledReason),
    UnsettledDispute(transaction::Transaction, UnsettledReason),
    UnsettledResolve(transaction::Transaction, UnsettledReason),
    UnsettledChargeback(transaction::Transaction, UnsettledReason),
}
