use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Number, Value};

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct DecimalNumber(Decimal);

impl DecimalNumber {
    pub const ZERO: Self = Self(Decimal::ZERO);

    #[must_use]
    pub const fn as_decimal(self) -> Decimal {
        self.0
    }
}

impl FromStr for DecimalNumber {
    type Err = rust_decimal::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Decimal::from_str(value).map(Self)
    }
}

impl Serialize for DecimalNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let text = if self.0.is_zero() {
            "0.0".to_owned()
        } else {
            self.0.to_string()
        };
        let number = Number::from_str(&text).map_err(serde::ser::Error::custom)?;
        number.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DecimalNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Number(number) => Decimal::from_str(&number.to_string())
                .map(Self)
                .map_err(serde::de::Error::custom),
            Value::String(text) => Decimal::from_str(&text)
                .map(Self)
                .map_err(serde::de::Error::custom),
            other => Err(serde::de::Error::custom(format!(
                "expected decimal number, found {other}"
            ))),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct ApiResponse<T> {
    #[serde(alias = "Status")]
    pub status: i32,
    #[serde(alias = "Message")]
    pub message: String,
    #[serde(alias = "Data")]
    pub data: Option<T>,
}

pub type RawData = Value;
