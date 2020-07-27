extern crate chrono;

mod models;
mod utils;

use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use dotenv::dotenv;
use models::FormData;
use std::env;
use utils::{number_to_day_of_month, number_to_day_of_week, number_to_month, respond};

#[post("/")]
async fn mucv(form: web::Form<FormData>) -> impl Responder {
    let date = chrono::Utc::now();
    let month = date.month0();
    let day_of_month = date.day0();
    let day_of_week = date.weekday().num_days_from_sunday();

    if form.text.eq_ignore_ascii_case("help") {
        let res = format!("Use this command by either typing /mucv, /mucv hvse, or /mucv nettv");

        respond(res)
    } else if form.text.eq_ignore_ascii_case("hvse") {
        let month = format!("mucv hvse {}t os", number_to_month(month));

        respond(month)
    } else if form.text.eq_ignore_ascii_case("nettv") {
        let day = format!(
            "mucv nettv {} {} os",
            number_to_day_of_week(day_of_week),
            number_to_day_of_month(day_of_month)
        );

        respond(day)
    } else if form.text == "" {
        let date = format!(
            "mucv nettv {} {} {} os",
            number_to_day_of_week(day_of_week),
            number_to_month(month),
            number_to_day_of_month(day_of_month)
        );

        respond(date)
    } else {
        let res = format!(
            "We're sorry, we did not understand your command. Please try again using /mucv, /mucv hvse, or /mucv nettv"
        );

        respond(res)
    }
}

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let host = env::var("APP_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("APP_PORT").unwrap_or(8080.to_string());

    let host_and_port = format!("{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(mucv)
            .service(test)
    })
    .bind(host_and_port)?
    .run()
    .await
}
