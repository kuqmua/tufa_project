#[cfg(test)]
mod tests {
    use macro_clippy_check_cmn::clippy_check;
    use optml::Optml;
    use proc_macro2::TokenStream as Ts2;
    use quote::quote;
    use token_patterns::AllowClippyArbitrarySrcItemOrdering;
    #[test]
    fn clippy() {
        clippy_check(
            "gen_pg_tbl_test_cnt",
            "../pg_crud/pg_tbl/",
            r#"[dependencies]
axum.workspace = true
http.workspace = true
sqlx.workspace = true
reqwest.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
utoipa.workspace = true
git_info = {path = "../../../git_info"}
loc_lib = {path = "../../../loc_lib"}
pg_crud = {path = "../../../pg_crud", features = ["test-utils"]}
optml = {path = "../../../optml"}
[dev-dependencies]
num_cpus.workspace = true
futures.workspace = true
secrecy.workspace = true
tokio.workspace = true
tracing-subscriber.workspace = true
uuid.workspace = true
itertools.workspace = true
app_state = {path = "../../../app_state"}
config_lib = {path = "../../../config_lib"}
server_app_state = {path = "../../../server_app_state"}
server_config = {path = "../../../server_config"}"#,
            &{
                #[derive(Optml)]
                enum AddGenPgTblPk {
                    False,
                    True,
                }
                let gen_tbl_example_ts = |add_gen_pg_tbl_pk: AddGenPgTblPk| {
                    let mb_gen_pg_tbl_pk_ts = match add_gen_pg_tbl_pk {
                        AddGenPgTblPk::False => Ts2::new(),
                        AddGenPgTblPk::True => {
                            quote! {#[gen_pg_tbl_pk]}
                        }
                    };
                    quote! {
                        #AllowClippyArbitrarySrcItemOrdering
                        #[derive(Debug, Clone, Copy, optml::Optml)]
                        #[pg_crud::gen_pg_tbl_config{{
                            "cm_write_into_file": "False",
                            "co_write_into_file": "False",
                            "rm_write_into_file": "False",
                            "ro_write_into_file": "False",
                            "um_write_into_file": "False",
                            "uo_write_into_file": "False",
                            "dm_write_into_file": "False",
                            "dlo_write_into_file": "False",
                            "tests_write_into_file": "False",
                            "cmn_write_into_file": "False",
                            "whole_write_into_file": "False"
                        }}]
                        #[pg_crud::cm_er_vrts{enum CmErVrts{}}]
                        #[pg_crud::co_er_vrts{enum CoErVrts{}}]
                        #[pg_crud::rm_er_vrts{enum RmErVrts{}}]
                        #[pg_crud::ro_er_vrts{enum RoErVrts{}}]
                        #[pg_crud::um_er_vrts{enum UmErVrts{}}]
                        #[pg_crud::uo_er_vrts{enum UoErVrts{}}]
                        #[pg_crud::dm_er_vrts{enum DmErVrts{}}]
                        #[pg_crud::dlo_er_vrts{enum DloErVrts{}}]
                        #[pg_crud::cmn_er_vrts{
                            enum CmnErVrts {
                                CheckCommit {
                                    #[eo_loc]
                                    check_commit: pg_crud::check_commit::CommitEr,
                                    loc: loc_lib::loc::Loc,
                                },
                            }
                        }]
                        #[pg_crud::cm_logic{}]
                        #[pg_crud::co_logic{}]
                        #[pg_crud::rm_logic{}]
                        #[pg_crud::ro_logic{}]
                        #[pg_crud::um_logic{}]
                        #[pg_crud::uo_logic{}]
                        #[pg_crud::dm_logic{}]
                        #[pg_crud::dlo_logic{}]
                        #[pg_crud::cmn_logic{}]
                        pub struct TblExample {
                            #mb_gen_pg_tbl_pk_ts
                            pub pk_col:
                                pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg,
                            pub col_0: pg_crud::I16AsNnInt2,
                            pub col_1: pg_crud::OptI16AsNlInt2,
                            pub col_2: pg_crud::VecOfI16AsNnArrOfNnInt2,
                        }
                    }
                };
                let ts = gen_pg_tbl_src::gen_pg_tbl(gen_tbl_example_ts(AddGenPgTblPk::True));
                let tbl_struct_ts = gen_tbl_example_ts(AddGenPgTblPk::False);
                quote! {
                    #ts
                    #tbl_struct_ts
                }
            }
            .to_string(),
        );
    }
}
