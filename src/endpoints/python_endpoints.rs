include!("../../pyembedded/default_python_config.rs");

use actix_web::{HttpResponse, Responder};

pub async fn python_hello_world() -> impl Responder {
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
