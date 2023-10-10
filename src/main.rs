mod actions;
mod endpoints;
mod models;
mod schema;

#[macro_use]
extern crate diesel;
extern crate env_logger;
extern crate log;

use actix_web::{web, App, HttpServer};
use env_logger::Env;
use log::info;

use crate::endpoints::config_endpoints;
use crate::endpoints::python_endpoints;
use crate::endpoints::test_endpoints;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "auto");

    env_logger::init_from_env(env);

    info!("Starting http server on `http:localhost:8080`...");
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("config")
                    .service(config_endpoints::add_event_handler)
                    .service(config_endpoints::add_event_handler)
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
