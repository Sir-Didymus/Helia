//! This module contains the [ActionId] struct.

use crate::model::common::id::Id;

/// The id of an [Action](crate::model::action::Action).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ActionId(Id);

impl ActionId {
    /// Creates a new [ActionId] instance.
    pub fn new() -> Self {
        Self(Id::new())
    }
}

impl Default for ActionId {
    fn default() -> Self {
        ActionId::new()
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
