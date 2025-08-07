//! Creation dates of [`Action`s](super::Action).
//!
//! This module contains the [ActionCreateDate] struct.
//! It represents the creation date of an action in UTC.
//! Creation dates contain both a date and a time value.

use chrono::{DateTime, Utc};

use crate::model::common::date::Date;

/// The date and time an [Action](crate::model::action::Action) was created.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActionCreateDate(Date);

impl ActionCreateDate {
    /// Creates a new [ActionCreateDate] instance of the current time.
    pub fn now() -> Self {
        Self(Date::now())
    }

    /// Returns the [DateTime] inside this [ActionCreateDate] instance.
    pub fn value(&self) -> &DateTime<Utc> {
        self.0.value()
    }
}

impl Default for ActionCreateDate {
    fn default() -> Self {
        Self::now()
    }
}

impl std::fmt::Display for ActionCreateDate {
    /// Nicer formatting for [ActionCreateDate].
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ActionCreateDate({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_create_dates_same_date_value() {
        let date1 = ActionCreateDate::now();
        let date2 = ActionCreateDate::now();

        assert_eq!(date1.value().date_naive(), date2.value().date_naive());
    }

    #[test]
    fn test_display() {
        let action_create_date = ActionCreateDate::now();
        assert_eq!(
            format!("{action_create_date}"),
            format!("ActionCreateDate(Date({}))", action_create_date.value())
        );
    }
}
