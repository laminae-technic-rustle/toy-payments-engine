use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

/* 
 * Precision up to four digits behind comma 
 * f64 for easy division further down
 * */
const MONETARY_PRECISION: f64 = 10000.0;

#[derive(Debug, Deserialize, Serialize)]
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
 * Deserialization and Serialization will be done to 64 bit unsigned integers so
 * 1. No floating point arithmetic - should be faster, but more importantly, 
 *    correct, with no rounding errors.
 * 2. I'm making the assumption here that the biggest number for amount is less
 *    than 1.844.674.407.370.955,0000
 *            16.150.000.000.000
 *    The biggest btc transaction in sats is over a 100 times smaller. I think 
 *    we're safe for now. If need be, this can be upped to 128 bits if need be
 * */
#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    #[serde(deserialize_with = "from_float")]
    #[serde(serialize_with = "to_float")]
    pub amount: u64,
}

fn from_float<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>()
        .map_err(D::Error::custom)
        .map(|f| (f * MONETARY_PRECISION) as u64)
}

fn to_float<S>(x: &u64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    /* Technically, this may loose precision */
    s.serialize_f64((*x as f64) / MONETARY_PRECISION)
}
