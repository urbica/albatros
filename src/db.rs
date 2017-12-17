use std::error::Error;

use persistent::Read;
use iron::typemap::Key;
use iron::prelude::{Plugin, Request};

use r2d2;
use diesel::PgConnection;
use r2d2_diesel::ConnectionManager;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type Connection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub struct DB;
impl Key for DB { type Value = Pool; }

pub fn setup_connection_pool(cn_str: &str, pool_size: u32) -> Result<Pool, Box<Error>> {
    let manager = ConnectionManager::<PgConnection>::new(cn_str);
    let pool = try!(r2d2::Pool::builder()
        .max_size(pool_size)
        .build(manager));

    Ok(pool)
}

pub fn get_connection(req: &mut Request) -> Result<Connection, Box<Error>> {
    let pool = try!(req.get::<Read<DB>>());
    let conn = try!(pool.get());
    Ok(conn)
}
