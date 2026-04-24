use app_state::{
    GetEnableApiGitCommitCheck, GetMaximumSizeOfHttpBodyInBytes, GetPgPool, GetSrcPlaceType,
    GetTimezone,
};
use cmn_routes::CmnRoutesPrms;
use config_lib::types::SrcPlaceType;
use git_info::ProjectGitInfo;
use optml::Optml;
use pg_crud::CombinationOfAppStateLogicTraits;
use server_config::Config;
use sqlx::PgPool;
macro_rules! impl_cfg_getter {
    ($trait_name:ident, $fn_name:ident, $ret_ty:ty) => {
        impl $trait_name for ServerAppState<'_> {
            fn $fn_name(&self) -> &$ret_ty {
                self.cfg_ref().$fn_name()
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
impl ServerAppState<'_> {
    #[allow(clippy::single_call_fn)] // keeps config forwarding in one place for all generated trait impls
    const fn cfg_ref(&self) -> &Config {
        &self.config
    }
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
impl AsRef<str> for ServerAppState<'_> {
    fn as_ref(&self) -> &str {
        self.project_git_info.commit
    }
}
#[cfg(test)]
mod tests {
    use super::ServerAppState;
    use app_state::{
        GetEnableApiGitCommitCheck as _, GetMaximumSizeOfHttpBodyInBytes as _, GetPgPool as _,
        GetSrcPlaceType as _, GetTimezone as _,
    };
    use config_lib::types::{SrcPlaceType, TracingLevel};
    use git_info::{GetGitCommitLink as _, ProjectGitInfo};
    use secrecy::SecretBox;
    use server_config::Config;
    use std::ptr;
    const TEST_COMMIT: &str = "abc123";
    #[allow(clippy::single_call_fn)] // shared fixture keeps commit test input consistent across ServerAppState tests
    fn mk_git_info() -> ProjectGitInfo<'static> {
        ProjectGitInfo {
            commit: TEST_COMMIT,
        }
    }
    fn mk_st<'state_lt>(
        project_git_info: &'state_lt ProjectGitInfo<'state_lt>,
    ) -> ServerAppState<'state_lt> {
        ServerAppState {
            config: Config {
                cors_allow_origin: "*".to_owned(),
                database_url: SecretBox::new(Box::new("postgres://db".to_owned())),
                maximum_size_of_http_body_in_bytes: 16_384,
                service_socket_address: "127.0.0.1:3000".parse().expect("73f8bc91"),
                pg_pool_max_connections: 7,
                timezone: chrono::FixedOffset::east_opt(3i32 * 3_600i32).expect("a95d3c17"),
                src_place_type: SrcPlaceType::Github,
                tracing_level: TracingLevel::Info,
                enable_api_git_commit_check: true,
            },
            pg_pool: sqlx::PgPool::connect_lazy("postgres://usr:pwd@localhost:5432/db")
                .expect("4bd3f0a1"),
            project_git_info,
        }
    }
    #[tokio::test]
    async fn cfg_getters_forward_to_inner_config() {
        let git_info = mk_git_info();
        let st = mk_st(&git_info);
        assert_eq!(st.get_src_place_type(), &SrcPlaceType::Github);
        assert_eq!(st.get_timezone().local_minus_utc(), 3i32 * 3_600i32);
        assert_eq!(st.get_maximum_size_of_http_body_in_bytes(), &16_384);
        assert!(st.get_enable_api_git_commit_check());
    }
    #[tokio::test]
    async fn get_pg_pool_returns_same_pool_ref() {
        let git_info = mk_git_info();
        let st = mk_st(&git_info);
        let lhs = ptr::from_ref(st.get_pg_pool());
        let rhs = ptr::from_ref(&st.pg_pool);
        assert_eq!(lhs, rhs);
    }
    #[tokio::test]
    async fn as_ref_and_git_commit_link_are_consistent() {
        let git_info = mk_git_info();
        let st = mk_st(&git_info);
        assert_eq!(st.as_ref(), TEST_COMMIT);
        assert_eq!(
            st.get_git_commit_link(),
            git_info::git_commit_link(TEST_COMMIT)
        );
    }
}
