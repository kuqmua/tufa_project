// #[derive(Debug, strum_macros::Display)]
// pub enum TryGetRedisSessionStorageError {
//     Redis(std::string::String), //anyhow::Error
// }

// pub trait TryGetRedisSessionStorage {
//     async fn try_get_redis_session_storage(
//         &self,
//     ) -> Result<actix_session::storage::RedisSessionStore, TryGetRedisSessionStorageError>;
// }

// impl<SelfGeneric> TryGetRedisSessionStorage for SelfGeneric
// where
//     Self: crate::common::config::config_fields::GetRedisUrl,
// {
//     async fn try_get_redis_session_storage(
//         &self,
//     ) -> Result<actix_session::storage::RedisSessionStore, TryGetRedisSessionStorageError> {
//         match actix_session::storage::RedisSessionStore::new(secrecy::ExposeSecret::expose_secret(self.get_redis_url()))
//         .await
//         {
//             Ok(redis_session_store) => Ok(redis_session_store),
//             Err(e) => Err(TryGetRedisSessionStorageError::Redis(format!("{e}"))),
//         }
//     }
// }
