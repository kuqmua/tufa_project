pub struct AppInfo<'a> {
    pub postgres_pool: sqlx::PgPool,
    pub config: &'a crate::repositories_types::server::config::config_struct::Config,
    pub project_git_info: &'a crate::common::git::project_git_info::ProjectGitInfo<'a>,
}

impl<'a> postgresql_crud::app_info_state::GetConfigGetPostgresPool
    for AppInfo<'a>
{
}

impl<'a> config_lib::config_fields::GetEnableApiGitCommitCheck for AppInfo<'a> {
    fn get_enable_api_git_commit_check(&self) -> &bool {
        self.config.get_enable_api_git_commit_check()
    }
}
impl<'a> config_lib::config_fields::GetSourcePlaceType for AppInfo<'a> {
    fn get_source_place_type(&self) -> &config_lib::source_place_type::SourcePlaceType {
        self.config.get_source_place_type()
    }
}
impl<'a> config_lib::config_fields::GetTimezone for AppInfo<'a> {
    fn get_timezone(&self) -> &chrono::FixedOffset {
        self.config.get_timezone()
    }
}

impl<'a> crate::server::routes::git_info::GitInfoRouteParameters for AppInfo<'a> {}
impl<'a> crate::server::routes::not_found::NotFoundRouteParameters for AppInfo<'a> {}

impl<'a> app_state::get_postgres_pool::GetPostgresPool for AppInfo<'a> {
    fn get_postgres_pool(&self) -> &sqlx::PgPool {
        &self.postgres_pool
    }
}

impl<'a> crate::common::git::get_git_commit_link::GetGitCommitLink for AppInfo<'a> {
    fn get_git_commit_link(&self) -> std::string::String {
        self.project_git_info.get_git_commit_link()
    }
}
