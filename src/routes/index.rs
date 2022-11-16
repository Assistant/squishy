use crate::auth::AuthenticatedUser;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub(crate) fn index(user: Option<AuthenticatedUser>) -> Template {
    let admin = user.is_some();
    Template::render("index", context! { admin, selected: "/" })
}
