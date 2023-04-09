use crate::db::db_pool::DbPool;

pub trait DbRepository {
    type ID;
    type T;

    fn new(db_pool: DbPool) -> Self;

    fn save(&self, entity: &Self::T);

    fn find_by_id(&self, id: &Self::ID) -> Option<Self::T>;

    fn find_all(&self) -> Vec<Self::T>;
}
