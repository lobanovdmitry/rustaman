use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{insert_into, OptionalExtension, QueryDsl, RunQueryDsl};

use crate::db::db_pool::{DbConnSupplier, DbPool};
use crate::db::db_repository;
use crate::db::models::AppUser;
use crate::db::schema::app_user::dsl::app_user;

pub struct UserRepository {
    db_pool: DbPool,
}

impl db_repository::DbRepository for UserRepository {
    type ID = i64;
    type T = AppUser;

    fn new(db_pool: DbPool) -> Self {
        UserRepository { db_pool }
    }

    fn save(&self, new_user: &AppUser) {
        insert_into(app_user::table())
            .values(new_user)
            .execute(&mut self.db_pool.get_conn())
            .expect(format!("Error while inserting user: {:?}.", new_user).as_str());
    }

    fn find_by_id(&self, id_param: &i64) -> Option<AppUser> {
        use super::schema::app_user::dsl::id as id_column;
        app_user::table()
            .filter(id_column.eq(id_param))
            .first(&mut self.db_pool.get_conn())
            .optional()
            .expect(format!("Error while loading user by id: {:?}", id_param).as_str())
    }

    fn find_all(&self) -> Vec<AppUser> {
        app_user
            .load::<AppUser>(&mut self.db_pool.get_conn())
            .expect("Error loading all users.")
    }
}
