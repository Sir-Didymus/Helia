//! This module contains the [ActionId] struct.

use uuid::Uuid;

use crate::model::common::id::Id;

/// The id of an [Action](crate::model::action::Action).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ActionId(Id);

impl ActionId {
    /// Creates a new [ActionId] instance.
    pub fn new() -> Self {
        Self(Id::new())
    }

    /// Returns the underlying [uuid::Uuid].
    pub fn uuid(&self) -> &Uuid {
        self.0.uuid()
    }
}

impl Default for ActionId {
    /// Default constructor for [ActionId].
    fn default() -> Self {
        ActionId::new()
    }
}

impl std::fmt::Display for ActionId {
    /// Nicer formatting for [ActionId].
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ActionId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_should_be_different() {
        let id_1 = ActionId::new();
        let id_2 = ActionId::new();
        assert_ne!(id_1, id_2);
    }
}
