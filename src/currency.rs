use serde::{de::Error, Deserialize, Deserializer, Serializer};

/*
 * Precision up to four digits behind comma
 * */
const PRECISION: f64 = 10000.0;

/*
 * To deal with currency properly, we'll make a type with which it is almost
 * impossible do anything wrong. This means we'll get compile time guarantees
 * of wether or not we're dealing with currency properly. Currency is immutable
 * by design. Any operation with currency get's new currency.
 *
 * Deserialization and Serialization will be done into u64 so:
 * 1. No floating point arithmetic - should be faster, but more importantly,
 *    correct, with no rounding errors.
 * 2. I'm making the assumption here that the biggest number for amount is less
 *    than 1.844.674.407.370.955,0000
 *            16.150.000.000.000
 *    The biggest btc transaction ever, in sats is over a 100 times smaller.
 *    I think we're safe for now. If need be, this can be upped to 128 bits if
 *    need be.
 * */

#[derive(Debug, Clone, Copy)]
pub struct Currency(pub u64);

fn to_f64(x: Currency) -> f64 {
    x.0 as f64
}

/* -------------------------- */
/* ---- Money Operations ---- */
/* -------------------------- */
pub fn from_float(x: f64) -> Currency {
    Currency((x * PRECISION) as u64)
}

pub fn add(x: Currency, y: Currency) -> Currency {
    Currency(x.0 + y.0)
}

pub fn unsafe_subtract(x: Currency, y: Currency) -> Currency {
    Currency(x.0 - y.0)
}

pub fn safe_subtract_verbose(x: Currency, y: Currency) -> Option<Currency> {
    if x.0 > y.0 {
        Some(Currency(x.0 - y.0))
    } else {
        None
    }
}

pub fn safe_subtract_silent(x: Currency, y: Currency) -> Currency {
    if x.0 > y.0 {
        Currency(x.0 - y.0)
    } else {
        x
    }
}

/* ------------------------- */
/* Serializer / Deserializer */
/* ------------------------- */
pub fn from_float_string<'de, D>(deserializer: D) -> Result<Currency, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>()
        .map_err(D::Error::custom)
        .map(|x| Currency((x * PRECISION) as u64))
}

pub fn to_float_string<S>(x: &Currency, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_f64((to_f64(*x)) / PRECISION)
}
