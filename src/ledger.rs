use crate::account::{new_from_client_id, Account};
use crate::currency::{add, safe_subtract_verbose, unsafe_subtract};
use crate::error::{TransactionError, UnsettledReason};
use crate::option;
use crate::transaction::{Transaction, TransactionType};
use std::collections::HashMap;

pub fn parse_transactions(
    transactions: &Vec<Transaction>,
) -> (Vec<Account>, Vec<TransactionError>) {
    /*
     * By the way this is stated in the brief, it appears only deposits can be
     * disputed. Disputing a withdrawal, would imply not subtracting the amount
     * to the client, but adding it. I would probably do this differently.
     * */
    let mut deposits: HashMap<u32, Transaction> = HashMap::with_capacity(transactions.len());
    let mut disputes: HashMap<u32, Transaction> = HashMap::new();
    let mut accounts: HashMap<u16, Account> = HashMap::new();
    let mut transaction_errors: Vec<TransactionError> = vec![];

    transactions.iter().for_each(|transaction| {
        let new_account = match &transaction.tx_type {
            /* Deposit */
            TransactionType::Deposit => {
                deposits.insert(transaction.tx, *transaction);
                accounts.get(&transaction.client).map_or(
                    Ok(Account {
                        available: transaction.amount,
                        total: transaction.amount,
                        ..new_from_client_id(transaction.client)
                    }),
                    |account| {
                        Ok(Account {
                            available: add(account.available, transaction.amount),
                            total: add(account.total, transaction.amount),
                            ..*account
                        })
                    },
                )
            }
            /* Withdrawal */
            TransactionType::Withdrawal => accounts.get(&transaction.client).map_or(
                Err(TransactionError::AccountlessAction(*transaction)),
                |account| {
                    option::sequence(
                        safe_subtract_verbose(account.available, transaction.amount),
                        safe_subtract_verbose(account.total, transaction.amount),
                    )
                    .map_or(
                        Err(TransactionError::UnsettledWithdrawal(
                            *transaction,
                            UnsettledReason::InsufficientFunds,
                        )),
                        |(available, total)| {
                            Ok(Account {
                                available,
                                total,
                                ..*account
                            })
                        },
                    )
                },
            ),
            /* Dispute */
            TransactionType::Dispute => option::sequence(
                accounts.get(&transaction.client),
                deposits.get(&transaction.tx),
            )
            .map(|(account, past_transaction)| {
                disputes.insert(past_transaction.tx, *past_transaction);
                Ok(Account {
                    available: unsafe_subtract(account.available, past_transaction.amount),
                    held: add(account.held, past_transaction.amount),
                    ..*account
                })
            })
            .unwrap_or(Err(TransactionError::UnsettledDispute(
                *transaction,
                UnsettledReason::TransactionOrAccountNotFound,
            ))),
            /* Resolve */
            TransactionType::Resolve => option::sequence(
                accounts.get(&transaction.client),
                deposits.get(&transaction.tx),
            )
            .map(|(account, past_transaction)| {
                match safe_subtract_verbose(account.held, past_transaction.amount) {
                    Some(held) => Ok(Account {
                        available: add(account.available, past_transaction.amount),
                        held,
                        ..*account
                    }),
                    None => Err(TransactionError::UnsettledResolve(
                        *transaction,
                        UnsettledReason::InsufficientFunds,
                    )),
                }
            })
            .unwrap_or(Err(TransactionError::UnsettledResolve(
                *transaction,
                UnsettledReason::TransactionOrAccountNotFound,
            ))),
            /* Chargeback */
            TransactionType::Chargeback => option::sequence(
                accounts.get(&transaction.client),
                disputes.get(&transaction.tx),
            )
            .map(|(account, past_transaction)| {
                match option::sequence(
                    safe_subtract_verbose(account.held, past_transaction.amount),
                    safe_subtract_verbose(account.total, past_transaction.amount),
                ) {
                    Some((held, total)) => Ok(Account {
                        held,
                        total,
                        locked: true,
                        ..*account
                    }),
                    None => Err(TransactionError::UnsettledChargeback(
                        *transaction,
                        UnsettledReason::InsufficientFunds,
                    )),
                }
            })
            .unwrap_or(Err(TransactionError::UnsettledChargeback(
                *transaction,
                UnsettledReason::TransactionOrAccountNotFound,
            ))),
        };

        /*
         * If we were able to create the new account variant, we should add it.
         */
        match new_account {
            Ok(account) => {
                accounts.insert(transaction.client, account);
                ()
            }
            Err(e) => transaction_errors.push(e),
        }
    });

    (
        accounts.into_values().collect::<Vec<Account>>(),
        transaction_errors,
    )
}

#[cfg(test)]
mod tests {
    use crate::account::Account;
    use crate::currency::from_float;
    use crate::error::{TransactionError, UnsettledReason};
    use crate::ledger;
    use crate::transaction::{Transaction, TransactionType};
    #[test]
    fn it_should_handle_deposits_and_withdrawals() {
        let transactions = vec![
            Transaction {
                tx_type: TransactionType::Deposit,
                client: 0,
                tx: 0,
                amount: from_float(5.0),
            },
            Transaction {
                tx_type: TransactionType::Withdrawal,
                client: 0,
                tx: 1,
                amount: from_float(2.5),
            },
            Transaction {
                tx_type: TransactionType::Withdrawal,
                client: 0,
                tx: 2,
                amount: from_float(25.0),
            },
        ];
        let (parsed, failed) = ledger::parse_transactions(&transactions);
        let output = vec![Account {
            client: 0,
            available: from_float(2.5),
            held: from_float(0.0),
            total: from_float(2.5),
            locked: false,
        }];
        let failed_output = vec![TransactionError::UnsettledWithdrawal(
            Transaction {
                tx_type: TransactionType::Withdrawal,
                client: 0,
                tx: 2,
                amount: from_float(25.0),
            },
            UnsettledReason::InsufficientFunds,
        )];
        assert_eq!(parsed, output);
        assert_eq!(failed, failed_output);
    }

    #[test]
    fn it_should_handle_deposits_and_disputes() {
        let transactions = vec![
            Transaction {
                tx_type: TransactionType::Deposit,
                client: 0,
                tx: 0,
                amount: from_float(5.0),
            },
            Transaction {
                tx_type: TransactionType::Dispute,
                client: 0,
                tx: 0,
                amount: from_float(0.0),
            },
        ];
        let (parsed, failed) = ledger::parse_transactions(&transactions);
        let output = vec![Account {
            client: 0,
            available: from_float(0.0),
            held: from_float(5.0),
            total: from_float(5.0),
            locked: false,
        }];
        assert_eq!(parsed, output);
        assert_eq!(failed, vec![]);
    }
    #[test]
    fn it_should_handle_deposits_and_disputes_and_resolves() {
        let transactions = vec![
            Transaction {
                tx_type: TransactionType::Deposit,
                client: 0,
                tx: 0,
                amount: from_float(5.0),
            },
            Transaction {
                tx_type: TransactionType::Dispute,
                client: 0,
                tx: 0,
                amount: from_float(0.0),
            },
            Transaction {
                tx_type: TransactionType::Resolve,
                client: 0,
                tx: 0,
                amount: from_float(0.0),
            },
        ];
        let (parsed, failed) = ledger::parse_transactions(&transactions);
        let output = vec![Account {
            client: 0,
            available: from_float(5.0),
            held: from_float(0.0),
            total: from_float(5.0),
            locked: false,
        }];
        assert_eq!(parsed, output);
        assert_eq!(failed, vec![]);
    }
    #[test]
    fn it_should_handle_deposits_and_disputes_and_chargebacks() {
        let transactions = vec![
            Transaction {
                tx_type: TransactionType::Deposit,
                client: 0,
                tx: 0,
                amount: from_float(5.0),
            },
            Transaction {
                tx_type: TransactionType::Dispute,
                client: 0,
                tx: 0,
                amount: from_float(0.0),
            },
            Transaction {
                tx_type: TransactionType::Chargeback,
                client: 0,
                tx: 0,
                amount: from_float(0.0),
            },
        ];
        let (parsed, failed) = ledger::parse_transactions(&transactions);
        let output = vec![Account {
            client: 0,
            available: from_float(0.0),
            held: from_float(0.0),
            total: from_float(0.0),
            locked: true,
        }];
        assert_eq!(parsed, output);
        assert_eq!(failed, vec![]);
    }
}
