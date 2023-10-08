use serde::{Deserialize, Serialize};

use crate::schema::event_handlers;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = event_handlers)]
pub struct EventHandler {
    pub name: String,
    pub slug: String,
    pub python_code: String,
}

impl EventHandler {
    /// Constructs new user details from name.
    #[cfg(test)] // only needed in tests
    pub fn new(
        name: impl Into<String>,
        slug: impl Into<String>,
        python_code: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            slug: url.into(),
            python_code: python_code.into(),
        }
    }
}
