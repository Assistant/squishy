use std::collections::BTreeMap;
use surrealdb::sql;

macro_rules! params {
    () => {
        Option::<std::collections::BTreeMap<String, surrealdb::sql::Value>>::None
    };
    ($($key:ident $(: $value:expr)?),*$(,)?) => {
        params!(_[$(params!(_$key $(: $value)?)),*])
    };

    (_$key:ident) => {
        (stringify!($key), $key)
    };
    (_$key:ident : $value:expr) => {
        (stringify!($key), $value)
    };
    (_[$($input:expr),*]) => {
        Some(crate::db::utils::get_params(vec![$($input),*]))
    };
}
pub(crate) use params;

fn parse_id(id: &str) -> sql::Value {
    let mut parts = id.split(':');
    let (Some(tb), Some(id)) = (parts.next(), parts.next()) else {
        return sql::Value::None;
    };
    let (tb, id) = (tb.into(), id.into());
    sql::Value::Thing(sql::Thing { tb, id })
}

pub(crate) fn get_params(params: Vec<(&str, &str)>) -> BTreeMap<String, sql::Value> {
    params
        .iter()
        .map(|p| match p {
            ("id", v) => ("id".to_string(), parse_id(v)),
            (k, v) => (k.to_string(), sql::Value::from(*v)),
        })
        .collect()
}

pub(crate) static SONGS: &str = "SELECT * FROM song ORDER BY modified DESC;";
pub(crate) static ADD_SONG: &str =
    "CREATE song SET artist = $artist, title = $title, year = $year, modified = time::now();";
pub(crate) static CHECK_SONG: &str =
    "SELECT * FROM song WHERE artist = $artist AND title = $title AND year = $year";

pub(crate) static GAMES: &str = "SELECT * FROM game ORDER BY modified DESC;";
pub(crate) static ADD_GAME: &str =
    "CREATE game SET name = $name, status = $status, modified = time::now();";
pub(crate) static CHECK_GAME: &str = "SELECT * FROM game WHERE name = $name;";
pub(crate) static CHECK_GAME_ID: &str = "SELECT * FROM $id;";
pub(crate) static UPDATE_GAME: &str = "UPDATE $id SET status = $status, modified = time::now();";

pub(crate) static AUTH: &str = "SELECT * FROM crypto::argon2::compare(type::string((SELECT password FROM user WHERE username = $username LIMIT 1)), $password);";
pub(crate) static ADD_USER: &str =
    "CREATE user SET username = $username, password = crypto::argon2::generate($password);";
pub(crate) static CHECK_USER: &str = "SELECT * FROM user WHERE username = $username;";

pub(crate) static ADD_INVITE: &str = "CREATE invite SET code = crypto::sha1(time::now());";
pub(crate) static CHECK_INVITE: &str = "SELECT * FROM invite WHERE code = $invite;";
pub(crate) static DEL_INVITE: &str = "DELETE invite WHERE code = $invite;";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_none() {
        let params = params! {};
        assert_eq!(params, None);
    }

    #[test]
    fn params_single() {
        let test = "string";
        let params = params! { test };

        let mut map = BTreeMap::new();
        map.insert("test".to_string(), sql::Value::from("string"));

        assert_eq!(params, Some(map));
    }

    #[test]
    fn params_expression() {
        let params = params! { test: &"string".to_uppercase() };

        let mut map = BTreeMap::new();
        map.insert("test".to_string(), sql::Value::from("STRING"));

        assert_eq!(params, Some(map));
    }

    #[test]
    fn params_trailing_comma() {
        let test = "string";
        let params = params! {
            test,
        };

        let mut map = BTreeMap::new();
        map.insert("test".to_string(), sql::Value::from("string"));

        assert_eq!(params, Some(map));
    }

    #[test]
    fn params_multi() {
        let test = "string";
        let test2 = "string2";
        let params = params! { test, test2 };

        let mut map = BTreeMap::new();
        map.insert("test".to_string(), sql::Value::from("string"));
        map.insert("test2".to_string(), sql::Value::from("string2"));

        assert_eq!(params, Some(map));
    }

    #[test]
    fn params_rename() {
        let test = "string";
        let params = params! { name: test };

        let mut map = BTreeMap::new();
        map.insert("name".to_string(), sql::Value::from("string"));

        assert_eq!(params, Some(map));
    }

    #[test]
    fn params_multi_rename() {
        let test = "string";
        let test2 = "string2";
        let params = params! { test, name: test2 };

        let mut map = BTreeMap::new();
        map.insert("test".to_string(), sql::Value::from("string"));
        map.insert("name".to_string(), sql::Value::from("string2"));

        assert_eq!(params, Some(map));
    }

    #[test]
    fn params_id() {
        let id = "string:idstring";
        let params = params! { id };

        let mut parts = id.split(':');
        let tb = parts.next().unwrap();
        let id = parts.next().unwrap();
        let (tb, id) = (tb.into(), id.into());
        let id = sql::Value::Thing(sql::Thing { tb, id });

        let mut map = BTreeMap::new();
        map.insert("id".to_string(), id);

        assert_eq!(params, Some(map));
    }

    #[test]
    fn params_id_rename() {
        let test = "string:idstring";
        let params = params! { id: test };

        let mut parts = test.split(':');
        let tb = parts.next().unwrap();
        let id = parts.next().unwrap();
        let (tb, id) = (tb.into(), id.into());
        let id = sql::Value::Thing(sql::Thing { tb, id });

        let mut map = BTreeMap::new();
        map.insert("id".to_string(), id);

        assert_eq!(params, Some(map));
    }
}
