use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
  pub id: Uuid,
  pub email: String,
  pub password: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}