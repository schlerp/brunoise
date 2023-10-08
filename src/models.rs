use serde::{Deserialize, Serialize};

use crate::schema::event_handlers;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = event_handlers)]
pub struct EventHandler {
    pub id: String,
    pub name: String,
    pub url: String,
    pub python_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEventHandler {
    pub name: String,
    pub url: String,
    pub python_code: String,
}

impl NewEventHandler {
    /// Constructs new user details from name.
    #[cfg(test)] // only needed in tests
    pub fn new(
        name: impl Into<String>,
        url: impl Into<String>,
        python_code: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            python_code: python_code.into(),
        }
    }
}
