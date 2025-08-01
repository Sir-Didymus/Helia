//! This module contains the [ActionCreateDate] struct.

use chrono::{DateTime, Utc};

use crate::model::common::create_date::CreateDate;

/// The date and time an [Action](crate::model::action::Action) was created.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActionCreateDate(CreateDate);

impl ActionCreateDate {
    /// Creates a new [CreateDate] instance of the current time.
    pub fn now() -> Self {
        Self(CreateDate::now())
    }

    /// Returns the [DateTime] inside this [ActionCreateDate] instance.
    pub fn value(&self) -> DateTime<Utc> {
        self.0.value()
    }
}

impl Default for ActionCreateDate {
    fn default() -> Self {
        Self::now()
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
}
