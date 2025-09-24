use serde::{Deserialize, Serialize};
pub mod arrays {
    use std::{convert::TryInto, marker::PhantomData};
    use serde::{
        de::{SeqAccess, Visitor},
        ser::SerializeTuple, Deserialize, Deserializer, Serialize, Serializer,
    };
    pub fn serialize<S: Serializer, T: Serialize, const N: usize>(
        data: &[T; N],
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        let mut s = ser.serialize_tuple(N)?;
        for item in data {
            s.serialize_element(item)?;
        }
        s.end()
    }
    struct ArrayVisitor<T, const N: usize>(PhantomData<T>);
    impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
    where
        T: Deserialize<'de>,
    {
        type Value = [T; N];
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str(&format!("an array of length {}", N))
        }
        #[inline]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut data = Vec::with_capacity(N);
            for _ in 0..N {
                match (seq.next_element())? {
                    Some(val) => data.push(val),
                    None => return Err(serde::de::Error::invalid_length(N, &self)),
                }
            }
            match data.try_into() {
                Ok(arr) => Ok(arr),
                Err(_) => unreachable!(),
            }
        }
    }
    pub fn deserialize<'de, D, T, const N: usize>(
        deserializer: D,
    ) -> Result<[T; N], D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        deserializer.deserialize_tuple(N, ArrayVisitor::<T, N>(PhantomData))
    }
}
#[repr(transparent)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ArrayWrapper<T, const N: usize> {
    #[serde(with = "arrays")]
    #[serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))]
    #[serde(untagged)]
    Arr([T; N]),
}

pub fn serde_ignore_none<T>() -> Option<T> {
    None
}

pub fn arbitrary_anyhow_error(u: &mut arbitrary::Unstructured) -> arbitrary::Result<anyhow::Error> {
    u.arbitrary::<String>().map(|s| anyhow::anyhow!(s))
}

pub struct SerdeAnyhowError;

use serde::{Serializer, Deserializer};
use serde_with::{SerializeAs, DeserializeAs};

impl SerializeAs<anyhow::Error> for SerdeAnyhowError {
    fn serialize_as<S>(source: &anyhow::Error, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(source)
    }
}

impl<'de> DeserializeAs<'de, anyhow::Error> for SerdeAnyhowError {
    fn deserialize_as<D>(deserializer: D) -> Result<anyhow::Error, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Visitor, Error};
        struct Helper;
        impl<'de> Visitor<'de> for Helper {
            type Value = anyhow::Error;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(anyhow::anyhow!(value.to_owned()))
            }
        }
        deserializer.deserialize_str(Helper)
    }
}

pub struct SerdeOrdering;

impl SerializeAs<std::cmp::Ordering> for SerdeOrdering {
    fn serialize_as<S>(source: &std::cmp::Ordering, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i8(*source as i8)
    }
}

impl<'de> DeserializeAs<'de, std::cmp::Ordering> for SerdeOrdering {
    fn deserialize_as<D>(deserializer: D) -> Result<std::cmp::Ordering, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::Deserialize;
        let integer = i8::deserialize(deserializer)?;
        match integer {
            -1 => Ok(std::cmp::Ordering::Less),
            0 => Ok(std::cmp::Ordering::Equal),
            1 => Ok(std::cmp::Ordering::Greater),
            _ => Err(serde::de::Error::custom("expect -1, 0 or 1")),
        }
    }
}

macro_rules! custom_float_serde {
    ($wrapper:ident, $fty:ident, $ityser:ident, $ity:ident) => {
        pub struct $wrapper;

        impl SerializeAs<$fty> for $wrapper {
            fn serialize_as<S>(source: &$fty, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.$ityser(source.to_bits())
            }
        }
        
        impl<'de> DeserializeAs<'de, $fty> for $wrapper {
            fn deserialize_as<D>(deserializer: D) -> Result<$fty, D::Error>
            where
                D: Deserializer<'de>,
            {
                use serde::Deserialize;
        
                Ok($fty::from_bits($ity::deserialize(deserializer)?))
            }
        }

    };
}
custom_float_serde!(MyFloat32, f32, serialize_u32, u32);
custom_float_serde!(MyFloat64, f64, serialize_u64, u64);
