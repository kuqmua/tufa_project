#[cfg(test)]
mod tests {
    use macro_clippy_check_common::clippy_check;
    #[test]
    fn clippy() {
        use proc_macro2::TokenStream as Ts2;
        use quote::quote;
        use token_patterns::AllowClippyArbitrarySourceItemOrdering;
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
error_occurence_lib = {path = "../../../error_occurence_lib"}
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
                    let allow_clippy_arbitrary_source_item_ordering_ts = AllowClippyArbitrarySourceItemOrdering;
                    let maybe_gen_pg_table_primary_key_ts = match should_add_gen_pg_table_primary_key {
                        ShouldAddGenPgTablePrimaryKey::False => Ts2::new(),
                        ShouldAddGenPgTablePrimaryKey::True => quote!{#[gen_pg_table_primary_key]},
                    };
                    quote!{
                        #allow_clippy_arbitrary_source_item_ordering_ts
                        #[derive(Debug, Clone, Copy)]
                        #[pg_crud::gen_pg_table_config{{
                            "create_many_content_write_into_gen_pg_table_create_many": "False",
                            "create_one_content_write_into_gen_pg_table_create_one": "False",
                            "read_many_content_write_into_gen_pg_table_read_many": "False",
                            "read_one_content_write_into_gen_pg_table_read_one": "False",
                            "update_many_content_write_into_gen_pg_table_update_many": "False",
                            "update_one_content_write_into_gen_pg_table_update_one": "False",
                            "delete_many_content_write_into_gen_pg_table_delete_many": "False",
                            "delete_one_content_write_into_gen_pg_table_delete_one": "False",
                            "tests_content_write_into_gen_pg_table_tests": "False",
                            "common_content_write_into_gen_pg_table_common": "False",
                            "whole_content_write_into_gen_pg_table": "False"
                        }}]
                        #[pg_crud::create_many_additional_error_variants{enum CreateManyAdditionalErrorVariants{}}]
                        #[pg_crud::create_one_additional_error_variants{enum CreateOneAdditionalErrorVariants{}}]
                        #[pg_crud::read_many_additional_error_variants{enum ReadManyAdditionalErrorVariants{}}]
                        #[pg_crud::read_one_additional_error_variants{enum ReadOneAdditionalErrorVariants{}}]
                        #[pg_crud::update_many_additional_error_variants{enum UpdateManyAdditionalErrorVariants{}}]
                        #[pg_crud::update_one_additional_error_variants{enum UpdateOneAdditionalErrorVariants{}}]
                        #[pg_crud::delete_many_additional_error_variants{enum DeleteManyAdditionalErrorVariants{}}]
                        #[pg_crud::delete_one_additional_error_variants{enum DeleteOneAdditionalErrorVariants{}}]
                        #[pg_crud::common_additional_error_variants{
                            enum CommonAdditionalErrorVariants {
                                CheckCommit {
                                    #[eo_error_occurence]
                                    check_commit: pg_crud::check_commit::CommitError,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
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
                                pg_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPg,
                            pub column_0: pg_crud::I16AsNotNullInt2,
                            pub column_1: pg_crud::OptionI16AsNullableInt2,
                            pub column_2: pg_crud::VecOfI16AsNotNullArrayOfNotNullInt2,
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
