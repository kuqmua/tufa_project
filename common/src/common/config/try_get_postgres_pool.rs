#[derive(Debug)]
pub enum TryGetPostgresPoolError {
    Connect(sqlx::Error),
}

impl std::fmt::Display for TryGetPostgresPoolError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connect(error) => write!(formatter, "{error}"),
        }
    }
}

impl error_occurence_lib::ToStdStringString for TryGetPostgresPoolError {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}

#[allow(async_fn_in_trait)] //todo wait for async trait stabilization

pub trait TryGetPostgresPool {
    async fn try_get_postgres_pool(
        &self,
    ) -> Result<sqlx::Pool<sqlx::Postgres>, TryGetPostgresPoolError>;
}

impl<SelfGeneric> TryGetPostgresPool for SelfGeneric
where
    Self: app_state::GetDatabaseUrl + Sync + Send, //meaning postgres. sqlx::query! macro uses DATABASE_URL env var for compile time checks
{
    async fn try_get_postgres_pool(
        &self,
    ) -> Result<sqlx::Pool<sqlx::Postgres>, TryGetPostgresPoolError> {
        println!("trying to create postgres pool...");
        match sqlx::postgres::PgPoolOptions::new()
            .connect(secrecy::ExposeSecret::expose_secret(
                self.get_database_url(),
            ))
            .await
        {
            Err(error) => Err(TryGetPostgresPoolError::Connect(error)),
            Ok(pool) => Ok(pool),
        }
    }
}
