use std::{error::Error as StdError, fmt, str::FromStr};

use chrono_tz::Tz;
use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};

#[derive(AsExpression, Clone, Copy, Debug, Eq, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
pub struct Timezone(pub Tz);

impl fmt::Display for Timezone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.0.name(), f)
    }
}

impl FromStr for Timezone {
    type Err = TimezoneParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FromStr::from_str(s)
            .map(Timezone)
            .map_err(TimezoneParseError)
    }
}

impl<DB> serialize::ToSql<Text, DB> for Timezone
where
    DB: Backend,
    str: serialize::ToSql<Text, DB>,
{
    fn to_sql<'b>(&self, out: &mut serialize::Output<'b, '_, DB>) -> serialize::Result {
        self.0.name().to_sql(out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Timezone
where
    DB: Backend,
    *const str: deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|string: String| {
            string
                .parse::<Timezone>()
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
        })
    }
}

impl From<Tz> for Timezone {
    fn from(tz: Tz) -> Self {
        Timezone(tz)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimezoneParseError(String);

impl fmt::Display for TimezoneParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse timezone: {}", self.0)
    }
}

impl StdError for TimezoneParseError {
    fn description(&self) -> &str {
        "Failed to parse timezone"
    }

    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}
