pub struct AppState<'a> {
    pub postgres_pool: sqlx::PgPool,
    pub config: &'a crate::repositories_types::server::config::config_struct::Config,
    pub project_git_info: &'a git_info::ProjectGitInfo<'a>,
}

impl<'a> postgresql_crud::app_state::CombinationOfTraitsForPostgresqlCrudLogic for AppState<'a> {}
impl<'a> crate::server::routes::git_info::GitInfoRouteParameters for AppState<'a> {}
impl<'a> crate::server::routes::not_found::NotFoundRouteParameters for AppState<'a> {}

impl<'a> config_lib::config_fields::GetEnableApiGitCommitCheck for AppState<'a> {
    fn get_enable_api_git_commit_check(&self) -> &bool {
        self.config.get_enable_api_git_commit_check()
    }
}
impl<'a> config_lib::config_fields::GetSourcePlaceType for AppState<'a> {
    fn get_source_place_type(&self) -> &config_lib::source_place_type::SourcePlaceType {
        self.config.get_source_place_type()
    }
}
impl<'a> config_lib::config_fields::GetTimezone for AppState<'a> {
    fn get_timezone(&self) -> &chrono::FixedOffset {
        self.config.get_timezone()
    }
}
impl<'a> config_lib::config_fields::GetMaximumSizeOfHttpBodyInBytes for AppState<'a> {
    fn get_maximum_size_of_http_body_in_bytes(&self) -> &std::primitive::usize {
        self.config.get_maximum_size_of_http_body_in_bytes()
    }
}

impl<'a> app_state::get_postgres_pool::GetPostgresPool for AppState<'a> {
    fn get_postgres_pool(&self) -> &sqlx::PgPool {
        &self.postgres_pool
    }
}

impl<'a> crate::common::git::get_git_commit_link::GetGitCommitLink for AppState<'a> {
    fn get_git_commit_link(&self) -> std::string::String {
        self.project_git_info.get_git_commit_link()
    }
}

