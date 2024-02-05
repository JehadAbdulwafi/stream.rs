use anyhow::Result;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod error; 
mod http;
mod utils;

#[derive(Clone)]
pub struct State {
    pub pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let pool = PgPoolOptions::new()
        .connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await?;

    let app_state = State { pool: pool.clone() };

    sqlx::migrate!().run(&pool).await?;

    http::serve(app_state).await
}
