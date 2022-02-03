use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dropshot::HttpError;
use lazy_static::lazy_static;
use std::time::Duration;
use std::{env, fs};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// Needed so we can later trigger a migrate at startup time.
embed_migrations!();

// Initialize the pool lazily.
lazy_static! {
    static ref POOL: Pool = {
        let db_url = match env::var("DATABASE_URL") {
            Ok(v) => v,
            Err(_) => match env::var("DATABASE_URL_FILE") {
                Ok(path) => fs::read_to_string(path)
                    .expect("Could not read file pointed by 'DATABASE_URL_FILE'")
                    .trim()
                    .to_owned(),
                Err(_) => panic!("Either provide DATABASE_URL or DATABASE_URL_FILE"),
            },
        };

        let timeout = Duration::from_secs(5 * 60);
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
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
        .expect("Failed to run migrations!");
}

// Try to get a connection from the pool.
pub fn connection() -> Result<DbConnection, HttpError> {
    POOL.get()
        .map_err(|e| HttpError::for_internal_error(format!("Failed getting db connection: {}", e)))
}
