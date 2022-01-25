//! Simple deserializer for chrono::NaiveDateTime.
//!
//! This module implements a proper way for serde::de to traverse a string
//! representing a chrono::NaiveDateTime. It accepts:
//!
//!   - ISO 8601: Javascripts String.prototype.toISOString().
//!   - Abbreviated form: like "2021-12-01 09:20:00". If no time is given, then
//!     "00:00:00" will be assumed.
//!
//! This deserializer could be further generalized, but I've kept it simple and
//! maintain the formats we might get from HTTP clients.

use chrono::NaiveDateTime;
use serde::de;
use std::fmt;

/// The complete ISO 8601.
const ISO_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

/// Abbreviated format sometimes given by clients.
const ABBREV_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// Given an abbreviated-formatted string, append to it a default time if it
/// doesn't have any. The end result should always match `ABBREV_FORMAT`.
fn corrected_time(s: &str) -> String {
    if s.contains(':') {
        s.to_owned()
    } else {
        format!("{} 00:00:00", s)
    }
}

/// Private struct to visit chrono::NaiveDateTime.
struct NaiveDateTimeVisitor;

impl<'de> de::Visitor<'de> for NaiveDateTimeVisitor {
    type Value = NaiveDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string representing a valid datetime")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match NaiveDateTime::parse_from_str(s, ISO_FORMAT) {
            Ok(t) => Ok(t),
            Err(e) => {
                // chrono::ParseError is pretty lousy because it does not export
                // its private chrono::ParseErrorKind. Thus, instead of
                // performing a proper `match` statement again, we have to do
                // crappy string comparisons...
                if e.to_string() == "premature end of input"
                    || e.to_string() == "input contains invalid characters"
                {
                    let cs = corrected_time(s);
                    match NaiveDateTime::parse_from_str(cs.as_str(), ABBREV_FORMAT) {
                        Ok(t) => Ok(t),
                        Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
                    }
                } else {
                    Err(de::Error::invalid_value(de::Unexpected::Str(s), &self))
                }
            }
        }
    }
}

/// Deserializer for chrono::NaiveDateTime attributes, which allows some formats
/// specific to this application (see module's top documentation).
pub fn from_datetime<'de, D>(d: D) -> Result<NaiveDateTime, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_str(NaiveDateTimeVisitor)
}

/// Deserializer for Option<chrono::NaiveDateTime> attributes. This is similar
/// to its chrono::NaiveDateTime counterpart but with the Option<NaiveDateTime>
/// wrapping. In case of a parsing error, it simply returns a valid result with
/// None.
pub fn from_optional_datetime<'de, D>(d: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: de::Deserializer<'de>,
{
    match d.deserialize_str(NaiveDateTimeVisitor) {
        Ok(v) => Ok(Some(v)),
        Err(_) => Ok(None),
    }
}

/// Default value for the Option<chrono::NaiveDateTime> type. It simply returns
/// None, but apparently serde really wanted a function for this.
pub fn default_optional_datetime() -> Option<NaiveDateTime> {
    None
}
