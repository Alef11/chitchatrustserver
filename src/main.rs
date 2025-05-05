#[macro_use]
extern crate rocket;

use rocket::config::{Config, TlsConfig};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    let config = Config {
        port: 443,
        address: "0.0.0.0".parse().unwrap(),
        tls: Some(TlsConfig::from_paths(
            "certs/certificate.pem",
            "certs/private.pem",
        )),
        ..Config::default()
    };

    rocket::custom(config).mount("/", routes![index])
}
