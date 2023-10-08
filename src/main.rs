include!("../pyembedded/default_python_config.rs");

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn python_hello_world() -> impl Responder {
    println!("Running python hello world...");
    // Get config from default_python_config.rs.
    let config = default_python_config();

    println!("Setting up pyembed interp...");
    let interp = pyembed::MainPythonInterpreter::new(config).unwrap();

    println!("Running interp with GIL...");
    // `py` is a `pyo3::Python` instance.
    interp.with_gil(|py| {
        py.run("print('from python: hello, world')", None, None)
            .unwrap();
    });

    println!("Successfully ran Python!");
    HttpResponse::Ok().body("I ran python!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/py", web::get().to(python_hello_world))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
