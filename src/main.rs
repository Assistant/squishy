use rocket::{
    figment::{
        providers::{Format, Toml},
        Figment,
    },
    fs::{relative, FileServer},
    Config,
};
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;
mod auth;
mod db;
mod routes;

#[launch]
fn rocket() -> _ {
    let figment = Figment::from(Config::default())
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Toml::file("Secret.toml").nested())
        .merge(Toml::file("Surreal.toml").nested());
    rocket::custom(figment)
        .mount("/", routes::routes())
        .mount("/", FileServer::from(relative!("static")))
        .attach(db::DbFairing)
        .attach(Template::fairing())
}
