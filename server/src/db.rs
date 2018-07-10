use actix::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use error::{Error, Result};

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn init(db_url: &str) -> Result<Self> {
        let conn_manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(conn_manager)
            .map_err(|_| Error::DbPoolCreationError)
            .map(|pool| DbExecutor(pool))
    }
}