use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

use crate::config::app_config::DbConfig;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbPoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn new_pg_pool(config: &DbConfig) -> DbPool {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.db_name
    );
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to Postgres")
}

pub trait DbConnSupplier {
    fn get_conn(&self) -> DbPoolConnection;
}

impl DbConnSupplier for DbPool {
    fn get_conn(&self) -> DbPoolConnection {
        self.get().expect("Failed to get db connection!")
    }
}
