use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dropshot::HttpError;
use lazy_static::lazy_static;
use r2d2;
use std::env;
use std::time::Duration;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// Needed so we can later trigger a migrate at startup time.
embed_migrations!();

// Initialize the pool lazily.
lazy_static! {
    static ref POOL: Pool = {
        let timeout = Duration::from_secs(2 * 60);
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::builder()
            .connection_timeout(timeout)
            .build(manager)
            .expect("Failed to create database pool!")
    };
}

// Initialize the database. This should be done before starting the web server.
pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    embedded_migrations::run(&conn).unwrap();
}

// Try to get a connection from the pool.
pub fn connection() -> Result<DbConnection, HttpError> {
    POOL.get()
        .map_err(|e| HttpError::for_internal_error(format!("Failed getting db connection: {}", e)))
}
