use std::fmt::{self, Display, Formatter};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PostedAtValue(DateTime<Local>);

impl core::ops::Deref for PostedAtValue {
    type Target = DateTime<Local>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl Display for PostedAtValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format("%Y-%m-%d %H:%M:%S"))
    }
}

impl From<DateTime<Utc>> for PostedAtValue {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value.with_timezone(&Local))
    }
}

impl From<PostedAtValue> for DateTime<Utc> {
    fn from(val: PostedAtValue) -> Self {
        val.with_timezone(&Utc)
    }
}

impl From<DateTime<Local>> for PostedAtValue {
    fn from(value: DateTime<Local>) -> Self {
        Self(value)
    }
}

impl From<PostedAtValue> for DateTime<Local> {
    fn from(val: PostedAtValue) -> Self {
        val.0
    }
}
