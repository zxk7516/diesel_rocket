use std::ops::Deref;

use r2d2;
use diesel::mysql::MysqlConnection;
use r2d2_mysql::MysqlConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub type DBPool = r2d2::Pool<MysqlConnectionManager>;

pub fn init_pool(database_url: &str) -> DBPool {
    let config = r2d2::Config::default();
    let manager =   MysqlConnectionManager::new(database_url).unwrap();
    r2d2::Pool::new(config, manager).expect("Error connecting to Postgres")
}

pub struct Connection(r2d2::PooledConnection<MysqlConnectionManager>);

impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, ()> {
        let pool = match <State<DBPool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        match pool.get() {
            Ok(connection) => Outcome::Success(Connection(connection)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}