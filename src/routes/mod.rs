pub(crate) mod auth;
pub(crate) mod games;
pub(crate) mod index;
pub(crate) mod songs;
pub(crate) mod utils;

pub(crate) fn routes() -> Vec<rocket::Route> {
    routes![
        auth::index,
        auth::login,
        auth::register,
        auth::register_page,
        auth::add_invite,
        songs::index,
        songs::add_song,
        games::index,
        games::edit_games,
        index::index,
    ]
}
