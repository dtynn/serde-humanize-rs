use de::DE;
use humanize_rs::bytes::Bytes;
use serde::de::{Deserializer, Error, Unexpected, Visitor};
use std::convert::TryInto;
use std::fmt;

macro_rules! impl_de {
    ($t:ident) => {
        impl DE for $t {
            fn de<'de, D>(d: D) -> Result<Self, D::Error>
            where
                Self: Sized,
                D: Deserializer<'de>,
            {
                struct V;

                impl<'v> Visitor<'v> for V {
                    type Value = $t;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("expecting byte-size")
                    }

                    fn visit_str<E>(self, s: &str) -> Result<$t, E>
                    where
                        E: Error,
                    {
                        let bytes = s
                            .parse::<Bytes>()
                            .map_err(|_| Error::invalid_value(Unexpected::Str(s), &self))?;

                        Ok(bytes
                            .size()
                            .try_into()
                            .map_err(|_| Error::invalid_value(Unexpected::Str(s), &self))?)
                    }
                }

                d.deserialize_str(V)
            }
        }
    };
}

impl_de!(usize);
impl_de!(u128);
impl_de!(u64);
impl_de!(u32);
impl_de!(u16);
impl_de!(u8);
