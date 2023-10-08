use crate::models;
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_event_handler(
    conn: &mut SqliteConnection,
    name_in: &str,
    slug_in: &str,
    python_code_in: &str,
) -> Result<models::EventHandler, DbError> {
    use crate::schema::event_handlers::dsl::*;

    let new_event_handler = models::EventHandler {
        name: name_in.to_owned(),
        slug: slug_in.to_owned(),
        python_code: python_code_in.to_owned(),
    };

    diesel::insert_into(event_handlers)
        .values(&new_event_handler)
        .execute(conn)?;

    Ok(new_event_handler)
}

/// Run query using Diesel to find user by uid and return it.
pub fn find_event_handler_by_slug(
    conn: &mut SqliteConnection,
    slug_in: &str,
) -> Result<Option<models::EventHandler>, DbError> {
    use crate::schema::event_handlers::dsl::*;

    let event_handler = event_handlers
        .filter(slug.eq(slug_in.to_string()))
        .first::<models::EventHandler>(conn)
        .optional()?;

    Ok(event_handler)
}
