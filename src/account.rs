use crate::currency::{to_float_string, Currency};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Account {
    pub client: u16,
    #[serde(serialize_with = "to_float_string")]
    pub available: Currency,
    #[serde(serialize_with = "to_float_string")]
    pub held: Currency,
    #[serde(serialize_with = "to_float_string")]
    pub total: Currency,
    pub locked: bool,
}
