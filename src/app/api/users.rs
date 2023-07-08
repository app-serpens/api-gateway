use actix_web::{web, Responder, HttpResponse, post};
use diesel::RunQueryDsl;

use crate::app::db::DbPool;
use crate::app::models::user::User;

#[post("/login")]
pub async fn login(db_pool: web::Data<DbPool>) -> impl Responder {
  use crate::schema::users;

  let conn_result = db_pool.get();
  if let Err(e) = conn_result {
    return HttpResponse::InternalServerError().body(format!("{:?}", e))
  }

  let mut conn = conn_result.unwrap();
  let query_result = web::block(move || users::table.load::<User>(&mut conn).unwrap()).await;

  if let Err(e) = query_result {
    return HttpResponse::InternalServerError().body(format!("{:?}", e));
  }

  let users = query_result.unwrap();
  let user = users.first();
  HttpResponse::Ok().json(user)
}