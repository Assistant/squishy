use super::utils::redirect;
use crate::auth::AuthenticatedUser;
use crate::db::functions;
use rocket::serde::{Deserialize, Serialize};
use rocket::{form::Form, response::Redirect, State};
use rocket_dyn_templates::{context, Template};

type Db = surrealdb::Surreal<surrealdb::engine::local::Db>;

#[get("/games")]
pub(crate) async fn index(db: &State<Db>, user: Option<AuthenticatedUser>) -> Template {
    let admin = user.is_some();
    let list: GameList = functions::games(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .fold(GameList::default(), |mut a, g| {
            match g.status {
                GameStatus::Active => a.active.push(g),
                GameStatus::Hold => a.hold.push(g),
                GameStatus::Completed => a.completed.push(g),
                GameStatus::Planned => a.planned.push(g),
            }
            a
        });
    Template::render(
        "games/page",
        context! { list, admin, selected: "/games", zero: Game{ name: Some("???".into()), status: GameStatus::Planned, id: None } },
    )
}

#[post("/games", data = "<game>")]
pub(crate) async fn edit_games(
    _user: AuthenticatedUser,
    db: &State<Db>,
    game: Form<Game>,
) -> Redirect {
    let Game { name, status, id } = game.into_inner();
    if let Some(id) = id {
        functions::update_game(db, &id, status.into()).await;
    } else if let Some(name) = name {
        functions::add_game(db, &name, status.into()).await;
    }
    redirect!("/games")
}

#[derive(Deserialize, Serialize, Debug, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Game {
    name: Option<String>,
    status: GameStatus,
    id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, FromFormField)]
#[serde(crate = "rocket::serde")]
pub(crate) enum GameStatus {
    Active,
    Hold,
    Completed,
    Planned,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
struct GameList {
    active: Vec<Game>,
    hold: Vec<Game>,
    completed: Vec<Game>,
    planned: Vec<Game>,
}

impl From<GameStatus> for &str {
    fn from(status: GameStatus) -> Self {
        match status {
            GameStatus::Active => "Active",
            GameStatus::Hold => "Hold",
            GameStatus::Completed => "Completed",
            GameStatus::Planned => "Planned",
        }
    }
}
