#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};

mod endpoints;
mod models;
mod schema;

use crate::endpoints::python_endpoints;
use crate::endpoints::test_endpoints;
use crate::models::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("config")
                    .app_data(web::Data::new(test_endpoints::HelloFromState {
                        greeter_name: String::from("config"),
                    }))
                    .route("/hey", web::get().to(test_endpoints::hello_from)),
            )
            .service(
                web::scope("events")
                    .app_data(web::Data::new(test_endpoints::HelloFromState {
                        greeter_name: String::from("events"),
                    }))
                    .route("/hey", web::get().to(test_endpoints::hello_from)),
            )
            .service(
                web::scope("test")
                    .service(test_endpoints::echo)
                    .route("/py", web::get().to(python_endpoints::python_hello_world))
                    .app_data(web::Data::new(test_endpoints::HelloFromState {
                        greeter_name: String::from("events"),
                    }))
                    .route("/hey", web::get().to(test_endpoints::hello_from)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
