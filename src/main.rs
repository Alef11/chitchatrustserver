#[macro_use]
extern crate rocket;

use chitchatrustserver::db::db::{self};
use chitchatrustserver::log;
use chitchatrustserver::logger::logger::{LogExpect, log_error, shutdown_logging};
use chitchatrustserver::utils::communication_structs::{
    ErrorResponse, LoginRequest, LoginResponse, RegisterRequest, UserResponse,
};
use chitchatrustserver::utils::db_waiter;
use chitchatrustserver::utils::tls_gen;
use chitchatrustserver::utils::token;
use chitchatrustserver::utils::xtime::Xtime;
use chitchatrustserver::utils::{file_gen, logics};
use rocket::Request;
use rocket::config::SecretKey;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::http::SameSite;
use rocket::serde::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::net::IpAddr;
use tokio::signal;

#[cfg(unix)]
use tokio::signal::unix;
#[cfg(unix)]
use tokio::signal::unix::{SignalKind, signal};

#[get("/")]
fn index(client_ip: IpAddr) -> String {
    log!("[{}]: GET to / route", client_ip);
    format!("Client IP: {}", client_ip)
}

#[post("/login", data = "<login>")]
fn login(
    login: Json<LoginRequest>,
    client_ip: IpAddr,
    cookies: &CookieJar<'_>,
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
        let _ = db::update_user_last_online(username);

        let exp_time = Xtime::now_plus_year(1);
        let exp_time_str = exp_time.to_string();
        let ttoken = token::new_user_token(result.1.unwrap(), exp_time);

        log!(
            "[{}]: Generated token for User {} expiering at {}",
            client_ip,
            username,
            exp_time_str
        );

        let mut token_cookie = Cookie::new("token", ttoken.clone());
        token_cookie.set_path("/");
        token_cookie.set_http_only(true);
        token_cookie.set_secure(true);
        token_cookie.set_same_site(SameSite::None);
        cookies.add_private(token_cookie);

        // Build and set private username cookie
        let mut username_cookie = Cookie::new("username", username.clone());
        username_cookie.set_path("/");
        username_cookie.set_http_only(true);
        username_cookie.set_secure(true);
        username_cookie.set_same_site(SameSite::None);
        cookies.add_private(username_cookie);

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
    cookies: &CookieJar<'_>,
) -> Result<Json<LoginResponse>, Json<ErrorResponse>> {
    log!("[{}]: POST to /register", client_ip);

    let username = &register.username;
    let password = &register.password;
    let email = &register.email;

    let result = logics::register_logic(username, password, email);

    if result.0 {
        log!("[{}]: Registered User {} successfully", client_ip, username);
        let user_id = result.1.unwrap();
        let exp_time = Xtime::now_plus_year(1);
        let exp_time_str = exp_time.to_string();
        let ttoken = token::new_user_token(result.1.unwrap(), exp_time);

        // Build and set private token cookie
        let mut token_cookie = Cookie::new("token", ttoken.clone());
        token_cookie.set_path("/");
        token_cookie.set_http_only(true);
        token_cookie.set_secure(true);
        token_cookie.set_same_site(SameSite::None);
        cookies.add_private(token_cookie);

        // Build and set private username cookie
        let mut username_cookie = Cookie::new("username", username.clone());
        username_cookie.set_path("/");
        username_cookie.set_http_only(true);
        username_cookie.set_secure(true);
        username_cookie.set_same_site(SameSite::None);
        cookies.add_private(username_cookie);

        log!(
            "[{}]: Generated token for User {} expiering at {}",
            client_ip,
            username,
            exp_time_str
        );

        Ok(Json(LoginResponse {
            token: ttoken,
            user_id,
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

#[post("/logout")]
pub fn logout(client_ip: IpAddr, cookies: &CookieJar<'_>) -> Json<ErrorResponse> {
    log!("POST to /logout route");

    let token_cookie = cookies.get_private("token");
    if let Some(cookie) = token_cookie {
        let token_value = cookie.value();

        let uuid_cookie = cookies.get_private("username");
        if let Some(cookie) = uuid_cookie {
            if let Ok(user_id) = cookie.value().parse::<u32>() {
                let username = db::get_user_by_id(user_id)
                    .expect("Failed to get user by ID")
                    .unwrap()
                    .username;

                log!("[{}]: Deleted token for User {}", client_ip, username);
            }
        }

        db::delete_token(token_value).log_expect("Failed to delete token", file!());
    }

    // Remove the token cookie
    cookies.remove_private(Cookie::from("token"));
    cookies.remove_private(Cookie::from("username"));

    Json(ErrorResponse {
        error: "Logged out successfully".into(),
    })
}

#[get("/user")]
fn user(cookies: &CookieJar<'_>) -> Result<Json<UserResponse>, Json<ErrorResponse>> {
    log!("GET to /user route");
    let token_cookie = cookies.get_private("token").ok_or(Json(ErrorResponse {
        error: "Token not found".into(),
    }))?;
    let token = token_cookie.value();

    let user = db::get_user_by_token(token).map_err(|_| {
        Json(ErrorResponse {
            error: "Failed to retrieve user".into(),
        })
    })?;

    if let Some(user) = user {
        Ok(Json(UserResponse {
            username: user.username,
            status: "Logged in".to_string(),
        }))
    } else {
        Err(Json(ErrorResponse {
            error: "User not found".into(),
        }))
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
async fn main() -> Result<(), rocket::Error> {
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
        secret_key: SecretKey::generate().expect("Failed to generate secret key"),
        ..rocket::Config::default()
    };

    let allowed_origins = AllowedOrigins::some_exact(&["https://localhost"]);

    let cors = CorsOptions {
        allowed_origins,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error setting up CORS");

    let rocket = rocket::custom(config)
        .mount("/", routes![index, login, register, user])
        .register("/", catchers![forbidden_request])
        .attach(cors);

    // Spawn Rocket in a background task
    let _ = tokio::spawn(async move {
        if let Err(_e) = rocket.launch().await {
            log_error("Rocket launch failed: {}", file!());
        }
    });

    // Signal handling
    #[cfg(unix)]
    {
        let mut sigterm =
            signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");
        let mut sighup = signal(SignalKind::hangup()).expect("Failed to register SIGHUP handler");

        tokio::select! {
            _ = signal::ctrl_c() => {
                log!("Received SIGINT (Ctrl+C) - shutting down.");
            }
            _ = sigterm.recv() => {
                log!("Received SIGTERM (docker stop) - shutting down.");
            }
            _ = sighup.recv() => {
                log!("Received SIGHUP (restart) - restarting.");
            }
        }
    }

    #[cfg(not(unix))]
    {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
        log!("Received Ctrl+C");
    }

    shutdown_logging("End of main");
    Ok(())
}
