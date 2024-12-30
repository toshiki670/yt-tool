use std::fmt::{self, Display, Formatter};

use chrono::prelude::*;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub(crate) struct PostedAtValue(DateTime<Utc>);

impl core::ops::Deref for PostedAtValue {
    type Target = DateTime<Utc>;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
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
        Self(value)
    }
}

impl Into<DateTime<Utc>> for PostedAtValue {
    fn into(self) -> DateTime<Utc> {
        self.0
    }
}