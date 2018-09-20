use de::DE;
use humanize_rs::duration::parse;
use serde::de::{Deserializer, Error, Unexpected, Visitor};
use std::fmt;
use std::time::Duration;

impl DE for Duration {
    fn de<'de, D>(d: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl<'v> Visitor<'v> for V {
            type Value = Duration;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expecting duration")
            }

            fn visit_str<E>(self, s: &str) -> Result<Duration, E>
            where
                E: Error,
            {
                parse(s).map_err(|_| Error::invalid_value(Unexpected::Str(s), &self))
            }
        }

        d.deserialize_str(V)
    }
}
