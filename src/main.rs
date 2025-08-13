use rocket::figment::providers::{Format, Toml};
use rocket::figment::Figment;
use rocket::fs::FileServer;
use rocket::Config;
use rocket_dyn_templates::Template;
use std::env;

#[macro_use]
extern crate rocket;
mod auth;
mod db;
mod routes;

#[launch]
fn rocket() -> _ {
    let rocket_config = env::var("ROCKET_CONFIG").expect("ROCKET_CONFIG");
    let surreal_config = env::var("SURREAL_CONFIG").expect("SURREAL_CONFIG");
    let secret = env::var("SECRET").expect("SECRET");
    let static_dir = env::var("STATIC_DIR").expect("STATIC_DIR");

    let figment = Figment::from(Config::default())
        .merge(Toml::file(rocket_config).nested())
        .merge(Toml::file(surreal_config).nested())
        .merge(Toml::file(secret).nested());
    rocket::custom(figment)
        .mount("/", routes::routes())
        .mount("/", FileServer::from(static_dir))
        .attach(db::DbFairing)
        .attach(Template::fairing())
}
