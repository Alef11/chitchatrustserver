#[macro_use]
extern crate rocket;

use chitchatrustserver::log;
use chitchatrustserver::logger::logger::LogExpect;
use chitchatrustserver::db::db;
use chitchatrustserver::utils::communication_structs::{
    ErrorResponse, LoginRequest, LoginResponse,
};
use chitchatrustserver::utils::db_waiter;
use chitchatrustserver::utils::{file_gen, logics};
use chitchatrustserver::utils::tls_gen;
use chitchatrustserver::utils::token;
use chitchatrustserver::utils::xtime::Xtime;
use rocket::serde::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/login", data = "<login>")]
fn login(login: Json<LoginRequest>) -> Result<Json<LoginResponse>, Json<ErrorResponse>> {
    let username = &login.username;
    let password = &login.password;

    let result = logics::login_logic(username, password);

    if result.0 {
        let exp_time = Xtime::now_plus_year(1);
        let ttoken = token::new_user_token(result.1.unwrap(), exp_time);

        Ok(Json(LoginResponse {
            token: ttoken,
            user_id: result.1.unwrap(),
        }))
    } else {
        return Err(Json(ErrorResponse {
            error: "Invalid username or password.".into(),
        }));
    }
}

#[tokio::main]
async fn main() {
    log!("Initializing Chitchat Backend");

    file_gen::generate_certs_directory();
    tls_gen::generate_localhost_certs();

    db_waiter::ensure_db_connection_ready().await;
    
    db::init_db().log_expect("Failed to initialize database", file!());

    log!("Finished init");
    log!("----- Chitchat Backend -----");

    let config = rocket::Config {
        port: 443,
        address: "0.0.0.0".parse().unwrap(),
        tls: Some(rocket::config::TlsConfig::from_paths(
            "certs/certificate.pem",
            "certs/private.pem",
        )),
        ..rocket::Config::default()
    };

    let _ = rocket::custom(config)
        .mount("/", routes![index, login])
        .launch()
        .await;
}

