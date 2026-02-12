#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        use proc_macro2::TokenStream as Ts2;
        use quote::quote;
        macro_clippy_check_common::clippy_check(
            "gen_postgres_table_test_content",
            "../postgres_crud/postgres_table/",
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
postgres_crud = {path = "../../../postgres_crud", features = ["test-utils"]}

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
                enum ShouldAddGenPostgresTablePrimaryKey {
                    False,
                    True,
                }
                let gen_table_example_ts = |
                    should_add_gen_postgres_table_primary_key: ShouldAddGenPostgresTablePrimaryKey
                |{
                    let allow_clippy_arbitrary_source_item_ordering_ts = token_patterns::AllowClippyArbitrarySourceItemOrdering;
                    let maybe_gen_postgres_table_primary_key_ts = match should_add_gen_postgres_table_primary_key {
                        ShouldAddGenPostgresTablePrimaryKey::False => Ts2::new(),
                        ShouldAddGenPostgresTablePrimaryKey::True => quote!{#[gen_postgres_table_primary_key]},
                    };
                    quote!{
                        #allow_clippy_arbitrary_source_item_ordering_ts
                        #[derive(Debug, Clone, Copy)]
                        #[postgres_crud::gen_postgres_table_config{{
                            "create_many_content_write_into_gen_postgres_table_create_many": "False",
                            "create_one_content_write_into_gen_postgres_table_create_one": "False",
                            "read_many_content_write_into_gen_postgres_table_read_many": "False",
                            "read_one_content_write_into_gen_postgres_table_read_one": "False",
                            "update_many_content_write_into_gen_postgres_table_update_many": "False",
                            "update_one_content_write_into_gen_postgres_table_update_one": "False",
                            "delete_many_content_write_into_gen_postgres_table_delete_many": "False",
                            "delete_one_content_write_into_gen_postgres_table_delete_one": "False",
                            "tests_content_write_into_gen_postgres_table_tests": "False",
                            "common_content_write_into_gen_postgres_table_common": "False",
                            "whole_content_write_into_gen_postgres_table": "False"
                        }}]
                        #[postgres_crud::create_many_additional_error_variants{enum CreateManyAdditionalErrorVariants{}}]
                        #[postgres_crud::create_one_additional_error_variants{enum CreateOneAdditionalErrorVariants{}}]
                        #[postgres_crud::read_many_additional_error_variants{enum ReadManyAdditionalErrorVariants{}}]
                        #[postgres_crud::read_one_additional_error_variants{enum ReadOneAdditionalErrorVariants{}}]
                        #[postgres_crud::update_many_additional_error_variants{enum UpdateManyAdditionalErrorVariants{}}]
                        #[postgres_crud::update_one_additional_error_variants{enum UpdateOneAdditionalErrorVariants{}}]
                        #[postgres_crud::delete_many_additional_error_variants{enum DeleteManyAdditionalErrorVariants{}}]
                        #[postgres_crud::delete_one_additional_error_variants{enum DeleteOneAdditionalErrorVariants{}}]
                        #[postgres_crud::common_additional_error_variants{
                            enum CommonAdditionalErrorVariants {
                                CheckCommit {
                                    #[eo_error_occurence]
                                    check_commit: postgres_crud::check_commit::ErrorNamed,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            }
                        }]
                        #[postgres_crud::create_many_additional_logic{}]
                        #[postgres_crud::create_one_additional_logic{}]
                        #[postgres_crud::read_many_additional_logic{}]
                        #[postgres_crud::read_one_additional_logic{}]
                        #[postgres_crud::update_many_additional_logic{}]
                        #[postgres_crud::update_one_additional_logic{}]
                        #[postgres_crud::delete_many_additional_logic{}]
                        #[postgres_crud::delete_one_additional_logic{}]
                        #[postgres_crud::common_additional_logic{}]
                        pub struct TableExample {
                            #maybe_gen_postgres_table_primary_key_ts
                            pub primary_key_column:
                                postgres_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgres,
                            pub column_0: postgres_crud::StdPrimitiveI16AsNotNullInt2,
                            pub column_1: postgres_crud::OptionStdPrimitiveI16AsNullableInt2,
                            pub column_2: postgres_crud::VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2,
                        }
                    }
                };
                let ts =
                    gen_postgres_table_source::gen_postgres_table(
                        gen_table_example_ts(
                            ShouldAddGenPostgresTablePrimaryKey::True
                        )
                    );
                let table_struct_ts = gen_table_example_ts(
                    ShouldAddGenPostgresTablePrimaryKey::False
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
