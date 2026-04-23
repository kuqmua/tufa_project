use app_state::{
    GetEnableApiGitCommitCheck, GetMaximumSizeOfHttpBodyInBytes, GetPgPool, GetSrcPlaceType,
    GetTimezone,
};
use cmn_routes::CmnRoutesPrms;
use config_lib::types::SrcPlaceType;
use git_info::{GetGitCommitId, ProjectGitInfo};
use optml::Optml;
use pg_crud::CombinationOfAppStateLogicTraits;
use server_config::Config;
use sqlx::PgPool;
macro_rules! impl_cfg_getter {
    ($trait_name:ident, $fn_name:ident, $ret_ty:ty) => {
        impl $trait_name for ServerAppState<'_> {
            fn $fn_name(&self) -> &$ret_ty {
                self.config.$fn_name()
            }
        }
    };
}
#[derive(Debug, Optml)]
pub struct ServerAppState<'lt> {
    pub config: Config,
    pub pg_pool: PgPool,
    pub project_git_info: &'lt ProjectGitInfo<'lt>,
}
impl CmnRoutesPrms for ServerAppState<'_> {}
impl CombinationOfAppStateLogicTraits for ServerAppState<'_> {}
impl_cfg_getter!(
    GetEnableApiGitCommitCheck,
    get_enable_api_git_commit_check,
    bool
);
impl_cfg_getter!(GetSrcPlaceType, get_src_place_type, SrcPlaceType);
impl_cfg_getter!(GetTimezone, get_timezone, chrono::FixedOffset);
impl_cfg_getter!(
    GetMaximumSizeOfHttpBodyInBytes,
    get_maximum_size_of_http_body_in_bytes,
    usize
);
impl GetPgPool for ServerAppState<'_> {
    fn get_pg_pool(&self) -> &PgPool {
        &self.pg_pool
    }
}
impl GetGitCommitId for ServerAppState<'_> {
    fn get_git_commit_id(&self) -> String {
        self.project_git_info.get_git_commit_id()
    }
    fn get_git_commit_id_ref(&self) -> Option<&str> {
        self.project_git_info.get_git_commit_id_ref()
    }
}
