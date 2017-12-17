use std::env;
use dotenv::dotenv;
extern crate dotenv;

extern crate iron;
extern crate mount;
extern crate router as iron_router;
extern crate bodyparser;
extern crate persistent;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_infer_schema;

use iron::Iron;
use persistent::Read;

mod db;
pub mod schema;
mod router;
mod project;

fn main() {
    dotenv().ok();

    let conn_string: String = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    println!("Connecting to postgres: {}", conn_string);
    let pool = db::setup_connection_pool(&conn_string, 25)
        .expect("Error connectiong to postgres");

    let mut router = router::create();
    router.link(Read::<db::DB>::both(pool));

    let port = 3000;
    let bind_addr = format!("0.0.0.0:{}", port);
    println!("Server has been started on {}.", bind_addr);
    Iron::new(router).http(bind_addr.as_str()).unwrap();
}
