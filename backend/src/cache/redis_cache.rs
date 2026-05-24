use redis::{
    Client,
    Commands,
    RedisResult,
};

pub struct RedisCache {
    pub client: Client,
}

impl RedisCache {
    pub fn new(
        redis_url: &str,
    ) -> RedisResult<Self> {
        let client =
            Client::open(redis_url)?;

        Ok(Self { client })
    }

    pub fn set(
        &self,
        key: &str,
        value: &str,
    ) -> RedisResult<()> {
        let mut connection =
            self.client.get_connection()?;

        connection.set(key, value)
    }

    pub fn get(
        &self,
        key: &str,
    ) -> RedisResult<String> {
        let mut connection =
            self.client.get_connection()?;

        connection.get(key)
    }

    pub fn delete(
        &self,
        key: &str,
    ) -> RedisResult<()> {
        let mut connection =
            self.client.get_connection()?;

        connection.del(key)
    }
}