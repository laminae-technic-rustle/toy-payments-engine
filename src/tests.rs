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
