use actix_web::http::StatusCode;
use actix_web::{get, post};
use actix_web::{web, Scope};
use actix_web::{HttpResponse, Responder};

use crate::db::db_pool::DbPool;
use crate::db::db_repository::DbRepository;
use crate::db::models::AppUser;
use crate::db::user_repository::UserRepository;
use crate::util::Clock;

pub fn user_endpoints(db_pool: DbPool) -> Scope {
    let user_repo = UserRepository::new(db_pool);
    web::scope("/users")
        .app_data(web::Data::new(user_repo))
        .service(get_users)
        .service(add_user)
        .service(get_user_by_id)
}

#[get("")]
pub async fn get_users(user_repo: web::Data<UserRepository>) -> actix_web::Result<impl Responder> {
    let users = web::block(move || user_repo.find_all()).await?;
    Ok(HttpResponse::Ok().json(users))
}

#[get("/{user_id}")]
pub async fn get_user_by_id(
    path: web::Path<i64>,
    user_repo: web::Data<UserRepository>,
) -> actix_web::Result<impl Responder> {
    let id = path.into_inner();
    let users = web::block(move || user_repo.find_by_id(&id)).await?;
    Ok(HttpResponse::Ok().json(users))
}

#[post("")]
pub async fn add_user(
    new_user: web::Json<AppUser>,
    user_repo: web::Data<UserRepository>,
    clock: web::Data<Clock>,
) -> actix_web::Result<impl Responder> {
    let user = AppUser {
        id: new_user.id,
        username: format!("{}/{}", new_user.username, clock.get_current_time()),
        updated_at: None,
    };
    web::block(move || user_repo.save(&user)).await?;
    Ok(HttpResponse::new(StatusCode::CREATED))
}
