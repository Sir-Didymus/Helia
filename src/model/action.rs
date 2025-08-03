//! A GTD action.
//!
//! This can be any GTD action with any action state.

use crate::model::action::{
    acion_create_date::ActionCreateDate, action_id::ActionId, action_name::ActionName,
};

// The id of an action.
pub mod action_id;
// The name of an action.
pub mod action_name;
// The creation date of an action.
pub mod acion_create_date;

// A GTD Action.
pub struct Action {
    action_id: ActionId,
    action_name: ActionName,
    action_create_date: ActionCreateDate,
}

impl Action {
    /// Returns a new [Action] instance.
    pub fn new(
        action_id: ActionId,
        action_name: ActionName,
        action_create_date: ActionCreateDate,
    ) -> Self {
        Action {
            action_id,
            action_name,
            action_create_date,
        }
    }

    /// Return a reference to the [ActionId].
    pub fn action_id(&self) -> &ActionId {
        &self.action_id
    }

    /// Returns a reference to the [ActionName].
    pub fn action_name(&self) -> &ActionName {
        &self.action_name
    }

    /// Returns a reference to the [ActionCreateDate].
    pub fn action_create_date(&self) -> &ActionCreateDate {
        &self.action_create_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_getters() {
        let my_action_id = ActionId::new();
        let my_action_name = ActionName::new("my_action");
        let my_action_create_date = ActionCreateDate::now();
        let my_action = Action::new(
            my_action_id.clone(),
            my_action_name.clone(),
            my_action_create_date.clone(),
        );
        assert_eq!(my_action.action_id, my_action_id);
        assert_eq!(my_action.action_name, my_action_name);
        assert_eq!(my_action.action_create_date, my_action_create_date)
    }
}

#[cfg(test)]
pub mod test_utils {

    use super::*;

    pub fn dummy_action() -> Action {
        Action {
            action_id: ActionId::new(),
            action_name: ActionName::new("Dummy Action"),
            action_create_date: ActionCreateDate::now(),
        }
    }
}
