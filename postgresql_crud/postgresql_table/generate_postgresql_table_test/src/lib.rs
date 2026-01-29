mod clippy;

#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        // let path = "src/clippy.rs";
        // std::fs::write(path, "").expect("404ab180-10f0-4b82-95ef-5635488fe436");
        // let mut file = std::fs::File::create("src/clippy.rs").expect("failed to create file");
        // std::io::Write::write_all(
        //     &mut file,
        //     {
        //         let input_token_stream = quote::quote! {
        //             #[derive(Debug, Clone, Copy, postgresql_crud::GeneratePostgresqlTable)]
        //             #[postgresql_crud::generate_postgresql_table_config{{
        //                 "create_many_content_write_into_generate_postgresql_table_create_many": "False",
        //                 "create_one_content_write_into_generate_postgresql_table_create_one": "False",
        //                 "read_many_content_write_into_generate_postgresql_table_read_many": "False",
        //                 "read_one_content_write_into_generate_postgresql_table_read_one": "False",
        //                 "update_many_content_write_into_generate_postgresql_table_update_many": "False",
        //                 "update_one_content_write_into_generate_postgresql_table_update_one": "False",
        //                 "delete_many_content_write_into_generate_postgresql_table_delete_many": "False",
        //                 "delete_one_content_write_into_generate_postgresql_table_delete_one": "False",
        //                 "tests_content_write_into_generate_postgresql_table_tests": "False",
        //                 "common_content_write_into_generate_postgresql_table_common": "False",
        //                 "whole_content_write_into_generate_postgresql_table": "False"
        //             }}]
        //             #[postgresql_crud::create_many_additional_error_variants{enum CreateManyAdditionalErrorVariants{}}]
        //             #[postgresql_crud::create_one_additional_error_variants{enum CreateOneAdditionalErrorVariants{}}]
        //             #[postgresql_crud::read_many_additional_error_variants{enum ReadManyAdditionalErrorVariants{}}]
        //             #[postgresql_crud::read_one_additional_error_variants{enum ReadOneAdditionalErrorVariants{}}]
        //             #[postgresql_crud::update_many_additional_error_variants{enum UpdateManyAdditionalErrorVariants{}}]
        //             #[postgresql_crud::update_one_additional_error_variants{enum UpdateOneAdditionalErrorVariants{}}]
        //             #[postgresql_crud::delete_many_additional_error_variants{enum DeleteManyAdditionalErrorVariants{}}]
        //             #[postgresql_crud::delete_one_additional_error_variants{enum DeleteOneAdditionalErrorVariants{}}]
        //             #[postgresql_crud::common_additional_error_variants{
        //                 enum CommonAdditionalErrorVariants {
        //                     CheckCommit {
        //                         #[eo_error_occurence]
        //                         check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        //                         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        //                     },
        //                 }
        //             }]
        //             #[postgresql_crud::create_many_additional_logic{}]
        //             #[postgresql_crud::create_one_additional_logic{}]
        //             #[postgresql_crud::read_many_additional_logic{}]
        //             #[postgresql_crud::read_one_additional_logic{}]
        //             #[postgresql_crud::update_many_additional_logic{}]
        //             #[postgresql_crud::update_one_additional_logic{}]
        //             #[postgresql_crud::delete_many_additional_logic{}]
        //             #[postgresql_crud::delete_one_additional_logic{}]
        //             #[postgresql_crud::common_additional_logic{}]
        //             pub struct TableExample {
        //                 #[generate_postgresql_table_primary_key]
        //                 pub primary_key_column:
        //                     postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql,
        //                 pub column_0: postgresql_crud::StdPrimitiveI16AsNotNullInt2,
        //                 pub column_1: postgresql_crud::OptionStdPrimitiveI16AsNullableInt2,
        //                 pub column_2: postgresql_crud::VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2,
        //             }
        //         };
        //         let token_stream =
        //             generate_postgresql_table_source::generate_postgresql_table(input_token_stream.clone());
        //         quote::quote! {
        //             #input_token_stream
        //             #token_stream
        //         }
        //     }
        //     .to_string()
        //     .as_bytes(),
        // )
        // .expect("37c3fa31-2603-496c-bb76-3d8019886352");
        // let status = std::process::Command::new("cargo")
        //     .args(["fmt", "--", path])
        //     .status()
        //     .expect("a681e0ea-4c3f-4ce8-9ba8-4c2e357ad0ea");
        // assert!(status.success(), "35cc9fc3-e941-4369-a658-d7f67d0ab728");
        // let clippy_status = std::process::Command::new("cargo")
        //     .args([
        //         "clippy",
        //         "--all-targets",
        //         "--all-features",
        //         "--",
        //         "-D",
        //         "warnings",
        //     ])
        //     .status()
        //     .expect("eed6c145-08a7-447d-8adf-c63b6347310a");
        // assert!(
        //     clippy_status.success(),
        //     "2c037283-420c-4076-8042-1eac09ba1a23"
        // );
        // std::fs::write(path, "").expect("79231418-b44a-4dac-8a88-3d8403024827");
    }
}
