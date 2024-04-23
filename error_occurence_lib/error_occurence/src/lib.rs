#![deny(clippy::unwrap_used, clippy::float_arithmetic)]
#![allow(clippy::too_many_arguments)]

//there is a possibility for not doing with_serialize_deserialize case (then type does not implement serde::Serialize and serde::Deserialize) https://serde.rs/remote-derive.html
//todo not all implementations of Unnnamed ErrorOccurence are support Sized config Generic. fix its(hashmap\vec impl)
//todo change how hashmap shows in console
//todo maybe structs that are enums or containing enums - maybe convert them not into std::string::String, but some custom type that copies all logic of the type?
//todo maybe add multiple lifetimes supports with attribute parameters like this
// #[derive(Serialize)]
// struct Foo {
//     #[doc = include_str!("x.md")]
//     x: u32
// }
//todo - maybe remove possibility to use references for display, display_foreign_type, error occurence for WithSerializeDeserialize
#[proc_macro_derive(
    ErrorOccurence,
    attributes(
        eo_display,
        eo_display_with_serialize_deserialize,
        eo_display_foreign_type,
        eo_display_foreign_type_with_serialize_deserialize,
        eo_error_occurence,
        //todo error_occurence version for - after errors after deserialization
        eo_vec_display,//todo maybe add version without generation \n for each element?
        eo_vec_display_with_serialize_deserialize,
        eo_vec_display_foreign_type,
        eo_vec_display_foreign_type_with_serialize_deserialize,
        eo_vec_error_occurence,
        eo_hashmap_key_display_with_serialize_deserialize_value_display,
        eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize,
        eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type,
        eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize,
        eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence,
        eo_hashmap_key_display_foreign_type_value_display,
        eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize,
        eo_hashmap_key_display_foreign_type_value_display_foreign_type,
        eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize,
        eo_hashmap_key_display_foreign_type_value_error_occurence,
    )
)]
pub fn error_occurence(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name =
        proc_macro_helpers::naming_conventions::error_occurence_upper_camel_case_stringified();
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|_| {
        panic!(
            "{proc_macro_name} {}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    let error_occurence_snake_case =
        proc_macro_helpers::naming_conventions::error_occurence_snake_case_stringified();
    let trait_lifetime_stringified =
        format!("'{error_occurence_snake_case}_proc_macro_reserved_lifetime_name");
    let ident = &ast.ident;
    let ident_stringified = ident.to_string();
    let proc_macro_name_ident_stringified = format!("{proc_macro_name} {ident_stringified}");
    let data_enum = if let syn::Data::Enum(data_enum) = ast.data {
        data_enum
    } else {
        panic!(
            "{proc_macro_name_ident_stringified} {} syn::Data::Enum",
            naming_constants::SUPPORTS_ONLY_STRINGIFIED
        );
    };
    //todo ident lifetimes removed. maybe some other logic must be removed too
    let generics_len = ast.generics.params.len();
    let generics = {
        let mut lifetimes_stringified =
            ast.generics
                .params
                .iter()
                .fold(std::string::String::new(), |mut acc, gen_param| {
                    if let syn::GenericParam::Lifetime(lifetime_deref) = gen_param {
                        acc.push_str(&format!("'{},", lifetime_deref.lifetime.ident));
                        acc
                    } else {
                        panic!(
                            "{proc_macro_name_ident_stringified} {} syn::GenericParam::Lifetime",
                            naming_constants::SUPPORTS_ONLY_STRINGIFIED
                        );
                    }
                });
        let _: std::option::Option<std::primitive::char> = lifetimes_stringified.pop();
        assert!(!lifetimes_stringified.contains(&trait_lifetime_stringified), "{proc_macro_name_ident_stringified} must not contain reserved by macro lifetime name: {trait_lifetime_stringified}");
        lifetimes_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {lifetimes_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let named_snake_case = <naming_constants::Named as naming_constants::Naming>::snake_case_stringified();
    let unnamed_upper_camel_case =
        <naming_constants::Unnamed as naming_constants::Naming>::upper_camel_case_stringified();
    let supported_enum_variant =
        proc_macro_helpers::error_occurence::supported_enum_variant::create(
            &data_enum,
            &proc_macro_name_ident_stringified,
        );
    let trait_lifetime_token_stream = trait_lifetime_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {trait_lifetime_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let with_serialize_deserialize_upper_camel_case = proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified();
    let ident_with_serialize_deserialize_stringified =
        format!("{ident}{with_serialize_deserialize_upper_camel_case}");
    let ident_with_serialize_deserialize_token_stream = ident_with_serialize_deserialize_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {ident_with_serialize_deserialize_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let config_upper_camel_case_stringified =
        <naming_constants::Config as naming_constants::Naming>::upper_camel_case_stringified();
    let config_generic_upper_camel_case = format!("{config_upper_camel_case_stringified}Generic");
    let config_generic_token_stream = config_generic_upper_camel_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {config_generic_upper_camel_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let source_upper_camel_case_stringified =
        <naming_constants::Source as naming_constants::Naming>::upper_camel_case_stringified();
    let to_string_upper_camel_case = format!(
        "To{}",
        <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
    );
    let to_string_with_config_upper_camel_case = format!(
        "{to_string_upper_camel_case}{}{config_upper_camel_case_stringified}",
        <naming_constants::With as naming_constants::Naming>::upper_camel_case_stringified()
    );
    let source_to_string_with_config_upper_camel_case =
        format!("{source_upper_camel_case_stringified}{to_string_with_config_upper_camel_case}");
    let unnamed_snake_case_stringified =
        proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &unnamed_upper_camel_case,
        );
    let error_occurence_named_upper_camel_case = format!(
        "{proc_macro_name}{}",
        <naming_constants::Named as naming_constants::Naming>::upper_camel_case_stringified()
    );
    let error_occurence_named_snake_case =
        format!("{error_occurence_snake_case}_{named_snake_case}");
    let error_occurence_named_token_stream = error_occurence_named_snake_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_named_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let error_occurence_unnamed_upper_camel_case =
        format!("{proc_macro_name}{unnamed_upper_camel_case}");
    let error_occurence_unnamed_snake_case =
        format!("{error_occurence_snake_case}_{unnamed_snake_case_stringified}");
    let error_occurence_unnamed_token_stream = error_occurence_unnamed_snake_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_unnamed_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let error_occurence_lib_stringified = "error_occurence_lib";
    let error_occurence_lib_error_occurence_named_error_occurence_named_stringified = format!("{error_occurence_lib_stringified}::{error_occurence_named_snake_case}::{error_occurence_named_upper_camel_case}");
    let error_occurence_lib_error_occurence_named_error_occurence_named_token_stream = error_occurence_lib_error_occurence_named_error_occurence_named_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_error_occurence_named_error_occurence_named_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let error_occurence_lib_error_occurence_unnamed_error_occurence_unnamed_stringified = format!("{error_occurence_lib_stringified}::{error_occurence_unnamed_snake_case}::{error_occurence_unnamed_upper_camel_case}");
    let error_occurence_lib_error_occurence_unnamed_error_occurence_unnamed_token_stream = error_occurence_lib_error_occurence_unnamed_error_occurence_unnamed_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_error_occurence_unnamed_error_occurence_unnamed_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let to_string_without_config_upper_camel_case = format!(
        "{to_string_upper_camel_case}{}out{config_upper_camel_case_stringified}",
        <naming_constants::With as naming_constants::Naming>::upper_camel_case_stringified()
    );
    let to_string_without_config_snake_case_stringified =
        proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &to_string_without_config_upper_camel_case,
        );
    let error_occurence_lib_to_string_without_config_to_string_without_config_stringified = format!("{error_occurence_lib_stringified}::{to_string_without_config_snake_case_stringified}::{to_string_without_config_upper_camel_case}");
    let error_occurence_lib_to_string_without_config_to_string_without_config_token_stream = error_occurence_lib_to_string_without_config_to_string_without_config_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_to_string_without_config_to_string_without_config_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let error_occurence_lib_to_string_without_config_with_serialize_deserialize_stringified = format!("{error_occurence_lib_to_string_without_config_to_string_without_config_stringified}{with_serialize_deserialize_upper_camel_case}");
    let error_occurence_lib_to_string_without_config_with_serialize_deserialize_token_stream = error_occurence_lib_to_string_without_config_with_serialize_deserialize_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_to_string_without_config_with_serialize_deserialize_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let crate_common_config_stringified = "config_lib"; //crate::common::config
    let crate_common_config_path_stringified =
        format!("{crate_common_config_stringified}::");
    let get_upper_camel_case_stringified =
        <naming_constants::Get as naming_constants::Naming>::upper_camel_case_stringified();
    let crate_common_config_path_get_source_place_type_stringified = format!("{crate_common_config_path_stringified}{get_upper_camel_case_stringified}{source_upper_camel_case_stringified}PlaceType");
    let crate_common_config_path_get_source_place_type_token_stream =
    crate_common_config_path_get_source_place_type_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_config_path_get_source_place_type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let crate_common_config_path_get_timezone_stringified = format!(
        "{crate_common_config_path_stringified}{get_upper_camel_case_stringified}Timezone"
    );
    let crate_common_config_path_get_timezone_token_stream =
    crate_common_config_path_get_timezone_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_config_path_get_timezone_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let source_snake_case_stringified =
        proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &source_upper_camel_case_stringified,
        );
    let to_string_with_config_snake_case_stringified =
        proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &to_string_with_config_upper_camel_case,
        );
    let source_to_string_with_config_stringified =
        format!("{source_snake_case_stringified}_{to_string_with_config_snake_case_stringified}");
    let source_to_string_with_config_token_stream =
    source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {source_to_string_with_config_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let to_string_without_config_token_stream =
    to_string_without_config_snake_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {to_string_without_config_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let with_serialize_deserialize_snake_case =
        proc_macro_helpers::naming_conventions::with_serialize_deserialize_snake_case_stringified();
    let with_serialize_deserialize_stringified = format!(
        "{to_string_without_config_snake_case_stringified}_{with_serialize_deserialize_snake_case}"
    );
    let with_serialize_deserialize_token_stream =
    with_serialize_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {with_serialize_deserialize_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let into_upper_camel_case_stringified =
        <naming_constants::Into as naming_constants::Naming>::upper_camel_case_stringified();
    let into_snake_case_stringified =
        proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(
            &into_upper_camel_case_stringified,
        );
    let serialize_deserialize_snake_case_stringified =
        proc_macro_helpers::naming_conventions::serialize_deserialize_snake_case_stringified();

    // proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&proc_macro_helpers::naming_conventions::SERIALIZE_DESERIALIZE_UPPER_CAMEL_CASE);
    let into_serialize_deserialize_version_stringified = format!(
        "{into_snake_case_stringified}_{serialize_deserialize_snake_case_stringified}_version"
    );
    let into_serialize_deserialize_version_token_stream = into_serialize_deserialize_version_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {into_serialize_deserialize_version_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let supports_only_supported_container_stringified =
        proc_macro_helpers::naming_conventions::supports_only_supported_container_stringified();
    let syn_type_path_stringified =
        proc_macro_helpers::naming_conventions::syn_type_path_stringified();
    let compile_time_check_error_occurence_members_stringified =
        format!("_compile_time_check_{error_occurence_snake_case}_members");
    let compile_time_check_error_occurence_members_token_stream = compile_time_check_error_occurence_members_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {compile_time_check_error_occurence_members_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let error_occurence_lib_to_string_with_config_to_string_with_config_stringified = format!("{error_occurence_lib_stringified}::{to_string_with_config_snake_case_stringified}::{to_string_with_config_upper_camel_case}");
    let error_occurence_lib_to_string_with_config_to_string_with_config_token_stream =
    error_occurence_lib_to_string_with_config_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_to_string_with_config_to_string_with_config_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let to_string_with_config_token_stream =
    to_string_with_config_snake_case_stringified.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {to_string_with_config_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let key_snake_case = <naming_constants::Key as naming_constants::Naming>::upper_camel_case_stringified();
    let value_snake_case =
        <naming_constants::Value as naming_constants::Naming>::upper_camel_case_stringified();
    let hashmap_snake_case =
        <naming_constants::HashMap as naming_constants::Naming>::snake_case_stringified();
    let vec_snake_case = <naming_constants::Vec as naming_constants::Naming>::snake_case_stringified();
    let enum_with_serialize_deserialize_logic = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_with_serialize_deserialize_version(
        &supported_enum_variant,
        &data_enum.variants.iter().collect(),
        &proc_macro_name_ident_stringified,
        generics_len,
        &ident_with_serialize_deserialize_token_stream,
        None,
        true,
        true
    );
    let token_stream = match supported_enum_variant {
        proc_macro_helpers::error_occurence::supported_enum_variant::SuportedEnumVariant::Named => {
            let code_occurence_upper_camel_case_stringified = proc_macro_helpers::naming_conventions::code_occurence_upper_camel_case_stringified();
            let code_occurence_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&code_occurence_upper_camel_case_stringified);
            let foreign_type_upper_camel_case_stringified = format!(
                "{}{}",
                <naming_constants::Foreign as naming_constants::Naming>::upper_camel_case_stringified(),
                <naming_constants::Type as naming_constants::Naming>::upper_camel_case_stringified()
            );
            let display_upper_camel_case_stringified = <naming_constants::Display as naming_constants::Naming>::upper_camel_case_stringified();
            let display_foreign_type_upper_camel_case = format!("{display_upper_camel_case_stringified}{foreign_type_upper_camel_case_stringified}");
            let display_foreign_type_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&display_foreign_type_upper_camel_case);
            let display_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&display_upper_camel_case_stringified);
            let attribute_prefix_stringified = "eo_";
            let attribute_display_stringified = format!("{attribute_prefix_stringified}{display_snake_case_stringified}");
            let attribute_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{display_snake_case_stringified}_{with_serialize_deserialize_snake_case}");
            let attribute_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{display_foreign_type_snake_case_stringified}");
            let attribute_display_foreign_type_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{display_foreign_type_snake_case_stringified}_{with_serialize_deserialize_snake_case}");
            let attribute_error_occurence_stringified = format!("{attribute_prefix_stringified}{error_occurence_snake_case}");
            let attribute_vec_display_stringified = format!("{attribute_prefix_stringified}{vec_snake_case}_{display_snake_case_stringified}");
            let attribute_vec_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{vec_snake_case}_{display_snake_case_stringified}_{with_serialize_deserialize_snake_case}");
            let attribute_vec_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{vec_snake_case}_{display_foreign_type_snake_case_stringified}");
            let attribute_vec_display_foreign_type_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{vec_snake_case}_{display_foreign_type_snake_case_stringified}_{with_serialize_deserialize_snake_case}");
            let attribute_vec_error_occurence_stringified = format!("{attribute_prefix_stringified}{vec_snake_case}_{error_occurence_snake_case}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_display_stringified = format!("{attribute_prefix_stringified}{hashmap_snake_case}_{key_snake_case}_{display_snake_case_stringified}_{with_serialize_deserialize_snake_case}_{value_snake_case}_{display_snake_case_stringified}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_snake_case}_{key_snake_case}_{display_snake_case_stringified}_{with_serialize_deserialize_snake_case}_{value_snake_case}_{display_snake_case_stringified}_{with_serialize_deserialize_snake_case}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{hashmap_snake_case}_{key_snake_case}_{display_snake_case_stringified}_{with_serialize_deserialize_snake_case}_{value_snake_case}_{display_foreign_type_snake_case_stringified}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_snake_case}_{key_snake_case}_{display_snake_case_stringified}_{with_serialize_deserialize_snake_case}_{value_snake_case}_{display_foreign_type_snake_case_stringified}_{with_serialize_deserialize_snake_case}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_error_occurence_stringified = format!("{attribute_prefix_stringified}{hashmap_snake_case}_{key_snake_case}_{display_snake_case_stringified}_{with_serialize_deserialize_snake_case}_{value_snake_case}_{error_occurence_snake_case}");
            let attribute_hashmap_key_display_foreign_type_value_display_stringified = format!("{attribute_prefix_stringified}{hashmap_snake_case}_{key_snake_case}_{display_foreign_type_snake_case_stringified}_{value_snake_case}_{display_snake_case_stringified}");
            let attribute_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_snake_case}_{key_snake_case}_{display_foreign_type_snake_case_stringified}_{value_snake_case}_{display_snake_case_stringified}_{with_serialize_deserialize_snake_case}");
            let attribute_hashmap_key_display_foreign_type_value_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{hashmap_snake_case}_{key_snake_case}_{display_foreign_type_snake_case_stringified}_{value_snake_case}_{display_foreign_type_snake_case_stringified}");
            let attribute_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_snake_case}_{key_snake_case}_{display_foreign_type_snake_case_stringified}_{value_snake_case}_{display_foreign_type_snake_case_stringified}_{with_serialize_deserialize_snake_case}");
            let attribute_hashmap_key_display_foreign_type_value_error_occurence_stringified = format!("{attribute_prefix_stringified}{hashmap_snake_case}_{key_snake_case}_{display_foreign_type_snake_case_stringified}_{value_snake_case}_{error_occurence_snake_case}");
            let variants_vec = data_enum.variants.into_iter().map(|variant| {
                let variant_fields_vec = if let syn::Fields::Named(fields_named) = variant.fields {
                    fields_named.named.into_iter().map(|field|{
                        let field_ident = field.ident.unwrap_or_else(|| panic!(
                            "{proc_macro_name_ident_stringified} field.ident {}",
                            naming_constants::IS_NONE_STRINGIFIED
                        ));
                        let error_or_code_occurence = if field_ident == *code_occurence_snake_case_stringified {
                            let (code_occurence_type_stringified, code_occurence_lifetime) = {
                                if let syn::Type::Path(type_path) = &field.ty {
                                    (
                                        {
                                            let mut code_occurence_type_repeat_checker = false;
                                            let code_occurence_segments_stringified_handle = type_path.path.segments.iter()
                                            .fold(std::string::String::new(), |mut acc, path_segment| {
                                                let path_segment_ident = &path_segment.ident;
                                                if *path_segment_ident == code_occurence_upper_camel_case_stringified {
                                                    assert!(!code_occurence_type_repeat_checker, "{proc_macro_name_ident_stringified} code_occurence_ident detected more than one {code_occurence_upper_camel_case_stringified} inside type path");
                                                    acc.push_str(&path_segment_ident.to_string());
                                                    code_occurence_type_repeat_checker = true;
                                                }
                                                else {
                                                    acc.push_str(&format!("{path_segment_ident}::"));
                                                }
                                                acc
                                            });
                                            assert!(code_occurence_type_repeat_checker, "{proc_macro_name_ident_stringified} no {code_occurence_upper_camel_case_stringified} named field");
                                            code_occurence_segments_stringified_handle
                                        },
                                        proc_macro_helpers::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                            &type_path.path.segments,
                                            &proc_macro_name_ident_stringified
                                        ),
                                    )
                                  }
                                else {
                                    panic!("{proc_macro_name_ident_stringified} {code_occurence_snake_case_stringified} {} {syn_type_path_stringified}", naming_constants::SUPPORTS_ONLY_STRINGIFIED);
                                }
                            };
                            proc_macro_helpers::error_occurence::error_field_or_code_occurence::ErrorFieldOrCodeOccurence::CodeOccurence {
                                field_type: code_occurence_type_stringified,
                                vec_lifetime: code_occurence_lifetime
                            }
                        }
                        else {
                            let attribute = {
                                let mut option_attribute = None;
                                field.attrs.iter().for_each(|attr|{
                                    if attr.path().segments.len() == 1 {
                                        let error_message = format!("{proc_macro_name_ident_stringified} two or more supported attributes!");
                                        if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_display_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_display_with_serialize_deserialize_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_display_foreign_type_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayForeignType);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_display_foreign_type_with_serialize_deserialize_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayForeignTypeWithSerializeDeserialize);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_error_occurence_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_vec_display_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecDisplay);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_vec_display_with_serialize_deserialize_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecDisplayWithSerializeDeserialize);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_vec_display_foreign_type_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecDisplayForeignType);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_vec_display_foreign_type_with_serialize_deserialize_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecDisplayForeignTypeWithSerializeDeserialize);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_vec_error_occurence_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecErrorOccurence);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_hashmap_key_display_with_serialize_deserialize_value_display_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_hashmap_key_display_with_serialize_deserialize_value_error_occurence_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_hashmap_key_display_foreign_type_value_display_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplay);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_hashmap_key_display_foreign_type_value_display_foreign_type_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignType);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize);
                                            }
                                        }
                                        else if attr.path().segments.first().expect("no first value in punctuated").ident == attribute_hashmap_key_display_foreign_type_value_error_occurence_stringified {
                                            if option_attribute.is_some() {
                                                panic!("{error_message}");
                                            }
                                            else {
                                                option_attribute = Some(proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueErrorOccurence);
                                            }
                                        }
                                        else {
                                            //clippy lint forces to add empty else
                                        }
                                    }//other attributes are not for this proc_macro
                                });
                                option_attribute.unwrap_or_else(|| panic!(
                                    "{proc_macro_name_ident_stringified} option attribute {}",
                                    naming_constants::IS_NONE_STRINGIFIED
                                ))
                            };
                            let syn_type_reference = format!(
                                "syn::Type::{}",
                                <naming_constants::Reference as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let error_message = format!("{} {syn_type_path_stringified} and {syn_type_reference}", naming_constants::SUPPORTS_ONLY_STRINGIFIED);
                            let supported_container = match field.ty {
                                syn::Type::Path(type_path) => {
                                    let path = proc_macro_helpers::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments);
                                    let vec_lifetime = proc_macro_helpers::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                        &type_path.path.segments,
                                        &proc_macro_name_ident_stringified
                                    );
                                    let path_segment = type_path.path.segments.into_iter().last()
                                    .unwrap_or_else(|| panic!(
                                        "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().last() {}",
                                        naming_constants::IS_NONE_STRINGIFIED
                                    ));
                                    if path_segment.ident == <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified() {
                                        let vec_element_type = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = path_segment.arguments {
                                            if angle_brackets_generic_arguments.args.len() == 1 {
                                                if let syn::GenericArgument::Type(type_handle) =
                                                    angle_brackets_generic_arguments.args
                                                    .into_iter().next()
                                                    .unwrap_or_else(|| panic!(
                                                        "{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.into_iter().nth(0) {}",
                                                        naming_constants::IS_NONE_STRINGIFIED
                                                    ))
                                                {
                                                    match type_handle {
                                                        syn::Type::Path(type_path) => proc_macro_helpers::error_occurence::vec_element_type::VecElementType::Path{
                                                            element_path: proc_macro_helpers::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
                                                            vec_lifetime: proc_macro_helpers::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                                                &type_path.path.segments,
                                                                &proc_macro_name_ident_stringified
                                                            )
                                                        },
                                                        syn::Type::Reference(type_reference) => {
                                                            let reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem {
                                                                if type_path.path.segments.len() == 1 {
                                                                    type_path.path.segments
                                                                    .into_iter().next()
                                                                    .unwrap_or_else(|| panic!(
                                                                        "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
                                                                        naming_constants::IS_NONE_STRINGIFIED
                                                                    ))
                                                                    .ident
                                                                }
                                                                else {
                                                                    panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                                                }
                                                            }
                                                            else {
                                                                panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {syn_type_path_stringified}", naming_constants::SUPPORTS_ONLY_STRINGIFIED);
                                                            };
                                                            proc_macro_helpers::error_occurence::vec_element_type::VecElementType::Reference {
                                                                reference_ident,
                                                                lifetime_ident: type_reference.lifetime.unwrap_or_else(|| panic!(
                                                                    "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
                                                                    naming_constants::IS_NONE_STRINGIFIED
                                                                )).ident
                                                            }
                                                        },
                                                        _ => panic!("{proc_macro_name_ident_stringified} type_handle {} {syn_type_path_stringified} and {syn_type_reference}", naming_constants::SUPPORTS_ONLY_STRINGIFIED),
                                                    }
                                                }
                                                else {
                                                    panic!(
                                                        "{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args[0] {} {}", naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                        naming_constants::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
                                                    );
                                                }
                                            }
                                            else {
                                                panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() == 1");
                                            }
                                        }
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} path_segment.arguments {} syn::PathArguments::AngleBracketed", naming_constants::SUPPORTS_ONLY_STRINGIFIED);
                                        };
                                        proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Vec{
                                            path,
                                            vec_element_type
                                        }
                                    }
                                    else if path_segment.ident == <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified() {
                                        let (
                                            hashmap_key_type,
                                            hashmap_value_type
                                        ) = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = path_segment.arguments {
                                            if angle_brackets_generic_arguments.args.len() == 2 {
                                                let (
                                                    key_generic_argument,
                                                    value_generic_argument
                                                ) = {
                                                    let mut key_generic_argument_option = None;
                                                    let mut value_generic_argument_option = None;
                                                    angle_brackets_generic_arguments.args
                                                    .into_iter()
                                                    .enumerate()
                                                    .for_each(|(index, generic_argument)|{
                                                        match index {
                                                            0 => {
                                                                key_generic_argument_option = Some(generic_argument);
                                                            }
                                                            1 => {
                                                                value_generic_argument_option = Some(generic_argument);
                                                            }
                                                            _ => panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() != 2")
                                                        }
                                                    });
                                                    (
                                                        key_generic_argument_option.unwrap_or_else(|| panic!(
                                                            "{proc_macro_name_ident_stringified} key_generic_argument_option {}",
                                                            naming_constants::IS_NONE_STRINGIFIED
                                                        )),
                                                        value_generic_argument_option.unwrap_or_else(|| panic!(
                                                            "{proc_macro_name_ident_stringified} value_generic_argument_option {}",
                                                            naming_constants::IS_NONE_STRINGIFIED
                                                        ))
                                                    )
                                                };
                                                let hashmap_key_type
                                                = if let syn::GenericArgument::Type(type_handle) =
                                                    key_generic_argument
                                                {
                                                    match type_handle {
                                                        syn::Type::Path(type_path) => {
                                                            proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path{
                                                                key_segments_stringified: proc_macro_helpers::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
                                                                key_vec_lifetime: proc_macro_helpers::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                                                    &type_path.path.segments,
                                                                    &proc_macro_name_ident_stringified
                                                                )
                                                            }
                                                        },
                                                        syn::Type::Reference(type_reference) => {
                                                            let key_reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem {
                                                                if type_path.path.segments.len() == 1 {
                                                                    type_path.path.segments
                                                                    .into_iter().next()
                                                                    .unwrap_or_else(|| panic!(
                                                                        "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
                                                                        naming_constants::IS_NONE_STRINGIFIED
                                                                    ))
                                                                    .ident
                                                                }
                                                                else {
                                                                    panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                                                }
                                                            }
                                                            else {
                                                                panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {syn_type_path_stringified}", naming_constants::SUPPORTS_ONLY_STRINGIFIED);
                                                            };
                                                            proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                                key_reference_ident,
                                                                key_lifetime_ident: type_reference.lifetime.unwrap_or_else(|| panic!(
                                                                    "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
                                                                    naming_constants::IS_NONE_STRINGIFIED
                                                                )).ident
                                                            }
                                                        },
                                                        _ => panic!("{proc_macro_name_ident_stringified} type_handle {} {syn_type_path_stringified} and {syn_type_reference}", naming_constants::SUPPORTS_ONLY_STRINGIFIED),
                                                    }
                                                }
                                                else {
                                                    panic!(
                                                        "{proc_macro_name_ident_stringified} key_generic_argument {} {}", 
                                                        naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                        naming_constants::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
                                                    );
                                                };
                                                let hashmap_value_type = if let syn::GenericArgument::Type(type_handle) = value_generic_argument {
                                                    match type_handle {
                                                        syn::Type::Path(type_path) => {
                                                           proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path{
                                                                value_segments_stringified: proc_macro_helpers::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
                                                                value_vec_lifetime: proc_macro_helpers::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
                                                                    &type_path.path.segments,
                                                                    &proc_macro_name_ident_stringified
                                                                )
                                                            }
                                                        },
                                                        syn::Type::Reference(type_reference) => {
                                                            let value_reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem {
                                                                if type_path.path.segments.len() == 1 {
                                                                    type_path.path.segments
                                                                    .into_iter().next()
                                                                    .unwrap_or_else(|| panic!(
                                                                        "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
                                                                        naming_constants::IS_NONE_STRINGIFIED
                                                                    ))
                                                                    .ident
                                                                }
                                                                else {
                                                                    panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                                                }
                                                            }
                                                            else {
                                                                panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {syn_type_path_stringified}", naming_constants::SUPPORTS_ONLY_STRINGIFIED);
                                                            };
                                                           proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                                value_reference_ident,
                                                                value_lifetime_ident: type_reference.lifetime.unwrap_or_else(|| panic!(
                                                                    "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
                                                                    naming_constants::IS_NONE_STRINGIFIED
                                                                )).ident
                                                            }
                                                        },
                                                        _ => panic!("{proc_macro_name_ident_stringified} type_handle {} {syn_type_path_stringified} and syn::Type::Reference", naming_constants::SUPPORTS_ONLY_STRINGIFIED),
                                                    }
                                                }
                                                else {
                                                    panic!(
                                                        "{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args[0] {} {}", naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                        naming_constants::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
                                                    );
                                                };
                                                (
                                                    hashmap_key_type,
                                                    hashmap_value_type,
                                                )
                                            }
                                            else {
                                                panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() == 2");
                                            }
                                        }
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} path_segment.arguments {} syn::PathArguments::AngleBracketed", naming_constants::SUPPORTS_ONLY_STRINGIFIED);
                                        };
                                        proc_macro_helpers::error_occurence::supported_container::SupportedContainer::HashMap{
                                            path,
                                            hashmap_key_type,
                                            hashmap_value_type
                                        }
                                    }
                                    else {
                                        proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Path{
                                            path,
                                            vec_lifetime,
                                        }
                                    }
                                },
                                syn::Type::Reference(type_reference) => {
                                    let reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem {
                                        if type_path.path.segments.len() == 1 {
                                            type_path.path.segments
                                            .into_iter().next()
                                            .unwrap_or_else(|| panic!(
                                                "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
                                                naming_constants::IS_NONE_STRINGIFIED
                                            ))
                                            .ident
                                        }
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {syn_type_path_stringified}", naming_constants::SUPPORTS_ONLY_STRINGIFIED);
                                    };
                                    proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Reference{
                                        reference_ident,
                                        lifetime_ident: type_reference.lifetime.unwrap_or_else(|| panic!(
                                            "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
                                            naming_constants::IS_NONE_STRINGIFIED
                                        )).ident,
                                    }
                                },
                                _ => panic!("{proc_macro_name_ident_stringified} {code_occurence_snake_case_stringified} {error_message}"),
                            };
                            proc_macro_helpers::error_occurence::error_field_or_code_occurence::ErrorFieldOrCodeOccurence::ErrorField {
                                attribute,
                                supported_container,
                            }
                        };
                        (
                            field_ident,
                            error_or_code_occurence,
                        )
                    })
                    .collect::<Vec<(
                        proc_macro2::Ident,
                        proc_macro_helpers::error_occurence::error_field_or_code_occurence::ErrorFieldOrCodeOccurence
                    )>>()
                }
                else {
                    panic!("{proc_macro_name_ident_stringified} expected fields would be named");
                };
                (
                    variant.ident,
                    variant_fields_vec,
                )
            })
            .collect::<Vec<(
                proc_macro2::Ident,
                 Vec<(
                    proc_macro2::Ident,
                    proc_macro_helpers::error_occurence::error_field_or_code_occurence::ErrorFieldOrCodeOccurence
                )>
            )>>();
            let source_to_string_without_config_upper_camel_case = format!("{source_upper_camel_case_stringified}{to_string_without_config_upper_camel_case}");
            let source_to_string_without_config_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&source_to_string_without_config_upper_camel_case);
            let source_to_string_without_config_token_stream =
            source_to_string_without_config_snake_case_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {source_to_string_without_config_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            //
            let error_occurence_lib_source_to_string_without_config_source_to_string_without_config_stringified = format!("{error_occurence_lib_stringified}::{source_to_string_without_config_snake_case_stringified}::{source_to_string_without_config_upper_camel_case}");
            let error_occurence_lib_source_to_string_without_config_source_to_string_without_config_token_stream =
            error_occurence_lib_source_to_string_without_config_source_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_source_to_string_without_config_source_to_string_without_config_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            //
            let error_occurence_lib_source_to_string_with_config_source_to_string_with_config_stringified = format!("{error_occurence_lib_stringified}::{source_to_string_with_config_stringified}::{source_to_string_with_config_upper_camel_case}");
            let error_occurence_lib_source_to_string_with_config_source_to_string_with_config_token_stream =
            error_occurence_lib_source_to_string_with_config_source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_source_to_string_with_config_source_to_string_with_config_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let mut lifetimes_for_serialize_deserialize = Vec::with_capacity(generics_len);
            let mut logic_for_source_to_string_with_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_source_to_string_without_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_get_code_occurence: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            // let mut logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_source_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_get_code_occurence_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_into_serialize_deserialize_version: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_compile_time_check_error_occurence_members: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut should_generate_impl_compile_time_check_error_occurence_members = false;
            variants_vec.into_iter().for_each(|(
                variant_ident,
                fields_vec
            )|{
                let mut enum_fields_logic_for_source_to_string_with_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_source_to_string_without_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_get_code_occurence: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                // let mut enum_fields_logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_source_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_get_code_occurence_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_into_serialize_deserialize_version: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_compile_time_check_error_occurence_members: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut format_logic_for_source_to_string_with_or_without_config: Vec<&str> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_source_to_string_with_config_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_source_to_string_without_config_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_source_with_serialize_deserialize_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_into_serialize_deserialize_version_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_compile_time_check_error_occurence_members_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                fields_vec.into_iter().enumerate().for_each(|(index, (field_ident, error_or_code_occurence))|{
                    let unused_argument_handle_stringified = format!("_unused_argument_{index}");
                    let unused_argument_handle_token_stream = unused_argument_handle_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {unused_argument_handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                    let to_string_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&to_string_upper_camel_case);
                    let hashmap_display_to_string_without_config_to_string_snake_case = format!("{hashmap_snake_case}_{display_snake_case_stringified}_{to_string_without_config_snake_case_stringified}_{to_string_snake_case_stringified}");
                    match error_or_code_occurence {
                        proc_macro_helpers::error_occurence::error_field_or_code_occurence::ErrorFieldOrCodeOccurence::ErrorField {
                            attribute,
                            supported_container,
                        } => {
                            let field_name_with_field_value_token_stream = {
                                let field_name_with_field_value_stringified = format!("\"{field_ident}: {{}}\"");
                                field_name_with_field_value_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {field_name_with_field_value_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            let display_foreign_type_snake_case_token_stream =
                            display_foreign_type_snake_case_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {display_foreign_type_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let to_string_token_stream = to_string_snake_case_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {to_string_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let lines_space_backslash_upper_camel_case = "LinesSpaceBackslash";
                            let lines_space_backslash_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&lines_space_backslash_upper_camel_case);
                            let lines_space_backslash_snake_case_token_stream =
                            lines_space_backslash_snake_case_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {lines_space_backslash_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let error_occurence_lib_lines_space_backslash_lines_space_backslash_stringified = format!("{error_occurence_lib_stringified}::{lines_space_backslash_snake_case_stringified}::{lines_space_backslash_upper_camel_case}");
                            let error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream =
                            error_occurence_lib_lines_space_backslash_lines_space_backslash_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_lines_space_backslash_lines_space_backslash_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let error_occurence_lib_display_foreign_type_stringified = format!("{error_occurence_lib_stringified}::{display_foreign_type_upper_camel_case}");
                            let error_occurence_lib_display_foreign_type_token_stream =
                            error_occurence_lib_display_foreign_type_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_display_foreign_type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let vec_display_to_string_upper_camel_case = format!(
                                "{}{display_upper_camel_case_stringified}{to_string_upper_camel_case}",
                                <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let vec_display_to_string_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&vec_display_to_string_upper_camel_case);
                            let vec_display_to_string_snake_case_token_stream =
                            vec_display_to_string_snake_case_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {vec_display_to_string_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            //


                            let error_occurence_lib_vec_display_to_string_vec_display_to_string_stringified = format!("{error_occurence_lib_stringified}::{vec_display_to_string_snake_case_stringified}::{vec_display_to_string_upper_camel_case}");
                            let error_occurence_lib_vec_display_to_string_vec_display_to_string_token_stream =
                            error_occurence_lib_vec_display_to_string_vec_display_to_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_vec_display_to_string_vec_display_to_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let hashmap_display_display_to_string_snake_case = format!("{hashmap_snake_case}_{display_snake_case_stringified}_{display_snake_case_stringified}_{to_string_snake_case_stringified}");
                            let hashmap_display_display_to_string_snake_case_token_stream =
                            hashmap_display_display_to_string_snake_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_display_to_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_stringified = format!(
                                "{error_occurence_lib_stringified}::{hashmap_display_display_to_string_snake_case}::{}{display_upper_camel_case_stringified}{display_upper_camel_case_stringified}{to_string_upper_camel_case}",
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream =
                            error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let hashmap_display_display_foreign_type_to_string_snake_case = format!("{hashmap_snake_case}_{display_snake_case_stringified}_{display_foreign_type_snake_case_stringified}_{to_string_snake_case_stringified}");
                            let hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_snake_case = format!("{hashmap_snake_case}_{display_snake_case_stringified}_{to_string_without_config_snake_case_stringified}_{to_string_snake_case_stringified}_{with_serialize_deserialize_snake_case}");
                            let hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_snake_case_token_stream =
                            hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_snake_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let error_occurence_lib_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_stringified = format!(
                                "{error_occurence_lib_stringified}::{hashmap_display_to_string_without_config_to_string_snake_case}::{}{display_upper_camel_case_stringified}{to_string_without_config_upper_camel_case}{to_string_upper_camel_case}{with_serialize_deserialize_upper_camel_case}",
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let error_occurence_lib_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_token_stream =
                            error_occurence_lib_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let string_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&<naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified());
                            let hashmap_display_display_foreign_type_into_hashmap_display_string_snake_case = format!("{hashmap_snake_case}_{display_snake_case_stringified}_{display_foreign_type_snake_case_stringified}_{into_snake_case_stringified}_{hashmap_snake_case}_{display_snake_case_stringified}_{string_snake_case_stringified}");
                            let std_stringified = "std";
                            let std_string_string_token_stream = proc_macro_common::std_string_string_token_stream();
                            let hashmap_display_display_foreign_type_to_string_snake_case_token_stream =
                            hashmap_display_display_foreign_type_to_string_snake_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_display_foreign_type_to_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let error_occurence_lib_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_stringified = format!(
                                "{error_occurence_lib_stringified}::{hashmap_display_display_foreign_type_to_string_snake_case}::{}{display_upper_camel_case_stringified}{display_foreign_type_upper_camel_case}{to_string_upper_camel_case}",
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let error_occurence_lib_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream =
                            error_occurence_lib_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let vec_element_type_path_stringified = format!(
                                "proc_macro_helpers::error_occurence::vec_element_type::VecElementType::{}",
                                <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let vec_display_foreign_type_to_string_upper_camel_case = format!(
                                "{}{display_foreign_type_upper_camel_case}{to_string_upper_camel_case}",
                                <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let vec_display_foreign_type_to_string_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&vec_display_foreign_type_to_string_upper_camel_case);
                            let vec_display_foreign_type_to_string_snake_case_token_stream =
                            vec_display_foreign_type_to_string_snake_case_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {vec_display_foreign_type_to_string_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let error_occurence_lib_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_stringified = format!("{error_occurence_lib_stringified}::{vec_display_foreign_type_to_string_snake_case_stringified}::{vec_display_foreign_type_to_string_upper_camel_case}");
                            let error_occurence_lib_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream =
                            error_occurence_lib_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let hashmap_display_foreign_type_display_to_string_snake_case = format!("{hashmap_snake_case}_{display_foreign_type_snake_case_stringified}_{display_snake_case_stringified}_{to_string_snake_case_stringified}");
                            let hashmap_display_foreign_type_display_to_string_snake_case_token_stream =
                            hashmap_display_foreign_type_display_to_string_snake_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_display_to_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let error_occurence_lib_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_stringified = format!(
                                "{error_occurence_lib_stringified}::{hashmap_display_foreign_type_display_to_string_snake_case}::{}{display_foreign_type_upper_camel_case}{display_upper_camel_case_stringified}{to_string_upper_camel_case}",
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let error_occurence_lib_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_token_stream =
                            error_occurence_lib_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let hashmap_display_foreign_type_display_foreign_type_to_string_snake_case = format!("{hashmap_snake_case}_{display_foreign_type_snake_case_stringified}_{display_foreign_type_snake_case_stringified}_{to_string_snake_case_stringified}");
                            let hashmap_display_foreign_type_display_foreign_type_to_string_snake_case_token_stream =
                            hashmap_display_foreign_type_display_foreign_type_to_string_snake_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_display_foreign_type_to_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_stringified = format!(
                                "{error_occurence_lib_stringified}::{hashmap_display_foreign_type_display_foreign_type_to_string_snake_case}::{}{display_foreign_type_upper_camel_case}{display_foreign_type_upper_camel_case}{to_string_upper_camel_case}", 
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_token_stream =
                            error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let as_std_collections_hashmap_key_type_stringified = format!(
                                "as {std_stringified}::collections::{} key type",
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let str_stringified = "str";
                            let string_string_stringified: std::string::String = format!(
                                "{string_snake_case_stringified}::{}",
                                <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let std_string_string_stringified = format!("{std_stringified}::{string_string_stringified}");
                            let must_be_used_with_stringified = "must be used with";
                            let does_not_support_stringified = "does not support";
                            let type_upper_camel_case = "Type";
                            let hashmap_key_type_stringified = format!(
                                "{}{}{type_upper_camel_case}",
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::Key as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let hashmap_value_type_stringified = format!(
                                "{}{}{type_upper_camel_case}",
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::Value as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let hashmap_key_type_path_stringified = format!(
                                "{hashmap_key_type_stringified}::{}",
                                <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let hashmap_key_type_reference_stringified = format!(
                                "{hashmap_key_type_stringified}::{}",
                                <naming_constants::Reference as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let hashmap_value_type_path_stringified = format!(
                                "{hashmap_value_type_stringified}::{}",
                                <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let hashmap_value_type_reference_stringified = format!(
                                "{hashmap_value_type_stringified}::{}",
                                <naming_constants::Reference as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let inform_use_str_string_in_different_attribute = |
                                path: std::string::String,
                                wrong_attribute: &String,
                                attribute_to_use: &String
                            | {
                                let wrong_attribute_view = proc_macro_helpers::error_occurence::named_attribute::attribute_view(wrong_attribute);
                                let attribute_to_use_view = proc_macro_helpers::error_occurence::named_attribute::attribute_view(attribute_to_use);
                                //maybe additional cases exists
                                if path == str_stringified {
                                    panic!("{proc_macro_name_ident_stringified} {wrong_attribute_view} {str_stringified} {must_be_used_with_stringified} {attribute_to_use_view}");
                                }
                                else if path == std_string_string_stringified {
                                    panic!("{proc_macro_name_ident_stringified} {wrong_attribute_view} {std_string_string_stringified} {must_be_used_with_stringified} {attribute_to_use_view}");
                                }
                                else if path == string_string_stringified {
                                    panic!("{proc_macro_name_ident_stringified} {wrong_attribute_view} {string_string_stringified} {must_be_used_with_stringified} {attribute_to_use_view}");
                                }
                                else if path == <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified() {
                                    panic!(
                                        "{proc_macro_name_ident_stringified} {wrong_attribute_view} {} {must_be_used_with_stringified} {attribute_to_use_view}",
                                        <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                                    );
                                }
                                else {
                                    //clippy lint forces to add empty else
                                }
                            };
                            let vec_display_into_vec_string_upper_camel_case: std::string::String = format!(
                                "{}{display_upper_camel_case_stringified}{into_upper_camel_case_stringified}{}{}",
                                <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let vec_display_into_vec_string_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&vec_display_into_vec_string_upper_camel_case);
                            let vec_display_into_vec_string_token_stream = vec_display_into_vec_string_snake_case_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {vec_display_into_vec_string_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let hashmap_display_display_into_hashmap_string_string_upper_camel_case = format!(
                                "{}{display_upper_camel_case_stringified}{display_upper_camel_case_stringified}{into_upper_camel_case_stringified}{}{}{}",
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let hashmap_display_display_into_hashmap_string_string_snake_case = format!("{hashmap_snake_case}_{display_snake_case_stringified}_{display_snake_case_stringified}_{into_snake_case_stringified}_{hashmap_snake_case}_{string_snake_case_stringified}_{string_snake_case_stringified}");
                            let hashmap_display_display_into_hashmap_string_string_token_stream = hashmap_display_display_into_hashmap_string_string_snake_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_display_into_hashmap_string_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let error_occurence_lib_hashmap_display_display_into_hashmap_string_string_hash_map_display_display_into_hashmap_string_string_stringified = format!("{error_occurence_lib_stringified}::{hashmap_display_display_into_hashmap_string_string_snake_case}::{hashmap_display_display_into_hashmap_string_string_upper_camel_case}");
                            let error_occurence_lib_hashmap_display_display_into_hashmap_string_string_hash_map_display_display_into_hashmap_string_string_token_stream =
                            error_occurence_lib_hashmap_display_display_into_hashmap_string_string_hash_map_display_display_into_hashmap_string_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_display_into_hashmap_string_string_hash_map_display_display_into_hashmap_string_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let hashmap_display_display_foreign_type_into_hashmap_string_string_upper_camel_case = format!(
                                "{}{display_upper_camel_case_stringified}{display_foreign_type_upper_camel_case}{into_upper_camel_case_stringified}{}{}{}",
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let hashmap_display_display_foreign_type_into_hashmap_string_string_snake_case = format!("{hashmap_snake_case}_{display_snake_case_stringified}_{display_foreign_type_snake_case_stringified}_{into_snake_case_stringified}_{hashmap_snake_case}_{string_snake_case_stringified}_{string_snake_case_stringified}");
                            let hashmap_display_display_foreign_type_into_hashmap_string_string_token_stream = hashmap_display_display_foreign_type_into_hashmap_string_string_snake_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_display_foreign_type_into_hashmap_string_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let crate_common_error_logs_logs_hashmap_display_display_foreign_type_into_hashmap_string_string_hashmap_display_display_foreign_type_into_hashmap_string_string_stringified = format!("{error_occurence_lib_stringified}::{hashmap_display_display_foreign_type_into_hashmap_string_string_snake_case}::{hashmap_display_display_foreign_type_into_hashmap_string_string_upper_camel_case}");
                            let crate_common_error_logs_logs_hashmap_display_display_foreign_type_into_hashmap_string_string_hashmap_display_display_foreign_type_into_hashmap_string_string_token_stream = crate_common_error_logs_logs_hashmap_display_display_foreign_type_into_hashmap_string_string_hashmap_display_display_foreign_type_into_hashmap_string_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logs_hashmap_display_display_foreign_type_into_hashmap_string_string_hashmap_display_display_foreign_type_into_hashmap_string_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let hashmap_display_display_into_hashmap_display_string_upper_camel_case = format!(
                                "{}{display_upper_camel_case_stringified}{display_upper_camel_case_stringified}{into_upper_camel_case_stringified}{}{display_upper_camel_case_stringified}{}",
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let hashmap_display_display_into_hashmap_display_string_snake_case = format!("{hashmap_snake_case}_{display_snake_case_stringified}_{display_snake_case_stringified}_{into_snake_case_stringified}_{hashmap_snake_case}_{display_snake_case_stringified}_{string_snake_case_stringified}");
                            let error_occurence_lib_hashmap_display_display_into_hashmap_display_string_hashmap_display_display_into_hashmap_display_string_stringified = format!("{error_occurence_lib_stringified}::{hashmap_display_display_into_hashmap_display_string_snake_case}::{hashmap_display_display_into_hashmap_display_string_upper_camel_case}");
                            let error_occurence_lib_hashmap_display_display_into_hashmap_display_string_hashmap_display_display_into_hashmap_display_string_token_stream = error_occurence_lib_hashmap_display_display_into_hashmap_display_string_hashmap_display_display_into_hashmap_display_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_display_into_hashmap_display_string_hashmap_display_display_into_hashmap_display_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let hashmap_display_display_into_hashmap_display_string_token_stream =
                            hashmap_display_display_into_hashmap_display_string_snake_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_display_into_hashmap_display_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let hashmap_display_foreign_type_display_into_hashmap_string_string_upper_camel_case = format!(
                                "{}{display_foreign_type_upper_camel_case}{display_upper_camel_case_stringified}{into_upper_camel_case_stringified}{}{}{}",
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified(),
                                <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                            );
                            let hashmap_display_foreign_type_display_into_hashmap_string_string_snake_case = format!("{hashmap_snake_case}_{display_foreign_type_snake_case_stringified}_{display_snake_case_stringified}_{into_snake_case_stringified}_{hashmap_snake_case}_{string_snake_case_stringified}_{string_snake_case_stringified}");
                            let error_occurence_lib_hashmap_display_foreign_type_display_into_hashmap_string_string_hashmap_display_foreign_type_display_into_hashmap_string_string_stringified = format!("{error_occurence_lib_stringified}::{hashmap_display_foreign_type_display_into_hashmap_string_string_snake_case}::{hashmap_display_foreign_type_display_into_hashmap_string_string_upper_camel_case}");
                            let error_occurence_lib_hashmap_display_foreign_type_display_into_hashmap_string_string_hashmap_display_foreign_type_display_into_hashmap_string_string_token_stream = error_occurence_lib_hashmap_display_foreign_type_display_into_hashmap_string_string_hashmap_display_foreign_type_display_into_hashmap_string_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_foreign_type_display_into_hashmap_string_string_hashmap_display_foreign_type_display_into_hashmap_string_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                            let hashmap_display_foreign_type_display_into_hashmap_string_string_token_stream = hashmap_display_foreign_type_display_into_hashmap_string_string_snake_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_display_into_hashmap_string_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));

                            //
                            let (
                                logic_for_source_to_string_with_config_for_attribute,
                                logic_for_source_to_string_without_config_for_attribute,
                                logic_for_source_with_serialize_deserialize_for_attribute,
                                logic_for_into_serialize_deserialize_version_for_attribute,
                                field_type_with_serialize_deserialize_token_stream,
                                _serde_borrow_attribute_token_stream,
                                enum_fields_logic_for_compile_time_check_error_occurence_members_used_unused,
                                logic_for_compile_time_check_error_occurence_members_for_attribute
                            ) = match attribute {
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplay => {
                                    if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Path { path, vec_lifetime: _vec_lifetime } = supported_container {
                                        inform_use_str_string_in_different_attribute(
                                            path,
                                            &attribute.to_string(),
                                            &attribute_display_with_serialize_deserialize_stringified
                                        );
                                        (
                                            quote::quote! {
                                                #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                    &format!(
                                                        #field_name_with_field_value_token_stream,
                                                        #field_ident
                                                    )
                                                )
                                            },
                                            quote::quote! {
                                                #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                    &format!(
                                                        #field_name_with_field_value_token_stream,
                                                        #field_ident
                                                    )
                                                )
                                            },
                                            quote::quote! {
                                                #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                    &format!(
                                                        #field_name_with_field_value_token_stream,
                                                        #field_ident
                                                    )
                                                )
                                            },
                                            quote::quote! {
                                                {
                                                    #field_ident.#to_string_token_stream()
                                                }
                                            },
                                            quote::quote! {
                                                #std_string_string_token_stream
                                            },
                                            proc_macro2::TokenStream::new(),
                                            quote::quote! {
                                                #field_ident: #unused_argument_handle_token_stream
                                            },
                                            proc_macro2::TokenStream::new(),
                                        )
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {} {}{}", 
                                            attribute.attribute_view_stringified(),
                                            naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                            naming_constants::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT,
                                            <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified()
                                        )
                                    }
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayWithSerializeDeserialize => {
                                    match supported_container {
                                        proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Path { path, vec_lifetime } => {
                                            let (type_token_stream, serde_borrow_token_stream) = (
                                                {
                                                    let type_stringified = format!("{path}{}", proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    vec_lifetime,
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                )
                                            );
                                            (
                                                quote::quote! {
                                                    #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                        &format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                    )
                                                },
                                                quote::quote! {
                                                    #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                        &format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                    )
                                                },
                                                quote::quote! {
                                                    #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                        &format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                    )
                                                },
                                                quote::quote! {
                                                    {
                                                        #field_ident
                                                    }
                                                },
                                                type_token_stream,
                                                serde_borrow_token_stream,
                                                quote::quote! {
                                                    #field_ident: #unused_argument_handle_token_stream
                                                },
                                                proc_macro2::TokenStream::new(),
                                            )
                                        },
                                        proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Reference{ reference_ident, lifetime_ident } => {
                                            proc_macro_helpers::error_occurence::panic_if_not_str::panic_if_not_str(
                                                &reference_ident,
                                                str_stringified,
                                                &proc_macro_name_ident_stringified,
                                                naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                &attribute
                                            );
                                            proc_macro_helpers::error_occurence::possible_lifetime_addition::possible_lifetime_addition(
                                                lifetime_ident.to_string(),
                                                &mut lifetimes_for_serialize_deserialize
                                            );
                                            (
                                                quote::quote! {
                                                    #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                        &format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                    )
                                                },
                                                quote::quote! {
                                                    #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                        &format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                    )
                                                },
                                                quote::quote! {
                                                    #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                        &format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                    )
                                                },
                                                quote::quote! {
                                                    {
                                                        #field_ident.to_string()
                                                    }
                                                },
                                                quote::quote!{#std_string_string_token_stream},
                                                quote::quote!{#[serde(borrow)]},
                                                quote::quote! {
                                                    #field_ident: #unused_argument_handle_token_stream
                                                },
                                                proc_macro2::TokenStream::new(),
                                            )
                                        },
                                        _ => panic!(
                                            "{proc_macro_name_ident_stringified} {} only supports {}{} and {}{}", 
                                            attribute.attribute_view_stringified(),
                                            naming_constants::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT,
                                            <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified(),
                                            naming_constants::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT,
                                            <naming_constants::Reference as naming_constants::Naming>::upper_camel_case_stringified()
                                        ),
                                    }
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayForeignType => {
                                    if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Path { path: _path, vec_lifetime: _vec_lifetime } = supported_container {}
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    }
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_display_foreign_type_token_stream::#display_foreign_type_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_display_foreign_type_token_stream::#display_foreign_type_snake_case_token_stream(
                                                         #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_display_foreign_type_token_stream::#display_foreign_type_snake_case_token_stream(
                                                &#field_ident
                                            )
                                        },
                                        quote::quote! {
                                            #std_string_string_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoDisplayForeignTypeWithSerializeDeserialize => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Path { path, vec_lifetime } = supported_container {
                                        (
                                            {
                                                let type_stringified = format!("{path}{}", proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            },
                                            get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                vec_lifetime,
                                                &mut lifetimes_for_serialize_deserialize,
                                                &trait_lifetime_stringified,
                                                &proc_macro_name_ident_stringified
                                            )
                                        )
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_display_foreign_type_token_stream::#display_foreign_type_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_display_foreign_type_token_stream::#display_foreign_type_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_display_foreign_type_token_stream::#display_foreign_type_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #field_ident
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoErrorOccurence => {
                                    if !should_generate_impl_compile_time_check_error_occurence_members {
                                        should_generate_impl_compile_time_check_error_occurence_members = true;
                                    }
                                    let (type_token_stream, serde_borrow_token_stream) = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Path { path, vec_lifetime } = supported_container {
                                        (
                                            {
                                                let type_stringified = format!("{path}{with_serialize_deserialize_upper_camel_case}");
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            },
                                            get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                vec_lifetime,
                                                &mut lifetimes_for_serialize_deserialize,
                                                &trait_lifetime_stringified,
                                                &proc_macro_name_ident_stringified
                                            )
                                        )
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::Path as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_to_string_with_config_to_string_with_config_token_stream::#to_string_with_config_token_stream(
                                                        #field_ident,
                                                        config
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_to_string_without_config_to_string_without_config_token_stream::#to_string_without_config_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_to_string_without_config_with_serialize_deserialize_token_stream::#with_serialize_deserialize_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident.#into_serialize_deserialize_version_token_stream()
                                            }
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_error_occurence_named_error_occurence_named_token_stream::#error_occurence_named_token_stream(
                                                #field_ident
                                            )
                                        },
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecDisplay => {
                                    let type_token_stream = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Vec {
                                        path,
                                        vec_element_type
                                    } = supported_container {
                                        if let proc_macro_helpers::error_occurence::vec_element_type::VecElementType::Path { element_path, vec_lifetime: _vec_lifetime } = vec_element_type {
                                            inform_use_str_string_in_different_attribute(
                                                element_path,
                                                &attribute.to_string(),
                                                &attribute_vec_display_with_serialize_deserialize_stringified
                                            );
                                            let type_stringified = format!("{path}<{std_string_string_stringified}>");
                                            type_stringified
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                        }
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}", attribute.attribute_view_stringified(), naming_constants::SUPPORTS_ONLY_STRINGIFIED);
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    let error_occurence_lib_vec_display_into_vec_string_vec_display_into_vec_string_stringified = format!("{error_occurence_lib_stringified}::{vec_display_into_vec_string_snake_case_stringified}::{vec_display_into_vec_string_upper_camel_case}");
                                    let error_occurence_lib_vec_display_into_vec_string_vec_display_into_vec_string_token_stream = error_occurence_lib_vec_display_into_vec_string_vec_display_into_vec_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_vec_display_into_vec_string_vec_display_into_vec_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_display_to_string_vec_display_to_string_token_stream::#vec_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_display_to_string_vec_display_to_string_token_stream::#vec_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_display_to_string_vec_display_to_string_token_stream::#vec_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_vec_display_into_vec_string_vec_display_into_vec_string_token_stream::#vec_display_into_vec_string_token_stream(
                                                #field_ident
                                            )
                                        },
                                        type_token_stream,
                                        proc_macro2::TokenStream::new(),
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecDisplayWithSerializeDeserialize => {
                                    let (
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic
                                    ) = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Vec {
                                        path,
                                        vec_element_type
                                    } = supported_container {
                                        match vec_element_type {
                                            proc_macro_helpers::error_occurence::vec_element_type::VecElementType::Path { element_path, vec_lifetime } => (
                                                {
                                                    let type_stringified = format!("{path}<{element_path}{}>", proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    vec_lifetime,
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                ),
                                                quote::quote! {
                                                    {
                                                        #field_ident
                                                    }
                                                }
                                            ),
                                            proc_macro_helpers::error_occurence::vec_element_type::VecElementType::Reference { reference_ident, lifetime_ident } => {
                                                proc_macro_helpers::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                let error_occurence_lib_vec_display_into_vec_string_vec_display_into_vec_string_stringified = format!("{error_occurence_lib_stringified}::{vec_display_into_vec_string_snake_case_stringified}::{vec_display_into_vec_string_upper_camel_case}");
                                                let error_occurence_lib_vec_display_into_vec_string_vec_display_into_vec_string_token_stream =
                                                error_occurence_lib_vec_display_into_vec_string_vec_display_into_vec_string_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_vec_display_into_vec_string_vec_display_into_vec_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                                (
                                                    {
                                                        let type_stringified = format!("{path}<{std_string_string_stringified}>");
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                    },
                                                    {
                                                        proc_macro_helpers::error_occurence::possible_lifetime_addition::possible_lifetime_addition(
                                                            lifetime_ident.to_string(),
                                                            &mut lifetimes_for_serialize_deserialize,
                                                        );
                                                        quote::quote!{#[serde(borrow)]}
                                                    },
                                                    quote::quote! {
                                                        #error_occurence_lib_vec_display_into_vec_string_vec_display_into_vec_string_token_stream::#vec_display_into_vec_string_token_stream(
                                                            #field_ident
                                                        )
                                                    }
                                                )
                                            },
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_display_to_string_vec_display_to_string_token_stream::#vec_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_display_to_string_vec_display_to_string_token_stream::#vec_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_display_to_string_vec_display_to_string_token_stream::#vec_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        into_serialize_deserialize_logic,
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecDisplayForeignType => {
                                    if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Vec {
                                        path: _path,
                                        vec_element_type
                                    } = supported_container {
                                        if let proc_macro_helpers::error_occurence::vec_element_type::VecElementType::Path { element_path: _element_path, vec_lifetime: _vec_lifetime } = vec_element_type {}
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}", attribute.attribute_view_stringified(), naming_constants::SUPPORTS_ONLY_STRINGIFIED);
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    }
                                    let vec_display_foreign_type_into_vec_string_upper_camel_case = format!(
                                        "{}{display_foreign_type_upper_camel_case}{into_upper_camel_case_stringified}{}{}",
                                        <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified(),
                                        <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified(),
                                        <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                                    );
                                    let vec_display_foreign_type_into_vec_string_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&vec_display_foreign_type_into_vec_string_upper_camel_case);
                                    let vec_display_foreign_type_into_vec_string_snake_case_token_stream =
                                    vec_display_foreign_type_into_vec_string_snake_case_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {vec_display_foreign_type_into_vec_string_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let error_occurence_lib_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_stringified = format!("{error_occurence_lib_stringified}::{vec_display_foreign_type_into_vec_string_snake_case_stringified}::{vec_display_foreign_type_into_vec_string_upper_camel_case}");
                                    let error_occurence_lib_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_token_stream =
                                    error_occurence_lib_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream::#vec_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream::#vec_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_display_to_string_vec_display_to_string_token_stream::#vec_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_token_stream::#vec_display_foreign_type_into_vec_string_snake_case_token_stream(
                                                #field_ident
                                            )
                                        },
                                        quote::quote! {
                                            std::vec::Vec<#std_string_string_token_stream>
                                        },
                                        proc_macro2::TokenStream::new(),
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecDisplayForeignTypeWithSerializeDeserialize => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Vec {
                                        path,
                                        vec_element_type
                                    } = supported_container {
                                        if let proc_macro_helpers::error_occurence::vec_element_type::VecElementType::Path { element_path, vec_lifetime } = vec_element_type {
                                            (
                                                {
                                                    let type_stringified = format!("{path}<{element_path}{}>", proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    vec_lifetime,
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                )
                                            )
                                        }
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}", attribute.attribute_view_stringified(), naming_constants::SUPPORTS_ONLY_STRINGIFIED);
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream::#vec_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream::#vec_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream::#vec_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #field_ident
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoVecErrorOccurence => {
                                    if !should_generate_impl_compile_time_check_error_occurence_members {
                                        should_generate_impl_compile_time_check_error_occurence_members = true;
                                    }
                                    let (type_token_stream, serde_borrow_token_stream) = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::Vec {
                                        path,
                                        vec_element_type
                                    } = supported_container {
                                        if let proc_macro_helpers::error_occurence::vec_element_type::VecElementType::Path { element_path, vec_lifetime } = vec_element_type  {
                                            (
                                                {
                                                    let type_stringified = format!("{path}<{element_path}{with_serialize_deserialize_upper_camel_case}{}>", proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    vec_lifetime,
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                )
                                            )
                                        }
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}", attribute.attribute_view_stringified(), naming_constants::SUPPORTS_ONLY_STRINGIFIED);
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    let vec_to_string_without_config_to_string_upper_camel_case = format!(
                                        "{}{to_string_without_config_upper_camel_case}{to_string_upper_camel_case}",
                                        <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified()
                                    );
                                    let vec_to_string_without_config_to_string_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&vec_to_string_without_config_to_string_upper_camel_case);
                                    let vec_to_string_without_config_to_string_snake_case_token_stream =
                                    vec_to_string_without_config_to_string_snake_case_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {vec_to_string_without_config_to_string_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let error_occurence_lib_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_stringified = format!("{error_occurence_lib_stringified}::{vec_to_string_without_config_to_string_snake_case_stringified}::{vec_to_string_without_config_to_string_upper_camel_case}");
                                    let error_occurence_lib_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_token_stream =
                                    error_occurence_lib_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let with_serialize_deserialize_upper_camel_case = format!(
                                        "{}{to_string_without_config_upper_camel_case}{to_string_upper_camel_case}{with_serialize_deserialize_upper_camel_case}",
                                        <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified()
                                    );
                                    let with_serialize_deserialize_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&with_serialize_deserialize_upper_camel_case);
                                    let with_serialize_deserialize_snake_case_token_stream =
                                    with_serialize_deserialize_snake_case_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {with_serialize_deserialize_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let error_occurence_lib_with_serialize_deserialize_with_serialize_deserialize_stringified = format!("{error_occurence_lib_stringified}::{vec_to_string_without_config_to_string_snake_case_stringified}::{with_serialize_deserialize_upper_camel_case}");
                                    let error_occurence_lib_with_serialize_deserialize_with_serialize_deserialize_token_stream =
                                    error_occurence_lib_with_serialize_deserialize_with_serialize_deserialize_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_with_serialize_deserialize_with_serialize_deserialize_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let vec_to_string_with_config_to_string_upper_camel_case = format!(
                                        "{}{to_string_with_config_upper_camel_case}{to_string_upper_camel_case}",
                                        <naming_constants::Vec as naming_constants::Naming>::upper_camel_case_stringified()
                                    );
                                    let vec_to_string_with_config_to_string_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&vec_to_string_with_config_to_string_upper_camel_case);
                                    let error_occurence_lib_vec_to_string_with_config_to_string_vec_to_string_with_config_to_string_stringified = format!("{error_occurence_lib_stringified}::{vec_to_string_with_config_to_string_snake_case_stringified}::{vec_to_string_with_config_to_string_upper_camel_case}");
                                    let error_occurence_lib_vec_to_string_with_config_to_string_vec_to_string_with_config_to_string_token_stream = error_occurence_lib_vec_to_string_with_config_to_string_vec_to_string_with_config_to_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_vec_to_string_with_config_to_string_vec_to_string_with_config_to_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let vec_to_string_with_config_to_string_token_stream = vec_to_string_with_config_to_string_snake_case_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {vec_to_string_with_config_to_string_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_to_string_with_config_to_string_vec_to_string_with_config_to_string_token_stream::#vec_to_string_with_config_to_string_token_stream(
                                                        #field_ident,
                                                        config
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_token_stream::#vec_to_string_without_config_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_with_serialize_deserialize_with_serialize_deserialize_token_stream::#with_serialize_deserialize_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident.into_iter()
                                                .map(|i| i.#into_serialize_deserialize_version_token_stream())
                                                .collect()
                                            }
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident
                                        },
                                        quote::quote! {
                                            #field_ident.iter().for_each(|i|
                                                #error_occurence_lib_error_occurence_unnamed_error_occurence_unnamed_token_stream::#error_occurence_unnamed_token_stream(
                                                    i
                                                )
                                            );
                                        },
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay => {
                                    let (
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic
                                    ) = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type,
                                    } = supported_container {
                                        let hashmap_key_type_path_case = |
                                            key_segments_stringified: std::string::String,
                                            key_vec_lifetime: Vec<proc_macro_helpers::error_occurence::lifetime::Lifetime>,
                                            lifetimes_for_serialize_deserialize: &mut Vec<String>
                                        | -> (
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream
                                        ) {
                                            proc_macro_helpers::error_occurence::panic_if_not_string::panic_if_not_string(
                                                &key_segments_stringified,
                                                &std_string_string_stringified,
                                                &proc_macro_name_ident_stringified,
                                                naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                &as_std_collections_hashmap_key_type_stringified,
                                                &attribute
                                            );
                                            (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{key_segments_stringified}{}, {std_string_string_stringified}>",
                                                        proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime),
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    key_vec_lifetime,
                                                    lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                ),
                                                quote::quote! {
                                                    #error_occurence_lib_hashmap_display_display_into_hashmap_display_string_hashmap_display_display_into_hashmap_display_string_token_stream::#hashmap_display_display_into_hashmap_display_string_token_stream(
                                                        #field_ident
                                                    )
                                                }
                                            )
                                        };
                                        let hashmap_key_type_reference_case = |
                                            key_lifetime_ident: proc_macro2::Ident,
                                            lifetimes_for_serialize_deserialize: &mut Vec<String>
                                        | -> (
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream
                                        ) {
                                            (
                                                {
                                                    let type_stringified = format!("{path}<{std_string_string_stringified}, {std_string_string_stringified}>");
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                },
                                                {
                                                    proc_macro_helpers::error_occurence::possible_lifetime_addition::possible_lifetime_addition(
                                                        key_lifetime_ident.to_string(),
                                                        lifetimes_for_serialize_deserialize
                                                    );
                                                    quote::quote!{#[serde(borrow)]}
                                                },
                                                quote::quote! {
                                                    #error_occurence_lib_hashmap_display_display_into_hashmap_string_string_hash_map_display_display_into_hashmap_string_string_token_stream::#hashmap_display_display_into_hashmap_string_string_token_stream(
                                                        #field_ident
                                                    )
                                                }
                                            )
                                        };
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified,
                                                    key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => {
                                                inform_use_str_string_in_different_attribute(
                                                    value_segments_stringified,
                                                    &attribute.to_string(),
                                                    &attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified
                                                );
                                                hashmap_key_type_path_case(
                                                    key_segments_stringified,
                                                    key_vec_lifetime,
                                                    &mut lifetimes_for_serialize_deserialize
                                                )
                                            },
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => {
                                                inform_use_str_string_in_different_attribute(
                                                    value_segments_stringified,
                                                    &attribute.to_string(),
                                                    &attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified
                                                );
                                                hashmap_key_type_reference_case(
                                                    key_lifetime_ident,
                                                    &mut lifetimes_for_serialize_deserialize
                                                )
                                            },
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream::#hashmap_display_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream::#hashmap_display_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream::#hashmap_display_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream::#hashmap_display_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        into_serialize_deserialize_logic,
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize => {
                                    let (
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic,
                                    ) = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified,
                                                    key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                proc_macro_helpers::error_occurence::panic_if_not_string::panic_if_not_string(
                                                    &key_segments_stringified,
                                                    &std_string_string_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                    &as_std_collections_hashmap_key_type_stringified,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{key_segments_stringified}{}, {value_segments_stringified}{}>",
                                                            proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime),
                                                            proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                    },
                                                    get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
                                                        key_vec_lifetime,
                                                        value_vec_lifetime,
                                                        &mut lifetimes_for_serialize_deserialize,
                                                            &trait_lifetime_stringified,
                                                            &proc_macro_name_ident_stringified,
                                                    ),
                                                    quote::quote! {
                                                        {
                                                            #field_ident
                                                        }
                                                    }
                                                )
                                            },
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified,
                                                    key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident,
                                                    value_lifetime_ident
                                                }
                                            ) => {
                                                proc_macro_helpers::error_occurence::panic_if_not_string::panic_if_not_string(
                                                    &key_segments_stringified,
                                                    &std_string_string_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                    &as_std_collections_hashmap_key_type_stringified,
                                                    &attribute
                                                );
                                                proc_macro_helpers::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &value_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{key_segments_stringified}{}, {std_string_string_stringified}>",
                                                            proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime)
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                    },
                                                    get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
                                                        key_vec_lifetime,
                                                        vec![proc_macro_helpers::error_occurence::lifetime::Lifetime::Specified(value_lifetime_ident.to_string())],
                                                        &mut lifetimes_for_serialize_deserialize,
                                                            &trait_lifetime_stringified,
                                                            &proc_macro_name_ident_stringified,
                                                    ),
                                                    quote::quote! {
                                                        #error_occurence_lib_hashmap_display_display_into_hashmap_display_string_hashmap_display_display_into_hashmap_display_string_token_stream::#hashmap_display_display_into_hashmap_display_string_token_stream(
                                                            #field_ident
                                                        )
                                                    }
                                                )
                                            },
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident,
                                                    key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                proc_macro_helpers::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &key_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{std_string_string_stringified}, {value_segments_stringified}{}>",
                                                            proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                    },
                                                    {
                                                        proc_macro_helpers::error_occurence::possible_lifetime_addition::possible_lifetime_addition(
                                                            key_lifetime_ident.to_string(),
                                                            &mut lifetimes_for_serialize_deserialize
                                                        );
                                                        quote::quote!{#[serde(borrow)]}
                                                    },
                                                    {
                                                        let hashmap_display_display_into_hashmap_string_display_upper_camel_case = format!(
                                                            "{}{display_upper_camel_case_stringified}{display_upper_camel_case_stringified}{into_upper_camel_case_stringified}{}{}{display_upper_camel_case_stringified}",
                                                            <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                                            <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                                            <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                                                        );
                                                        let hashmap_display_display_into_hashmap_string_display_snake_case = format!("{hashmap_snake_case}_{display_snake_case_stringified}_{display_snake_case_stringified}_{into_snake_case_stringified}_{hashmap_snake_case}_{string_snake_case_stringified}_{display_snake_case_stringified}");
                                                        let hashmap_display_display_into_hashmap_string_display_token_stream =
                                                        hashmap_display_display_into_hashmap_string_display_snake_case
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_display_into_hashmap_string_display_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                                        let error_occurence_lib_hashmap_display_display_into_hashmap_string_display_hashmap_display_display_into_hashmap_string_display_stringified =
                                                        format!("{error_occurence_lib_stringified}::{hashmap_display_display_into_hashmap_string_display_snake_case}::{hashmap_display_display_into_hashmap_string_display_upper_camel_case}");
                                                        let error_occurence_lib_hashmap_display_display_into_hashmap_string_display_hashmap_display_display_into_hashmap_string_display_token_stream =
                                                        error_occurence_lib_hashmap_display_display_into_hashmap_string_display_hashmap_display_display_into_hashmap_string_display_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_display_into_hashmap_string_display_hashmap_display_display_into_hashmap_string_display_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                                        quote::quote! {
                                                            #error_occurence_lib_hashmap_display_display_into_hashmap_string_display_hashmap_display_display_into_hashmap_string_display_token_stream::#hashmap_display_display_into_hashmap_string_display_token_stream(
                                                                #field_ident
                                                            )
                                                        }
                                                    }
                                                )
                                            },
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident,
                                                    key_lifetime_ident: _,
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident,
                                                    value_lifetime_ident: _,
                                                }
                                            ) => {
                                                proc_macro_helpers::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &key_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                proc_macro_helpers::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &value_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!("{path}<{std_string_string_stringified}, {std_string_string_stringified}>");
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                    },
                                                    {
                                                        // get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
                                                        //     vec![proc_macro_helpers::error_occurence::lifetime::Lifetime::Specified(key_lifetime_ident.to_string())],
                                                        //     vec![proc_macro_helpers::error_occurence::lifetime::Lifetime::Specified(value_lifetime_ident.to_string())],
                                                        //     &mut lifetimes_for_serialize_deserialize,
                                                        //         &trait_lifetime_stringified,
                                                        //         &proc_macro_name_ident_stringified,
                                                        // );
                                                        quote::quote!{#[serde(borrow)]}
                                                    },
                                                    quote::quote! {
                                                        #error_occurence_lib_hashmap_display_display_into_hashmap_string_string_hash_map_display_display_into_hashmap_string_string_token_stream::#hashmap_display_display_into_hashmap_string_string_token_stream(
                                                            #field_ident
                                                        )
                                                    }
                                                )
                                            },
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    (
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream::#hashmap_display_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream::#hashmap_display_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream::#hashmap_display_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        into_serialize_deserialize_logic,
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType => {
                                    let (
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic,
                                    ) = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        let hashmap_key_type_path_case = |
                                            key_segments_stringified: std::string::String,
                                            key_vec_lifetime: Vec<proc_macro_helpers::error_occurence::lifetime::Lifetime>,
                                            lifetimes_for_serialize_deserialize: &mut Vec<String>
                                        | -> (
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream
                                        ) {
                                            let hashmap_display_display_foreign_type_into_hashmap_display_string_snake_case_token_stream =
                                            hashmap_display_display_foreign_type_into_hashmap_display_string_snake_case
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_display_foreign_type_into_hashmap_display_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                            let error_occurence_lib_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_stringified = format!(
                                                "{error_occurence_lib_stringified}::{hashmap_display_display_foreign_type_into_hashmap_display_string_snake_case}::{}{display_upper_camel_case_stringified}{display_foreign_type_upper_camel_case}{into_upper_camel_case_stringified}{}{display_upper_camel_case_stringified}{}",
                                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                                <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                                <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                                            );
                                            let error_occurence_lib_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_token_stream =
                                            error_occurence_lib_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_stringified
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                            (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{key_segments_stringified}{},{std_string_string_stringified}>",
                                                        proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    key_vec_lifetime,
                                                    lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                ),
                                                quote::quote! {
                                                    #error_occurence_lib_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_token_stream::#hashmap_display_display_foreign_type_into_hashmap_display_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                }
                                            )
                                        };
                                        let hashmap_key_type_reference_case = |
                                            _key_reference_ident: proc_macro2::Ident,
                                            key_lifetime_ident: proc_macro2::Ident,
                                            lifetimes_for_serialize_deserialize: &mut Vec<String>
                                        | -> (
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream
                                        ) {
                                            (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{std_string_string_stringified},{std_string_string_stringified}>"
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                },
                                                {
                                                    proc_macro_helpers::error_occurence::possible_lifetime_addition::possible_lifetime_addition(
                                                        key_lifetime_ident.to_string(),
                                                        lifetimes_for_serialize_deserialize
                                                    );
                                                    quote::quote!{#[serde(borrow)]}
                                                },
                                                quote::quote! {
                                                    #crate_common_error_logs_logs_hashmap_display_display_foreign_type_into_hashmap_string_string_hashmap_display_display_foreign_type_into_hashmap_string_string_token_stream::#hashmap_display_display_foreign_type_into_hashmap_string_string_token_stream(
                                                        #field_ident
                                                    )
                                                }
                                            )
                                        };
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified,
                                                    key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => hashmap_key_type_path_case(
                                                key_segments_stringified,
                                                key_vec_lifetime,
                                                &mut lifetimes_for_serialize_deserialize
                                            ),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident,
                                                    key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => hashmap_key_type_reference_case(
                                                key_reference_ident,
                                                key_lifetime_ident,
                                                &mut lifetimes_for_serialize_deserialize
                                            ),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    (
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream::#hashmap_display_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream::#hashmap_display_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream::#hashmap_display_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        into_serialize_deserialize_logic,
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize => {
                                    let (
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic
                                    ) = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified,
                                                    key_vec_lifetime
                                                },
                                                proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                proc_macro_helpers::error_occurence::panic_if_not_string::panic_if_not_string(
                                                    &key_segments_stringified,
                                                    &std_string_string_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                    &as_std_collections_hashmap_key_type_stringified,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{key_segments_stringified}{},{value_segments_stringified}{}>",
                                                            proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime),
                                                            proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime),
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                    },
                                                    get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
                                                        key_vec_lifetime,
                                                        value_vec_lifetime,
                                                        &mut lifetimes_for_serialize_deserialize,
                                                            &trait_lifetime_stringified,
                                                            &proc_macro_name_ident_stringified
                                                    ),
                                                    quote::quote! {
                                                        #field_ident
                                                    }
                                                )
                                            },
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident,
                                                    key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                proc_macro_helpers::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &key_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{std_string_string_stringified},{value_segments_stringified}{}>",
                                                            proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime),
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                    },
                                                    {
                                                        proc_macro_helpers::error_occurence::possible_lifetime_addition::possible_lifetime_addition(
                                                            key_lifetime_ident.to_string(),
                                                            &mut lifetimes_for_serialize_deserialize
                                                        );
                                                        quote::quote!{#[serde(borrow)]}
                                                    },
                                                    quote::quote! {
                                                        #crate_common_error_logs_logs_hashmap_display_display_foreign_type_into_hashmap_string_string_hashmap_display_display_foreign_type_into_hashmap_string_string_token_stream::#hashmap_display_display_foreign_type_into_hashmap_string_string_token_stream(
                                                            #field_ident
                                                        )
                                                    }
                                                )
                                            },
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    (
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream::#hashmap_display_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream::#hashmap_display_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream::#hashmap_display_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        into_serialize_deserialize_logic,
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence => {
                                    if !should_generate_impl_compile_time_check_error_occurence_members {
                                        should_generate_impl_compile_time_check_error_occurence_members = true;
                                    }
                                    let (
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic
                                    ) = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified,
                                                    key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                proc_macro_helpers::error_occurence::panic_if_not_string::panic_if_not_string(
                                                    &key_segments_stringified,
                                                    &std_string_string_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                    &as_std_collections_hashmap_key_type_stringified,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{key_segments_stringified}{}, {value_segments_stringified}{with_serialize_deserialize_upper_camel_case}{}>",
                                                            proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&key_vec_lifetime),
                                                            proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                    },
                                                    get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
                                                        key_vec_lifetime,
                                                        value_vec_lifetime,
                                                        &mut lifetimes_for_serialize_deserialize,
                                                        &trait_lifetime_stringified,
                                                        &proc_macro_name_ident_stringified
                                                    ),
                                                    quote::quote! {
                                                        {
                                                            #field_ident.into_iter()
                                                            .map(|(k, v)| (k, { v.#into_serialize_deserialize_version_token_stream() }))
                                                            .collect()
                                                        }
                                                    }
                                                )
                                            },
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident,
                                                    key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                proc_macro_helpers::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &key_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{std_string_string_stringified}, {value_segments_stringified}{with_serialize_deserialize_upper_camel_case}{}>",
                                                            proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                    },
                                                    {
                                                        proc_macro_helpers::error_occurence::possible_lifetime_addition::possible_lifetime_addition(
                                                            key_lifetime_ident.to_string(),
                                                            &mut lifetimes_for_serialize_deserialize
                                                        );
                                                        quote::quote!{#[serde(borrow)]}
                                                    },
                                                    quote::quote! {
                                                        {
                                                            #field_ident.into_iter()
                                                            .map(|(k, v)| (k.to_string(), { v.#into_serialize_deserialize_version_token_stream() }))
                                                            .collect()
                                                        }
                                                    }
                                                )
                                            },
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}",
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    let hashmap_display_to_string_without_config_to_string_snake_case_token_stream =
                                    hashmap_display_to_string_without_config_to_string_snake_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_to_string_without_config_to_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let error_occurence_lib_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_stringified = format!(
                                        "{error_occurence_lib_stringified}::{hashmap_display_to_string_without_config_to_string_snake_case}::{}{display_upper_camel_case_stringified}{to_string_without_config_upper_camel_case}{to_string_upper_camel_case}",
                                        <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                                    );
                                    let error_occurence_lib_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_token_stream =
                                    error_occurence_lib_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let hashmap_display_to_string_with_config_to_string_upper_camel_case = format!(
                                        "{}{display_upper_camel_case_stringified}{to_string_with_config_upper_camel_case}{to_string_upper_camel_case}",
                                        <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                                    );
                                    let hashmap_display_to_string_with_config_to_string_snake_case = format!("{hashmap_snake_case}_{display_snake_case_stringified}_{to_string_with_config_snake_case_stringified}_{to_string_snake_case_stringified}");
                                    let error_occurence_lib_hashmap_display_to_string_with_config_to_string_hashmap_display_to_string_with_config_to_string_stringified = format!("{error_occurence_lib_stringified}::{hashmap_display_to_string_with_config_to_string_snake_case}::{hashmap_display_to_string_with_config_to_string_upper_camel_case}");
                                    let error_occurence_lib_hashmap_display_to_string_with_config_to_string_hashmap_display_to_string_with_config_to_string_token_stream = error_occurence_lib_hashmap_display_to_string_with_config_to_string_hashmap_display_to_string_with_config_to_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_to_string_with_config_to_string_hashmap_display_to_string_with_config_to_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let hashmap_display_to_string_with_config_to_string_token_stream = hashmap_display_to_string_with_config_to_string_snake_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_to_string_with_config_to_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_to_string_with_config_to_string_hashmap_display_to_string_with_config_to_string_token_stream::#hashmap_display_to_string_with_config_to_string_token_stream(
                                                        #field_ident,
                                                        config
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_token_stream::#hashmap_display_to_string_without_config_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_token_stream::#hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        into_serialize_deserialize_logic,
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident
                                        },
                                        quote::quote! {
                                            #field_ident.values().for_each(|v|
                                                #error_occurence_lib_error_occurence_unnamed_error_occurence_unnamed_token_stream::#error_occurence_unnamed_token_stream(
                                                    v
                                                )
                                            );
                                        },
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplay => {
                                    let type_token_stream = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        let hashmap_key_type_path_case = || -> proc_macro2::TokenStream {
                                            let type_stringified = format!("{path}<{std_string_string_stringified},{std_string_string_stringified}>");
                                            type_stringified
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                        };
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => {
                                                inform_use_str_string_in_different_attribute(
                                                    value_segments_stringified,
                                                    &attribute.to_string(),
                                                    &attribute_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_stringified
                                                );
                                                hashmap_key_type_path_case()
                                            },
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                 },
                                                proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view_stringified()),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view_stringified()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}",
                                            <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                            attribute.attribute_view_stringified()
                                        );
                                    };
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_token_stream::#hashmap_display_foreign_type_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_token_stream::#hashmap_display_foreign_type_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream::#hashmap_display_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_hashmap_display_foreign_type_display_into_hashmap_string_string_hashmap_display_foreign_type_display_into_hashmap_string_string_token_stream::#hashmap_display_foreign_type_display_into_hashmap_string_string_token_stream(
                                                #field_ident
                                            )
                                        },
                                        type_token_stream,
                                        proc_macro2::TokenStream::new(),
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize => {
                                    let (
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic
                                    ) = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => {
                                                let hashmap_display_foreign_type_display_into_hashmap_string_display_snake_case = format!("{hashmap_snake_case}_{display_foreign_type_snake_case_stringified}_{display_snake_case_stringified}_{into_snake_case_stringified}_{hashmap_snake_case}_{string_snake_case_stringified}_{display_snake_case_stringified}");
                                                let hashmap_display_foreign_type_display_into_hashmap_string_display_snake_case_token_stream =
                                                hashmap_display_foreign_type_display_into_hashmap_string_display_snake_case
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_display_into_hashmap_string_display_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                                let error_occurence_lib_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_stringified = format!(
                                                    "{error_occurence_lib_stringified}::{hashmap_display_foreign_type_display_into_hashmap_string_display_snake_case}::{}{display_foreign_type_upper_camel_case}{display_upper_camel_case_stringified}{into_upper_camel_case_stringified}{}{}{display_upper_camel_case_stringified}",
                                                    <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                                    <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                                    <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                                                );
                                                let error_occurence_lib_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_token_stream =
                                                error_occurence_lib_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{std_string_string_stringified},{value_segments_stringified}{}>",
                                                            proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                    },
                                                    get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                        value_vec_lifetime,
                                                        &mut lifetimes_for_serialize_deserialize,
                                                        &trait_lifetime_stringified,
                                                        &proc_macro_name_ident_stringified
                                                    ),
                                                    quote::quote! {
                                                        #error_occurence_lib_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_token_stream::#hashmap_display_foreign_type_display_into_hashmap_string_display_snake_case_token_stream(
                                                            #field_ident
                                                        )
                                                    }
                                                )
                                            },
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident,
                                                    value_lifetime_ident
                                                }
                                            ) => {
                                                proc_macro_helpers::error_occurence::panic_if_not_str::panic_if_not_str(
                                                    &value_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{std_string_string_stringified},{std_string_string_stringified}>"
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                    },
                                                    get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                        vec![proc_macro_helpers::error_occurence::lifetime::Lifetime::Specified(value_lifetime_ident.to_string())],
                                                        &mut lifetimes_for_serialize_deserialize,
                                                        &trait_lifetime_stringified,
                                                        &proc_macro_name_ident_stringified
                                                    ),
                                                    quote::quote! {
                                                        #error_occurence_lib_hashmap_display_foreign_type_display_into_hashmap_string_string_hashmap_display_foreign_type_display_into_hashmap_string_string_token_stream::#hashmap_display_foreign_type_display_into_hashmap_string_string_token_stream(
                                                            #field_ident
                                                        )
                                                    }
                                                )
                                            },
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view_stringified()),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_token_stream::#hashmap_display_foreign_type_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_token_stream::#hashmap_display_foreign_type_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream::#hashmap_display_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        into_serialize_deserialize_logic,
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignType => {
                                    let type_token_stream = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => {
                                                let type_stringified = format!("{path}<{std_string_string_stringified},{std_string_string_stringified}>");
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                            },
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime:  _key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view_stringified()),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    let hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_snake_case = format!("{hashmap_snake_case}_{display_foreign_type_snake_case_stringified}_{display_foreign_type_snake_case_stringified}_{into_snake_case_stringified}_{hashmap_snake_case}_{string_snake_case_stringified}_{string_snake_case_stringified}");
                                    let hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_snake_case_token_stream =
                                    hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_snake_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_stringified = format!(
                                        "{error_occurence_lib_stringified}::{hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_snake_case}::{}{display_foreign_type_upper_camel_case}{display_foreign_type_upper_camel_case}{into_upper_camel_case_stringified}{}{}{}",
                                        <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                        <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                        <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified(),
                                        <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                                    );
                                    let error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_token_stream =
                                    error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_token_stream::#hashmap_display_foreign_type_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_token_stream::#hashmap_display_foreign_type_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream::#hashmap_display_display_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_token_stream::#hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_snake_case_token_stream(
                                                #field_ident
                                            )
                                        },
                                        type_token_stream,
                                        proc_macro2::TokenStream::new(),
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => (
                                                {
                                                   let type_stringified = format!(
                                                        "{path}<{std_string_string_stringified},{value_segments_stringified}{}>",
                                                        proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime),
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    value_vec_lifetime,
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                )
                                            ),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view_stringified()),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    let hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_upper_camel_case = format!(
                                        "{}{display_foreign_type_upper_camel_case}{display_foreign_type_upper_camel_case}{into_upper_camel_case_stringified}{}{}{display_foreign_type_upper_camel_case}",
                                        <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                        <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified(),
                                        <naming_constants::String as naming_constants::Naming>::upper_camel_case_stringified()
                                    );
                                    let hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_snake_case = format!("{hashmap_snake_case}_{display_foreign_type_snake_case_stringified}_{display_foreign_type_snake_case_stringified}_{into_snake_case_stringified}_{hashmap_snake_case}_{string_snake_case_stringified}_{display_foreign_type_snake_case_stringified}");
                                    let hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_token_stream = hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_snake_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_hash_map_display_foreign_type_display_foreign_type_into_hash_map_string_display_foreign_type_stringified = format!("{error_occurence_lib_stringified}::{hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_snake_case}::{hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_upper_camel_case}");
                                    let error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_hash_map_display_foreign_type_display_foreign_type_into_hash_map_string_display_foreign_type_token_stream = error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_hash_map_display_foreign_type_display_foreign_type_into_hash_map_string_display_foreign_type_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_hash_map_display_foreign_type_display_foreign_type_into_hash_map_string_display_foreign_type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_token_stream::#hashmap_display_foreign_type_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_token_stream::#hashmap_display_foreign_type_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream::#hashmap_display_display_foreign_type_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                            }
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_hash_map_display_foreign_type_display_foreign_type_into_hash_map_string_display_foreign_type_token_stream::#hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_token_stream(
                                                #field_ident
                                            )
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                proc_macro_helpers::error_occurence::named_attribute::NamedAttribute::EoHashMapKeyDisplayForeignTypeValueErrorOccurence => {
                                    if !should_generate_impl_compile_time_check_error_occurence_members {
                                        should_generate_impl_compile_time_check_error_occurence_members = true;
                                    }
                                    let (type_token_stream, serde_borrow_token_stream) = if let proc_macro_helpers::error_occurence::supported_container::SupportedContainer::HashMap {
                                        path,
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified,
                                                    value_vec_lifetime
                                                }
                                            ) => (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{std_string_string_stringified}, {value_segments_stringified}{with_serialize_deserialize_upper_camel_case}{}>",
                                                        proc_macro_helpers::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&value_vec_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    value_vec_lifetime,
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                )
                                            ),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Path {
                                                    key_segments_stringified: _key_segments_stringified,
                                                    key_vec_lifetime: _key_vec_lifetime
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view_stringified()),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Path {
                                                    value_segments_stringified: _value_segments_stringified,
                                                    value_vec_lifetime: _value_vec_lifetime
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view_stringified()),
                                            (
                                                proc_macro_helpers::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
                                                    key_reference_ident: _key_reference_ident,
                                                    key_lifetime_ident: _key_lifetime_ident
                                                },
                                               proc_macro_helpers::error_occurence::hashmap_key_type::HashMapValueType::Reference {
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_key_type_reference_stringified}", attribute.attribute_view_stringified()),
                                        }
                                    }
                                    else {
                                        panic!(
                                            "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}", 
                                            attribute.attribute_view_stringified(),
                                            <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                                        );
                                    };
                                    let hashmap_display_foreign_type_to_string_without_config_to_string_snake_case = format!("{hashmap_snake_case}_{display_foreign_type_snake_case_stringified}_{to_string_without_config_snake_case_stringified}_{to_string_snake_case_stringified}");
                                    let hashmap_display_foreign_type_to_string_without_config_to_string_snake_case_token_stream =
                                    hashmap_display_foreign_type_to_string_without_config_to_string_snake_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_to_string_without_config_to_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let error_occurence_lib_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_stringified = format!(
                                        "{error_occurence_lib_stringified}::{hashmap_display_foreign_type_to_string_without_config_to_string_snake_case}::{}{display_foreign_type_upper_camel_case}{to_string_without_config_upper_camel_case}{to_string_upper_camel_case}",
                                        <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                                    );
                                    let error_occurence_lib_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_token_stream =
                                    error_occurence_lib_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let hashmap_display_foreign_type_to_string_with_config_to_string_upper_camel_case = format!(
                                        "{}{display_foreign_type_upper_camel_case}{to_string_with_config_upper_camel_case}{to_string_upper_camel_case}",
                                        <naming_constants::HashMap as naming_constants::Naming>::upper_camel_case_stringified()
                                    );
                                    let hashmap_display_foreign_type_to_string_with_config_to_string_snake_case = format!("{hashmap_snake_case}_{display_foreign_type_snake_case_stringified}_{to_string_with_config_snake_case_stringified}_{to_string_snake_case_stringified}");
                                    let error_occurence_lib_hashmap_display_foreign_type_to_string_with_config_to_string_hashmap_display_foreign_type_to_string_with_config_to_string_stringified = format!("{error_occurence_lib_stringified}::{hashmap_display_foreign_type_to_string_with_config_to_string_snake_case}::{hashmap_display_foreign_type_to_string_with_config_to_string_upper_camel_case}");
                                    let error_occurence_lib_hashmap_display_foreign_type_to_string_with_config_to_string_hashmap_display_foreign_type_to_string_with_config_to_string_token_stream = error_occurence_lib_hashmap_display_foreign_type_to_string_with_config_to_string_hashmap_display_foreign_type_to_string_with_config_to_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_hashmap_display_foreign_type_to_string_with_config_to_string_hashmap_display_foreign_type_to_string_with_config_to_string_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    let hashmap_display_foreign_type_to_string_with_config_to_string_token_stream = hashmap_display_foreign_type_to_string_with_config_to_string_snake_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_to_string_with_config_to_string_snake_case} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                                    (
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_foreign_type_to_string_with_config_to_string_hashmap_display_foreign_type_to_string_with_config_to_string_token_stream::#hashmap_display_foreign_type_to_string_with_config_to_string_token_stream(
                                                        #field_ident,
                                                        config
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_token_stream::#hashmap_display_foreign_type_to_string_without_config_to_string_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            #error_occurence_lib_lines_space_backslash_lines_space_backslash_token_stream::#lines_space_backslash_snake_case_token_stream(
                                                &format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #error_occurence_lib_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_token_stream::#hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_snake_case_token_stream(
                                                        #field_ident
                                                    )
                                                )
                                            )
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident.into_iter()
                                                .map(|(k, v)| {
                                                    (
                                                        #error_occurence_lib_display_foreign_type_token_stream::#display_foreign_type_snake_case_token_stream(k),
                                                        { v.#into_serialize_deserialize_version_token_stream() },
                                                    )
                                                })
                                                .collect()
                                            }
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident
                                        },
                                        quote::quote! {
                                            #field_ident.values().for_each(|v|
                                                #error_occurence_lib_error_occurence_unnamed_error_occurence_unnamed_token_stream::#error_occurence_unnamed_token_stream(
                                                    v
                                                )
                                            );
                                        },
                                    )
                                },
                            };
                            enum_fields_logic_for_source_to_string_with_config.push(quote::quote! {
                                #field_ident
                            });
                            enum_fields_logic_for_source_to_string_without_config.push(quote::quote! {
                                #field_ident
                            });
                            enum_fields_logic_for_get_code_occurence.push(quote::quote!{
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            // enum_fields_logic_for_enum_with_serialize_deserialize.push(quote::quote!{
                            //     #field_ident: #field_type_with_serialize_deserialize_token_stream
                            // });
                            enum_fields_logic_for_source_with_serialize_deserialize.push(quote::quote!{
                                #field_ident
                            });
                            enum_fields_logic_for_get_code_occurence_with_serialize_deserialize.push(quote::quote!{
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_into_serialize_deserialize_version.push(quote::quote!{
                                #field_ident
                            });
                            enum_fields_logic_for_compile_time_check_error_occurence_members.push(quote::quote!{
                                #enum_fields_logic_for_compile_time_check_error_occurence_members_used_unused
                            });
                            format_logic_for_source_to_string_with_or_without_config.push("{}");
                            fields_logic_for_source_to_string_with_config_for_attribute.push(logic_for_source_to_string_with_config_for_attribute);
                            fields_logic_for_source_to_string_without_config_for_attribute.push(logic_for_source_to_string_without_config_for_attribute);
                            fields_logic_for_source_with_serialize_deserialize_for_attribute.push(logic_for_source_with_serialize_deserialize_for_attribute);
                            fields_logic_for_into_serialize_deserialize_version_for_attribute.push(quote::quote!{
                                #field_ident: #logic_for_into_serialize_deserialize_version_for_attribute
                            });
                            fields_logic_for_compile_time_check_error_occurence_members_for_attribute.push(quote::quote!{
                                #logic_for_compile_time_check_error_occurence_members_for_attribute;
                            });
                        },
                        proc_macro_helpers::error_occurence::error_field_or_code_occurence::ErrorFieldOrCodeOccurence::CodeOccurence {
                            field_type,
                            vec_lifetime: _vec_lifetime,
                         } => {
                            let code_occurence_type_with_serialize_deserialize_token_stream = {
                                let code_occurence_type_with_serialize_deserialize_stringified =
                                format!("{field_type}{with_serialize_deserialize_upper_camel_case}");
                                code_occurence_type_with_serialize_deserialize_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {code_occurence_type_with_serialize_deserialize_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                            };
                            enum_fields_logic_for_source_to_string_with_config.push(quote::quote! {
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_source_to_string_without_config.push(quote::quote! {
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_get_code_occurence.push(quote::quote!{
                                #field_ident
                            });
                            // enum_fields_logic_for_enum_with_serialize_deserialize.push(quote::quote!{
                            //     #field_ident: #code_occurence_type_with_serialize_deserialize_token_stream
                            // });
                            enum_fields_logic_for_source_with_serialize_deserialize.push(quote::quote!{
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_get_code_occurence_with_serialize_deserialize.push(quote::quote!{
                                 #field_ident
                            });
                            enum_fields_logic_for_into_serialize_deserialize_version.push(quote::quote!{
                                #field_ident
                            });
                            enum_fields_logic_for_compile_time_check_error_occurence_members.push(quote::quote!{
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            fields_logic_for_into_serialize_deserialize_version_for_attribute.push(quote::quote!{
                                #field_ident: #field_ident
                            });
                            fields_logic_for_compile_time_check_error_occurence_members_for_attribute.push(proc_macro2::TokenStream::new());
                        },
                    }
                });
                let format_logic_for_source_to_string_with_or_without_config_stringified = format_logic_for_source_to_string_with_or_without_config.iter().fold(
                    std::string::String::from(""), 
                    |mut acc, path_segment| {
                        acc.push_str(path_segment);
                        acc
                    }
                );
                let start_scope_stringified = "{{";
                let end_scope_stringified = "}}";
                let format_logic_for_source_to_string_with_or_without_config_handle_stringified = format!("\"{start_scope_stringified}\n{format_logic_for_source_to_string_with_or_without_config_stringified}{end_scope_stringified}\"");
                let format_logic_for_source_to_string_with_or_without_config_handle_token_stream = format_logic_for_source_to_string_with_or_without_config_handle_stringified
                .parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {format_logic_for_source_to_string_with_or_without_config_handle_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                logic_for_source_to_string_with_config.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_source_to_string_with_config),*
                    } => {
                        format!(
                            #format_logic_for_source_to_string_with_or_without_config_handle_token_stream
                            ,
                            #(#fields_logic_for_source_to_string_with_config_for_attribute),*
                        )
                    }
                });
                logic_for_source_to_string_without_config.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_source_to_string_without_config),*
                    } => {
                        format!(
                            #format_logic_for_source_to_string_with_or_without_config_handle_token_stream
                            ,
                            #(#fields_logic_for_source_to_string_without_config_for_attribute),*
                        )
                    }
                });
                logic_for_get_code_occurence.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_get_code_occurence),*
                    } => {
                        code_occurence
                    }
                });
                // logic_for_enum_with_serialize_deserialize.push(quote::quote! {
                //     #variant_ident {
                //         #(#enum_fields_logic_for_enum_with_serialize_deserialize),*
                //     }
                // });
                logic_for_source_with_serialize_deserialize.push(quote::quote! {
                    #ident_with_serialize_deserialize_token_stream::#variant_ident {
                        #(#enum_fields_logic_for_source_with_serialize_deserialize),*
                    } => {
                        format!(
                            #format_logic_for_source_to_string_with_or_without_config_handle_token_stream
                            ,
                            #(#fields_logic_for_source_with_serialize_deserialize_for_attribute),*
                        )
                    }
                });
                logic_for_get_code_occurence_with_serialize_deserialize.push(quote::quote! {
                    #ident_with_serialize_deserialize_token_stream::#variant_ident {
                        #(#enum_fields_logic_for_get_code_occurence_with_serialize_deserialize),*
                    } => {
                        code_occurence
                    }
                });
                logic_for_into_serialize_deserialize_version.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_into_serialize_deserialize_version),*
                    } => {
                        #ident_with_serialize_deserialize_token_stream::#variant_ident {
                            #(#fields_logic_for_into_serialize_deserialize_version_for_attribute),*
                        }
                    }
                });
                logic_for_compile_time_check_error_occurence_members.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_compile_time_check_error_occurence_members),*
                    } => {
                        #(#fields_logic_for_compile_time_check_error_occurence_members_for_attribute)*
                    }
                });
            });
            let logic_for_source_to_string_with_config = logic_for_source_to_string_with_config.iter();
            let logic_for_source_to_string_without_config = logic_for_source_to_string_without_config.iter();
            let logic_for_get_code_occurence = logic_for_get_code_occurence.iter();
            let logic_for_source_with_serialize_deserialize = logic_for_source_with_serialize_deserialize.iter();
            let logic_for_get_code_occurence_with_serialize_deserialize = logic_for_get_code_occurence_with_serialize_deserialize.iter();
            let logic_for_into_serialize_deserialize_version = logic_for_into_serialize_deserialize_version.iter();
            let logic_for_compile_time_check_error_occurence_members = logic_for_compile_time_check_error_occurence_members.iter();
            let get_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&get_upper_camel_case_stringified);
            let error_occurence_lib_code_occurence_get_stringified = format!("{error_occurence_lib_stringified}::{code_occurence_snake_case_stringified}::{get_upper_camel_case_stringified}");
            let error_occurence_lib_code_occurence_get_token_stream =
            error_occurence_lib_code_occurence_get_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_code_occurence_get_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let error_occurence_lib_code_occurence_code_occurence_stringified = format!("{error_occurence_lib_stringified}::{code_occurence_snake_case_stringified}::{code_occurence_upper_camel_case_stringified}");
            let error_occurence_lib_code_occurence_code_occurence_token_stream =
            error_occurence_lib_code_occurence_code_occurence_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_lib_code_occurence_code_occurence_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let get_token_stream =
            get_snake_case_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {get_snake_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let compile_time_check_error_occurence_members_impl_token_stream = if should_generate_impl_compile_time_check_error_occurence_members {
                quote::quote!{
                    impl<#generics> #ident<#generics> {
                        fn #compile_time_check_error_occurence_members_token_stream(&self) {
                            match self {
                                #(#logic_for_compile_time_check_error_occurence_members),*
                            }
                        }
                    }
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                impl<
                    #trait_lifetime_token_stream,
                >
                    #error_occurence_lib_source_to_string_with_config_source_to_string_with_config_token_stream<
                        #trait_lifetime_token_stream,
                    > for #ident
                {
                    fn #source_to_string_with_config_token_stream
                    <
                        #config_generic_token_stream:
                        #crate_common_config_path_get_source_place_type_token_stream
                        + #crate_common_config_path_get_timezone_token_stream
                        +
                        ?Sized
                    >
                    (
                        &self,
                        config: &#config_generic_token_stream
                    ) -> std::string::String {
                        match self {
                            #(#logic_for_source_to_string_with_config),*
                        }
                    }
                }
                impl<#trait_lifetime_token_stream>
                    #error_occurence_lib_source_to_string_without_config_source_to_string_without_config_token_stream<
                        #trait_lifetime_token_stream
                    > for #ident
                {
                    fn #source_to_string_without_config_token_stream(&self) -> std::string::String {
                        match self {
                            #(#logic_for_source_to_string_without_config),*
                        }
                    }
                }
                impl #error_occurence_lib_code_occurence_get_token_stream for #ident {
                    fn #get_token_stream(&self) -> &#error_occurence_lib_code_occurence_code_occurence_token_stream
                    {
                        match self {
                            #(#logic_for_get_code_occurence),*
                        }
                    }
                }
                impl <#trait_lifetime_token_stream> #error_occurence_lib_source_to_string_without_config_source_to_string_without_config_token_stream<#trait_lifetime_token_stream> for #ident_with_serialize_deserialize_token_stream
                {
                    fn #source_to_string_without_config_token_stream(&self) -> std::string::String {
                        match self {
                            #(#logic_for_source_with_serialize_deserialize),*
                        }
                    }
                }
                impl #error_occurence_lib_code_occurence_get_token_stream
                    for #ident_with_serialize_deserialize_token_stream
                {
                    fn #get_token_stream(
                        &self
                    ) -> &#error_occurence_lib_code_occurence_code_occurence_token_stream
                    {
                        match self {
                            #(#logic_for_get_code_occurence_with_serialize_deserialize),*
                        }
                    }
                }
                impl #ident {
                    pub fn #into_serialize_deserialize_version_token_stream(self) -> #ident_with_serialize_deserialize_token_stream {
                        match self {
                            #(#logic_for_into_serialize_deserialize_version),*
                        }
                    }
                }
                impl std::fmt::Display for #ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(
                            f,
                            "{}", 
                            #error_occurence_lib_to_string_without_config_to_string_without_config_token_stream::#to_string_without_config_token_stream(self)
                        )
                    }
                }
                impl std::fmt::Display for #ident_with_serialize_deserialize_token_stream {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(
                            f,
                            "{}",
                            #error_occurence_lib_to_string_without_config_to_string_without_config_token_stream::#to_string_without_config_token_stream(
                                self
                            )
                        )
                    }
                }
                impl #error_occurence_lib_error_occurence_named_error_occurence_named_token_stream for #ident {
                    fn #error_occurence_named_token_stream(&self) {}
                }
                #compile_time_check_error_occurence_members_impl_token_stream
            }
        },
        proc_macro_helpers::error_occurence::supported_enum_variant::SuportedEnumVariant::Unnamed => {
            let data_enum_variants_len = data_enum.variants.len();
            let mut logic_for_to_string_with_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(data_enum_variants_len);
            let mut logic_for_to_string_without_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(data_enum_variants_len);
            // let mut logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(data_enum_variants_len);
            let mut logic_for_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(data_enum_variants_len);
            let mut logic_for_into_serialize_deserialize_version: Vec<proc_macro2::TokenStream> = Vec::with_capacity(data_enum_variants_len);
            let mut logic_for_compile_time_check_error_occurence_members: Vec<proc_macro2::TokenStream> = Vec::with_capacity(data_enum_variants_len);
            data_enum.variants.iter().for_each(|variant|{
                let variant_ident = &variant.ident;
                let field_type = if let syn::Fields::Unnamed(fields_unnamed) = &variant.fields {
                    let unnamed = &fields_unnamed.unnamed;
                    assert!(unnamed.len() == 1, 
                       "{proc_macro_name_ident_stringified} {}::{unnamed_upper_camel_case} variant fields unnamed len != 1",
                        naming_constants::SUPPORTED_ENUM_VARIANT_STRINGIFIED
                    );
                    &unnamed.first().expect("no first value in punctuated").ty
                }
                else {
                    panic!("{proc_macro_name_ident_stringified} {} {}::{unnamed_upper_camel_case}", naming_constants::SUPPORTS_ONLY_STRINGIFIED, naming_constants::SYN_FIELDS);
                };
                let type_token_stream = if let syn::Type::Path(type_path) = field_type {
                    let type_stringified = format!(
                        "{}{with_serialize_deserialize_upper_camel_case}",
                        proc_macro_helpers::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
                    );
                    type_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                }
                else {
                    panic!("{proc_macro_name_ident_stringified} {} {syn_type_path_stringified}", naming_constants::SUPPORTS_ONLY_STRINGIFIED)
                };
                logic_for_to_string_with_config.push(quote::quote!{
                    #ident::#variant_ident(i) => {
                        i.#to_string_with_config_token_stream(config)
                    }
                });
                logic_for_to_string_without_config.push(quote::quote!{
                    #ident::#variant_ident(i) => {
                        i.#to_string_without_config_token_stream()
                    }
                });
                // logic_for_enum_with_serialize_deserialize.push({
                //     quote::quote!{
                //         #variant_ident(#type_token_stream)
                //     }
                // });
                logic_for_with_serialize_deserialize.push(quote::quote!{
                    #ident_with_serialize_deserialize_token_stream::#variant_ident(i) => {
                         i.#with_serialize_deserialize_token_stream()
                    }
                });
                logic_for_into_serialize_deserialize_version.push(quote::quote!{
                     #ident::#variant_ident(i) => {
                        #ident_with_serialize_deserialize_token_stream::#variant_ident(i.#into_serialize_deserialize_version_token_stream())
                     }
                });
                logic_for_compile_time_check_error_occurence_members.push(quote::quote!{
                     #ident::#variant_ident(i) => {
                        #error_occurence_lib_error_occurence_named_error_occurence_named_token_stream::#error_occurence_named_token_stream(i);
                     }
                });
            });
            let logic_for_to_string_with_config_generated = logic_for_to_string_with_config.iter();
            let logic_for_to_string_without_config_generated = logic_for_to_string_without_config.iter();
            // let logic_for_enum_with_serialize_deserialize_generated = logic_for_enum_with_serialize_deserialize.iter();
            let logic_for_with_serialize_deserialize_generated = logic_for_with_serialize_deserialize.iter();
            let logic_for_into_serialize_deserialize_version_generated = logic_for_into_serialize_deserialize_version.iter();
            let logic_for_compile_time_check_error_occurence_members_generated = logic_for_compile_time_check_error_occurence_members.iter();
            quote::quote! {
                impl<
                    #trait_lifetime_token_stream
                >
                    #error_occurence_lib_to_string_with_config_to_string_with_config_token_stream<
                        #trait_lifetime_token_stream
                    > for #ident
                {
                    fn #to_string_with_config_token_stream<
                        #config_generic_token_stream: #crate_common_config_path_get_source_place_type_token_stream
                        + #crate_common_config_path_get_timezone_token_stream
                        + ?Sized,
                    >
                    (&self, config: &#config_generic_token_stream) -> std::string::String {
                        match self {
                            #(#logic_for_to_string_with_config_generated),*
                        }
                    }
                }
                impl<
                    #trait_lifetime_token_stream
                > #error_occurence_lib_to_string_without_config_to_string_without_config_token_stream<
                    #trait_lifetime_token_stream
                >
                    for #ident
                {
                    fn #to_string_without_config_token_stream(&self) -> std::string::String {
                        match self {
                            #(#logic_for_to_string_without_config_generated),*
                        }
                    }
                }
                impl<#trait_lifetime_token_stream>
                    #error_occurence_lib_to_string_without_config_with_serialize_deserialize_token_stream<
                        #trait_lifetime_token_stream
                    >
                    for #ident_with_serialize_deserialize_token_stream
                {
                    fn #with_serialize_deserialize_token_stream(&self) -> std::string::String {
                        match self {
                            #(#logic_for_with_serialize_deserialize_generated),*
                        }
                    }
                }
                impl #ident {
                    pub fn #into_serialize_deserialize_version_token_stream(self) -> #ident_with_serialize_deserialize_token_stream {
                        match self {
                            #(#logic_for_into_serialize_deserialize_version_generated),*
                        }
                    }
                }
                impl std::fmt::Display for #ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(
                            f,
                            "{}", 
                            #error_occurence_lib_to_string_without_config_to_string_without_config_token_stream::#to_string_without_config_token_stream(
                                self
                            )
                        )
                    }
                }
                impl std::fmt::Display for #ident_with_serialize_deserialize_token_stream {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(
                            f,
                            "{}", 
                            #error_occurence_lib_to_string_without_config_with_serialize_deserialize_token_stream::#with_serialize_deserialize_token_stream(
                                self
                            )
                        )
                    }
                }
                impl<#generics> #error_occurence_lib_error_occurence_unnamed_error_occurence_unnamed_token_stream for #ident<#generics> {
                    fn #error_occurence_unnamed_token_stream(&self) {}
                }
                impl #ident {
                    fn #compile_time_check_error_occurence_members_token_stream(&self) {
                        match self {
                            #(#logic_for_compile_time_check_error_occurence_members_generated),*
                        }
                    }
                }
            }
        },
    };
    let gen = quote::quote! {
        #token_stream
        #enum_with_serialize_deserialize_logic
    };
    // if ident == "" {
    //     proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         &proc_macro_name,
    //         &gen,
    //         &proc_macro_name_ident_stringified
    //     );
    // }
    gen.into()
}

fn get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
    vec_lifetime: Vec<proc_macro_helpers::error_occurence::lifetime::Lifetime>,
    lifetimes_for_serialize_deserialize: &mut Vec<String>,
    trait_lifetime_stringified: &String,
    proc_macro_name_ident_stringified: &String,
) -> proc_macro2::TokenStream {
    let token_stream = match vec_lifetime_to_lifetime(&vec_lifetime) {
        proc_macro_helpers::error_occurence::lifetime::Lifetime::Specified(_) => {
            quote::quote! {#[serde(borrow)]}
        }
        proc_macro_helpers::error_occurence::lifetime::Lifetime::NotSpecified => {
            proc_macro2::TokenStream::new()
        }
    };
    vec_lifetime.into_iter().for_each(|element|{
        if let proc_macro_helpers::error_occurence::lifetime::Lifetime::Specified(specified_lifetime) = element {
            assert!(&specified_lifetime != trait_lifetime_stringified, "{proc_macro_name_ident_stringified} must not contain reserved by macro lifetime name: {trait_lifetime_stringified}");
            proc_macro_helpers::error_occurence::possible_lifetime_addition::possible_lifetime_addition(
                specified_lifetime,
                lifetimes_for_serialize_deserialize
            );
        }
    });
    token_stream
}
//potential support for few lifetime annotations, but now supported only one lifetime annotation
fn get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
    key_vec_lifetime: Vec<proc_macro_helpers::error_occurence::lifetime::Lifetime>,
    value_vec_lifetime: Vec<proc_macro_helpers::error_occurence::lifetime::Lifetime>,
    lifetimes_for_serialize_deserialize: &mut Vec<String>,
    trait_lifetime_stringified: &String,
    proc_macro_name_ident_stringified: &String,
) -> proc_macro2::TokenStream {
    let key_lifetime_enum = vec_lifetime_to_lifetime(&key_vec_lifetime);
    let value_lifetime_enum = vec_lifetime_to_lifetime(&value_vec_lifetime);
    let token_stream = match (key_lifetime_enum, value_lifetime_enum) {
        (
            proc_macro_helpers::error_occurence::lifetime::Lifetime::Specified(_) |
            proc_macro_helpers::error_occurence::lifetime::Lifetime::NotSpecified,
            proc_macro_helpers::error_occurence::lifetime::Lifetime::Specified(_)
        ) |
        (
            proc_macro_helpers::error_occurence::lifetime::Lifetime::Specified(_),
            proc_macro_helpers::error_occurence::lifetime::Lifetime::NotSpecified
        ) => quote::quote! {#[serde(borrow)]},
        (
            proc_macro_helpers::error_occurence::lifetime::Lifetime::NotSpecified,
            proc_macro_helpers::error_occurence::lifetime::Lifetime::NotSpecified,
        ) => proc_macro2::TokenStream::new(),
    };
    let error_message = "must not contain reserved by macro lifetime name:";
    key_vec_lifetime.into_iter().for_each(|element|{
        if let proc_macro_helpers::error_occurence::lifetime::Lifetime::Specified(key_lifetime_specified) = element {
            assert!(&key_lifetime_specified != trait_lifetime_stringified, "{proc_macro_name_ident_stringified} {error_message} {trait_lifetime_stringified}");
            proc_macro_helpers::error_occurence::possible_lifetime_addition::possible_lifetime_addition(
                key_lifetime_specified,
                lifetimes_for_serialize_deserialize
            );
        }
    });
    value_vec_lifetime.into_iter().for_each(|element|{
        if let proc_macro_helpers::error_occurence::lifetime::Lifetime::Specified(value_lifetime_specified) = element {
            assert!(&value_lifetime_specified != trait_lifetime_stringified, "{proc_macro_name_ident_stringified} {error_message} {trait_lifetime_stringified}");
            proc_macro_helpers::error_occurence::possible_lifetime_addition::possible_lifetime_addition(
                value_lifetime_specified,
                lifetimes_for_serialize_deserialize
            );
        }
    });
    token_stream
}

fn vec_lifetime_to_lifetime(
    vec: &Vec<proc_macro_helpers::error_occurence::lifetime::Lifetime>,
) -> &proc_macro_helpers::error_occurence::lifetime::Lifetime {
    let mut lifetime_handle =
        &proc_macro_helpers::error_occurence::lifetime::Lifetime::NotSpecified;
    for lft in vec {
        if let proc_macro_helpers::error_occurence::lifetime::Lifetime::Specified(_) = lft {
            lifetime_handle = lft;
            break;
        }
    }
    lifetime_handle
}
