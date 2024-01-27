#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
async fn get_stored_credentials<'a>(
    username: &str,
    pool: &sqlx::PgPool,
) -> Result<Option<(uuid::Uuid, secrecy::Secret<String>)>, tufa_common::repositories_types::tufa_server::authentication::password::GetStoredCredentialsErrorNamed>{
    match sqlx::query!(
        r#"
        SELECT user_id, password_hash
        FROM users
        WHERE username = $1
        "#,
        username,
    )
    .fetch_optional(pool)
    .await
    {
        Err(e) => Err(tufa_common::repositories_types::tufa_server::authentication::password::GetStoredCredentialsErrorNamed::FetchOptional {
            fetch_optional: e,
            code_occurence: tufa_common::code_occurence!()
        }),
        Ok(option_record) => Ok(option_record.map(|row| (row.user_id, secrecy::Secret::new(row.password_hash)))),
    }
}

pub async fn validate_credentials<'a>(
    credentials: tufa_common::common::postgres_credentials::PostgresCredentials,
    pool: &sqlx::PgPool,
) -> Result<uuid::Uuid, tufa_common::repositories_types::tufa_server::authentication::password::ValidateCredentialsErrorNamed>{
    let mut user_id = None;
    let mut expected_password_hash = secrecy::Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );
    match get_stored_credentials(&credentials.username, pool).await {
        Err(e) => {
            return Err(tufa_common::repositories_types::tufa_server::authentication::password::ValidateCredentialsErrorNamed::GetStoredCredentials {
                get_stored_credentials: e,
                code_occurence: tufa_common::code_occurence!()
            });
        }
        Ok(option) => {
            if let Some((stored_user_id, stored_password_hash)) = option {
                user_id = Some(stored_user_id);
                expected_password_hash = stored_password_hash;
            }
        }
    }
    match tufa_common::repositories_types::tufa_server::telemetry::spawn_blocking_with_tracing::spawn_blocking_with_tracing(
        move || {
            verify_password_hash(expected_password_hash, credentials.password)
        }
    ).await {
        Err(e) => Err(tufa_common::repositories_types::tufa_server::authentication::password::ValidateCredentialsErrorNamed::SpawnBlockingWithTracing {
            spawn_blocking_with_tracing: e,
            code_occurence: tufa_common::code_occurence!()
        }),
        Ok(result) => match result {
            Err(e) => Err(tufa_common::repositories_types::tufa_server::authentication::password::ValidateCredentialsErrorNamed::VerifyPasswordHash {
                spawn_blocking_with_tracing: e,
                code_occurence: tufa_common::code_occurence!()
            }),
            Ok(_) => match user_id {
                None => Err(tufa_common::repositories_types::tufa_server::authentication::password::ValidateCredentialsErrorNamed::UnknownUsername {
                    message: "Unknown username".to_string(),
                    code_occurence: tufa_common::code_occurence!()
                }),
                Some(uuid) => Ok(uuid),
            },
        },
    }
}

#[tracing::instrument(
    name = "Validate credentials",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash<'a>(
    expected_password_hash: secrecy::Secret<String>,
    password_candidate: secrecy::Secret<String>,
) -> Result<(), tufa_common::repositories_types::tufa_server::authentication::password::VerifyPasswordHashErrorNamed>{
    match argon2::PasswordHash::new(secrecy::ExposeSecret::expose_secret(&expected_password_hash)) {
        Err(e) => Err(tufa_common::repositories_types::tufa_server::authentication::password::VerifyPasswordHashErrorNamed::ExposeSecret {
            expose_secret: e,
            code_occurence: tufa_common::code_occurence!()
        }),
        Ok(expected_password_hash) => match argon2::PasswordVerifier::verify_password(
                &argon2::Argon2::default(), 
                secrecy::ExposeSecret::expose_secret(&password_candidate).as_bytes(),
                &expected_password_hash
            ) {
            Err(e) => Err(tufa_common::repositories_types::tufa_server::authentication::password::VerifyPasswordHashErrorNamed::InvalidPassword {
                invalid_password: e,
                code_occurence: tufa_common::code_occurence!()
            }),
            Ok(_) => Ok(())
        }
    }
}

pub async fn change_password<'a>(
    user_id: uuid::Uuid,
    password: secrecy::Secret<String>,
    pool: &sqlx::PgPool,
) -> Result<(), tufa_common::repositories_types::tufa_server::authentication::password::ChangePasswordErrorNamed>{
    match tufa_common::repositories_types::tufa_server::telemetry::spawn_blocking_with_tracing::spawn_blocking_with_tracing(
        move || compute_password_hash(password)
    ).await {
        Err(e) => Err(tufa_common::repositories_types::tufa_server::authentication::password::ChangePasswordErrorNamed::SpawnBlockingWithTracing {
            spawn_blocking_with_tracing: e,
            code_occurence: tufa_common::code_occurence!()
        }),
        Ok(res) => match res {
            Err(e) => Err(tufa_common::repositories_types::tufa_server::authentication::password::ChangePasswordErrorNamed::ComputePasswordHash {
                compute_password_hash: e,
                code_occurence: tufa_common::code_occurence!()
            }),
            Ok(password_hash) => match sqlx::query!(
                r#"
                        UPDATE users
                        SET password_hash = $1
                        WHERE user_id = $2
                    "#,
                    secrecy::ExposeSecret::expose_secret(&password_hash),
                    user_id
            )
            .execute(pool)
            .await {
                Err(e) => Err(tufa_common::repositories_types::tufa_server::authentication::password::ChangePasswordErrorNamed::PostgresQuery {
                    query_error: e,
                    code_occurence: tufa_common::code_occurence!()
                }),
                Ok(_) => Ok(()),
            }
        },
    }
}

fn compute_password_hash<'a>(password: secrecy::Secret<String>) -> Result<secrecy::Secret<String>, tufa_common::repositories_types::tufa_server::authentication::password::ComputePasswordHashErrorNamed>{
    match argon2::PasswordHasher::hash_password(
        &argon2::Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::new(15000, 2, 1, None).unwrap(),
        ), 
        secrecy::ExposeSecret::expose_secret(&password).as_bytes(), 
        &argon2::password_hash::SaltString::generate(&mut rand::thread_rng())
    ) {
        Ok(password_hash) => Ok(secrecy::Secret::new(password_hash.to_string())),
        Err(e) => Err(
            tufa_common::repositories_types::tufa_server::authentication::password::ComputePasswordHashErrorNamed::PasswordHash {
                argon2_password_hash_error: e,
                code_occurence: tufa_common::code_occurence!(),
            }
        ),
    }
}
