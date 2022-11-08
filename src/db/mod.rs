pub(crate) mod fairing;
pub(crate) mod functions;
pub(crate) mod utils;

pub(crate) use fairing::DbFairing;

pub(crate) struct Db {
    session: surrealdb::Session,
    datastore: surrealdb::Datastore,
}
