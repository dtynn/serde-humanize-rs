use de::DE;
use humanize_rs::bytes::Bytes;
use serde::de::{Deserializer, Error, Unexpected, Visitor};
use std::fmt;

impl DE for usize {
    fn de<'de, D>(d: D) -> Result<Self, D::Error>
    where
        Self: Sized,
        D: Deserializer<'de>,
    {
        struct V;

        impl<'v> Visitor<'v> for V {
            type Value = usize;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expecting byte-size")
            }

            fn visit_str<E>(self, s: &str) -> Result<usize, E>
            where
                E: Error,
            {
                let bytes = s
                    .parse::<Bytes>()
                    .map_err(|_| Error::invalid_value(Unexpected::Str(s), &self))?;

                Ok(bytes.size())
            }
        }

        d.deserialize_str(V)
    }
}
