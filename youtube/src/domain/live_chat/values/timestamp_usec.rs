use chrono::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TimestampUsec(DateTime<Utc>);

impl core::ops::Deref for TimestampUsec {
    type Target = DateTime<Utc>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for TimestampUsec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        let timestamp = match value {
            serde_json::Value::Number(num) => num.as_i64().ok_or_else(|| {
                de::Error::custom(format!("Failed to parse from Number: {:?}", num))
            })?,
            serde_json::Value::String(s) => s
                .parse::<i64>()
                .map_err(|_| de::Error::custom(format!("Failed to parse from String: {:?}", s)))?,
            _ => {
                return Err(de::Error::custom(format!(
                    "Unsupported type for timestamp: {:?}",
                    value
                )));
            }
        };

        let datetime = Utc.timestamp_micros(timestamp).single().ok_or_else(|| {
            de::Error::custom(format!("Failed to init DateTime<Utc>: {:?}", timestamp))
        })?;

        Ok(TimestampUsec(datetime))
    }
}

impl Serialize for TimestampUsec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp_usec = self.timestamp_micros().to_string();
        serializer.serialize_str(&timestamp_usec)
    }
}

impl From<TimestampUsec> for DateTime<Utc> {
    fn from(val: TimestampUsec) -> Self {
        val.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    struct Example {
        timestamp_usec: TimestampUsec,
    }

    mod deserialize {
        use super::*;
        use pretty_assertions::assert_eq;
        use test_case::test_case;
        #[test_case(r#"{"timestampUsec":"1733370114906095"}"# ; "when a value of json_data is a string")]
        #[test_case(r#"{"timestampUsec":1733370114906095}"#  ; "when a value of json_data is a number")]
        fn it_deserialize(json_data: &str) -> anyhow::Result<()> {
            let expected = Utc
                .timestamp_micros(1733370114906095)
                .unwrap()
                .timestamp_micros();

            let example: Example = serde_json::from_str(json_data)?;
            let actual = example.timestamp_usec.timestamp_micros();

            assert_eq!(expected, actual);
            Ok(())
        }
    }

    mod serialize {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_serialize() -> anyhow::Result<()> {
            let expected = r#"{"timestampUsec":"1733370114906095"}"#;

            let time = Utc.timestamp_micros(1733370114906095).unwrap();
            let timestamp_usec = TimestampUsec(time);
            let example = Example { timestamp_usec };
            let actual = serde_json::to_string(&example).unwrap();

            assert_eq!(expected, actual);
            Ok(())
        }
    }
}
