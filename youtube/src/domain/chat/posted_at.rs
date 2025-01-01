use std::fmt::{self, Display, Formatter};

use chrono::prelude::*;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub(crate) struct PostedAtValue(DateTime<Local>);

impl core::ops::Deref for PostedAtValue {
    type Target = DateTime<Local>;

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
        Self(value.with_timezone(&Local))
    }
}

impl Into<DateTime<Utc>> for PostedAtValue {
    fn into(self) -> DateTime<Utc> {
        self.with_timezone(&Utc)
    }
}

impl From<DateTime<Local>> for PostedAtValue {
    fn from(value: DateTime<Local>) -> Self {
        Self(value)
    }
}

impl Into<DateTime<Local>> for PostedAtValue {
    fn into(self) -> DateTime<Local> {
        self.0
    }
}
