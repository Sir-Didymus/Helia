//! This module contains abstractions and functionality for working with ids.

use uuid::Uuid;

/// A globally unique identifier.
///
/// The `Id` struct uses the [uuid] crate to create unique identifiers.
/// UUIDS have the advantage that ids can be created without checking a central authority
/// for uniqueness.
/// The UUID version used for this id type is version 4.
///
/// Notice that this id type is not directly available in the API.
/// Use the more specific types like [ActionId](crate::model::action::action_id::ActionId).
/// This prevents accidental mix-up of different id types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id {
    uuid: Uuid,
}

impl Id {
    /// Creates a new [Id] instance.
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}

impl Default for Id {
    fn default() -> Self {
        Id::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_should_be_different() {
        let id_1 = Id::new();
        let id_2 = Id::new();
        assert_ne!(id_1, id_2);
    }
}
