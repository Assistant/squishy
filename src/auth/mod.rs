use super::db::{functions::auth, Db};
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};
use std::error::Error;

pub(crate) struct AuthenticatedUser;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = Box<dyn Error>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Box<dyn Error>> {
        let jar = req.cookies();
        let Some(db) = req.rocket().state::<Db>() else {
            return Outcome::Forward(());
        };
        let Some(username) = jar.get("x-username") else {
            return Outcome::Forward(());
        };
        let Some(password) = jar.get_private("x-auth") else {
            return Outcome::Forward(());
        };

        match auth(db, username.value(), password.value()).await {
            Ok(true) => Outcome::Success(AuthenticatedUser {}),
            _ => Outcome::Forward(()),
        }
    }
}
