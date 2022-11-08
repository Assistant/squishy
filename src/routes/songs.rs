use super::utils::redirect;
use crate::{
    auth::AuthenticatedUser,
    db::{functions, Db},
};
use rocket::{
    form::Form,
    response::Redirect,
    serde::{Deserialize, Serialize},
    State,
};
use rocket_dyn_templates::{context, Template};

#[get("/songs")]
pub(crate) async fn index(db: &State<Db>, user: Option<AuthenticatedUser>) -> Template {
    let admin = user.is_some();
    let list = functions::songs(db).await.unwrap_or_default();
    Template::render("songs/page", context! { songs: list, admin })
}

#[post("/songs", data = "<song>")]
pub(crate) async fn add_song(
    _user: AuthenticatedUser,
    db: &State<Db>,
    song: Form<Song>,
) -> Redirect {
    functions::add_song(db, &song.artist, &song.title, &song.year).await;
    redirect!("/songs")
}

#[derive(Deserialize, Serialize, Debug, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Song {
    artist: String,
    title: String,
    year: String,
}
