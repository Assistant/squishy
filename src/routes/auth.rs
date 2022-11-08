use super::utils::redirect;
use crate::{
    auth::AuthenticatedUser,
    db::{functions, Db},
};
use rocket::{
    form::Form,
    http::{Cookie, CookieJar, Status},
    response::Redirect,
    serde::Deserialize,
    State,
};
use rocket_dyn_templates::{context, Template};
use std::collections::HashMap;

#[get("/login?<params..>")]
#[allow(clippy::result_large_err)]
pub(crate) fn index(
    user: Option<AuthenticatedUser>,
    params: HashMap<String, String>,
) -> Result<Template, Redirect> {
    if user.is_some() {
        Err(redirect!("/"))
    } else {
        let failed = params.contains_key("failed");
        Ok(Template::render("login", context! { failed }))
    }
}

#[post("/login", data = "<credentials>")]
pub(crate) async fn login(
    credentials: Form<Login>,
    cookies: &CookieJar<'_>,
    db: &State<Db>,
) -> Redirect {
    let Login { username, password } = credentials.into_inner();
    if log_in(db, cookies, &username, &password).await {
        redirect!("/")
    } else {
        redirect!("/login?failed")
    }
}

#[get("/register/<invite>")]
pub(crate) async fn register_page(db: &State<Db>, invite: &str) -> Result<Template, Status> {
    if functions::check_invite(db, invite).await {
        Ok(Template::render("register", context! { invite }))
    } else {
        Err(Status::NotFound)
    }
}

#[post("/register/<invite>", data = "<user>")]
pub(crate) async fn register(
    db: &State<Db>,
    cookies: &CookieJar<'_>,
    invite: &str,
    user: Form<Login>,
) -> Redirect {
    if functions::add_user(db, &user.username, &user.password, invite).await {
        log_in(db, cookies, &user.username, &user.password).await;
    }
    redirect!("/")
}

#[get("/invite")]
pub(crate) async fn add_invite(db: &State<Db>, _user: AuthenticatedUser) -> Result<String, Status> {
    if let Ok(Invite { code }) = functions::add_invite(db).await {
        Ok(code)
    } else {
        Err(Status::InternalServerError)
    }
}

#[derive(FromForm, Debug)]
pub(crate) struct Login {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(FromForm, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Invite {
    pub(crate) code: String,
}

async fn log_in(db: &Db, cookies: &CookieJar<'_>, username: &str, password: &str) -> bool {
    if let Ok(true) = functions::auth(db, username, password).await {
        cookies.add(Cookie::new("x-username", username.to_string()));
        cookies.add_private(Cookie::new("x-auth", password.to_string()));
        true
    } else {
        false
    }
}
