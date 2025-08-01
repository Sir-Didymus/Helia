//! This module contains the `Request` types.
//!
//! These requests are used to interact with the API.
//! For example, when inserting data, rather than providing the domain model directly,
//! you need to create a `Request`, which you can then give to the API.

use crate::model::action::{acion_create_date::ActionCreateDate, action_id::ActionId, action_name::ActionName, Action};

/// A request to create an [`Action`].
pub struct CreateActionRequest {
    pub name: String,
}

impl CreateActionRequest {
    pub fn into_action(self) -> Action {
        Action::new(
            ActionId::new(),
            ActionName::new(&self.name),
            ActionCreateDate::now(),
        )
    }
}
