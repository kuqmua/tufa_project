#[cfg(test)]
mod tests {
    use macro_clippy_check_common::clippy_check;
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
                enum ShouldAddGenPgTablePrimaryKey {
                    False,
                    True,
                }
                let gen_table_example_ts = |
                    should_add_gen_pg_table_primary_key: ShouldAddGenPgTablePrimaryKey
                |{
                    let maybe_gen_pg_table_primary_key_ts = match should_add_gen_pg_table_primary_key {
                        ShouldAddGenPgTablePrimaryKey::False => Ts2::new(),
                        ShouldAddGenPgTablePrimaryKey::True => quote!{#[gen_pg_table_primary_key]},
                    };
                    quote!{
                        #AllowClippyArbitrarySourceItemOrdering
                        #[derive(Debug, Clone, Copy)]
                        #[pg_crud::gen_pg_table_config{{
                            "create_many_write_into_file": "False",
                            "create_one_write_into_file": "False",
                            "read_many_write_into_file": "False",
                            "read_one_write_into_file": "False",
                            "update_many_write_into_file": "False",
                            "update_one_write_into_file": "False",
                            "delete_many_write_into_file": "False",
                            "delete_one_write_into_file": "False",
                            "tests_write_into_file": "False",
                            "common_write_into_file": "False",
                            "whole_write_into_file": "False"
                        }}]
                        #[pg_crud::create_many_additional_er_vrts{enum CreateManyAdditionalErVrts{}}]
                        #[pg_crud::create_one_additional_er_vrts{enum CreateOneAdditionalErVrts{}}]
                        #[pg_crud::read_many_additional_er_vrts{enum ReadManyAdditionalErVrts{}}]
                        #[pg_crud::read_one_additional_er_vrts{enum ReadOneAdditionalErVrts{}}]
                        #[pg_crud::update_many_additional_er_vrts{enum UpdateManyAdditionalErVrts{}}]
                        #[pg_crud::update_one_additional_er_vrts{enum UpdateOneAdditionalErVrts{}}]
                        #[pg_crud::delete_many_additional_er_vrts{enum DeleteManyAdditionalErVrts{}}]
                        #[pg_crud::delete_one_additional_er_vrts{enum DeleteOneAdditionalErVrts{}}]
                        #[pg_crud::common_additional_er_vrts{
                            enum CommonAdditionalErVrts {
                                CheckCommit {
                                    #[eo_location]
                                    check_commit: pg_crud::check_commit::CommitEr,
                                    loc: location_lib::loc::Loc,
                                },
                            }
                        }]
                        #[pg_crud::create_many_additional_logic{}]
                        #[pg_crud::create_one_additional_logic{}]
                        #[pg_crud::read_many_additional_logic{}]
                        #[pg_crud::read_one_additional_logic{}]
                        #[pg_crud::update_many_additional_logic{}]
                        #[pg_crud::update_one_additional_logic{}]
                        #[pg_crud::delete_many_additional_logic{}]
                        #[pg_crud::delete_one_additional_logic{}]
                        #[pg_crud::common_additional_logic{}]
                        pub struct TableExample {
                            #maybe_gen_pg_table_primary_key_ts
                            pub primary_key_column:
                                pg_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitByPg,
                            pub column_0: pg_crud::I16AsNotNullInt2,
                            pub column_1: pg_crud::OptI16AsNullableInt2,
                            pub column_2: pg_crud::VecOfI16AsNotNullArrOfNotNullInt2,
                        }
                    }
                };
                let ts =
                    gen_pg_table_source::gen_pg_table(
                        gen_table_example_ts(
                            ShouldAddGenPgTablePrimaryKey::True
                        )
                    );
                let table_struct_ts = gen_table_example_ts(
                    ShouldAddGenPgTablePrimaryKey::False
                );
                quote! {
                    #ts
                    #table_struct_ts
                }
            }
            .to_string()
        );
    }
}
