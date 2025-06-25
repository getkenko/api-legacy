// TODO(swaglord): use ConnectionManager for automatic reconnections

use redis::{aio::MultiplexedConnection, AsyncCommands};

#[derive(Clone)]
pub struct Cache {
    conn: MultiplexedConnection,
}

impl Cache {
    // swaglord: I have no idea what other return type I could set xdd
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

    pub async fn increment_requests(&self, key: &str) -> redis::RedisResult<u32> {
        let mut conn = self.conn.clone();

        let new_value = conn.incr(&key, 1).await?;

        // set expiration to one minute for new keys
        if new_value <= 1 {
            let _: () = conn.expire(&key, 60).await?;
        }

        Ok(new_value)
    }
}
