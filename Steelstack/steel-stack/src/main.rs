use diesel::{mysql::MysqlConnection, prelude::*};
use mysql::{params::Params, Pool, PooledConn};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
