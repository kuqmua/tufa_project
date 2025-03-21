// pub fn sqlx_postgres_error_named_syn_variants(
//     proc_macro_name_upper_camel_case_ident_stringified: &str,
// ) -> [syn::Variant; 15] {
//     let std_string_string_syn_punctuated_punctuated =
//         crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
//             &["std", "string", "String"],
//             proc_macro_name_upper_camel_case_ident_stringified,
//         );
//     let code_occurence_field = crate::code_occurence_syn_field::code_occurence_syn_field();
//     let configuration_error_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             <naming::Configuration as naming::Naming>::upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr500InternalServerError,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &variant_name_snake_case_stringified,
//                     std_string_string_syn_punctuated_punctuated.clone()
//                 )
//             ]
//         )
//     };
//     //todo move it into custom macro attribute
//     let database_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             <naming::Database as naming::Naming>::upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr500InternalServerError,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &variant_name_snake_case_stringified,
//                     std_string_string_syn_punctuated_punctuated.clone()
//                 )
//             ]
//         )
//     };
//     let io_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             <naming::Io as naming::Naming>::upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr500InternalServerError,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
//                     &variant_name_snake_case_stringified,
//                     crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
//                         &["std","io","Error"],
//                         proc_macro_name_upper_camel_case_ident_stringified
//                     ),
//                 )
//             ]
//         )
//     };
//     let tls_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             <naming::Tls as naming::Naming>::upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr500InternalServerError,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &variant_name_snake_case_stringified,
//                     std_string_string_syn_punctuated_punctuated.clone()
//                 )
//             ]
//         )
//     };
//     let protocol_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             <naming::Protocol as naming::Naming>::upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr500InternalServerError,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &variant_name_snake_case_stringified,
//                     std_string_string_syn_punctuated_punctuated.clone()
//                 )
//             ]
//         )
//     };
//     let row_not_found_syn_variant = {
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr404NotFound,
//             &crate::naming::RowNotFoundUpperCamelCase,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &crate::naming::RowNotFoundSnakeCase,
//                     std_string_string_syn_punctuated_punctuated.clone()
//                 )
//             ]
//         )
//     };
//     let type_not_found_syn_variant = {
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr400BadRequest,
//             &crate::naming::TypeNotFoundUpperCamelCase,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &crate::naming::TypeNotFoundSnakeCase,
//                     std_string_string_syn_punctuated_punctuated.clone()
//                 )
//             ]
//         )
//     };
//     let column_index_out_of_bounds_syn_variant = {
//         let variant_name_upper_camel_case_stringified = crate::naming::ColumnIndexOutOfBoundsUpperCamelCase;
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr500InternalServerError,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &variant_name_snake_case_stringified,
//                     crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
//                         &["usize"],
//                         proc_macro_name_upper_camel_case_ident_stringified
//                     ),
//                 ),
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &naming::LenSnakeCase.to_string(),
//                     crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
//                         &["usize"],
//                         proc_macro_name_upper_camel_case_ident_stringified
//                     ),
//                 )
//             ]
//         )
//     };
//     let column_not_found_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             crate::naming::column_not_found_upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr400BadRequest,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &variant_name_snake_case_stringified,
//                     std_string_string_syn_punctuated_punctuated.clone()
//                 )
//             ]
//         )
//     };
//     let column_decode_syn_variant = crate::construct_syn_variant::construct_syn_variant_with_status_code(
//         crate::status_code::StatusCode::Tvfrr500InternalServerError,
//         &crate::naming::column_decode_upper_camel_case_stringified(),
//         &code_occurence_field,
//         vec![
//             (
//                 crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                 &crate::naming::column_decode_index_snake_case_stringified(),
//                 std_string_string_syn_punctuated_punctuated.clone()
//             ),
//             (
//                 crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                 &crate::naming::source_handle_snake_case_stringified(),
//                 std_string_string_syn_punctuated_punctuated.clone()
//             )
//         ]
//     );
//     let decode_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             <naming::Decode as naming::Naming>::upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr500InternalServerError,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &variant_name_snake_case_stringified,
//                     std_string_string_syn_punctuated_punctuated.clone()
//                 )
//             ]
//         )
//     };
//     let pool_timed_out_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             crate::naming::pool_timed_out_upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr408RequestTimeout,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &variant_name_snake_case_stringified,
//                     std_string_string_syn_punctuated_punctuated.clone()
//                 )
//             ]
//         )
//     };
//     let pool_closed_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             crate::naming::pool_closed_upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr500InternalServerError,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &variant_name_snake_case_stringified,
//                     std_string_string_syn_punctuated_punctuated.clone()
//                 )
//             ]
//         )
//     };
//     let worker_crashed_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             crate::naming::worker_crashed_upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr500InternalServerError,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &variant_name_snake_case_stringified,
//                     std_string_string_syn_punctuated_punctuated
//                 )
//             ]
//         )
//     };
//     let migrate_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             <naming::Migrate as naming::Naming>::upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr500InternalServerError,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
//                     &variant_name_snake_case_stringified,
//                     crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
//                         &["sqlx","migrate","MigrateError"],
//                         proc_macro_name_upper_camel_case_ident_stringified
//                     ),
//                 )
//             ]
//         )
//     };
//     [
//         configuration_error_syn_variant,
//         database_syn_variant,
//         io_syn_variant,
//         tls_syn_variant,
//         protocol_syn_variant,
//         row_not_found_syn_variant,
//         type_not_found_syn_variant,
//         column_index_out_of_bounds_syn_variant,
//         column_not_found_syn_variant,
//         column_decode_syn_variant,
//         decode_syn_variant,
//         pool_timed_out_syn_variant,
//         pool_closed_syn_variant,
//         worker_crashed_syn_variant,
//         migrate_syn_variant,
//     ]
// }

// pub fn json_extractor_error_named_syn_variants(
//     proc_macro_name_upper_camel_case_ident_stringified: &str,
// ) -> [syn::Variant; 4] {
//     let std_string_string_syn_punctuated_punctuated = crate::generate_simple_syn_punctuated_punctuated::std_string_string_syn_punctuated_punctuated(proc_macro_name_upper_camel_case_ident_stringified);
//     let code_occurence_field = crate::code_occurence_syn_field::code_occurence_syn_field();
//     let json_data_error_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             crate::naming::json_data_error_upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr400BadRequest,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
//                     &variant_name_snake_case_stringified,
//                     crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
//                         &["axum","extract","rejection","JsonDataError"],
//                         proc_macro_name_upper_camel_case_ident_stringified
//                     ),
//                 )
//             ]
//         )
//     };
//     let json_syntax_error_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             crate::naming::json_syntax_error_upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr400BadRequest,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
//                     &variant_name_snake_case_stringified,
//                     crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
//                         &["axum","extract","rejection","JsonSyntaxError"],
//                         proc_macro_name_upper_camel_case_ident_stringified
//                     ),
//                 )
//             ]
//         )
//     };
//     let missing_json_content_type_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             crate::naming::missing_json_content_type_upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr400BadRequest,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &variant_name_snake_case_stringified,
//                     std_string_string_syn_punctuated_punctuated.clone()
//                 )
//             ]
//         )
//     };
//     let bytes_rejection_syn_variant = {
//         let variant_name_upper_camel_case_stringified =
//             crate::naming::bytes_rejection_upper_camel_case_stringified();
//         let variant_name_snake_case_stringified = generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&variant_name_upper_camel_case_stringified);
//         crate::construct_syn_variant::construct_syn_variant_with_status_code(
//             crate::status_code::StatusCode::Tvfrr500InternalServerError,
//             &variant_name_upper_camel_case_stringified,
//             &code_occurence_field,
//             vec![
//                 (
//                     crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
//                     &variant_name_snake_case_stringified,
//                     std_string_string_syn_punctuated_punctuated
//                 )
//             ]
//         )
//     };
//     [
//         json_data_error_syn_variant,
//         json_syntax_error_syn_variant,
//         missing_json_content_type_syn_variant,
//         bytes_rejection_syn_variant,
//     ]
// }
