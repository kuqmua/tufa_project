#[cfg(test)]
mod tests {
    use macro_clippy_check_common::clippy_check;
    use optml::Optml;
    use proc_macro2::TokenStream as Ts2;
    use quote::quote;
    use token_patterns::AllowClippyArbitrarySourceItemOrdering;
    #[test]
    fn clippy() {
        clippy_check(
            "gen_pg_table_test_content",
            "../pg_crud/pg_table/",
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
location_lib = {path = "../../../location_lib"}
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
                enum AddGenPgTablePk {
                    False,
                    True,
                }
                let gen_table_example_ts = |add_gen_pg_table_pk: AddGenPgTablePk| {
                    let mb_gen_pg_table_pk_ts = match add_gen_pg_table_pk {
                        AddGenPgTablePk::False => Ts2::new(),
                        AddGenPgTablePk::True => {
                            quote! {#[gen_pg_table_pk]}
                        }
                    };
                    quote! {
                        #AllowClippyArbitrarySourceItemOrdering
                        #[derive(Debug, Clone, Copy, optml::Optml)]
                        #[pg_crud::gen_pg_table_config{{
                            "cm_write_into_file": "False",
                            "co_write_into_file": "False",
                            "rm_write_into_file": "False",
                            "ro_write_into_file": "False",
                            "um_write_into_file": "False",
                            "uo_write_into_file": "False",
                            "dm_write_into_file": "False",
                            "dlo_write_into_file": "False",
                            "tests_write_into_file": "False",
                            "common_write_into_file": "False",
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
                        #[pg_crud::common_er_vrts{
                            enum CommonErVrts {
                                CheckCommit {
                                    #[eo_location]
                                    check_commit: pg_crud::check_commit::CommitEr,
                                    loc: location_lib::loc::Loc,
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
                        #[pg_crud::common_logic{}]
                        pub struct TableExample {
                            #mb_gen_pg_table_pk_ts
                            pub pk_column:
                                pg_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitByPg,
                            pub column_0: pg_crud::I16AsNotNullInt2,
                            pub column_1: pg_crud::OptI16AsNullableInt2,
                            pub column_2: pg_crud::VecOfI16AsNotNullArrOfNotNullInt2,
                        }
                    }
                };
                let ts =
                    gen_pg_table_source::gen_pg_table(gen_table_example_ts(AddGenPgTablePk::True));
                let table_struct_ts = gen_table_example_ts(AddGenPgTablePk::False);
                quote! {
                    #ts
                    #table_struct_ts
                }
            }
            .to_string(),
        );
    }
}
