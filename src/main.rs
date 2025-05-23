#[macro_use]
extern crate rocket;

use chitchatrustserver::db::db;
use chitchatrustserver::log;
use chitchatrustserver::logger::logger::LogExpect;
use chitchatrustserver::utils::communication_structs::{
    ErrorResponse, LoginRequest, LoginResponse, RegisterRequest,
};
use chitchatrustserver::utils::db_waiter;
use chitchatrustserver::utils::tls_gen;
use chitchatrustserver::utils::token;
use chitchatrustserver::utils::xtime::Xtime;
use chitchatrustserver::utils::{file_gen, logics};
use rocket::Request;
use rocket::serde::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::net::IpAddr;

#[get("/")]
fn index(client_ip: IpAddr) -> String {
    log!("[{}]: GET to / route", client_ip);
    format!("Client IP: {}", client_ip)
}

#[post("/login", data = "<login>")]
fn login(
    login: Json<LoginRequest>,
    client_ip: IpAddr,
) -> Result<Json<LoginResponse>, Json<ErrorResponse>> {
    log!("[{}]: POST to /login", client_ip);

    let username = &login.username;
    let password = &login.password;

    let result = logics::login_logic(username, password);

    if result.0 {
        log!(
            "[{}]: Logged into User {} successfully",
            client_ip,
            username
        );
        let exp_time = Xtime::now_plus_year(1);
        let exp_time_str = exp_time.to_string();
        let ttoken = token::new_user_token(result.1.unwrap(), exp_time);

        log!(
            "[{}]: Generated token for User {} expiering at {}",
            client_ip,
            username,
            exp_time_str
        );

        Ok(Json(LoginResponse {
            token: ttoken,
            user_id: result.1.unwrap(),
        }))
    } else {
        log!(
            "[{}]: Failed to login User {}. Invalid username or password",
            client_ip,
            username
        );
        return Err(Json(ErrorResponse {
            error: "Invalid username or password.".into(),
        }));
    }
}

#[post("/register", data = "<register>")]
fn register(
    register: Json<RegisterRequest>,
    client_ip: IpAddr,
) -> Result<Json<LoginResponse>, Json<ErrorResponse>> {
    log!("[{}]: POST to /register", client_ip);

    let username = &register.username;
    let password = &register.password;
    let email = &register.email;

    let result = logics::register_logic(username, password, email);

    if result.0 {
        log!("[{}]: Registered User {} successfully", client_ip, username);
        let exp_time = Xtime::now_plus_year(1);
        let exp_time_str = exp_time.to_string();
        let ttoken = token::new_user_token(result.1.unwrap(), exp_time);

        log!(
            "[{}]: Generated token for User {} expiering at {}",
            client_ip,
            username,
            exp_time_str
        );

        Ok(Json(LoginResponse {
            token: ttoken,
            user_id: result.1.unwrap(),
        }))
    } else {
        log!(
            "[{}]: Failed to register User {}. User already exists",
            client_ip,
            username
        );
        return Err(Json(ErrorResponse {
            error: "User already exists".into(),
        }));
    }
}

#[catch(403)]
fn forbidden_request(req: &Request) -> Json<ErrorResponse> {
    let ip = req
        .client_ip()
        .map(|ip| ip.to_string())
        .unwrap_or("unknown".into());

    log!("[{}]: Forbidden request: 403", ip);
    Json(ErrorResponse {
        error: "Forbidden request: invalid or missing data.".into(),
    })
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

    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000", // Your frontend origin
    ]);

    let cors = CorsOptions {
        allowed_origins,
        allow_credentials: true, // needed for cookies
        ..Default::default()
    }
    .to_cors()
    .expect("error setting up CORS");

    let _ = rocket::custom(config)
        .mount("/", routes![index, login, register])
        .register("/", catchers![forbidden_request])
        .attach(cors)
        .launch()
        .await;
}
