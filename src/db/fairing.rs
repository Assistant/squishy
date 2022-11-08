use super::Db;
use rocket::{
    fairing::{Fairing, Info, Kind, Result},
    serde::{json, Deserialize},
    Build, Rocket,
};
use std::collections::BTreeMap;
use surrealdb::{sql::Value, Datastore, Error, Session};

impl Db {
    pub(crate) async fn new(namespace: &str, database: &str, datastore: &str) -> Self {
        Self {
            session: Session::for_db(namespace.to_string(), database.to_string()),
            datastore: Datastore::new(datastore).await.unwrap(),
        }
    }

    pub(crate) async fn query(
        &self,
        statement: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> Result<Value, Error> {
        let responses = self
            .datastore
            .execute(statement, &self.session, vars, false)
            .await?;
        responses
            .into_iter()
            .last()
            .and_then(|r| r.result.ok())
            .ok_or(Error::Ignore)
    }

    fn parse(value: Value) -> Result<json::Value, Error> {
        json::to_value(value).ok().ok_or(Error::Ignore)
    }

    pub(crate) async fn find_one(
        &self,
        statement: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> Result<json::Value, Error> {
        let result = self.find_many(statement, vars).await?;
        let value = if result.is_array() {
            if !result.as_array().unwrap().is_empty() {
                result[0].clone()
            } else {
                json::Value::Null
            }
        } else {
            result
        };
        Ok(value)
    }

    pub(crate) async fn find_many(
        &self,
        statement: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> Result<json::Value, Error> {
        Self::parse(self.query(statement, vars).await?)
    }

    pub(crate) async fn exists(
        &self,
        statement: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> Result<bool, Error> {
        let result = self.query(statement, vars).await?;
        Ok(result.is_truthy())
    }
}

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

        let db = Db::new(
            &db_config.namespace,
            &db_config.database,
            &db_config.datastore,
        )
        .await;

        Ok(rocket.manage(db))
    }
}
