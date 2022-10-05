use actix_web::middleware::Logger;
use actix_web::{Responder, HttpServer, App};
use actix_web::{get, web::Query, HttpResponse};
use env_logger::Env;
use serde::{Deserialize, Serialize};


struct Calculator;
impl Calculator {
    fn add(params: Params) -> CalculatorResult {
        CalculatorResult::new(params.a + params.b)
    }

    fn substract(params: Params) -> CalculatorResult {
        CalculatorResult::new(params.a / params.b)
    }

    fn multiply(params: Params) -> CalculatorResult {
        CalculatorResult::new(params.a * params.b)
    }

    fn divide(params: Params) -> CalculatorResult {
        CalculatorResult::new(params.a / params.b)
    }
}

#[derive(Deserialize)]
struct Params {
    a: f64, 
    b: f64,
}

#[derive(Serialize)]
struct CalculatorResult {
    result: f64,
}

impl CalculatorResult {
    fn new(result: f64) -> CalculatorResult {
        CalculatorResult { result }
    }
}

#[get("/add")]
async fn add(params: Query<Params>) -> impl Responder {
    HttpResponse::Ok()
        .json(CalculatorResult::new(params.a + params.b))
}

#[get("/substract")]
async fn substract(params: Query<Params>) -> impl Responder {
    HttpResponse::Ok()
        .json(CalculatorResult::new(params.a - params.b))
}

#[get("/multiply")]
async fn multiply(params: Query<Params>) -> impl Responder {
    HttpResponse::Ok()
        .json(CalculatorResult::new(params.a * params.b))
}

#[get("/divide")]
async fn divide(params: Query<Params>) -> impl Responder {
    HttpResponse::Ok()
        .json(CalculatorResult::new(params.a / params.b))
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
