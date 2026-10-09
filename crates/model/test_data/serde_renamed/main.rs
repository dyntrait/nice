

use std::{fmt::Display, str::FromStr};

use nice_model::enum_strum_serde;

#[derive(Debug)]
enum ConsumerEnum {
    Value,
}

impl Display for ConsumerEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("VALUE")
    }
}

impl FromStr for ConsumerEnum {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "VALUE" => Ok(Self::Value),
            _ => Err("invalid value"),
        }
    }
}

enum_strum_serde!(ConsumerEnum);

fn assert_serde<T>()
where
    T: renamed_serde::Serialize + for<'de> renamed_serde::Deserialize<'de>,
{
}

fn main() {
    assert_serde::<ConsumerEnum>();
}
