//! This module contains the [ActionName] struct.

/// The name of an [Action](crate::model::action::Action).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ActionName(String);

impl ActionName {
    /// Creates a new [ActionName] instance.
    pub fn new(action_name: &str) -> Self {
        ActionName(String::from(action_name))
    }

    /// Returns the name of the action as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_str_returns_string_of_name() {
        let action_name = ActionName::new("MyAction");
        assert_eq!(action_name.as_str(), "MyAction");
    }
}
