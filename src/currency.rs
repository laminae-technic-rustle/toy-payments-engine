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
 * Deserialization and Serialization will be done into i64 so:
 * 1. No floating point arithmetic - should be faster, but more importantly,
 *    correct, with no rounding errors.
 * 2. I'm making the assumption here that the biggest number for amount is less
 *    than what fits in an i64. The biggest btc transaction ever, in sats is 
 *    over a 100 times smaller.  I think we're safe for now. If need be, this 
 *    can be upped to 128 bits if need be.
 * */

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Currency(pub i64);

fn to_f64(x: Currency) -> f64 {
    x.0 as f64
}

/* -------------------------- */
/* ---- Money Operations ---- */
/* -------------------------- */
pub fn from_float(x: f64) -> Currency {
    Currency((x * PRECISION) as i64)
}

pub fn add(x: Currency, y: Currency) -> Currency {
    Currency(x.0 + y.0)
}

pub fn unsafe_subtract(x: Currency, y: Currency) -> Currency {
    Currency(x.0 - y.0)
}

pub fn safe_subtract_verbose(x: Currency, y: Currency) -> Option<Currency> {
    if x.0 >= y.0 {
        Some(Currency(x.0 - y.0))
    } else {
        None
    }
}

pub fn safe_subtract_silent(x: Currency, y: Currency) -> Currency {
    if x.0 >= y.0 {
        Currency(x.0 - y.0)
    } else {
        x
    }
}

/* ------------------------- */
/* Serializer / Deserializer */
/* ------------------------- */
/*
 * The serializer parses an empty string as 0. While technically incorrect, 
 * this is the best way to deal with it in the context of this toy
 * */
pub fn from_float_string<'de, D>(deserializer: D) -> Result<Currency, D::Error>
where
    D: Deserializer<'de>,
{
    match Deserialize::deserialize(deserializer)? {
        "" => Ok(Currency(0)),
        s => s
            .parse::<f64>()
            .map_err(D::Error::custom)
            .map(|x| Currency((x * PRECISION) as i64)),
    }
}

pub fn to_float_string<S>(x: &Currency, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_f64((to_f64(*x)) / PRECISION)
}
