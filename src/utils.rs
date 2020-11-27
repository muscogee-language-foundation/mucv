use actix_web::{http::header, HttpResponse, Responder};
use dotenv::dotenv;
use std::env;

use crate::models::Response;

const DAYS_OF_WEEK: [&str; 7] = [
    "tacakuce",
    "tvcakuce enhayvtke",
    "mvnte enhayvtke",
    "ennvrkvpv",
    "ennvrkvpv enhayvtke",
    "okkoskv nettv",
    "tacakcuse",
];

const MONTHS: [&str; 12] = [
    "rvfocuse",
    "hotvle hvse",
    "tasacuce",
    "tasace rakko",
    "ke hvse",
    "kvco hvse",
    "hiyuce",
    "hiyo rakko",
    "otowoskuce",
    "otowosko rakko",
    "ehole",
    "rvfo rakko",
];

const DAYS: [&str; 31] = [
    "'svhvmkat",
    "'svhokkolat",
    "'svtuccēnat",
    "'sostat",
    "'svcahkepat",
    "'svpakat",
    "eskolvpakat",
    "escenvpokakat",
    "'sostvkakat",
    "'svpalat",
    "palen 'svhvmkat",
    "palen 'svhokkolat",
    "palen 'svtuccēnat",
    "palen 'sostat",
    "palen 'svcahkepat",
    "palen 'svpakat",
    "palen eskolvpakat",
    "palen escenvpokakat",
    "palen 'sostvkakat",
    "pale 'svhokkolat",
    "pale-hokkolen 'svhvmkat",
    "pale-hokkolen 'svhokkolat",
    "pale-hokkolen 'svtuccēnat",
    "pale-hokkolen 'svostat",
    "pale-hokkolen 'svcahkepat",
    "pale-hokkolen 'svpakat",
    "pale-hokkolen eskolvpakat",
    "pale-hokkolen escenvpakat",
    "pale-hokkolen 'sostvkakat",
    "pale 'svtuccēnat",
    "pale-tuccēnan 'svhvmkat",
];

pub fn respond(text: String) -> impl Responder {
    mk_respond(mk_response(text))
}

fn mk_response(text: String) -> Response {
    let response = Response {
        response_type: "in_channel".to_string(),
        text,
    };

    response
}

fn mk_respond(response: Response) -> impl Responder {
    dotenv().ok();

    HttpResponse::Ok()
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                env::var("SLACK_TOKEN").expect("No slack token")
            ),
        )
        .header(header::CONTENT_TYPE, "application/json")
        .json(response)
}

pub fn number_to_day_of_month(day: u32) -> String {
    let result = *DAYS.get(day as usize).unwrap();

    result.to_string()
}

pub fn number_to_month(month: u32) -> String {
    let result = *MONTHS.get(month as usize).unwrap();

    result.to_string()
}

pub fn number_to_day_of_week(day: u32) -> String {
    let result = *DAYS_OF_WEEK.get(day as usize).unwrap();

    result.to_string()
}
