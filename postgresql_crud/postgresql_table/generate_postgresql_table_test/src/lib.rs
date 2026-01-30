#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        let crate_name = "generate_postgresql_table_test_content";
        let path = format!("../{crate_name}/");
        let cargo_toml_content = format!(
            r#"[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2024"
description = "description"
repository = "repository"
readme = "readme"
license = "license"
keywords = ["keyword"]
categories = ["category"]"#
        );
        let lib_rs_content = "\n";
        let path_lib_rs = format!("{path}src/lib.rs");
        let path_cargo_toml = format!("{path}Cargo.toml");
        std::fs::write(&path_lib_rs, lib_rs_content).expect("404ab180-10f0-4b82-95ef-5635488fe436");
        std::fs::write(&path_cargo_toml, {
            let additional_content = r#"[dependencies]
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
quote.workspace = true
proc-macro2.workspace = true
num_cpus.workspace = true
futures.workspace = true
secrecy.workspace = true
tokio.workspace = true
tracing-subscriber.workspace = true
uuid.workspace = true
itertools.workspace = true
server_types = {path = "../../../server_types"}
app_state = {path = "../../../app_state"}
config_lib = {path = "../../../config_lib"}
server_app_state = {path = "../../../server_app_state"}
server_config = {path = "../../../server_config"}"#;
            format!("{cargo_toml_content}\n{additional_content}")
        })
        .expect("3757da9b-0457-4301-9e68-efb60737dc71");
        std::fs::write(
            &path_lib_rs,
            {
                enum ShouldAddGeneratePostgresqlTablePrimaryKey {
                    True,
                    False
                }
                let generate_table_example_token_stream = |
                    should_add_generate_postgresql_table_primary_key: ShouldAddGeneratePostgresqlTablePrimaryKey
                |{
                    let maybe_generate_postgresql_table_primary_key_token_stream = match should_add_generate_postgresql_table_primary_key {
                        ShouldAddGeneratePostgresqlTablePrimaryKey::True => quote::quote!{#[generate_postgresql_table_primary_key]},
                        ShouldAddGeneratePostgresqlTablePrimaryKey::False => proc_macro2::TokenStream::new()
                    };
                    quote::quote!{
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
                                    check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
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
        ).expect("55124f90-c7c2-40b5-8b66-695635ea6afd");
        let return_to_previous = || {
            std::fs::write(&path_lib_rs, lib_rs_content)
                .expect("79231418-b44a-4dac-8a88-3d8403024827");
            std::fs::write(&path_cargo_toml, cargo_toml_content)
                .expect("ec801a87-2c48-4c64-9c6a-7e686db91094");
        };
        if let Ok(value_90318089) = std::process::Command::new("cargo")
            .args(["fmt", "--", &path_lib_rs])
            .status()
        {
            assert!(
                value_90318089.success(),
                "2a1deb01-ec64-4d13-94d5-47b647b2950d"
            );
        } else {
            return_to_previous();
            panic!("8dc4f045-93a0-46a3-a6f6-0eab002dbb0c");
        }
        if let Ok(value_f263835c) = std::process::Command::new("cargo")
            .args(["clippy", "--all-targets", "--all-features"])
            .status()
        {
            assert!(
                value_f263835c.success(),
                "2c037283-420c-4076-8042-1eac09ba1a23"
            );
            return_to_previous();
        } else {
            return_to_previous();
            panic!("cd48b869-7726-412d-b1b6-b89deca000c3");
        }
    }
}
