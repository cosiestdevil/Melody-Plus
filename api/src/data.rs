use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub type Connection = Pool<Postgres>;

pub async fn get_connection_pool() ->anyhow::Result<Connection>{
    let pool = PgPoolOptions::new()
        .connect(dotenvy::var("DATABASE_URL")?.as_str()).await.unwrap();
    Ok(pool)
}
