pub trait GetEmailClient {
    fn get_email_client(&self)
        -> crate::repositories_types::server::email_client::EmailClient;
}

impl<SelfGeneric> GetEmailClient for SelfGeneric
where
    Self: app_state::GetBaseUrl,
{
    fn get_email_client(
        &self,
    ) -> crate::repositories_types::server::email_client::EmailClient {
        crate::repositories_types::server::email_client::EmailClient::new(
            self.get_base_url().to_string(),
            crate::repositories_types::server::domain::SubscriberEmail::try_from(
                "test@gmail.com".to_string(),
            )
            .expect("Invalid sender email address."),
            secrecy::Secret::new("my-secret-token".to_string()),
            std::time::Duration::from_secs(10),
        )
    }
}
