//! Abstractions and functionality for working with dates.
//!
//! Contains the [Date] type.
//! Dates in Helia are always stored as UTC.

use chrono::{DateTime, Utc};

/// Contains a date and time value.
///
/// This type is not meant to be used directly.
/// Use one of the various wrapper types, like
/// [ActionCreateDate](crate::model::action::ActionCreateDate).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Date(DateTime<Utc>);

impl Date {
    /// Creates a new [Date] instance of the current time.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Returns the [DateTime] inside this [Date] instance.
    pub fn value(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl Default for Date {
    /// Default constructor for [Date].
    fn default() -> Self {
        Self::now()
    }
}

impl std::fmt::Display for Date {
    /// Nicer formatting for [Date].
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Date({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_create_dates_same_date_value() {
        let date1 = Date::now();
        let date2 = Date::now();

        assert_eq!(date1.0.date_naive(), date2.0.date_naive());
    }

    #[test]
    fn test_display() {
        let date = Date::now();
        assert_eq!(format!("{date}"), format!("Date({})", date.value()));
    }
}
