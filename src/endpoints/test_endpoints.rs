use actix_web::{post, web, HttpResponse, Responder};

pub struct HelloFromState {
    pub greeter_name: String,
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn hello_from(data: web::Data<HelloFromState>) -> impl Responder {
    let greeter_name = &data.greeter_name;
    HttpResponse::Ok().body(format!("Hello from {greeter_name}!"))
}
