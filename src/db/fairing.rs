use rocket::fairing::{Fairing, Info, Kind, Result};
use rocket::serde::Deserialize;
use rocket::{Build, Rocket};
use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;

pub(crate) struct DbFairing;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct DbConfig {
    namespace: String,
    database: String,
    datastore: String,
}

#[rocket::async_trait]
impl Fairing for DbFairing {
    fn info(&self) -> Info {
        Info {
            name: "Database",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result {
        let figment = rocket.figment().clone();

        let db_config: DbConfig = figment
            .select("database")
            .extract()
            .expect("Surreal.toml config");

        let db = Surreal::new::<RocksDb>(&db_config.datastore)
            .await
            .expect("Failed to initialize database");

        db.use_ns(&db_config.namespace)
            .use_db(&db_config.database)
            .await
            .expect("Failed to set namespace and database");

        Ok(rocket.manage(db))
    }
}
