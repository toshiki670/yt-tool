use std::fmt;

use indexmap::IndexMap;
use serde::{Serialize, Serializer};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Content(IndexMap<String, Option<String>>);

impl core::ops::Deref for Content {
    type Target = IndexMap<String, Option<String>>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Content {
    fn deref_mut(&'_ mut self) -> &'_ mut Self::Target {
        &mut self.0
    }
}

impl Content {
    pub fn new() -> Self {
        Self(IndexMap::new())
    }

    pub fn add(&mut self, key: &str, value: Option<String>) {
        self.insert(key.to_string(), value);
    }
}

impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let data = self.to_string();
        serializer.serialize_str(&data)
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content_string = self
            .iter()
            .map(|(key, value)| {
                if let Some(value) = value {
                    format!("{}: '{}'", key, value)
                } else {
                    key.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("; ");
        write!(f, "{}", content_string)
    }
}
