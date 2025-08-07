//! Abstractions and functionality for working with ids.
//!
//! This module contains the [Id] type, a globally unique identifier.

use uuid::Uuid;

/// A globally unique identifier.
///
/// The `Id` struct uses the [uuid] crate to create globally unique identifiers.
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

    /// Returns the underlying [uuid::Uuid].
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}

impl Default for Id {
    /// Default constructor for [Id].
    fn default() -> Self {
        Id::new()
    }
}

impl std::fmt::Display for Id {
    /// Nicer formatting for [Id].
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", self.uuid)
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

    #[test]
    fn test_display() {
        let id = Id {
            uuid: Uuid::parse_str("d3fc9750-88b1-4f70-b0cd-eacb0651b36b").unwrap(),
        };
        let expected = "Id(d3fc9750-88b1-4f70-b0cd-eacb0651b36b)";
        assert_eq!(format!("{id}"), expected)
    }
}
