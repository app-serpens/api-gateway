use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
  pub pool: DbPool,
}

pub fn get_pool() -> DbPool {
  dotenv().ok();
  let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
  r2d2::Pool::builder()
      .build(manager)
      .expect("Could not build connection pool")
}