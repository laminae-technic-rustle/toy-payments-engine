use crate::currency::{from_float, to_float_string, Currency};
use serde::Serialize;

#[derive(Eq, PartialEq, Debug, Serialize)]
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

pub fn new_from_client_id(client: u16) -> Account {
    Account {
        client: client,
        available: from_float(0.0),
        held: from_float(0.0),
        total: from_float(0.0),
        locked: false,
    }
}
