use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult},
    PgPool, Row,
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

    pub async fn init_table_platform_steam_users(&self) -> Result<PgQueryResult, Box<dyn Error>> {
        Ok(sqlx::query(
            "
                CREATE TABLE IF NOT EXISTS PlatformSteamUsers
                (
                    id SERIAL PRIMARY KEY,
                    user_steam_id BIGINT NOT NULL CHECK (user_steam_id > 0),
                    user_ip_addr VARCHAR(15) NOT NULL CHECK (user_steam_id < 100000000000000000)
                );",
        )
        .execute(
            self.pool
                .as_ref()
                .expect("Failed to execute query: pool is not initialized"),
        )
        .await?)
    }

    pub async fn query_table_platform_steam_users_count(&self) -> Result<i64, Box<dyn Error>> {
        Ok(sqlx::query("SELECT COUNT(id) FROM PlatformSteamUsers;")
            .fetch_one(
                self.pool
                    .as_ref()
                    .expect("Failed to execute query: pool is not initialized"),
            )
            .await?
            .get(0))
    }

    pub async fn query_table_platform_steam_users_list(
        &self,
        page: usize,
        per_page: usize,
    ) -> Result<Vec<(u64, String)>, Box<dyn Error>> {
        let users: Vec<(u64, String)> =
            sqlx::query_as(&format!("SELECT user_steam_id, user_ip_addr FROM PlatformSteamUsers ORDER BY id DESC LIMIT {} OFFSET {};", per_page, page * per_page))
                .fetch_all(self.pool.as_ref().expect("Failed to execute query: pool is not initialized"))
                .await?
                .iter()
                .map(|(id, ip): &(i64, String)| (*id as u64, ip.to_owned()))
                .collect::<Vec<(u64, String)>>();

        Ok(users)
    }

    pub async fn query_table_platform_steam_users_by_id(
        &self,
        steam_id: u64,
    ) -> Result<Vec<(u64, String)>, Box<dyn Error>> {
        let users: Vec<(u64, String)> = sqlx::query_as(&format!(
            "SELECT user_steam_id, user_ip_addr FROM PlatformSteamUsers WHERE user_steam_id = {} LIMIT 1000;",
            steam_id
        ))
        .fetch_all(
            self.pool
                .as_ref()
                .expect("Failed to execute query: pool is not initialized"),
        )
        .await?
        .iter()
        .map(|(id, ip): &(i64, String)| (*id as u64, ip.to_owned()))
        .collect::<Vec<(u64, String)>>();

        Ok(users)
    }
}
