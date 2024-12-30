use chrono::prelude::*;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TimestampUsec(DateTime<Utc>);

impl core::ops::Deref for TimestampUsec {
    type Target = DateTime<Utc>;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
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
                )))
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

impl Into<DateTime<Utc>> for TimestampUsec {
    fn into(self) -> DateTime<Utc> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Example {
        timestamp_usec: TimestampUsec,
    }

    mod deserialize {
        use super::*;

        #[test]
        fn it_deserialize_from_string() -> anyhow::Result<()> {
            let expected = Utc
                .timestamp_micros(1733370114906095)
                .unwrap()
                .timestamp_micros();

            let json_data = r#"{"timestampUsec":"1733370114906095"}"#;
            let example: Example = serde_json::from_str(json_data)?;
            let actual = example.timestamp_usec.timestamp_micros();

            assert_eq!(expected, actual);
            Ok(())
        }

        #[test]
        fn it_deserialize_from_number() -> anyhow::Result<()> {
            let expected = Utc
                .timestamp_micros(1733370114906095)
                .unwrap()
                .timestamp_micros();

            let json_data = r#"{"timestampUsec":1733370114906095}"#;
            let example: Example = serde_json::from_str(json_data)?;
            let actual = example.timestamp_usec.timestamp_micros();

            assert_eq!(expected, actual);
            Ok(())
        }
    }

    mod serialize {
        use super::*;

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
