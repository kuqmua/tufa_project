#[derive(Debug)]
pub struct ServerAppState<'lifetime> {
    pub config: server_config::Config,
    pub postgres_pool: sqlx::PgPool,
    pub project_git_info: &'lifetime git_info::ProjectGitInfo<'lifetime>,
}
impl common_routes::CommonRoutesParameters for ServerAppState<'_> {}
impl postgresql_crud::CombinationOfAppStateLogicTraits for ServerAppState<'_> {}
impl app_state::GetEnableApiGitCommitCheck for ServerAppState<'_> {
    fn get_enable_api_git_commit_check(&self) -> &bool {
        self.config.get_enable_api_git_commit_check()
    }
}
impl app_state::GetSourcePlaceType for ServerAppState<'_> {
    fn get_source_place_type(&self) -> &config_lib::types::SourcePlaceType {
        self.config.get_source_place_type()
    }
}
impl app_state::GetTimezone for ServerAppState<'_> {
    fn get_timezone(&self) -> &chrono::FixedOffset {
        self.config.get_timezone()
    }
}
impl app_state::GetMaximumSizeOfHttpBodyInBytes for ServerAppState<'_> {
    fn get_maximum_size_of_http_body_in_bytes(&self) -> &usize {
        self.config.get_maximum_size_of_http_body_in_bytes()
    }
}
impl app_state::GetPostgresPool for ServerAppState<'_> {
    fn get_postgres_pool(&self) -> &sqlx::PgPool {
        &self.postgres_pool
    }
}
impl git_info::GetGitCommitLink for ServerAppState<'_> {
    fn get_git_commit_link(&self) -> String {
        self.project_git_info.get_git_commit_link()
    }
}
