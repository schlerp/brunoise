use crate::actions;
use crate::models;
use actix_web::{error, get, post, web, HttpResponse, Responder};
use diesel::{prelude::*, r2d2};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

/// Creates new event handler.
///
/// Extracts:
/// - the database pool handle from application data
/// - a JSON form containing new user info from the request body
#[post("/handlers")]
pub async fn add_event_handler(
    pool: web::Data<DbPool>,
    form: web::Json<models::EventHandler>,
) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let user = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::insert_new_event_handler(&mut conn, &form.name, &form.slug, &form.python_code)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // user was added successfully; return 201 response with new user info
    Ok(HttpResponse::Created().json(user))
}

/// Finds event handler by slug.
///
/// Extracts:
/// - the database pool handle from application data
/// - a user UID from the request path
#[get("/handlers/{slug_in}")]
async fn get_user(
    pool: web::Data<DbPool>,
    slug_in: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let slug_in = slug_in.into_inner();
    let slug_in_for_closure = slug_in.clone(); // NOTE: am i dumb?

    // use web::block to offload blocking Diesel queries without blocking server thread
    let event_handler = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::find_event_handler_by_slug(&mut conn, &slug_in_for_closure)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match event_handler {
        // user was found; return 200 response with JSON formatted user object
        Some(user) => HttpResponse::Ok().json(user),

        // user was not found; return 404 response with error message
        None => {
            HttpResponse::NotFound().body(format!("No event handler found with slug: {slug_in}"))
        }
    })
}
