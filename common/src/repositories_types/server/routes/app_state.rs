#[derive(Debug)]
pub struct AppState<'a> {
    pub postgres_pool: sqlx::PgPool,
    pub config: &'a crate::repositories_types::server::config::Config,
    pub project_git_info: &'a git_info::ProjectGitInfo<'a>,
}
impl crate::server::routes::git_info::GitInfoRouteParameters for AppState<'_> {}
impl crate::server::routes::not_found::NotFoundRouteParameters for AppState<'_> {}
impl postgresql_crud::CombinationOfAppStateLogicTraits for AppState<'_> {}
impl app_state::GetEnableApiGitCommitCheck for AppState<'_> {
    fn get_enable_api_git_commit_check(&self) -> &std::primitive::bool {
        self.config.get_enable_api_git_commit_check()
    }
}
impl app_state::GetSourcePlaceType for AppState<'_> {
    fn get_source_place_type(&self) -> &config_lib::types::SourcePlaceType {
        self.config.get_source_place_type()
    }
}
impl app_state::GetTimezone for AppState<'_> {
    fn get_timezone(&self) -> &chrono::FixedOffset {
        self.config.get_timezone()
    }
}
impl app_state::GetMaximumSizeOfHttpBodyInBytes for AppState<'_> {
    fn get_maximum_size_of_http_body_in_bytes(&self) -> &std::primitive::usize {
        self.config.get_maximum_size_of_http_body_in_bytes()
    }
}
impl app_state::GetPostgresPool for AppState<'_> {
    fn get_postgres_pool(&self) -> &sqlx::PgPool {
        &self.postgres_pool
    }
}
impl git_info::GetGitCommitLink for AppState<'_> {
    fn get_git_commit_link(&self) -> std::string::String {
        self.project_git_info.get_git_commit_link()
    }
}