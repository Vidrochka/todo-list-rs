use std::env;

use slog::Logger;
use sqlx::{
    PgPool,
    migrate::Migrator
};

const DATABASE_URL_ENV: &str = "DATABASE_URL";

pub async fn prepare_db(logger: &Logger) -> anyhow::Result<PgPool> {
    let db_address = get_db_string();
    slog::info!(logger, "{DATABASE_URL_ENV}:[{db_address}]. Creating connection pool ...");

    let db_pool = PgPool::connect(&db_address).await?;
    let migrator = Migrator::new(std::path::Path::new("./migrations")).await?;
    migrator.run(&db_pool).await?;
    Ok(db_pool)
}

fn get_db_string() -> String {
    env::var(DATABASE_URL_ENV)
        .expect(&*format!("Env {DATABASE_URL_ENV} not found"))
}