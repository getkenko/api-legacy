// TODO: use ConnectionManager for automatic reconnections

use chrono::{DateTime, Duration, TimeZone, Utc};
use redis::{AsyncCommands, SetExpiry, SetOptions, aio::MultiplexedConnection};
use uuid::Uuid;

const RATE_LIMIT_PREFIX: &str = "rate-limit";
const LAST_CHECK_PREFIX: &str = "last-check";

#[derive(Clone)]
pub struct Cache {
    conn: MultiplexedConnection,
}

impl Cache {
    // I have no idea what other return type I could set xdd
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        // initialize connection
        let redis = redis::Client::open(url)?;

        // check if established
        let mut conn = redis.get_multiplexed_async_connection().await?;
        let (k, v) = ("foo", "bar");
        let _: () = conn.set(k, v).await?;
        let res: String = conn.get(k).await?;
        if res != v {
            anyhow::bail!("Failed to establish redis connection");
        }

        let conn = redis.get_multiplexed_async_connection().await?;

        Ok(Self { conn })
    }

    pub async fn increment_requests(&self, ip: &str) -> redis::RedisResult<u32> {
        let mut conn = self.conn.clone();
        let key = format!("{RATE_LIMIT_PREFIX}_{ip}");

        let new_value = conn.incr(&key, 1).await?;

        // set expiration to one minute for new keys
        if new_value <= 1 {
            let _: () = conn.expire(&key, 60).await?;
        }

        Ok(new_value)
    }

    pub async fn user_last_check(&self, user_id: Uuid) -> redis::RedisResult<DateTime<Utc>> {
        let mut conn = self.conn.clone();
        let key = format!("{LAST_CHECK_PREFIX}_{user_id}");

        let timestamp = conn.get::<_, Option<i64>>(key).await?.unwrap_or(1000);
        let date_time = Utc.timestamp_opt(timestamp, 0).unwrap();

        Ok(date_time)
    }

    pub async fn update_user_last_check(&self, user_id: Uuid) -> redis::RedisResult<()> {
        let mut conn = self.conn.clone();
        let key = format!("{LAST_CHECK_PREFIX}_{user_id}");

        let now = Utc::now().timestamp();
        let opt = SetOptions::default()
            .with_expiration(SetExpiry::EX(Duration::days(3).num_seconds() as _));
        let _: () = conn.set_options(key, now, opt).await?;

        Ok(())
    }
}
