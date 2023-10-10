include!("../../pyembedded/default_python_config.rs");

use actix_web::{HttpResponse, Responder};
use log::debug;

pub async fn python_hello_world() -> impl Responder {
    // Get config from default_python_config.rs.
    let config = default_python_config();
    let interp = pyembed::MainPythonInterpreter::new(config).unwrap();

    debug!("Starting python interp.with_gil()...");
    // `py` is a `pyo3::Python` instance.
    interp.with_gil(|py| {
        py.run("print('from python: hello, world')", None, None)
            .unwrap();
    });

    debug!("Finished python interp.with_gil()!");
    HttpResponse::Ok().body("I ran python!")
}
