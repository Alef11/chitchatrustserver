#[macro_use]
extern crate rocket;

use chitchatrustserver::db::db;
use chitchatrustserver::utils::communication_structs::{
    ErrorResponse, LoginRequest, LoginResponse,
};
use chitchatrustserver::utils::{file_gen, logics};
use chitchatrustserver::utils::tls_gen;
use chitchatrustserver::utils::token;
use chitchatrustserver::utils::xtime::Xtime;
use rocket::config::{Config, TlsConfig};
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

#[launch]
async fn rocket() -> _ {
    tls_gen::generate_localhost_certs();
    db::init_db().expect("Failed to initialize database");
    file_gen::generate_certs_directory();

    let config = Config {
        port: 443,
        address: "0.0.0.0".parse().unwrap(),
        tls: Some(TlsConfig::from_paths(
            "certs/certificate.pem",
            "certs/private.pem",
        )),
        ..Config::default()
    };

    rocket::custom(config).mount("/", routes![index, login])
}
