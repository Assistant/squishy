use super::{
    utils::{
        params, ADD_GAME, ADD_INVITE, ADD_SONG, ADD_USER, AUTH, CHECK_GAME, CHECK_GAME_ID,
        CHECK_INVITE, CHECK_SONG, CHECK_USER, DEL_INVITE, GAMES, SONGS, UPDATE_GAME,
    },
    Db,
};
use crate::routes::{auth::Invite, games::Game, songs::Song};
use rocket::serde::json::{self, Value};
use std::error::Error;

//
// Songs
//

pub(crate) async fn songs(db: &Db) -> Result<Vec<Song>, Box<dyn Error>> {
    let result = db.find_many(SONGS, None).await?;
    let songs = json::from_value(result)?;
    Ok(songs)
}

pub(crate) async fn add_song(db: &Db, artist: &str, title: &str, year: &str) {
    if artist.is_empty() || title.is_empty() || year.is_empty() {
        return;
    }
    let vars = params! { artist, title, year };
    if let Ok(false) = db.exists(CHECK_SONG, vars.clone()).await {
        _ = db.query(ADD_SONG, vars).await;
    }
}

//
// Games
//

pub(crate) async fn games(db: &Db) -> Result<Vec<Game>, Box<dyn Error>> {
    let result = db.find_many(GAMES, None).await?;
    let games = json::from_value(result)?;
    Ok(games)
}

pub(crate) async fn add_game(db: &Db, name: &str, status: &str) {
    if name.is_empty() {
        return;
    }
    let vars = params! { name, status };
    if let Ok(false) = db.exists(CHECK_GAME, vars.clone()).await {
        _ = db.query(ADD_GAME, vars).await;
    }
}

pub(crate) async fn update_game(db: &Db, id: &str, status: &str) {
    let vars = params! { id, status };
    if let Ok(true) = db.exists(CHECK_GAME_ID, vars.clone()).await {
        _ = db.query(UPDATE_GAME, vars).await;
    }
}

//
// Users
//

pub(crate) async fn auth(db: &Db, username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
    let vars = params! { username, password };
    let result = db.find_one(AUTH, vars).await?;

    Ok(matches!(result, Value::Bool(true)))
}

pub(crate) async fn add_user(db: &Db, username: &str, password: &str, invite: &str) -> bool {
    let vars = params! { username, password };
    let Ok(false) = db.exists(CHECK_USER, vars.clone()).await else {
        return false;
    };
    if !check_invite(db, invite).await {
        return false;
    }

    let Ok(_user) = db.query(ADD_USER, vars).await else {
        return false;
    };
    let vars = params! { invite };
    _ = db.query(DEL_INVITE, vars).await;
    true
}

pub(crate) async fn add_invite(db: &Db) -> Result<Invite, Box<dyn Error>> {
    let result = db.find_one(ADD_INVITE, None).await?;
    let value = json::from_value(result)?;
    Ok(value)
}

pub(crate) async fn check_invite(db: &Db, invite: &str) -> bool {
    let vars = params! { invite };
    db.exists(CHECK_INVITE, vars).await.unwrap_or(false)
}
