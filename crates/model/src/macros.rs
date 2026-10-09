

//! Model specific macros.

#[macro_export]
macro_rules! enum_strum_serde {
    ($type:ty) => {
        impl $crate::__serde::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: $crate::__serde::Serializer,
            {
                serializer.serialize_str(&::std::string::ToString::to_string(self))
            }
        }

        impl<'de> $crate::__serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: $crate::__serde::Deserializer<'de>,
            {
                let value =
                    <::std::borrow::Cow<'de, str> as $crate::__serde::Deserialize>::deserialize(
                        deserializer,
                    )?;
                <$type as ::core::str::FromStr>::from_str(value.as_ref())
                    .map_err($crate::__serde::de::Error::custom)
            }
        }
    };
}
