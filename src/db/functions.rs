use super::utils::{
    params, ADD_GAME, ADD_INVITE, ADD_SONG, ADD_USER, AUTH, CHECK_GAME, CHECK_GAME_ID,
    CHECK_INVITE, CHECK_SONG, CHECK_USER, DEL_INVITE, GAMES, SONGS, UPDATE_GAME,
};
use crate::routes::auth::{Invite, Login};
use crate::routes::{games::Game, songs::Song};
use rocket::serde::Deserialize;
use std::error::Error;
use surrealdb::{error::Db::QueryEmpty, Response, Surreal};

type Db = Surreal<surrealdb::engine::local::Db>;

fn exists<T: for<'a> Deserialize<'a>>(result: Result<Response, surrealdb::Error>) -> bool {
    match result {
        Ok(mut response) => match response.take::<Vec<T>>(0) {
            Ok(v) => !v.is_empty(),
            Err(_) => false,
        },
        Err(_) => false,
    }
}

//
// Songs
//

pub(crate) async fn songs(db: &Db) -> Result<Vec<Song>, Box<dyn Error>> {
    Ok(db.query(SONGS).await?.take(0)?)
}

pub(crate) async fn add_song(db: &Db, artist: &str, title: &str, year: &str) {
    if artist.is_empty() || title.is_empty() || year.is_empty() {
        return;
    }

    let vars = params! { artist, title, year };

    if !exists::<Song>(db.query(CHECK_SONG).bind(vars.clone()).await) {
        _ = db.query(ADD_SONG).bind(vars).await;
    }
}

//
// Games
//

pub(crate) async fn games(db: &Db) -> Result<Vec<Game>, Box<dyn Error>> {
    Ok(db.query(GAMES).await?.take(0)?)
}

pub(crate) async fn add_game(db: &Db, name: &str, status: &str) {
    if name.is_empty() {
        return;
    }

    let vars = params! { name, status };

    if !exists::<Game>(db.query(CHECK_GAME).bind(vars.clone()).await) {
        _ = db.query(ADD_GAME).bind(vars).await;
    }
}

pub(crate) async fn update_game(db: &Db, id: &str, status: &str) {
    let vars = params! { id, status };
    if exists::<Game>(db.query(CHECK_GAME_ID).bind(vars.clone()).await) {
        _ = db.query(UPDATE_GAME).bind(vars).await;
    }
}

//
// Users
//

pub(crate) async fn auth(db: &Db, username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
    let vars = params! { username, password };
    Ok(db
        .query(AUTH)
        .bind(vars)
        .await?
        .take::<Option<bool>>(0)
        .map(|v| v.unwrap_or(false))?)
}

pub(crate) async fn add_user(db: &Db, username: &str, password: &str, invite: &str) -> bool {
    let vars = params! { username, password };
    if exists::<Login>(db.query(CHECK_USER).bind(vars.clone()).await) {
        return false;
    }
    if !check_invite(db, invite).await {
        return false;
    }

    let Ok(_user) = db.query(ADD_USER).bind(vars).await else {
        return false;
    };
    let vars = params! { invite };
    _ = db.query(DEL_INVITE).bind(vars).await;
    true
}

pub(crate) async fn add_invite(db: &Db) -> Result<Invite, Box<dyn Error>> {
    Ok(db
        .query(ADD_INVITE)
        .await?
        .take::<Option<Invite>>(0)
        .and_then(|mut v| v.take().ok_or(surrealdb::Error::Db(QueryEmpty)))?)
}

pub(crate) async fn check_invite(db: &Db, invite: &str) -> bool {
    let vars = params! { invite };
    exists::<bool>(db.query(CHECK_INVITE).bind(vars).await)
}
