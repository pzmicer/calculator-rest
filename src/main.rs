mod calculator;
mod models;

use actix_web::middleware::Logger;
use actix_web::{Responder, HttpServer, App};
use actix_web::{get, web::Query, HttpResponse};
use env_logger::Env;

use crate::models::{Params, CalculatorResult};


#[get("/add")]
async fn add(params: Query<Params>) -> impl Responder {
    HttpResponse::Ok()
        .json(CalculatorResult::new(calculator::add(params.a, params.b)))
}

#[get("/substract")]
async fn substract(params: Query<Params>) -> impl Responder {
    HttpResponse::Ok()
        .json(CalculatorResult::new(calculator::substract(params.a, params.b)))
}

#[get("/multiply")]
async fn multiply(params: Query<Params>) -> impl Responder {
    HttpResponse::Ok()
        .json(CalculatorResult::new(calculator::multiply(params.a, params.b)))
}

#[get("/divide")]
async fn divide(params: Query<Params>) -> impl Responder {
    HttpResponse::Ok()
        .json(CalculatorResult::new(calculator::divide(params.a, params.b)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("trace")); //or "info"

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(add)
            .service(substract)
            .service(multiply)
            .service(divide)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
