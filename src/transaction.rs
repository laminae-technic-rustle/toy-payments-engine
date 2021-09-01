use crate::currency::{from_float_string, Currency};
use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "chargeback")]
    Chargeback,
}

/*
 * Ideally, these are enums with the struct inside, so dispute, resolve, and 
 * chargeback don't have an amount and it's safe at the typelevel. 
 * Unfortunately, tagged unions don't deserialize properly when coming in from 
 * CSV / writing a custom deserializer for this is a bit overkill for now.
 * */
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    /* If this is empty string, or null, it will serialize to 0. */
    #[serde(deserialize_with = "from_float_string")]
    pub amount: Currency,
}
