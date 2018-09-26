use serde::de::{Deserializer, Error, Unexpected, Visitor};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

pub trait DE {
    fn de<'de, D>(d: D) -> Result<Self, D::Error>
    where
        Self: Sized,
        D: Deserializer<'de>;
}

pub trait DEFromStr {}

/// Deserialize function.
pub fn deserialize<'de, T, D>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DE + Sized,
{
    <T as DE>::de(d)
}

impl<T: DE> DE for Option<T> {
    fn de<'de, D>(d: D) -> Result<Self, D::Error>
    where
        Self: Sized,
        D: Deserializer<'de>,
    {
        struct OptVisitor<T> {
            marker: PhantomData<T>,
        }

        impl<'de, T: DE> Visitor<'de> for OptVisitor<T> {
            type Value = Option<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("option")
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Option<T>, E>
            where
                E: Error,
            {
                Ok(None)
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Option<T>, E>
            where
                E: Error,
            {
                Ok(None)
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Option<T>, D::Error>
            where
                D: Deserializer<'de>,
            {
                T::de(deserializer).map(Some)
            }
        }

        d.deserialize_option(OptVisitor {
            marker: PhantomData,
        })
    }
}

impl<T: DEFromStr + FromStr> DE for T {
    fn de<'de, D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V<T> {
            marker: PhantomData<T>,
        };

        impl<'v, T: DEFromStr + FromStr> Visitor<'v> for V<T> {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expecting parse-able string")
            }

            fn visit_str<E>(self, s: &str) -> Result<T, E>
            where
                E: Error,
            {
                let parsed = s
                    .parse::<T>()
                    .map_err(|_| Error::invalid_value(Unexpected::Str(s), &self))?;

                Ok(parsed)
            }
        }

        d.deserialize_str(V {
            marker: PhantomData,
        })
    }
}
