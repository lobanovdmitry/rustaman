use crate::db::db_pool::DbPool;

#[derive(Clone)]
pub struct WebState {
    pub db_pool: DbPool,
}
