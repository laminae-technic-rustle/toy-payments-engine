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
     * We cache:
     * - deposits, as they're the only transaction that can reasonably be disputed automatically
     * - disputes: to check that when we're resolving, or doing a chargeback, we're only
     *     handling things that are actually disputed
     * - accounts: are created on demand, whenever a deposit occurs.
     *     NOT for anything else, imho - it doesn't make sense to keep accounts
     *     lingering around that don't have any funds, and are only trying to
     *     withdraw / dispute / resolve / chargeback
     * By design, any dispute refers to a deposit. We could eventually add a
     * flag as to wether it was disputed or not, which would be more memory efficient.
     */
    let mut deposits: HashMap<u32, Transaction> = HashMap::new();
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
                    option::sequence((
                        safe_subtract_verbose(account.available, transaction.amount),
                        safe_subtract_verbose(account.total, transaction.amount),
                    ))
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
            TransactionType::Dispute => option::sequence((
                accounts.get(&transaction.client),
                deposits.get(&transaction.tx),
            ))
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
            TransactionType::Resolve => option::sequence((
                accounts.get(&transaction.client),
                disputes.get(&transaction.tx),
            ))
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
            TransactionType::Chargeback => option::sequence((
                accounts.get(&transaction.client),
                disputes.get(&transaction.tx),
            ))
            .map(|(account, past_transaction)| {
                match option::sequence((
                    safe_subtract_verbose(account.held, past_transaction.amount),
                    safe_subtract_verbose(account.total, past_transaction.amount),
                )) {
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
