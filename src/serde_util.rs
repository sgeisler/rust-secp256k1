use core::fmt;
use core::marker::PhantomData;
use core::str;

pub struct HexVisitor<T> {
    expectation: &'static str,
    _pd: PhantomData<T>,
}

impl<T> HexVisitor<T> {
    pub fn new(expectation: &'static str) -> Self {
        HexVisitor {
            expectation,
            _pd: PhantomData
        }
    }
}

impl<'de, T: str::FromStr> ::serde::de::Visitor<'de> for HexVisitor<T>
    where <T as str::FromStr>::Err: fmt::Display
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.expectation)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: ::serde::de::Error,
    {
        if let Ok(hex) = str::from_utf8(v) {
            str::FromStr::from_str(hex).map_err(E::custom)
        } else {
            Err(E::invalid_value(::serde::de::Unexpected::Bytes(v), &self))
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: ::serde::de::Error,
    {
        str::FromStr::from_str(v).map_err(E::custom)
    }
}

pub struct BytesVisitor<F> {
    expectation: &'static str,
    parse_fn: F,
}

impl<F> BytesVisitor<F> {
    pub fn new(expectation: &'static str, parse_fn: F) -> Self {
        BytesVisitor {
            expectation,
            parse_fn
        }
    }
}

impl<'de, F, T, Err> ::serde::de::Visitor<'de> for BytesVisitor<F>
    where F: Fn(&[u8]) -> Result<T, Err>,
          Err: fmt::Display
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.expectation)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: ::serde::de::Error,
    {
        (self.parse_fn)(v).map_err(E::custom)
    }
}

