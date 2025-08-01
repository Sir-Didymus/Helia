//! This module contains abstractions and functionality for working with creation dates.

use chrono::{DateTime, Utc};

/// The date and time an entity was created.
///
/// This type is not meant to be used directly.
/// You should use one of the appropriate wrapper type.4
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateDate(DateTime<Utc>);

impl CreateDate {
    /// Creates a new [CreateDate] instance of the current time.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Returns the [DateTime] inside this [CreateDate] instance.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for CreateDate {
    fn default() -> Self {
        Self::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_create_dates_same_date_value() {
        let date1 = CreateDate::now();
        let date2 = CreateDate::now();

        assert_eq!(date1.0.date_naive(), date2.0.date_naive()); 
    }
}
