pub fn postgres_error_syn_variants(
    proc_macro_name_upper_camel_case_ident_stringified: &str
) -> [syn::Variant;15] {
    let std_string_string_syn_punctuated_punctuated = crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
        &["std","string","String"],
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let code_occurence_field = crate::code_occurence_syn_field::code_occurence_syn_field(&proc_macro_name_upper_camel_case_ident_stringified);
    let configuration_error_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::configuration_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    //todo move it into custom macro attribute
    let database_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::database_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let io_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::io_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplay, 
                    &variant_name_snake_case_stringified,
                    crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                        &["std","io","Error"],
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ),
                )
            ]
        )
    };
    let tls_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::tls_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let protocol_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::protocol_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let row_not_found_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::row_not_found_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr404NotFound,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let type_not_found_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::type_not_found_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr400BadRequest,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let column_index_out_of_bounds_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::column_index_out_of_bounds_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified,
                    crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                        &["usize"],
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ),
                ),
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    "len",
                    crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                        &["usize"],
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ),
                )
            ]
        )
    };
    let column_not_found_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::column_not_found_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr400BadRequest,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let column_decode_syn_variant = crate::construct_syn_variant::construct_syn_variant(
        crate::status_code::StatusCode::Tvfrr500InternalServerError,
        &crate::naming_conventions::column_decode_upper_camel_case_stringified(),
        &code_occurence_field,
        vec![
            (
                crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                "column_decode_index", 
                std_string_string_syn_punctuated_punctuated.clone()
            ),
            (
                crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                "source_handle", 
                std_string_string_syn_punctuated_punctuated.clone()
            )
        ]
    );
    let decode_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::decode_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let pool_timed_out_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::pool_timed_out_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr408RequestTimeout,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let pool_closed_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::pool_closed_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let worker_crashed_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::worker_crashed_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize, 
                    &variant_name_snake_case_stringified, 
                    std_string_string_syn_punctuated_punctuated.clone()
                )
            ]
        )
    };
    let migrate_syn_variant = {
        let variant_name_upper_camel_case_stringified = crate::naming_conventions::migrate_upper_camel_case_stringified();
        let variant_name_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&variant_name_upper_camel_case_stringified);
        crate::construct_syn_variant::construct_syn_variant(
            crate::status_code::StatusCode::Tvfrr500InternalServerError,
            &variant_name_upper_camel_case_stringified,
            &code_occurence_field,
            vec![
                (
                    crate::error_occurence::named_attribute::NamedAttribute::EoDisplay, 
                    &variant_name_snake_case_stringified,
                    crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                        &["sqlx","migrate","MigrateError"],
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ),
                )
            ]
        )
    };
    [
        configuration_error_syn_variant,
        database_syn_variant,
        io_syn_variant,
        tls_syn_variant,
        protocol_syn_variant,
        row_not_found_syn_variant,
        type_not_found_syn_variant,
        column_index_out_of_bounds_syn_variant,
        column_not_found_syn_variant,
        column_decode_syn_variant,
        decode_syn_variant,
        pool_timed_out_syn_variant,
        pool_closed_syn_variant,
        worker_crashed_syn_variant,
        migrate_syn_variant
    ]
}