use std::result;

pub type Result<T = ()> = result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    // Could not get the URL of the database
    DbUrlError,
    // Could not create a connection pool
    DbPoolCreationError,
    // Could not get a db connection from the pool
    DbPoolConnectionError,
    // Resource not found
    ResourceNotFound,
}