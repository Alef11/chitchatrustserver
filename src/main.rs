#[macro_use]
extern crate rocket;

use chitchatrustserver::utils::tls_gen;
use rocket::config::{Config, TlsConfig};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/login")]
fn login() -> &'static str {
    "No you don't!"
}

#[launch]
async fn rocket() -> _ {
    tls_gen::generate_localhost_certs();

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
