use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult},
    PgPool,
};
use std::env;
use std::error::Error;

#[derive(Debug)]
pub struct PgDb {
    url: String,
    pool: Option<PgPool>,
}

impl PgDb {
    pub fn new() -> Result<PgDb, Box<dyn Error>> {
        Ok(PgDb {
            url: format!(
                "postgres://{}:{}@{}:{}/{}",
                env::var("PSQL_USER")?,
                env::var("PSQL_PASS")?,
                env::var("PSQL_HOST")?,
                env::var("PSQL_PORT")?,
                env::var("PSQL_NAME")?
            ),
            pool: None,
        })
    }

    pub async fn init_pool(&mut self) -> Result<(), Box<dyn Error>> {
        self.pool = Some(
            PgPoolOptions::new()
                .max_connections(env::var("PSQL_MAXCONN")?.parse::<u32>()?)
                .connect(&self.url)
                .await?,
        );

        Ok(())
    }

    pub async fn init_table_platform_steam(&self) -> Result<PgQueryResult, sqlx::Error> {
        Ok(sqlx::query(
            "
                CREATE TABLE IF NOT EXISTS PlatformSteam
                (
                    id SERIAL PRIMARY KEY,
                    user_steam_id BIGINT NOT NULL,
                    ip_punished VARCHAR(15) NOT NULL
                );",
        )
        .execute(
            self.pool
                .as_ref()
                .expect("Failed to initialize PlatformSteam table - pool is not initialized"),
        )
        .await?)
    }
}

