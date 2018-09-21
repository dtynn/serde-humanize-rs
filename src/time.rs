use de::DE;
use humanize_rs::time::Time;
use humanize_rs::ParseError;
use serde::de::{Deserializer, Error, Unexpected, Visitor};
use std::fmt;
use std::time::SystemTime;

impl DE for SystemTime {
    fn de<'de, D>(d: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl<'v> Visitor<'v> for V {
            type Value = SystemTime;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expecting rfc3339 datetime string")
            }

            fn visit_str<E>(self, s: &str) -> Result<SystemTime, E>
            where
                E: Error,
            {
                s.parse::<Time>()
                    .and_then(|t| t.to_system_time().ok_or(ParseError::Overflow))
                    .map_err(|_| Error::invalid_value(Unexpected::Str(s), &self))
            }
        }

        d.deserialize_str(V)
    }
}
