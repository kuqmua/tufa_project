use app_state::{
    GetEnableApiGitCommitCheck, GetMaximumSizeOfHttpBodyInBytes, GetPgPool, GetSourcePlaceType,
    GetTimezone,
};
use chrono::FixedOffset;
use common_routes::CommonRoutesParameters;
use config_lib::types::SourcePlaceType;
use git_info::{GetGitCommitLink, ProjectGitInfo};
use pg_crud::CombinationOfAppStateLogicTraits;
use server_config::Config;
use sqlx::PgPool;

#[derive(Debug)]
pub struct ServerAppState<'lifetime> {
    pub config: Config,
    pub pg_pool: PgPool,
    pub project_git_info: &'lifetime ProjectGitInfo<'lifetime>,
}
impl CommonRoutesParameters for ServerAppState<'_> {}
impl CombinationOfAppStateLogicTraits for ServerAppState<'_> {}
impl GetEnableApiGitCommitCheck for ServerAppState<'_> {
    fn get_enable_api_git_commit_check(&self) -> &bool {
        self.config.get_enable_api_git_commit_check()
    }
}
impl GetSourcePlaceType for ServerAppState<'_> {
    fn get_source_place_type(&self) -> &SourcePlaceType {
        self.config.get_source_place_type()
    }
}
impl GetTimezone for ServerAppState<'_> {
    fn get_timezone(&self) -> &FixedOffset {
        self.config.get_timezone()
    }
}
impl GetMaximumSizeOfHttpBodyInBytes for ServerAppState<'_> {
    fn get_maximum_size_of_http_body_in_bytes(&self) -> &usize {
        self.config.get_maximum_size_of_http_body_in_bytes()
    }
}
impl GetPgPool for ServerAppState<'_> {
    fn get_pg_pool(&self) -> &PgPool {
        &self.pg_pool
    }
}
impl GetGitCommitLink for ServerAppState<'_> {
    fn get_git_commit_link(&self) -> String {
        self.project_git_info.get_git_commit_link()
    }
}
