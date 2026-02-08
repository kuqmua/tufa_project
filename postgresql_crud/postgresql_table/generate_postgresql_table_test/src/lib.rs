#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        macro_clippy_check_common::clippy_check(
            "generate_postgresql_table_test_content",
            "../postgresql_crud/postgresql_table/",
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
postgresql_crud = {path = "../../../postgresql_crud", features = ["test-utils"]}

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
                #[allow(clippy::arbitrary_source_item_ordering)]
                enum ShouldAddGeneratePostgresqlTablePrimaryKey {
                    True,
                    False
                }
                let generate_table_example_token_stream = |
                    should_add_generate_postgresql_table_primary_key: ShouldAddGeneratePostgresqlTablePrimaryKey
                |{
                    let allow_clippy_arbitrary_source_item_ordering_token_stream = token_patterns::AllowClippyArbitrarySourceItemOrdering;
                    let maybe_generate_postgresql_table_primary_key_token_stream = match should_add_generate_postgresql_table_primary_key {
                        ShouldAddGeneratePostgresqlTablePrimaryKey::True => quote::quote!{#[generate_postgresql_table_primary_key]},
                        ShouldAddGeneratePostgresqlTablePrimaryKey::False => proc_macro2::TokenStream::new()
                    };
                    quote::quote!{
                        #allow_clippy_arbitrary_source_item_ordering_token_stream
                        #[derive(Debug, Clone, Copy)]
                        #[postgresql_crud::generate_postgresql_table_config{{
                            "create_many_content_write_into_generate_postgresql_table_create_many": "False",
                            "create_one_content_write_into_generate_postgresql_table_create_one": "False",
                            "read_many_content_write_into_generate_postgresql_table_read_many": "False",
                            "read_one_content_write_into_generate_postgresql_table_read_one": "False",
                            "update_many_content_write_into_generate_postgresql_table_update_many": "False",
                            "update_one_content_write_into_generate_postgresql_table_update_one": "False",
                            "delete_many_content_write_into_generate_postgresql_table_delete_many": "False",
                            "delete_one_content_write_into_generate_postgresql_table_delete_one": "False",
                            "tests_content_write_into_generate_postgresql_table_tests": "False",
                            "common_content_write_into_generate_postgresql_table_common": "False",
                            "whole_content_write_into_generate_postgresql_table": "False"
                        }}]
                        #[postgresql_crud::create_many_additional_error_variants{enum CreateManyAdditionalErrorVariants{}}]
                        #[postgresql_crud::create_one_additional_error_variants{enum CreateOneAdditionalErrorVariants{}}]
                        #[postgresql_crud::read_many_additional_error_variants{enum ReadManyAdditionalErrorVariants{}}]
                        #[postgresql_crud::read_one_additional_error_variants{enum ReadOneAdditionalErrorVariants{}}]
                        #[postgresql_crud::update_many_additional_error_variants{enum UpdateManyAdditionalErrorVariants{}}]
                        #[postgresql_crud::update_one_additional_error_variants{enum UpdateOneAdditionalErrorVariants{}}]
                        #[postgresql_crud::delete_many_additional_error_variants{enum DeleteManyAdditionalErrorVariants{}}]
                        #[postgresql_crud::delete_one_additional_error_variants{enum DeleteOneAdditionalErrorVariants{}}]
                        #[postgresql_crud::common_additional_error_variants{
                            enum CommonAdditionalErrorVariants {
                                CheckCommit {
                                    #[eo_error_occurence]
                                    check_commit: postgresql_crud::check_commit::ErrorNamed,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                                },
                            }
                        }]
                        #[postgresql_crud::create_many_additional_logic{}]
                        #[postgresql_crud::create_one_additional_logic{}]
                        #[postgresql_crud::read_many_additional_logic{}]
                        #[postgresql_crud::read_one_additional_logic{}]
                        #[postgresql_crud::update_many_additional_logic{}]
                        #[postgresql_crud::update_one_additional_logic{}]
                        #[postgresql_crud::delete_many_additional_logic{}]
                        #[postgresql_crud::delete_one_additional_logic{}]
                        #[postgresql_crud::common_additional_logic{}]
                        pub struct TableExample {
                            #maybe_generate_postgresql_table_primary_key_token_stream
                            pub primary_key_column:
                                postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql,
                            pub column_0: postgresql_crud::StdPrimitiveI16AsNotNullInt2,
                            pub column_1: postgresql_crud::OptionStdPrimitiveI16AsNullableInt2,
                            pub column_2: postgresql_crud::VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2,
                        }
                    }
                };
                let token_stream =
                    generate_postgresql_table_source::generate_postgresql_table(
                        generate_table_example_token_stream(
                            ShouldAddGeneratePostgresqlTablePrimaryKey::True
                        )
                    );
                let table_struct_token_stream = generate_table_example_token_stream(
                    ShouldAddGeneratePostgresqlTablePrimaryKey::False
                );
                quote::quote! {
                    #table_struct_token_stream
                    #token_stream
                }
            }
            .to_string()
        );
    }
}
