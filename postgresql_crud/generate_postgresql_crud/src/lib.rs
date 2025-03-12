//todo checked and for path query headers too
//todo how to write filter logic for sqlx rust postgresql types?
//todo decide where to do error log (maybe add in some places)
//todo add regex filter to query parameters - now supports only in body variants
//todo regex filter support only for string-like types postgresql
//todo generate route what will return columns of the table and their rust and postgersql types
//todo - check if fields for filter are unique in the input array
//todo created at and updated at fields
//todo attributes for activation generation crud methods(like generate create, update_one, delete_one)
//todo authorization for returning concrete error or just minimal info(user role)
//todo generate rules and roles
//todo unique(meaning not primary key unique column) and nullable support
//todo add check on max postgresql bind elements
//todo add route name as argument for macro - generation constant and add to generation logic
//todo make sqlx macros instead of just queries?
//todo support for arrays as column values
//todo maybe add unnest sql types?
//todo maybe add unnest to filter parameters if its array ?
//todo swagger ui https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs
//todo test #[derive(generate_postgresql_crud::GeneratePostgresqlCrud)] for few structs in one module - will it work? fix if not will not work
//todo derive utoipa::ToSchema for what? original structs or with serialize deserialize?
//todo need to add utoipa::ToSchema annotation #[schema(value_type = YourToSchemaTraitImplStruct)] for all fields
//todo remove useless derives like useless serde::Serialize and Deserialize
//todo maybe add private generated code in specific mod in case of some compilation error - to show info like "no struct BlaBla found in mod ..." ?
//todo add ident prefix to public functions structs and other
//todo maybe generate compisite type for user defined type https://docs.rs/sqlx/0.7.3/sqlx/postgres/types/index.html#rust_decimal
//todo read again some interesting thoughts about sql as api https://habr.com/ru/companies/timeweb/articles/798937/
//todo sqlx query! macros not in actual api, but just unused function just for compile time query check
//todo reexport all crates what logic depends on (from crates.io) (use of undeclared crate or module `time`)
//todo write checks on fields one by one with early return. not need to check all and create combinatorial complexity
//todo add transaction isolation level (see postgresql docs)
//todo check on postgresql max length value of type
//todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
//todo update one and delete one must be a transaction
//todo postgresql json schema validation https://youtu.be/F6X60ln2VNc
// SET search_path TO public;
// DROP EXTENSION IF EXISTS "pg_jsonschema";
// CREATE EXTENSION "pg_jsonschema";
//todo generate json schema from rust type https://docs.rs/schemars/latest/schemars/
//todo support read table length

//todo postgresql json:
//* define postgresql json type
//* write checker what ensures what postgresql type equals to
//* write json schema in postgresql
//* validate insert json field with json schema
//* syncronize serde json type deserialize validation with postgresql insert validation
//* validate read json field - all recursive json tree variants to read
//* generate all recursive tree variants to read in serde read variants type
//* generate all recursive tree variants to read in postgresql query
//* validate update json field with json schema - all recursive tree variants to update
//* delete not need?

#[proc_macro_attribute]
pub fn create_many_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn create_one_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_one_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_many_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_one_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_many_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_one_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_many_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn common_additional_error_variants(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}

#[proc_macro_attribute]
pub fn create_many_additional_route_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn create_one_additional_route_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_many_additional_route_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn read_one_additional_route_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_many_additional_route_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn update_one_additional_route_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_many_additional_route_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn delete_one_additional_route_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn common_additional_route_logic(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}

#[proc_macro_derive(
    GeneratePostgresqlCrud,
    attributes(generate_postgresql_crud_primary_key)
)]
pub fn generate_postgresql_crud(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_snake_case_stringified = naming::ToTokensToSnakeCaseStringified::case(&ident);
    // #[derive(Debug, Clone)]
    // struct Generic<'a> {
    //     syn_angle_bracketed_generic_arguments: &'a syn::AngleBracketedGenericArguments,
    //     upper_camel_case_stringified: std::string::String,
    //     reader_upper_camel_case_stringified: std::string::String,
    //     options_to_read_upper_camel_case_stringified: std::string::String,
    //     field_to_read_upper_camel_case_stringified: std::string::String,
    //     generate_postgresql_json_type_from_vec_error_named_upper_camel_case_stringified: std::string::String,
    //     generate_postgresql_json_type_from_vec_error_named_snake_case_stringified: std::string::String,
    // }
    // #[derive(Debug, Clone)]
    // struct SynFieldWithAdditionalInfo<'a> {
    //     field: &'a syn::Field,
    //     field_ident: &'a syn::Ident,
    //     // rust_sqlx_map_to_postgres_type_variant: postgresql_crud_common::RustSqlxMapToPostgresTypeVariant, //todo maybe not need to add here
    //     original_type_token_stream: proc_macro2::TokenStream,
    //     original_type_with_generic_token_stream: proc_macro2::TokenStream,
    //     original_type_with_generic_reader_token_stream: proc_macro2::TokenStream,
    //     inner_type_token_stream: proc_macro2::TokenStream,
    //     inner_type_with_generic_token_stream: proc_macro2::TokenStream,
    //     inner_type_with_generic_reader_token_stream: proc_macro2::TokenStream,
    //     // where_inner_type_token_stream: proc_macro2::TokenStream,
    //     where_inner_type_with_generic_token_stream: proc_macro2::TokenStream,
    //     original_wrapper_type_token_stream: proc_macro2::TokenStream,
    //     // option_generic: std::option::Option<Generic<'a>>,
    // }
    // impl<'a> std::convert::TryFrom<&'a syn::Field> for SynFieldWithAdditionalInfo<'a> {
    //     type Error = std::string::String;
    //     fn try_from(value: &'a syn::Field) -> Result<Self, Self::Error> {
    //         let name = "SynFieldWithAdditionalInfo from syn::Field error";
    //         let field_ident = match value.ident.as_ref() {
    //             Some(value) => value,
    //             None => {
    //                 return Err(format!("{name} field ident is none"));
    //             }
    //         };
    //         let 
    //         // (
    //             rust_sqlx_map_to_postgres_type_variant
    //             // ,
    //             // option_generic
    //         // )
    //         = match &value.ty {
    //             syn::Type::Path(value) => {
    //                 if value.path.segments.len() != 2 {
    //                     return Err(std::string::String::from("value.path.segments.len() != 2"));
    //                 }
    //                 let first = match value.path.segments.first() {
    //                     Some(value) => value,
    //                     None => {
    //                         return Err(std::string::String::from("no first value in punctuated"));
    //                     }
    //                 };
    //                 if first.ident != &naming::PostgresqlCrudSnakeCase.to_string() {
    //                     return Err(format!("{name} field_type is not syn::Type::Path"));
    //                 }
    //                 match first.arguments {
    //                     syn::PathArguments::None => (),
    //                     syn::PathArguments::AngleBracketed(_) | syn::PathArguments::Parenthesized(_) => {
    //                         return Err(format!("{name} value.path().segments[0].arguments != syn::PathArguments::None"));
    //                     }
    //                 }
    //                 let second_element = match value.path.segments.iter().nth(1) {
    //                     Some(value) => value,
    //                     None => {
    //                         return Err(std::string::String::from("no second element"));
    //                     }
    //                 };
    //                 // let rust_sqlx_map_to_postgres_type_variant = match <postgresql_crud_common::RustSqlxMapToPostgresTypeVariant as std::str::FromStr>::from_str(&second_element.ident.to_string()) {
    //                 //     Ok(value) => value,
    //                 //     Err(error) => {
    //                 //         return Err(format!("{name} RustSqlxMapToPostgresTypeVariant::try_from failed {error}"));
    //                 //     }
    //                 // };
    //                 // let option_generic = match &second_element.arguments {
    //                 //     syn::PathArguments::None => None,
    //                 //     syn::PathArguments::AngleBracketed(value) => Some({
    //                 //         enum Case {
    //                 //             UpperCamel,
    //                 //             Snake,
    //                 //         }
    //                 //         let generate_generic_option_string = |syn_angle_bracketed_generic_arguments: &'a syn::AngleBracketedGenericArguments, postfix: &std::primitive::str, case: Case| -> Result<std::string::String, std::string::String> {
    //                 //             if syn_angle_bracketed_generic_arguments.args.len() != 1 {
    //                 //                 return Err("value.args.len() != 1".to_string());
    //                 //             }
    //                 //             if let Some(value) = syn_angle_bracketed_generic_arguments.args.first() {
    //                 //                 if let syn::GenericArgument::Type(value) = value {
    //                 //                     if let syn::Type::Path(value) = value {
    //                 //                         if value.path.segments.len() != 1 {
    //                 //                             return Err("value.path.segments.len() != 1".to_string());
    //                 //                         }
    //                 //                         if let Some(value) = value.path.segments.first() {
    //                 //                             Ok(match case {
    //                 //                                 Case::UpperCamel => naming::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&format!("{}{postfix}", &value.ident)),
    //                 //                                 Case::Snake => naming::AsRefStrToSnakeCaseStringified::new(&format!("{}_{postfix}", &value.ident)),
    //                 //                             })
    //                 //                         } else {
    //                 //                             return Err("value.path.segments.first() is None".to_string());
    //                 //                         }
    //                 //                     } else {
    //                 //                         return Err("value is not syn::Type::Path".to_string());
    //                 //                     }
    //                 //                 } else {
    //                 //                     return Err("value is not syn::GenericArgument::Type".to_string());
    //                 //                 }
    //                 //             } else {
    //                 //                 return Err("value.args.first() is None".to_string());
    //                 //             }
    //                 //         };
    //                 //         let upper_camel_case_stringified = match generate_generic_option_string(&value, "", Case::UpperCamel) {
    //                 //             Ok(value) => value,
    //                 //             Err(error) => {
    //                 //                 return Err(error);
    //                 //             }
    //                 //         };
    //                 //         let reader_upper_camel_case_stringified = match generate_generic_option_string(&value, &naming::ReaderUpperCamelCase.to_string(), Case::UpperCamel) {
    //                 //             Ok(value) => value,
    //                 //             Err(error) => {
    //                 //                 return Err(error);
    //                 //             }
    //                 //         };
    //                 //         let options_to_read_upper_camel_case_stringified = match generate_generic_option_string(&value, &naming::OptionsToReadUpperCamelCase.to_string(), Case::UpperCamel) {
    //                 //             Ok(value) => value,
    //                 //             Err(error) => {
    //                 //                 return Err(error);
    //                 //             }
    //                 //         };
    //                 //         let field_to_read_upper_camel_case_stringified = match generate_generic_option_string(&value, &naming::FieldToReadUpperCamelCase.to_string(), Case::UpperCamel) {
    //                 //             Ok(value) => value,
    //                 //             Err(error) => {
    //                 //                 return Err(error);
    //                 //             }
    //                 //         };
    //                 //         let generate_postgresql_json_type_from_vec_error_named_upper_camel_case_stringified = match generate_generic_option_string(&value, &naming::GeneratePostgresqlJsonTypeToReadFromVecErrorNamedUpperCamelCase.to_string(), Case::UpperCamel) {
    //                 //             Ok(value) => value,
    //                 //             Err(error) => {
    //                 //                 return Err(error);
    //                 //             }
    //                 //         };
    //                 //         let generate_postgresql_json_type_from_vec_error_named_snake_case_stringified = match generate_generic_option_string(&value, &naming::GeneratePostgresqlJsonTypeToReadFromVecErrorNamedSnakeCase.to_string(), Case::Snake) {
    //                 //             Ok(value) => value,
    //                 //             Err(error) => {
    //                 //                 return Err(error);
    //                 //             }
    //                 //         };
    //                 //         Generic {
    //                 //             syn_angle_bracketed_generic_arguments: value,
    //                 //             upper_camel_case_stringified,
    //                 //             reader_upper_camel_case_stringified,
    //                 //             options_to_read_upper_camel_case_stringified,
    //                 //             field_to_read_upper_camel_case_stringified,
    //                 //             generate_postgresql_json_type_from_vec_error_named_upper_camel_case_stringified,
    //                 //             generate_postgresql_json_type_from_vec_error_named_snake_case_stringified,
    //                 //         }
    //                 //     }),
    //                 //     syn::PathArguments::Parenthesized(_) => {
    //                 //         return Err(format!("{name} does not support syn::PathArguments::Parenthesized"));
    //                 //     }
    //                 // };
    //                 // (
    //                     rust_sqlx_map_to_postgres_type_variant
    //                     // ,
    //                     // option_generic
    //                 // )
    //             }
    //             _ => {
    //                 return Err(format!("{name} field_type is not syn::Type::Path"));
    //             }
    //         };
    //         let original_type_token_stream = {
    //             let value = &rust_sqlx_map_to_postgres_type_variant.get_original_type_stringified(""); //todo generic for json
    //             match value.parse::<proc_macro2::TokenStream>() {
    //                 Ok(value) => value,
    //                 Err(error) => {
    //                     return Err(format!("{name} {value} {} {error:#?}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    //                 }
    //             }
    //         };
    //         let original_type_with_generic_token_stream = {
    //             let value = format!(
    //                 "{}{}",
    //                 &rust_sqlx_map_to_postgres_type_variant.get_original_type_stringified(""),
    //                 match &option_generic {
    //                     Some(value) => {
    //                         let value = &value.syn_angle_bracketed_generic_arguments;
    //                         quote::quote! {#value}.to_string()
    //                     }
    //                     None => std::string::String::default(),
    //                 }
    //             );
    //             match value.parse::<proc_macro2::TokenStream>() {
    //                 Ok(value) => value,
    //                 Err(error) => {
    //                     return Err(format!("{name} {value} {} {error:#?}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    //                 }
    //             }
    //         };
    //         let original_type_with_generic_reader_token_stream = {
    //             let value = format!(
    //                 "{}{}",
    //                 &rust_sqlx_map_to_postgres_type_variant.get_original_type_stringified(""),
    //                 match &option_generic {
    //                     Some(value) => format!("<{}>", &value.reader_upper_camel_case_stringified),
    //                     None => std::string::String::default(),
    //                 }
    //             );
    //             match value.parse::<proc_macro2::TokenStream>() {
    //                 Ok(value) => value,
    //                 Err(error) => {
    //                     return Err(format!("{name} {value} {} {error:#?}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    //                 }
    //             }
    //         };
    //         let inner_type_token_stream = {
    //             let value = rust_sqlx_map_to_postgres_type_variant.get_inner_type_stringified("");
    //             match value.parse::<proc_macro2::TokenStream>() {
    //                 Ok(value) => value,
    //                 Err(error) => {
    //                     return Err(format!("{name} {value} {} {error:#?}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    //                 }
    //             }
    //         };
    //         let inner_type_with_generic_token_stream = {
    //             let value = format!(
    //                 "{}{}",
    //                 &rust_sqlx_map_to_postgres_type_variant.get_inner_type_stringified(""),
    //                 match &option_generic {
    //                     Some(value) => {
    //                         let value = &value.syn_angle_bracketed_generic_arguments;
    //                         quote::quote! {#value}.to_string()
    //                     }
    //                     None => std::string::String::default(),
    //                 }
    //             );
    //             match value.parse::<proc_macro2::TokenStream>() {
    //                 Ok(value) => value,
    //                 Err(error) => {
    //                     return Err(format!("{name} {value} {} {error:#?}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    //                 }
    //             }
    //         };
    //         let inner_type_with_generic_reader_token_stream = {
    //             let value = format!(
    //                 "{}{}",
    //                 &rust_sqlx_map_to_postgres_type_variant.get_inner_type_stringified(""),
    //                 match &option_generic {
    //                     Some(value) => format!("<{}>", &value.reader_upper_camel_case_stringified),
    //                     None => std::string::String::default(),
    //                 }
    //             );
    //             match value.parse::<proc_macro2::TokenStream>() {
    //                 Ok(value) => value,
    //                 Err(error) => {
    //                     return Err(format!("{name} {value} {} {error:#?}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    //                 }
    //             }
    //         };
    //         // let where_inner_type_token_stream = {
    //         //     let value = &rust_sqlx_map_to_postgres_type_variant.get_where_inner_type_stringified("");
    //         //     match value.parse::<proc_macro2::TokenStream>() {
    //         //         Ok(value) => value,
    //         //         Err(error) => {
    //         //             return Err(format!("{name} {value} {} {error:#?}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    //         //         }
    //         //     }
    //         // };
    //         let where_inner_type_with_generic_token_stream = {
    //             let value = format!(
    //                 "{}{}",
    //                 &rust_sqlx_map_to_postgres_type_variant.get_where_inner_type_stringified(""),
    //                 match &option_generic {
    //                     Some(value) => {
    //                         let value = &value.syn_angle_bracketed_generic_arguments;
    //                         quote::quote! {#value}.to_string()
    //                     }
    //                     None => std::string::String::default(),
    //                 }
    //             );
    //             match value.parse::<proc_macro2::TokenStream>() {
    //                 Ok(value) => value,
    //                 Err(error) => {
    //                     return Err(format!("{name} {value} {} {error:#?}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    //                 }
    //             }
    //         };
    //         // let original_wrapper_type_token_stream = {
    //         //     let value = postgresql_crud_common::SqlxPostgresType::from_supported_sqlx_postgres_type_removing_option(&rust_sqlx_map_to_postgres_type_variant.get_supported_sqlx_postgres_type()).get_path_stringified();
    //         //     match value.parse::<proc_macro2::TokenStream>() {
    //         //         Ok(value) => value,
    //         //         Err(error) => {
    //         //             return Err(format!("{name} {value} {} {error:#?}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    //         //         }
    //         //     }
    //         // };
    //         Ok(Self {
    //             field: value,
    //             field_ident,
    //             rust_sqlx_map_to_postgres_type_variant, //todo maybe not need to add here
    //             original_type_token_stream,
    //             original_type_with_generic_token_stream,
    //             original_type_with_generic_reader_token_stream,
    //             inner_type_token_stream,
    //             inner_type_with_generic_token_stream,
    //             inner_type_with_generic_reader_token_stream,
    //             // where_inner_type_token_stream,
    //             where_inner_type_with_generic_token_stream,
    //             original_wrapper_type_token_stream,
    //             option_generic,
    //         })
    //     }
    // }


// #[allow(clippy::enum_variant_names)]
// #[derive(Debug, Clone, Copy)]
// pub enum ErrorOccurenceFieldAttribute {
//     EoToStdStringString,
// }
// impl std::str::FromStr for ErrorOccurenceFieldAttribute {
//     type Err = ();
//     fn from_str(value: &std::primitive::str) -> Result<Self, Self::Err> {
//         if value == "eo_to_std_string_string" {
//             Ok(Self::EoToStdStringString)
//         } else if value == "eo_to_std_string_string_serialize_deserialize" {
//             Ok(Self::EoToStdStringStringSerializeDeserialize)
//         } else {
//             Err(())
//         }
//     }
// }
// impl std::convert::From<&syn::Field> for ErrorOccurenceFieldAttribute {
//     fn from(value: &syn::Field) -> Self {
//         let mut option_attribute = None;
//         for attr in &value.attrs {
//             if attr.path().segments.len() == 1 {
//                 let first_segment_ident = &attr.path().segments.first().expect("no first value in punctuated").ident;
//                 if let Ok(value) = {
//                     use std::str::FromStr;
//                     ErrorOccurenceFieldAttribute::from_str(&first_segment_ident.to_string())
//                 } {
//                     if option_attribute.is_some() {
//                         panic!("two or more supported attributes!");
//                     } else {
//                         option_attribute = Some(value);
//                     }
//                 }
//             } //other attributes are not for this proc_macro
//         }
//         option_attribute.unwrap_or_else(|| panic!("option attribute {}", naming::IS_NONE_STRINGIFIED))
//     }
// }

    #[derive(Debug, Clone)]
    struct SynFieldWrapper {
        syn_field: syn::Field,
        field_ident: syn::Ident,
    }
    let (
        primary_key_field,
        fields,
        fields_without_primary_key,
    ) = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            let mut option_primary_key_field: std::option::Option<SynFieldWrapper> = None;
            let mut fields = vec![];
            let mut fields_without_primary_key = vec![];
            for element in &fields_named.named {
                let field_ident = element.ident.clone().unwrap();
                let field_ident_len = field_ident.to_string().len();
                let max_postgresql_column_length = 63;
                //todo write runtime check
                if field_ident_len > max_postgresql_column_length {
                    panic!("Postgresql truncates column names to {max_postgresql_column_length} characters, this is more: {field_ident} ({field_ident_len} characters)");
                }
                fields.push(SynFieldWrapper {
                    syn_field: element.clone(),
                    field_ident: field_ident.clone(),
                });
                let mut is_primary_key = false;
                {
                    for attr in &element.attrs {
                        if attr.path().segments.len() == 1 {
                            let first_segment_ident = &attr.path().segments.first().expect("no first value in punctuated").ident;
                            let generate_postgresql_crud_primary_key_snake_case_stringified = naming::GeneratePostgresqlCrudPrimaryKeySnakeCase.to_string();
                            if first_segment_ident == &generate_postgresql_crud_primary_key_snake_case_stringified {
                                if option_primary_key_field.is_some() {
                                    panic!("two or more supported {generate_postgresql_crud_primary_key_snake_case_stringified} attributes!");
                                } else {
                                    option_primary_key_field = Some(SynFieldWrapper {
                                        syn_field: element.clone(),
                                        field_ident: field_ident.clone(),
                                    });
                                    is_primary_key = true;
                                }
                            }
                        }
                    }
                }
                if !is_primary_key {
                    fields_without_primary_key.push(SynFieldWrapper {
                        syn_field: element.clone(),
                        field_ident: field_ident.clone(),
                    });
                }
            }
            (
                option_primary_key_field.unwrap_or_else(|| panic!("primary_key_field is None")),
                fields,
                fields_without_primary_key,
            )
        } else {
            panic!("supports only syn::Fields::Named");
        }
    } else {
        panic!("does work only on structs!");
    };
    // let fields_without_primary_key_len = fields_without_primary_key.len();
    // let primary_key_field_type_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_type_last_segment(&primary_key_field.syn_field.ty);
    let primary_key_field_type_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(&primary_key_field.syn_field.ty);
    let primary_key_field_type_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_type_last_segment(&primary_key_field.syn_field.ty);
    let primary_key_field_type_to_delete_upper_camel_case = naming::parameter::SelfToDeleteUpperCamelCase::from_type_last_segment(&primary_key_field.syn_field.ty);
    // let contains_generic_json = {
    //     let mut contains_generic_json = false;
    //     for element in &syn_field_with_additional_info_fields_named {
    //         if element.option_generic.is_some() {
    //             contains_generic_json = true;
    //             break;
    //         }
    //     }
    //     contains_generic_json
    // };
    let postgresql_crud_snake_case_stringified = &naming::PostgresqlCrudSnakeCase.to_string();
    // let primary_key_syn_field_with_additional_info = {
    //     let mut primary_key_field_option = None;
    //     for element in &syn_field_with_additional_info_fields_named {
    //         match &element.field.ty {
    //             syn::Type::Path(value) => {
    //                 if value.path.segments.len() == 2 {
    //                     assert!(
    //                         value.path.segments.first().expect("no first value in punctuated").ident == postgresql_crud_snake_case_stringified,
    //                         "field_type is not syn::Type::Path"
    //                     );
    //                     match <postgresql_crud_common::RustSqlxMapToPostgresTypeVariant as std::str::FromStr>::from_str(&value.path.segments.iter().nth(1).expect("no second element").ident.to_string()) {
    //                         Ok(value) => {
    //                             if postgresql_crud_common::RustSqlxMapToPostgresTypeVariantPrimaryKey::try_from(&value).is_ok() {
    //                                 match primary_key_field_option {
    //                                     Some(_) => panic!("must have one PrimaryKey"),
    //                                     None => {
    //                                         primary_key_field_option = Some(element.clone());
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                         Err(error) => panic!("RustSqlxMapToPostgresTypeVariant::try_from failed {error}"),
    //                     }
    //                 } else {
    //                     panic!("field_type is not syn::Type::Path")
    //                 }
    //             }
    //             _ => panic!("field_type is not syn::Type::Path"),
    //         }
    //     }
    //     primary_key_field_option.map_or_else(|| panic!("no primary_key_field_option"), |value| value)
    // };
    // let primary_key_field = &primary_key_syn_field_with_additional_info.field;
    let primary_key_field_ident = &primary_key_field.field_ident;
    let primary_key_field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&primary_key_field_ident);
    let primary_key_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&primary_key_field_ident);
    // let primary_key_rust_sqlx_map_to_postgres_type_variant = &primary_key_syn_field_with_additional_info.rust_sqlx_map_to_postgres_type_variant;
    // let primary_key_original_type_token_stream = &primary_key_syn_field_with_additional_info.original_type_token_stream;
    // let primary_key_original_type_token_stream = &primary_key_field.syn_field.ty;//todo maybe remove it
    // let primary_key_inner_type_token_stream = &primary_key_field.syn_field.ty;
    let primary_key_field_type_update_token_stream = &naming::parameter::SelfUpdateUpperCamelCase::from_type_last_segment(&primary_key_field.syn_field.ty);
    // fn syn_ident_to_upper_camel_case_stringified(value: &syn::Ident) -> std::string::String {
    //     naming::ToTokensToUpperCamelCaseStringified::to_upper_camel_case_stringified(&value)
    // }
    // let syn_ident_to_upper_camel_case_token_stream = |value: &syn::Ident| -> proc_macro2::TokenStream {
    //     let value = syn_ident_to_upper_camel_case_stringified(value);
    //     value
    //         .parse::<proc_macro2::TokenStream>()
    //         .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    // };
    let std_string_string = token_patterns::StdStringString;
    // let syn_field_with_additional_info_fields_named_excluding_primary_key = syn_field_with_additional_info_fields_named.clone().into_iter().filter(|element| element.field != *primary_key_field).collect::<std::vec::Vec<SynFieldWithAdditionalInfo<'_>>>();
    // let fields_named_len = syn_field_with_additional_info_fields_named.len();
    // assert!(fields_named_len > 1, "false = fields_named.len() > 1");
    // let syn_field_with_additional_info_fields_named_excluding_primary_key_len = syn_field_with_additional_info_fields_named_excluding_primary_key.len();
    // let fields_named_from_or_try_from = {
    //     let mut value = postgresql_crud_common::FromOrTryFrom::From;
    //     for element in &syn_field_with_additional_info_fields_named {
    //         let from_or_try_from = element.rust_sqlx_map_to_postgres_type_variant.inner_type_from_or_try_from_inner_type_with_serialize_deserialize();
    //         if from_or_try_from == postgresql_crud_common::FromOrTryFrom::TryFrom {
    //             value = from_or_try_from;
    //             break;
    //         }
    //     }
    //     value
    // };


    let impl_ident_table_name_token_stream = {
        let table_name_snake_case = naming::TableNameSnakeCase;
        let ident_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_snake_case_stringified);
        quote::quote! {
            impl #ident {
                pub fn #table_name_snake_case() -> &'static str {
                    #ident_snake_case_double_quotes_token_stream
                }
            }
        }
    };

    let debug_upper_camel_case = naming::DebugUpperCamelCase;
    let error_snake_case = naming::ErrorSnakeCase;
    let eprintln_error_token_stream = quote::quote!{eprintln!("{error}");};
    let serde_serialize = token_patterns::SerdeSerialize;
    let serde_deserialize = token_patterns::SerdeDeserialize;
    let derive_debug_serde_serialize_serde_deserialize = token_patterns::DeriveDebugSerdeSerializeSerdeDeserialize;
    // // let from_str_upper_camel_case = naming::FromStrUpperCamelCase;
    // // let from_str_snake_case = naming::FromStrSnakeCase;
    let sqlx_row = token_patterns::SqlxRow;
    let ident_options_upper_camel_case = naming::parameter::SelfOptionsUpperCamelCase::from_tokens(&ident);
    let postgresql_crud_snake_case = &naming::PostgresqlCrudSnakeCase;
    let value_upper_camel_case = naming::ValueUpperCamelCase;
    let value_snake_case = naming::ValueSnakeCase;
    let from_snake_case = naming::FromSnakeCase;
    let generate_postgresql_crud_value_declaration_token_stream = |content_token_stream: &dyn quote::ToTokens| {
        quote::quote! {#postgresql_crud_snake_case::#value_upper_camel_case<#content_token_stream>}
    };
    let generate_postgresql_crud_value_initialization_token_stream = |content_token_stream: &dyn quote::ToTokens| {
        quote::quote! {#postgresql_crud_snake_case::#value_upper_camel_case{#value_snake_case: #content_token_stream}}
    };
    let ident_options_token_stream = {
        let field_attribute_serde_skip_serializing_if_option_is_none_token_stream = token_patterns::FieldAttributeSerdeSkipSerializingIfOptionIsNone;
        let field_option_primary_key_token_stream = {
            let postgresql_crud_value_declaration_token_stream = generate_postgresql_crud_value_declaration_token_stream(
                // &primary_key_inner_type_token_stream
                &naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(&primary_key_field.syn_field.ty)
            );
            quote::quote! {
                #field_attribute_serde_skip_serializing_if_option_is_none_token_stream
                pub #primary_key_field_ident: std::option::Option<#postgresql_crud_value_declaration_token_stream>
            }
        };
        let fields_options_excluding_primary_key_token_stream = fields_without_primary_key.iter().map(|element| {
            let field_vis = &element.syn_field.vis;
            let field_ident = &element.field_ident;
            let postgresql_crud_value_declaration_token_stream = generate_postgresql_crud_value_declaration_token_stream(
                // &element.syn_field.ty
                &naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(&element.syn_field.ty)
            );
            quote::quote! {
                #field_attribute_serde_skip_serializing_if_option_is_none_token_stream
                #field_vis #field_ident: std::option::Option<#postgresql_crud_value_declaration_token_stream>
            }
        });
        quote::quote! {
            #derive_debug_serde_serialize_serde_deserialize
            pub struct #ident_options_upper_camel_case {
                #field_option_primary_key_token_stream,
                #(#fields_options_excluding_primary_key_token_stream),*
            }
        }
    };


    // let from_ident_for_ident_postgresql_json_type_options_to_read_token_stream = {
    //     let postgresql_crud_value_initialization_token_stream = generate_postgresql_crud_value_initialization_token_stream(&quote::quote! {
    //         #primary_key_inner_type_token_stream::#from_snake_case(#value_snake_case.#primary_key_field_ident.0)
    //     });
    //     let ident_option_variant_primary_key_token_stream = quote::quote! {
    //         #primary_key_field_ident: Some(#postgresql_crud_value_initialization_token_stream),
    //     };
    //     let ident_option_variants_excluding_primary_key_token_stream = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().map(|element| {
    //         let field_ident = &element.field_ident;
    //         let inner_type_token_stream = &element.inner_type_token_stream;
    //         let postgresql_crud_value_initialization_token_stream = generate_postgresql_crud_value_initialization_token_stream(&match &element.option_generic {
    //             Some(value) => {
    //                 let generic_option_string_reader_token_stream = value.reader_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| {
    //                     panic!(
    //                         "{} {}",
    //                         &value.reader_upper_camel_case_stringified,
    //                         constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE
    //                     )
    //                 });
    //                 let generic_option_string_options_token_stream = value.options_to_read_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| {
    //                     panic!(
    //                         "{} {}",
    //                         &value.options_to_read_upper_camel_case_stringified,
    //                         constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE
    //                     )
    //                 });
    //                 quote::quote! {
    //                     #inner_type_token_stream (
    //                         sqlx::types::Json(
    //                             #generic_option_string_reader_token_stream(
    //                                 #generic_option_string_options_token_stream::#from_snake_case(#value_snake_case.#field_ident.0.0.0)
    //                             )
    //                         )
    //                     )
    //                 }
    //             }
    //             None => quote::quote! {#inner_type_token_stream::#from_snake_case(#value_snake_case.#field_ident.0)},
    //         });
    //         quote::quote! {
    //             #field_ident: Some(#postgresql_crud_value_initialization_token_stream)
    //         }
    //     });
    //     quote::quote! {
    //         impl std::convert::From<#ident> for #ident_options_upper_camel_case {
    //             fn from(value: #ident) -> Self {
    //                 Self {
    //                     #ident_option_variant_primary_key_token_stream
    //                     #(#ident_option_variants_excluding_primary_key_token_stream),*
    //                 }
    //             }
    //         }
    //     }
    // };
    let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementSnakeCase;
    let postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementCall;
    let postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudAllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementCall;

    let ident_column_upper_camel_case = naming::parameter::SelfColumnUpperCamelCase::from_tokens(&ident);
    let column_token_stream = {
        let ident_column_token_stream = {
            let variants = fields.iter().map(|element| {
                let serialize_deserialize_ident_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
                let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                let type_path_column_upper_camel_case = naming::parameter::SelfColumnUpperCamelCase::from_type_last_segment(&element.syn_field.ty);
                quote::quote! {
                    #[serde(rename(serialize = #serialize_deserialize_ident_token_stream, deserialize = #serialize_deserialize_ident_token_stream))]
                    #field_ident_upper_camel_case_token_stream(
                        #type_path_column_upper_camel_case
                    )
                }
            });
            quote::quote! {
                #[derive(
                    #debug_upper_camel_case,
                    #serde_serialize,
                    #serde_deserialize,
                    PartialEq,
                    Clone,
                )]
                pub enum #ident_column_upper_camel_case {
                    #(#variants),*
                }
            }
        };
        let impl_std_fmt_display_for_ident_column_token_stream = {
            // let display_variants = syn_field_with_additional_info_fields_named.iter().map(|element| {
            //     let field_ident_stringified = element.field_ident.to_string();
            //     let field_ident_double_quotes_token_stream= generate_quotes::double_quotes_token_stream(
            //         &field_ident_stringified,
            //         &proc_macro_name_upper_camel_case_ident_stringified,
            //     );
            //     let field_ident_upper_camel_case_token_stream = {
            //         let value = convert_case::Casing::to_case(&field_ident_stringified, convert_case::Case::UpperCamel);
            //         value.parse::<proc_macro2::TokenStream>()
            //         .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            //     };
            //     quote::quote! {
            //         Self::#field_ident_upper_camel_case_token_stream => write!(formatter, #field_ident_double_quotes_token_stream)
            //     }
            // });
            quote::quote! {
                impl std::fmt::Display for #ident_column_upper_camel_case {
                    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        // match self {
                        //     #(#display_variants),*
                        // }
                        write!(formatter, "{}", serde_json::to_string(&self).unwrap_or_else(|e|format!("cannot serialize into json: {e:?}")))
                    }
                }
            }
        };
        let impl_error_occurence_lib_to_std_string_string_for_ident_column_token_stream = {
            quote::quote! {
                //todo maybe reuse naming
                impl error_occurence_lib::ToStdStringString for #ident_column_upper_camel_case {
                    fn to_std_string_string(&self) -> #std_string_string {
                        format!("{self}")
                    }
                }
            }
        };
        let impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_column_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &ident_column_upper_camel_case,
            &{
                let elements_token_stream = fields.iter().map(|element| {
                    let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                    quote::quote! {
                        #ident_column_upper_camel_case::#field_ident_upper_camel_case_token_stream(#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream)
                    }
                });
                quote::quote! {vec![#(#elements_token_stream),*]}
            },
        );
        //todo this is temporary impl. maybe should write trait and implement different logic
        let pick_column_token_stream = {
            let fields_token_stream = fields.iter().map(|element| {
                let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
                quote::quote!{
                    Self::#field_ident_upper_camel_case(_) => #field_ident_double_quotes_token_stream.to_string()
                }
            });
            quote::quote!{
                impl #ident_column_upper_camel_case {
                    fn pick_column(&self) -> std::string::String {
                        match &self {
                            #(#fields_token_stream),*
                        }
                    }
                }
            }
        };
        quote::quote! {
            #ident_column_token_stream
            #impl_std_fmt_display_for_ident_column_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_ident_column_token_stream
            #impl_postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_ident_column_token_stream
            //todo this is temporary impl. maybe should write trait and implement different logic
            #pick_column_token_stream
        }
    };
    // println!("{column_token_stream}");
    let element_snake_case = naming::ElementSnakeCase;
    let acc_snake_case = naming::AccSnakeCase;
    let parameters_snake_case = naming::ParametersSnakeCase;
    let payload_snake_case = naming::PayloadSnakeCase;
    let select_snake_case = naming::SelectSnakeCase;
    #[derive(Debug)]
    struct SynVariantWrapper {
        variant: syn::Variant,
        status_code: std::option::Option<macros_helpers::status_code::StatusCode>,
    }
    impl SynVariantWrapper {
        fn get_syn_variant(&self) -> &syn::Variant {
            &self.variant
        }
        fn get_option_status_code(&self) -> &std::option::Option<macros_helpers::status_code::StatusCode> {
            &self.status_code
        }
    }
    let new_syn_variant_wrapper = |
        variant_name: &dyn std::fmt::Display,
        status_code: std::option::Option<macros_helpers::status_code::StatusCode>,
        fields: std::vec::Vec<(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute,&dyn std::fmt::Display, syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>)>
    | -> SynVariantWrapper {
        SynVariantWrapper {
            variant: syn::Variant {
                attrs: match &status_code {
                    Some(value) => vec![syn::Attribute {
                        pound_token: syn::token::Pound { spans: [proc_macro2::Span::call_site()] },
                        style: syn::AttrStyle::Outer,
                        bracket_token: syn::token::Bracket::default(),
                        meta: syn::Meta::Path(syn::Path {
                            leading_colon: None,
                            segments: {
                                let mut handle = syn::punctuated::Punctuated::new();
                                handle.push(syn::PathSegment {
                                    ident: proc_macro2::Ident::new(&naming::AsRefStrToSnakeCaseStringified::case(value), proc_macro2::Span::call_site()),
                                    arguments: syn::PathArguments::None,
                                });
                                handle
                            },
                        }),
                    }],
                    None => vec![],
                },
                ident: syn::Ident::new(&variant_name.to_string(), proc_macro2::Span::call_site()),
                fields: syn::Fields::Named(syn::FieldsNamed {
                    brace_token: syn::token::Brace::default(),
                    named: {
                        let mut handle = fields.into_iter().fold(syn::punctuated::Punctuated::new(), |mut acc, element| {
                            acc.push_value(syn::Field {
                                attrs: vec![syn::Attribute {
                                    pound_token: syn::token::Pound { spans: [proc_macro2::Span::call_site()] },
                                    style: syn::AttrStyle::Outer,
                                    bracket_token: syn::token::Bracket::default(),
                                    meta: syn::Meta::Path(syn::Path {
                                        leading_colon: None,
                                        segments: {
                                            let mut handle = syn::punctuated::Punctuated::new();
                                            handle.push(syn::PathSegment {
                                                ident: proc_macro2::Ident::new(macros_helpers::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&element.0), proc_macro2::Span::call_site()),
                                                arguments: syn::PathArguments::None,
                                            });
                                            handle
                                        },
                                    }),
                                }],
                                vis: syn::Visibility::Inherited,
                                mutability: syn::FieldMutability::None,
                                ident: Some(syn::Ident::new(&element.1.to_string(), proc_macro2::Span::call_site())),
                                colon_token: Some(syn::token::Colon { spans: [proc_macro2::Span::call_site()] }),
                                ty: syn::Type::Path(syn::TypePath {
                                    qself: None,
                                    path: syn::Path { leading_colon: None, segments: element.2 },
                                }),
                            });
                            acc.push_punct(syn::token::Comma { spans: [proc_macro2::Span::call_site()] });
                            acc
                        });
                        handle.push_value(macros_helpers::code_occurence_syn_field::code_occurence_syn_field());
                        handle
                    },
                }),
                discriminant: None,
            },
            status_code,
        }
    };
    let generate_initialization_token_stream = |syn_variant_wrapper: &SynVariantWrapper, file: &'static str, line: std::primitive::u32, column: std::primitive::u32| -> proc_macro2::TokenStream {
        let variant_ident = &syn_variant_wrapper.variant.ident;
        let fields_token_stream = if let syn::Fields::Named(value) = &syn_variant_wrapper.variant.fields {
            value.named.iter().enumerate().map(|(index, element)| {
                let field_ident = &element.ident;
                if *field_ident.as_ref().unwrap_or_else(|| panic!("{}", naming::FIELD_IDENT_IS_NONE)) == naming::CodeOccurenceSnakeCase.to_string() {
                    macros_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(file, line, column)
                } else {
                    let error_increment_snake_case = naming::parameter::ErrorSelfSnakeCase::from_display(&index);
                    quote::quote! {#field_ident: #error_increment_snake_case}
                }
            })
        } else {
            panic!(
                "syn::Fields::Named(value) != &self.variant.fields {}",
                constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE
            );
        };
        quote::quote! {
            #variant_ident {
                #(#fields_token_stream),*
            }
        }
    };
    let wrap_into_axum_response_token_stream = |axum_json_content_token_stream: &dyn quote::ToTokens, status_code_token_stream: &dyn quote::ToTokens| {
        //todo make explicit serde_json::to_vec https://docs.rs/serde_json/1.0.125/serde_json/fn.to_vec.html coz axum::Json() in case of error returns http response - and that is bad
        // proof https://docs.rs/axum/latest/src/axum/json.rs.html#182-210
        quote::quote! {
            let mut response = axum::response::IntoResponse::into_response(axum::Json(#axum_json_content_token_stream));
            *response.status_mut() = #status_code_token_stream;
            return response;
        }
    };
    let generate_operation_error_initialization_eprintln_response_creation_token_stream = |operation: &Operation, syn_variant_wrapper: &SynVariantWrapper, file: &'static str, line: std::primitive::u32, column: std::primitive::u32| {
        let try_operation_route_logic_error_named_upper_camel_case = naming::parameter::TrySelfRouteLogicErrorNamedUpperCamelCase::from_display(operation);
        let try_operation_route_logic_response_variants_upper_camel_case = naming::parameter::TrySelfRouteLogicResponseVariantsUpperCamelCase::from_display(operation);
        let syn_variant_initialization_token_stream = generate_initialization_token_stream(syn_variant_wrapper, file, line, column);
        let status_code_token_stream = syn_variant_wrapper
            .get_option_status_code()
            .unwrap_or_else(|| panic!("option_status_code is None"))
            .to_axum_http_status_code_token_stream();
        let wraped_into_axum_response_token_stream = wrap_into_axum_response_token_stream(&quote::quote! {#try_operation_route_logic_response_variants_upper_camel_case::#from_snake_case(#error_snake_case)}, &status_code_token_stream);
        quote::quote! {
            let #error_snake_case = #try_operation_route_logic_error_named_upper_camel_case::#syn_variant_initialization_token_stream;
            #eprintln_error_token_stream
            #wraped_into_axum_response_token_stream
        }
    };
    let error_0_token_stream = token_patterns::Error0;
    let error_1_token_stream = token_patterns::Error1;
    let error_2_token_stream = token_patterns::Error2;
    let error_3_token_stream = token_patterns::Error3;
    // let curly_brace_space_filter_snake_case_space_curly_braces_token_stream = quote::quote! {{ #filter_snake_case }};
    let generate_query_vec_column_token_stream = |
        // operation: &Operation
    | {
        let variants_token_stream = fields.iter().map(|element| {
            let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
            //todo refactor it. can be rewriten witout double match. just put in one struct with syn_angle_bracketed_generic_arguments and others
            let initialization_token_stream = 
            // match &element.option_generic {
            //     Some(value) => {
            //         let element_field_ident = &element.field_ident;
            //         let filter_as_field_ident_string_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}} as {}", &element_field_ident));
            //         let element_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element_field_ident);
            //         let generic_syn_variant_wrapper = new_syn_variant_wrapper(
            //             &value.generate_postgresql_json_type_from_vec_error_named_upper_camel_case_stringified,
            //             Some(macros_helpers::status_code::StatusCode::InternalServerError500),
            //             vec![(
            //                 macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
            //                 &value.generate_postgresql_json_type_from_vec_error_named_snake_case_stringified,
            //                 macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&value.generate_postgresql_json_type_from_vec_error_named_upper_camel_case_stringified]),
            //             )],
            //         );
            //         // let generic_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &generic_syn_variant_wrapper, file!(), line!(), column!());
            //         quote::quote! {
            //             #curly_brace_space_filter_snake_case_space_curly_braces_token_stream => format!(
            //                 #filter_as_field_ident_string_double_quotes_token_stream,//todo should support arrays or "key: array" is enough?
            //                 postgresql_crud::GeneratePostgresqlJsonTypeToRead::generate_postgresql_json_type_to_read_from_vec(
            //                     #filter_snake_case,
            //                     #element_field_ident_double_quotes_token_stream,
            //                     #element_field_ident_double_quotes_token_stream,
            //                 ) 
            //             )
            //         }
            //     }
            //     None => {
            //         let field_ident_string_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
            //         quote::quote! {=> #field_ident_string_double_quotes_token_stream.to_string()}
            //     }
            // };
            {
                let field_type = &element.syn_field.ty;
                let field_ident_string_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
                quote::quote! {
                    => <#field_type as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::column_query_part(
                        value,
                        #field_ident_string_double_quotes_token_stream
                    )
                }
            };
            quote::quote! {#ident_column_upper_camel_case::#field_ident_upper_camel_case_token_stream(value) #initialization_token_stream}
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        quote::quote! {
            {
                let mut #value_snake_case = #std_string_string::default();
                for #element_snake_case in &#parameters_snake_case.#payload_snake_case.#select_snake_case {
                    #value_snake_case.push_str(&match #element_snake_case {
                        #(#variants_token_stream),*
                    });
                    #value_snake_case.push_str(",");
                }
                let _ = #value_snake_case.pop();
                #value_snake_case
            }
        }
    };
    let sqlx_error_syn_punctuated_punctuated = macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["sqlx", "Error"]);
    let postgresql_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::PostgresqlUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &naming::PostgresqlSnakeCase, sqlx_error_syn_punctuated_punctuated.clone())],
    );
    // //todo find out how to declare lifetime on closures
    // //todo refactor as &[&'a SynRust...]
    let generate_self_fields_token_stream = |fields: &[&syn::Field]| -> std::vec::Vec<syn::Ident> {
        fields.iter()
        .map(|field| field.ident.as_ref().unwrap_or_else(|| panic!("{}", naming::FIELD_IDENT_IS_NONE)).clone())
        .collect()
    };
    #[derive(Debug, strum_macros::Display)]
    enum GeneratePostgresqlCrudAttribute {
        CreateManyAdditionalErrorVariants,
        CreateOneAdditionalErrorVariants,
        ReadManyAdditionalErrorVariants,
        ReadOneAdditionalErrorVariants,
        UpdateManyAdditionalErrorVariants,
        UpdateOneAdditionalErrorVariants,
        DeleteManyAdditionalErrorVariants,
        DeleteOneAdditionalErrorVariants,
        CommonAdditionalErrorVariants,
        CreateManyAdditionalRouteLogic,
        CreateOneAdditionalRouteLogic,
        ReadManyAdditionalRouteLogic,
        ReadOneAdditionalRouteLogic,
        UpdateManyAdditionalRouteLogic,
        UpdateOneAdditionalRouteLogic,
        DeleteManyAdditionalRouteLogic,
        DeleteOneAdditionalRouteLogic,
        CommonAdditionalRouteLogic,
    }
    impl GeneratePostgresqlCrudAttribute {
        fn generate_path_to_attribute(self) -> std::string::String {
            let value = match self {
                Self::CreateManyAdditionalErrorVariants => naming::CreateManyAdditionalErrorVariantsSnakeCase.to_string(),
                Self::CreateOneAdditionalErrorVariants => naming::CreateOneAdditionalErrorVariantsSnakeCase.to_string(),
                Self::ReadManyAdditionalErrorVariants => naming::ReadManyAdditionalErrorVariantsSnakeCase.to_string(),
                Self::ReadOneAdditionalErrorVariants => naming::ReadOneAdditionalErrorVariantsSnakeCase.to_string(),
                Self::UpdateManyAdditionalErrorVariants => naming::UpdateManyAdditionalErrorVariantsSnakeCase.to_string(),
                Self::UpdateOneAdditionalErrorVariants => naming::UpdateOneAdditionalErrorVariantsSnakeCase.to_string(),
                Self::DeleteManyAdditionalErrorVariants => naming::DeleteManyAdditionalErrorVariantsSnakeCase.to_string(),
                Self::DeleteOneAdditionalErrorVariants => naming::DeleteOneAdditionalErrorVariantsSnakeCase.to_string(),
                Self::CommonAdditionalErrorVariants => naming::CommonAdditionalErrorVariantsSnakeCase.to_string(),
                Self::CreateManyAdditionalRouteLogic => naming::CreateManyAdditionalRouteLogicSnakeCase.to_string(),
                Self::CreateOneAdditionalRouteLogic => naming::CreateOneAdditionalRouteLogicSnakeCase.to_string(),
                Self::ReadManyAdditionalRouteLogic => naming::ReadManyAdditionalRouteLogicSnakeCase.to_string(),
                Self::ReadOneAdditionalRouteLogic => naming::ReadOneAdditionalRouteLogicSnakeCase.to_string(),
                Self::UpdateManyAdditionalRouteLogic => naming::UpdateManyAdditionalRouteLogicSnakeCase.to_string(),
                Self::UpdateOneAdditionalRouteLogic => naming::UpdateOneAdditionalRouteLogicSnakeCase.to_string(),
                Self::DeleteManyAdditionalRouteLogic => naming::DeleteManyAdditionalRouteLogicSnakeCase.to_string(),
                Self::DeleteOneAdditionalRouteLogic => naming::DeleteOneAdditionalRouteLogicSnakeCase.to_string(),
                Self::CommonAdditionalRouteLogic => naming::CommonAdditionalRouteLogicSnakeCase.to_string(),
            };
            format!("{}::{value}", naming::PostgresqlCrudSnakeCase)
        }
    }
    #[derive(Debug, Clone, Copy, naming::AsRefStrEnumWithUnitFieldsToUpperCamelCaseStringified, naming::AsRefStrEnumWithUnitFieldsToSnakeCaseStringified)]
    enum Operation {
        CreateMany,
        CreateOne,
        ReadMany,
        ReadOne,
        UpdateMany,
        UpdateOne,
        DeleteMany,
        DeleteOne,
    }
    // impl std::fmt::Display for Operation {
    //     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         match &self {
    //             Self::CreateMany => write!(formatter, "CreateMany"),
    //             Self::CreateOne => write!(formatter, "CreateOne"),
    //             Self::ReadMany => write!(formatter, "ReadMany"),
    //             Self::ReadOne => write!(formatter, "ReadOne"),
    //             Self::UpdateMany => write!(formatter, "UpdateMany"),
    //             Self::UpdateOne => write!(formatter, "UpdateOne"),
    //             Self::DeleteMany => write!(formatter, "DeleteMany"),
    //             Self::DeleteOne => write!(formatter, "DeleteOne"),
    //         }
    //     }
    // }
    impl Operation {
        const fn http_method(&self) -> OperationHttpMethod {
            match self {
                Self::CreateMany | Self::CreateOne | Self::ReadMany | Self::ReadOne => OperationHttpMethod::Post,
                Self::UpdateMany | Self::UpdateOne => OperationHttpMethod::Patch,
                Self::DeleteMany | Self::DeleteOne => OperationHttpMethod::Delete,
            }
        }
        fn std_vec_vec_self_payload_element_token_stream(&self) -> proc_macro2::TokenStream {
            let operation_payload_element_upper_camel_case = naming::parameter::SelfPayloadElementUpperCamelCase::from_display(self);
            quote::quote! {std::vec::Vec<#operation_payload_element_upper_camel_case>}
        }
        const fn desirable_status_code(&self) -> macros_helpers::status_code::StatusCode {
            match self {
                Self::CreateMany | Self::CreateOne => macros_helpers::status_code::StatusCode::Created201,
                Self::ReadMany | Self::ReadOne | Self::UpdateMany | Self::UpdateOne | Self::DeleteMany | Self::DeleteOne => macros_helpers::status_code::StatusCode::Ok200,
            }
        }
        fn generate_postgresql_crud_attribute_additional_error_variants(&self) -> GeneratePostgresqlCrudAttribute {
            match self {
                Self::CreateMany => GeneratePostgresqlCrudAttribute::CreateManyAdditionalErrorVariants,
                Self::CreateOne => GeneratePostgresqlCrudAttribute::CreateOneAdditionalErrorVariants,
                Self::ReadMany => GeneratePostgresqlCrudAttribute::ReadManyAdditionalErrorVariants,
                Self::ReadOne => GeneratePostgresqlCrudAttribute::ReadOneAdditionalErrorVariants,
                Self::UpdateMany => GeneratePostgresqlCrudAttribute::UpdateManyAdditionalErrorVariants,
                Self::UpdateOne => GeneratePostgresqlCrudAttribute::UpdateOneAdditionalErrorVariants,
                Self::DeleteMany => GeneratePostgresqlCrudAttribute::DeleteManyAdditionalErrorVariants,
                Self::DeleteOne => GeneratePostgresqlCrudAttribute::DeleteOneAdditionalErrorVariants,
            }
        }
        fn generate_postgresql_crud_attribute_additional_route_logic(&self) -> GeneratePostgresqlCrudAttribute {
            match self {
                Self::CreateMany => GeneratePostgresqlCrudAttribute::CreateManyAdditionalRouteLogic,
                Self::CreateOne => GeneratePostgresqlCrudAttribute::CreateOneAdditionalRouteLogic,
                Self::ReadMany => GeneratePostgresqlCrudAttribute::ReadManyAdditionalRouteLogic,
                Self::ReadOne => GeneratePostgresqlCrudAttribute::ReadOneAdditionalRouteLogic,
                Self::UpdateMany => GeneratePostgresqlCrudAttribute::UpdateManyAdditionalRouteLogic,
                Self::UpdateOne => GeneratePostgresqlCrudAttribute::UpdateOneAdditionalRouteLogic,
                Self::DeleteMany => GeneratePostgresqlCrudAttribute::DeleteManyAdditionalRouteLogic,
                Self::DeleteOne => GeneratePostgresqlCrudAttribute::DeleteOneAdditionalRouteLogic,
            }
        }
    }
    impl std::fmt::Display for Operation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                Self::CreateMany => write!(f, "CreateMany"),
                Self::CreateOne => write!(f, "CreateOne"),
                Self::ReadMany => write!(f, "ReadMany"),
                Self::ReadOne => write!(f, "ReadOne"),
                Self::UpdateMany => write!(f, "UpdateMany"),
                Self::UpdateOne => write!(f, "UpdateOne"),
                Self::DeleteMany => write!(f, "DeleteMany"),
                Self::DeleteOne => write!(f, "DeleteOne"),
            }
        }
    }
    #[derive(naming::AsRefStrEnumWithUnitFieldsToSnakeCaseStringified)]
    enum OperationHttpMethod {
        Post,
        Patch,
        Delete,
    }
    let ref_std_primitive_str = token_patterns::RefStdPrimitiveStr;
    let generate_options_try_from_sqlx_row_token_stream = |operation: &Operation| {
        let declaration_primary_key_token_stream = {
            let postgresql_crud_value_declaration_token_stream = generate_postgresql_crud_value_declaration_token_stream(&primary_key_field_type_read_upper_camel_case);
            quote::quote! {
                let mut #primary_key_field_ident: std::option::Option<#postgresql_crud_value_declaration_token_stream> = None;
            }
        };
        let declaration_excluding_primary_key_token_stream = fields_without_primary_key.iter().map(|element| {
            let field_ident = &element.field_ident;
            let postgresql_crud_value_declaration_token_stream = generate_postgresql_crud_value_declaration_token_stream(
                // &element.inner_type_with_generic_reader_token_stream
                // &element.syn_field.ty
                &naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(&element.syn_field.ty)
            );
            quote::quote! {
                let mut #field_ident: std::option::Option<#postgresql_crud_value_declaration_token_stream> = None;
            }
        });
        let assignment_variant_primary_key_token_stream = {
            let primary_key_field_ident_string_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&primary_key_field_ident);
            let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!());
            let postgresql_crud_value_initialization_token_stream = generate_postgresql_crud_value_initialization_token_stream(&quote::quote! {
                // #primary_key_inner_type_token_stream(
                    #value_snake_case
                // )
            });
            quote::quote! {
                #ident_column_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(_) => match sqlx::Row::try_get::<
                    #primary_key_field_type_read_upper_camel_case,
                    #ref_std_primitive_str
                >(
                    &#value_snake_case,
                    #primary_key_field_ident_string_double_quotes_token_stream
                ) {
                    Ok(#value_snake_case) => {
                        #primary_key_field_ident = Some(#postgresql_crud_value_initialization_token_stream);
                    },
                    Err(#error_0_token_stream) => {
                        #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    }
                }
            }
        };
        let assignment_variants_excluding_primary_key_token_stream = fields_without_primary_key.iter().map(|element| {
            let field_ident = &element.field_ident;
            let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
            let maybe_generic_filter_token_stream = proc_macro2::TokenStream::new();
            // if element.option_generic.is_some() {
            //     &curly_brace_space_filter_snake_case_space_curly_braces_token_stream
            // } else {
            //     &proc_macro2::TokenStream::new()
            // };
            // let original_type_with_generic_reader_token_stream = &element.syn_field.ty;//&element.original_type_with_generic_reader_token_stream;
            let field_ident_string_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
            // let inner_type_token_stream = &element.inner_type_token_stream;
            let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!());
            let postgresql_crud_value_initialization_token_stream = generate_postgresql_crud_value_initialization_token_stream(&{
                // let inner_type_token_stream = &element.syn_field.ty;//&element.inner_type_token_stream;
                // let original_wrapper_type_token_stream = &element.original_wrapper_type_token_stream;
                // match postgresql_crud_common::SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::from(&postgresql_crud_common::SupportedSqlxPostgresType::from(&element.rust_sqlx_map_to_postgres_type_variant)) {
                //     postgresql_crud_common::SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::SqlxPostgresType(_) => quote::quote! {
                //         #inner_type_token_stream(#value_snake_case)
                //     },
                //     postgresql_crud_common::SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::OptionSupportedSqlxPostgresType(_) => quote::quote! {
                //         #inner_type_token_stream(match #value_snake_case {
                //             Some(#value_snake_case) => Some(#original_wrapper_type_token_stream(#value_snake_case)),
                //             None => None
                //         })
                //     },
                // }
                quote::quote! {
                    // #inner_type_token_stream(
                        #value_snake_case
                    // )
                }
            });
            let element_field_type_read_upper_camel_case_token_stream = &naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(&element.syn_field.ty);
            quote::quote! {
                #ident_column_upper_camel_case::#field_ident_upper_camel_case_token_stream #maybe_generic_filter_token_stream(_) => match sqlx::Row::try_get::<
                    // #original_type_with_generic_reader_token_stream,
                    #element_field_type_read_upper_camel_case_token_stream,
                    #ref_std_primitive_str
                >(
                    &#value_snake_case,
                    #field_ident_string_double_quotes_token_stream
                ) {
                    Ok(#value_snake_case) => {
                        #field_ident = Some(#postgresql_crud_value_initialization_token_stream);
                    },
                    Err(#error_0_token_stream) => {
                        #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    }
                }
            }
        }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
        let option_fields_initiation_token_stream = generate_self_fields_token_stream(&fields.iter().map(|element| &element.syn_field).collect::<std::vec::Vec<&syn::Field>>());
        quote::quote! {
            #declaration_primary_key_token_stream
            #(#declaration_excluding_primary_key_token_stream)*
            for #element_snake_case in &#parameters_snake_case.#payload_snake_case.#select_snake_case {
                match #element_snake_case {
                    #assignment_variant_primary_key_token_stream,
                    #(#assignment_variants_excluding_primary_key_token_stream),*
                }
            }
            #ident_options_upper_camel_case {
                #(#option_fields_initiation_token_stream),*
            }
        }
    };
    let column_snake_case = naming::ColumnSnakeCase;
    let order_snake_case = naming::OrderSnakeCase;
    let order_by_upper_camel_case = naming::OrderByUpperCamelCase;
    let postgresql_crud_order_by_token_stream = quote::quote! {#postgresql_crud_snake_case::#order_by_upper_camel_case};
    let postgresql_crud_order_token_stream = quote::quote! {#postgresql_crud_snake_case::Order};
    let allow_methods_token_stream = {
        let http_method_token_stream = quote::quote! {http::Method};
        quote::quote! {
            pub const ALLOW_METHODS: [#http_method_token_stream;4] = [
                #http_method_token_stream::GET,
                #http_method_token_stream::POST,
                #http_method_token_stream::PATCH,
                #http_method_token_stream::DELETE
            ];//todo new axum version does not support it or something - find out
        }
    };
    let ident_column_read_permission_token_stream = {
        let derive_debug_clone_copy = token_patterns::DeriveDebugCloneCopy;
        let ident_column_read_permission_upper_camel_case = naming::parameter::SelfColumnReadPermissionUpperCamelCase::from_display(&ident);
        let fields_permission_token_stream = fields.iter().map(|element| {
            let field_ident = &element.field_ident;
            //todo permissions for json 
            quote::quote! {
                #field_ident: std::primitive::bool
            }
        });
        quote::quote! {
            #derive_debug_clone_copy
            pub struct #ident_column_read_permission_upper_camel_case {
                #(#fields_permission_token_stream),*
            }
        }
    };
    // let reexport_postgresql_sqlx_column_types_token_stream = syn_field_with_additional_info_fields_named.iter().map(|element| {
    //     let inner_type_token_stream = &element.inner_type_token_stream;
    //     quote::quote! {pub use #inner_type_token_stream;}
    // });
    let derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema = token_patterns::DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
    let derive_debug = token_patterns::DeriveDebug;
    // //todo replace it with postgresql_crud::Value
    let create_table_if_not_exists_function_token_stream = {
        let pool_snake_case = naming::PoolSnakeCase;
        let create_table_if_not_exists_double_quotes_token_stream = {
            let acc = {
                let mut acc = std::string::String::new();
                fields.iter().for_each(|_| {
                    acc.push_str("{},");
                });
                let _ = acc.pop();
                acc
            };
            generate_quotes::double_quotes_token_stream(&format!("create table if not exists {ident_snake_case_stringified} ({acc})"))
        };
        let serde_json_to_string_schemars_schema_for_generic_unwrap_token_stream = {
            let generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_token_stream = |field_type: &syn::Type, field_ident: &syn::Ident, is_primary_key: std::primitive::bool|{
                let is_primary_key_token_stream: &dyn quote::ToTokens = if is_primary_key {
                    &naming::TrueSnakeCase
                }
                else {
                    &naming::FalseSnakeCase
                };
                let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                //todo reuse create_table_column_query_part token stream
                quote::quote!{
                    #field_type ::create_table_column_query_part(&#field_ident_double_quotes_token_stream, #is_primary_key_token_stream)
                }
            };
            let mut acc = vec![generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_token_stream(&primary_key_field.syn_field.ty, &primary_key_field.field_ident, true)];
            fields_without_primary_key.iter().for_each(|element|{
                acc.push(generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_token_stream(&element.syn_field.ty, &element.field_ident, false));
            });
            acc
        };
        quote::quote! {
            pub async fn create_table_if_not_exists(#pool_snake_case: &sqlx::Pool<sqlx::Postgres>) {
                let create_extension_if_not_exists_pg_jsonschema_query_stringified = "create extension if not exists pg_jsonschema";
                println!("{create_extension_if_not_exists_pg_jsonschema_query_stringified}");
                let _ = sqlx::query(create_extension_if_not_exists_pg_jsonschema_query_stringified).execute(#pool_snake_case).await.unwrap();
                let create_extension_if_not_exists_uuid_ossp_query_stringified = "create extension if not exists \"uuid-ossp\"";
                println!("{create_extension_if_not_exists_uuid_ossp_query_stringified}");
                let _ = sqlx::query(create_extension_if_not_exists_uuid_ossp_query_stringified).execute(#pool_snake_case).await.unwrap();
                let create_table_if_not_exists_query_stringified = format!(
                    #create_table_if_not_exists_double_quotes_token_stream,
                    #(#serde_json_to_string_schemars_schema_for_generic_unwrap_token_stream),*
                );
                println!("{create_table_if_not_exists_query_stringified}");
                let _ = sqlx::query(&create_table_if_not_exists_query_stringified).execute(#pool_snake_case).await.unwrap();
            }
        }
    };
    let query_string_snake_case = naming::QueryStringSnakeCase;
    let binded_query_snake_case = naming::BindedQuerySnakeCase;
    let rollback_snake_case = naming::RollbackSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let update_snake_case = naming::UpdateSnakeCase;
    let set_snake_case = naming::SetSnakeCase;
    let insert_snake_case = naming::InsertSnakeCase;
    let into_snake_case = naming::IntoSnakeCase;
    let values_snake_case = naming::ValuesSnakeCase;
    let delete_snake_case = naming::DeleteSnakeCase;
    let where_snake_case = naming::WhereSnakeCase;
    let use_postgresql_crud_try_stream_ext_token_stream = quote::quote! {use #postgresql_crud_snake_case::TryStreamExt};
    let returning_snake_case = naming::ReturningSnakeCase;
    let returning_primary_key_stringified = format!("{returning_snake_case} {primary_key_field_ident}");
    let returning_primary_key_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&returning_primary_key_stringified);
    // let postgresql_crud_snake_case = naming::PostgresqlCrudSnakeCase;
    let std_string_string_syn_punctuated_punctuated = macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["std", "string", "String"]);
    let checked_add_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::CheckedAddUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        std::vec::Vec::<(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute, &'static dyn std::fmt::Display, syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>)>::default(),
    );
    let row_and_rollback_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::RowAndRollbackUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![
            (macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &naming::RowSnakeCase, sqlx_error_syn_punctuated_punctuated.clone()),
            (macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &rollback_snake_case, sqlx_error_syn_punctuated_punctuated.clone()),
        ],
    );
    let std_vec_vec_primary_key_field_type_to_update_syn_punctuated_punctuated = {
        if let syn::Type::Path(value) = &primary_key_field.syn_field.ty {
            if let Some(last_path_segment) = value.path.segments.last() {
                let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                handle.push_value(syn::PathSegment {
                    ident: proc_macro2::Ident::new("std", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                });
                handle.push_punct(syn::token::PathSep {
                    spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                });
                handle.push_value(syn::PathSegment {
                    ident: proc_macro2::Ident::new("vec", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                });
                handle.push_punct(syn::token::PathSep {
                    spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                });
                handle.push_value(syn::PathSegment {
                    ident: proc_macro2::Ident::new("Vec", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: syn::token::Lt { spans: [proc_macro2::Span::call_site()] },
                        args: {
                            let mut handle = syn::punctuated::Punctuated::<syn::GenericArgument, syn::token::Comma>::new();
                            handle.push(syn::GenericArgument::Type(syn::Type::Path(syn::TypePath {
                                qself: None,
                                path: syn::Path {
                                    leading_colon: None,
                                    segments: {
                                        let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                                        for element in value.path.segments.iter().rev().skip(1).rev() {
                                            handle.push_value(element.clone());
                                            handle.push_punct(syn::token::PathSep {
                                                spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                                            });
                                        }
                                        handle.push_value(syn::PathSegment {
                                            ident: proc_macro2::Ident::new(
                                                &naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&last_path_segment.ident).to_string(),
                                                proc_macro2::Span::call_site()
                                            ),
                                            arguments: syn::PathArguments::None,
                                        });
                                        handle
                                    },
                                },
                            })));
                            handle
                        },
                        gt_token: syn::token::Gt { spans: [proc_macro2::Span::call_site()] },
                    }),
                });
                handle
            }
            else {
                panic!("no last path segment");
            }
        }
        else {
            panic!("primary key syn::Type in not syn::Type::Path");
        }
    };
    let std_vec_vec_primary_key_field_type_to_delete_syn_punctuated_punctuated = {
        if let syn::Type::Path(value) = &primary_key_field.syn_field.ty {
            if let Some(last_path_segment) = value.path.segments.last() {
                let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                handle.push_value(syn::PathSegment {
                    ident: proc_macro2::Ident::new("std", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                });
                handle.push_punct(syn::token::PathSep {
                    spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                });
                handle.push_value(syn::PathSegment {
                    ident: proc_macro2::Ident::new("vec", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                });
                handle.push_punct(syn::token::PathSep {
                    spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                });
                handle.push_value(syn::PathSegment {
                    ident: proc_macro2::Ident::new("Vec", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: syn::token::Lt { spans: [proc_macro2::Span::call_site()] },
                        args: {
                            let mut handle = syn::punctuated::Punctuated::<syn::GenericArgument, syn::token::Comma>::new();
                            handle.push(syn::GenericArgument::Type(syn::Type::Path(syn::TypePath {
                                qself: None,
                                path: syn::Path {
                                    leading_colon: None,
                                    segments: {
                                        let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                                        for element in value.path.segments.iter().rev().skip(1).rev() {
                                            handle.push_value(element.clone());
                                            handle.push_punct(syn::token::PathSep {
                                                spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                                            });
                                        }
                                        handle.push_value(syn::PathSegment {
                                            ident: proc_macro2::Ident::new(
                                                &naming::parameter::SelfToDeleteUpperCamelCase::from_tokens(&last_path_segment.ident).to_string(),
                                                proc_macro2::Span::call_site()
                                            ),
                                            arguments: syn::PathArguments::None,
                                        });
                                        handle
                                    },
                                },
                            })));
                            handle
                        },
                        gt_token: syn::token::Gt { spans: [proc_macro2::Span::call_site()] },
                    }),
                });
                handle
            }
            else {
                panic!("no last path segment");
            }
        }
        else {
            panic!("primary key syn::Type in not syn::Type::Path");
        }
    };
    let non_existing_primary_keys_to_update_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NonExistingPrimaryKeysUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString,
            &naming::NonExistingPrimaryKeysSnakeCase,
            std_vec_vec_primary_key_field_type_to_update_syn_punctuated_punctuated.clone(),
        )],
    );
    let non_existing_primary_keys_to_update_and_rollback_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NonExistingPrimaryKeysAndRollbackUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString,
                &naming::NonExistingPrimaryKeysSnakeCase,
                std_vec_vec_primary_key_field_type_to_update_syn_punctuated_punctuated.clone(),
            ),
            (macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &rollback_snake_case, sqlx_error_syn_punctuated_punctuated.clone()),
        ],
    );
    let non_existing_primary_keys_to_delete_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NonExistingPrimaryKeysUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString,
            &naming::NonExistingPrimaryKeysSnakeCase,
            std_vec_vec_primary_key_field_type_to_delete_syn_punctuated_punctuated.clone(),
        )],
    );
    let non_existing_primary_keys_to_delete_and_rollback_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NonExistingPrimaryKeysAndRollbackUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString,
                &naming::NonExistingPrimaryKeysSnakeCase,
                std_vec_vec_primary_key_field_type_to_delete_syn_punctuated_punctuated.clone(),
            ),
            (macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &rollback_snake_case, sqlx_error_syn_punctuated_punctuated.clone()),
        ],
    );
    let sqlx_query_sqlx_postgres_token_stream = quote::quote! {sqlx::query::<sqlx::Postgres>};
    //todo reuse BindQuery path
    let postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream = quote::quote! {#postgresql_crud_snake_case::BindQuery::bind_value_to_query};
    //todo rename
    let crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream = quote::quote! {#postgresql_crud_snake_case::BindQuery::try_generate_bind_increments};
    let increment_snake_case = naming::IncrementSnakeCase;
    let increment_initialization_token_stream = quote::quote! {let mut #increment_snake_case: std::primitive::u64 = 0;};
    let and_snake_case = naming::AndSnakeCase;
    let order_by_snake_case = naming::OrderBySnakeCase;
    let in_snake_case = naming::InSnakeCase;
    let response_snake_case = naming::ResponseSnakeCase;
    let status_code_snake_case = naming::StatusCodeSnakeCase;
    let body_snake_case = naming::BodySnakeCase;
    
    // let std_vec_vec_primary_key_field_type_to_create_token_stream = quote::quote! {std::vec::Vec::<#postgresql_type_primary_key_field_type_to_create_upper_camel_case>};
    let std_vec_vec_primary_key_field_type_read_token_stream = quote::quote! {std::vec::Vec::<#primary_key_field_type_read_upper_camel_case>};
    let std_vec_vec_primary_key_field_type_to_update_token_stream = quote::quote! {std::vec::Vec::<#primary_key_field_type_update_upper_camel_case>};
    let std_vec_vec_primary_key_field_type_to_delete_token_stream = quote::quote! {std::vec::Vec::<#primary_key_field_type_to_delete_upper_camel_case>};

    let std_vec_vec_struct_options_ident_token_stream = quote::quote! {std::vec::Vec::<#ident_options_upper_camel_case>};
    // //todo rename not_unique_column to something what mean json tree getter too
    let not_unique_column_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NotUniqueColumnUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
            &naming::NotUniqueColumnSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&ident_column_upper_camel_case.to_string()]),
        )],
    );
    // let json_ident_column_upper_camel_case_stringified = naming::parameter::SelfColumnUpperCamelCase::from_tokens(&ident).to_string();// format!("{ident}{}", naming::ColumnUpperCamelCase);
    // let empty_column_json_reader_syn_variant_wrapper = new_syn_variant_wrapper(
    //     &naming::EmptyColumnJsonReaderUpperCamelCase,
    //     Some(macros_helpers::status_code::StatusCode::InternalServerError500),
    //     vec![(
    //         macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
    //         &naming::EmptyColumnJsonReaderSnakeCase,
    //         macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&json_ident_column_upper_camel_case_stringified]),
    //     )],
    // );
    // let not_unique_column_json_reader_syn_variant_wrapper = new_syn_variant_wrapper(
    //     &naming::NotUniqueColumnJsonReaderUpperCamelCase,
    //     Some(macros_helpers::status_code::StatusCode::InternalServerError500),
    //     vec![(
    //         macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
    //         &naming::NotUniqueColumnJsonReaderSnakeCase,
    //         macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&json_ident_column_upper_camel_case_stringified]),
    //     )],
    // );
    let serde_json_to_string_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::SerdeJsonToStringUpperCamelCase,
        None,
        vec![(
            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
            &naming::SerdeJsonToStringSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"]),
        )],
    );
    let failed_to_get_response_text_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::FailedToGetResponseTextUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &status_code_snake_case,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["http", "StatusCode"]),
            ),
            (
                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming::HeadersSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "header", "HeaderMap"]),
            ),
            (
                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming::ReqwestSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "Error"]),
            ),
        ],
    );
    let deserialize_response_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::DeserializeResponseUpperCamelCase,
        None,
        vec![
            (
                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &status_code_snake_case,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["http", "StatusCode"]),
            ),
            (
                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming::HeadersSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "header", "HeaderMap"]),
            ),
            (
                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
                &naming::ResponseTextSnakeCase,
                std_string_string_syn_punctuated_punctuated.clone(),
            ),
            (
                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &naming::SerdeSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"]),
            ),
        ],
    );
    let reqwest_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::ReqwestUpperCamelCase,
        None,
        vec![(
            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
            &naming::ReqwestSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "Error"]),
        )],
    );
    let check_body_size_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::CheckBodySizeUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
            &naming::CheckBodySizeSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                &[&postgresql_crud_snake_case.to_string(), "check_body_size", &naming::CheckBodySizeErrorNamedUpperCamelCase.to_string()]
            ),
        )],
    );
    let serde_json_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::SerdeJsonUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
            &naming::SerdeJsonSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"]),
        )],
    );
    let bind_query_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::BindQueryUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![(
            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
            &naming::BindQuerySnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[postgresql_crud_snake_case_stringified, &naming::TryGenerateBindIncrementsErrorNamedUpperCamelCase.to_string()]),
        )],
    );
    let not_unique_primary_key_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NotUniquePrimaryKeyUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &naming::NotUniquePrimaryKeySnakeCase, {
            // if let syn::Type::Path(value) = &primary_key_field.syn_field.ty {
            //     if let Some(last_path_segment) = value.path.segments.last() {
            //         let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
            //         for element in value.path.segments.iter().rev().skip(1).rev() {
            //             handle.push_value(element.clone());
            //             handle.push_punct(syn::token::PathSep {
            //                 spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
            //             });
            //         }
            //         handle.push_value(syn::PathSegment {
            //             ident: proc_macro2::Ident::new(
            //                 &naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&last_path_segment.ident).to_string(),
            //                 proc_macro2::Span::call_site()
            //             ),
            //             arguments: syn::PathArguments::None,
            //         });
            //         handle
            //     }
            //     else {
            //         panic!("no last path segment");
            //     }
            // }
            // else {
            //     panic!("primary key syn::Type in not syn::Type::Path");
            // }
            if let syn::Type::Path(value) = &primary_key_field.syn_field.ty {
                value.path.segments.clone()
                //here
            }
            else {
                panic!("primary key syn::Type in not syn::Type::Path");
            }
        })],
    );
    let not_unique_primary_key_to_update_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NotUniquePrimaryKeyUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &naming::NotUniquePrimaryKeySnakeCase, {
            if let syn::Type::Path(value) = &primary_key_field.syn_field.ty {
                if let Some(last_path_segment) = value.path.segments.last() {
                    let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                    for element in value.path.segments.iter().rev().skip(1).rev() {
                        handle.push_value(element.clone());
                        handle.push_punct(syn::token::PathSep {
                            spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                        });
                    }
                    handle.push_value(syn::PathSegment {
                        ident: proc_macro2::Ident::new(
                            &naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&last_path_segment.ident).to_string(),
                            proc_macro2::Span::call_site()
                        ),
                        arguments: syn::PathArguments::None,
                    });
                    handle
                }
                else {
                    panic!("no last path segment");
                }
            }
            else {
                panic!("primary key syn::Type in not syn::Type::Path");
            }
        })],
    );
    let not_unique_primary_key_to_delete_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NotUniquePrimaryKeyUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &naming::NotUniquePrimaryKeySnakeCase, {
            if let syn::Type::Path(value) = &primary_key_field.syn_field.ty {
                if let Some(last_path_segment) = value.path.segments.last() {
                    let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                    for element in value.path.segments.iter().rev().skip(1).rev() {
                        handle.push_value(element.clone());
                        handle.push_punct(syn::token::PathSep {
                            spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                        });
                    }
                    handle.push_value(syn::PathSegment {
                        ident: proc_macro2::Ident::new(
                            &naming::parameter::SelfToDeleteUpperCamelCase::from_tokens(&last_path_segment.ident).to_string(),
                            proc_macro2::Span::call_site()
                        ),
                        arguments: syn::PathArguments::None,
                    });
                    handle
                }
                else {
                    panic!("no last path segment");
                }
            }
            else {
                panic!("primary key syn::Type in not syn::Type::Path");
            }
        })],
    );
    let no_payload_fields_primary_key_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NoPayloadFieldsPrimaryKeyUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &naming::NoPayloadFieldsPrimaryKeySnakeCase, {
            if let syn::Type::Path(value) = &primary_key_field.syn_field.ty {
                if let Some(last_path_segment) = value.path.segments.last() {
                    let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                    for element in value.path.segments.iter().rev().skip(1).rev() {
                        handle.push_value(element.clone());
                        handle.push_punct(syn::token::PathSep {
                            spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                        });
                    }
                    handle.push_value(syn::PathSegment {
                        ident: proc_macro2::Ident::new(
                            &naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&last_path_segment.ident).to_string(),
                            proc_macro2::Span::call_site()
                        ),
                        arguments: syn::PathArguments::None,
                    });
                    handle
                }
                else {
                    panic!("no last path segment");
                }
            }
            else {
                panic!("primary key syn::Type in not syn::Type::Path");
            }
            //
        })],
    );
    let common_http_request_syn_variants = {
        vec![
            serde_json_to_string_syn_variant_wrapper.get_syn_variant().clone(),
            failed_to_get_response_text_syn_variant_wrapper.get_syn_variant().clone(),
            deserialize_response_syn_variant_wrapper.get_syn_variant().clone(),
            reqwest_syn_variant_wrapper.get_syn_variant().clone(),
        ]
    };
    let generate_additional_error_variants = |syn_derive_input: &syn::DeriveInput, generate_postgresql_crud_attribute: GeneratePostgresqlCrudAttribute| -> std::vec::Vec<syn::Variant> {
        let generate_postgresql_crud_attribute_stringified = generate_postgresql_crud_attribute.to_string();
        let common_additional_error_variants_attribute_token_stream = macros_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(&syn_derive_input.attrs, &generate_postgresql_crud_attribute.generate_path_to_attribute());
        let value: syn::DeriveInput = syn::parse((*common_additional_error_variants_attribute_token_stream).clone().into()).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
        let value_ident_stringified = value.ident.to_string();
        assert!(
            value_ident_stringified == generate_postgresql_crud_attribute_stringified,
            "{value_ident_stringified} is not equal to {generate_postgresql_crud_attribute_stringified}"
        );
        let variants = if let syn::Data::Enum(data_enum) = value.data {
            data_enum.variants
        } else {
            panic!("value.data is not syn::Data::Enum");
        };
        variants.into_iter().collect()
    };
    let common_additional_error_variants = generate_additional_error_variants(&syn_derive_input, GeneratePostgresqlCrudAttribute::CommonAdditionalErrorVariants);
    let common_route_syn_variants = {
        let common_additional_error_variants_vec = common_additional_error_variants.iter().collect::<std::vec::Vec<&syn::Variant>>();
        let mut value = std::vec::Vec::with_capacity(common_additional_error_variants_vec.len() + common_additional_error_variants_vec.len());
        value.push(check_body_size_syn_variant_wrapper.get_syn_variant());
        value.push(postgresql_syn_variant_wrapper.get_syn_variant());
        value.push(serde_json_syn_variant_wrapper.get_syn_variant());
        // value.push(&bind_query_syn_variant);
        for element in common_additional_error_variants_vec {
            value.push(element);
        }
        value
    };
    let common_route_with_row_and_rollback_syn_variants = {
        let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 1);
        common_route_syn_variants.iter().for_each(|element| {
            value.push(*element);
        });
        value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
        value
    };
    let common_additional_route_logic_token_stream = macros_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(
        &syn_derive_input.attrs,
        &GeneratePostgresqlCrudAttribute::CommonAdditionalRouteLogic.generate_path_to_attribute()
    );
    let generate_fields_named_token_stream = |function: &dyn Fn(&SynFieldWrapper) -> proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let fields_token_stream = fields.iter().map(function);
        quote::quote! {#(#fields_token_stream),*}
    };
    let generate_fields_named_excluding_primary_key_token_stream = |function: &dyn Fn(&SynFieldWrapper) -> proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let fields_token_stream = fields_without_primary_key.iter().map(function);
        quote::quote! {#(#fields_token_stream),*}
    };
    let pub_fields_idents_std_option_option_std_vec_vec_where_inner_type_token_stream = generate_fields_named_token_stream(&|element: &SynFieldWrapper| -> proc_macro2::TokenStream {
        let field_ident = &element.field_ident;
        // let where_inner_type_with_generic_token_stream = &element.where_inner_type_with_generic_token_stream;
        let field_type_where_upper_camel_case = &naming::parameter::SelfWhereUpperCamelCase::from_type_last_segment(&element.syn_field.ty);
        quote::quote! {
            pub #field_ident: std::option::Option<#field_type_where_upper_camel_case>
        }
    });
    let generate_pub_handle_token_stream = |is_pub: bool| match is_pub {
        true => quote::quote! {pub},
        false => proc_macro2::TokenStream::new(),
    };
    //todo remove?
    // let generate_primary_key_inner_type_handle_token_stream = |is_original: bool| match is_original {
    //     true => &primary_key_inner_type_token_stream,
    //     false => &primary_key_inner_type_token_stream,
    // };
    let pub_handle_select_snake_case_std_vec_vec_ident_column_upper_camel_case_token_stream = {
        quote::quote! {pub #select_snake_case: std::vec::Vec<#ident_column_upper_camel_case>}
    };
    let generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream = |primary_key_type_token_stream: &dyn quote::ToTokens|{
        let is_pub = true;
        let pub_handle_token_stream = generate_pub_handle_token_stream(is_pub);
        // let primary_key_inner_type_handle_token_stream = generate_primary_key_inner_type_handle_token_stream(is_pub);
        quote::quote! {#pub_handle_token_stream #primary_key_field_ident: #primary_key_type_token_stream}
    };
    // let generate_filter_not_unique_column_route_logic_token_stream = |operation: &Operation| {
    //     let not_unique_column_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &not_unique_column_syn_variant_wrapper, file!(), line!(), column!());
    //     let generics_generate_postgresql_json_type_from_vec_error_named_syn_variants_wrappers_token_stream = {
    //         // let generics_fiter_checks_token_stream = fields.iter().map(|element| {
    //         //     let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
    //         //     // if element.option_generic.is_some() {
    //         //     //     let empty_column_json_reader_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(
    //         //     //         &operation,
    //         //     //         &empty_column_json_reader_syn_variant_wrapper,
    //         //     //         file!(),
    //         //     //         line!(),
    //         //     //         column!()
    //         //     //     );
    //         //     //     let not_unique_column_json_reader_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(
    //         //     //         &operation,
    //         //     //         &not_unique_column_json_reader_syn_variant_wrapper,
    //         //     //         file!(),
    //         //     //         line!(),
    //         //     //         column!()
    //         //     //     );
    //         //     //     quote::quote! {
    //         //     //         #ident_column_upper_camel_case::#field_ident_upper_camel_case_token_stream #curly_brace_space_filter_snake_case_space_curly_braces_token_stream => {
    //         //     //             if #filter_snake_case.is_empty() {
    //         //     //                 let #error_0_token_stream = #element_snake_case.clone();//here
    //         //     //                 #empty_column_json_reader_syn_variant_error_initialization_eprintln_response_creation_token_stream
    //         //     //             }
    //         //     //             {
    //         //     //                 let mut #acc_snake_case = vec![];
    //         //     //                 for element_handle in #filter_snake_case {
    //         //     //                     match #acc_snake_case.contains(&element_handle) {
    //         //     //                         true => {
    //         //     //                             let #error_0_token_stream = #element_snake_case.clone();//here
    //         //     //                             #not_unique_column_json_reader_syn_variant_error_initialization_eprintln_response_creation_token_stream
    //         //     //                         },
    //         //     //                         false => {
    //         //     //                             #acc_snake_case.push(element_handle);
    //         //     //                         }
    //         //     //                     }
    //         //     //                 }
    //         //     //             }
    //         //     //         },
    //         //     //     }
    //         //     // } else {
    //         //     //     quote::quote! {#ident_column_upper_camel_case::#field_ident_upper_camel_case_token_stream => ()}
    //         //     // }
    //         //     quote::quote! {#ident_column_upper_camel_case::#field_ident_upper_camel_case_token_stream => ()}
    //         // }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
    //         // if contains_generic_json {
    //         //     quote::quote! {
    //         //         match &#element_snake_case {
    //         //             #(#generics_fiter_checks_token_stream),*
    //         //         }
    //         //     }
    //         // } else {
    //         //     proc_macro2::TokenStream::new()
    //         // }
    //         proc_macro2::TokenStream::new()
    //     };
    //     quote::quote! {
    //         let mut #acc_snake_case = std::vec::Vec::new();
    //         for #element_snake_case in &#value_snake_case.#select_snake_case {
    //             if #acc_snake_case.contains(&#element_snake_case) {
    //                 let #error_0_token_stream = #element_snake_case.clone();
    //                 #not_unique_column_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
    //             }
    //             else {
    //                 #generics_generate_postgresql_json_type_from_vec_error_named_syn_variants_wrappers_token_stream
    //                 #acc_snake_case.push(#element_snake_case);
    //             }
    //         }
    //     }
    // };
    // let generate_filter_not_unique_column_http_request_token_stream = |operation: &Operation| {
    //     let try_operation_error_named_upper_camel_case = naming::parameter::TrySelfErrorNamedUpperCamelCase::from_display(operation);
    //     let not_unique_column_syn_variant_wrapper_initialization_token_stream = generate_initialization_token_stream(&not_unique_column_syn_variant_wrapper, file!(), line!(), column!());
    //     quote::quote! {
    //         let mut #acc_snake_case = std::vec::Vec::new();
    //         for #element_snake_case in &#parameters_snake_case.#payload_snake_case.#select_snake_case {
    //             if #acc_snake_case.contains(&#element_snake_case) {
    //                 let #error_0_token_stream = #element_snake_case.clone();
    //                 return Err(#try_operation_error_named_upper_camel_case::#not_unique_column_syn_variant_wrapper_initialization_token_stream);
    //             } else {
    //                 #acc_snake_case.push(#element_snake_case);
    //             }
    //         }
    //     }
    // };
    let update_fields_token_stream = {
        let fields_named_excluding_primary_key_token_stream = generate_fields_named_excluding_primary_key_token_stream(&|element: &SynFieldWrapper| {
            let field_ident = &element.field_ident;
            let path_value_token_stream = {
                let value = format!("{}::{}", naming::PostgresqlCrudSnakeCase, naming::ValueUpperCamelCase);
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            // let inner_type_token_stream = &element.inner_type_with_generic_token_stream;
            let field_ident_update_upper_camel_case = &naming::parameter::SelfUpdateUpperCamelCase::from_type_last_segment(&element.syn_field.ty);
            quote::quote! {
                pub #field_ident: std::option::Option<#path_value_token_stream<#field_ident_update_upper_camel_case>>
            }
        });
        quote::quote! {
            pub #primary_key_field_ident: #primary_key_field_type_update_token_stream,
            #fields_named_excluding_primary_key_token_stream
        }
    };
    let generate_filter_no_payload_fields_token_stream = |operation: &Operation, source_token_stream: &dyn quote::ToTokens| {
        let no_payload_fields_primary_key_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &no_payload_fields_primary_key_syn_variant_wrapper, file!(), line!(), column!());
        let none_fields_named_excluding_primary_key_token_stream = fields_without_primary_key.iter().map(|_| naming::NoneUpperCamelCase);
        let match_fields_named_excluding_primary_key_token_stream = fields_without_primary_key.iter().map(|element| {
            let field_ident = &element.field_ident;
            quote::quote! {&#source_token_stream.#field_ident}
        });
        quote::quote! {
            if let (#(#none_fields_named_excluding_primary_key_token_stream),*) = (#(#match_fields_named_excluding_primary_key_token_stream),*) {
                let #error_0_token_stream = #source_token_stream.#primary_key_field_ident.clone();
                #no_payload_fields_primary_key_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
            }
        }
    };
    let executor_snake_case = naming::ExecutorSnakeCase;
    let generate_match_postgres_transaction_rollback_await_token_stream = |
        operation: &Operation,
        postgresql_file: &'static str,
        postgresql_line: std::primitive::u32,
        postgresql_column: std::primitive::u32,
        row_and_rollback_file: &'static str,
        row_and_rollback_line: std::primitive::u32,
        row_and_rollback_column: std::primitive::u32
    | {
        let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(
            operation,
            &postgresql_syn_variant_wrapper, postgresql_file,
            postgresql_line,
            postgresql_column
        );
        let row_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(
            operation,
            &row_and_rollback_syn_variant_wrapper,
            row_and_rollback_file,
            row_and_rollback_line,
            row_and_rollback_column
        );
        quote::quote! {
            match #executor_snake_case.#rollback_snake_case().await {
                Ok(_) => {
                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                }
                Err(#error_1_token_stream) => {
                    #row_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream
                }
            }
        }
    };
    let rows_snake_case = naming::RowsSnakeCase;
    let generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream = |
        operation: &Operation,
        postgresql_file: &'static str,
        postgresql_line: std::primitive::u32,
        postgresql_column: std::primitive::u32,
        row_and_rollback_file: &'static str,
        row_and_rollback_line: std::primitive::u32,
        row_and_rollback_column: std::primitive::u32
    | {
        let match_postgres_transaction_rollback_await_token_stream = generate_match_postgres_transaction_rollback_await_token_stream(
            operation,
            postgresql_file,
            postgresql_line,
            postgresql_column,
            row_and_rollback_file,
            row_and_rollback_line,
            row_and_rollback_column
        );
        quote::quote! {
            drop(#rows_snake_case);
            #match_postgres_transaction_rollback_await_token_stream
        }
    };
    let expected_primary_keys_snake_case = naming::ExpectedPrimaryKeysSnakeCase;
    let generate_non_existing_primary_keys_check_token_stream = |operation: &Operation, expected_primary_keys_token_stream: &dyn quote::ToTokens| {
        let non_existing_primary_keys_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(
            operation,
            //todo refactor
            match operation {
                Operation::UpdateMany => &non_existing_primary_keys_to_update_syn_variant_wrapper,
                Operation::DeleteMany => &non_existing_primary_keys_to_delete_syn_variant_wrapper,
                _ => panic!("operation is not UpdateMany or DeleteMany")
            },
            file!(),
            line!(),
            column!()
        );
        let non_existing_primary_keys_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(
            operation,
            match operation {
                Operation::UpdateMany => &non_existing_primary_keys_to_update_and_rollback_syn_variant_wrapper,
                Operation::DeleteMany => &non_existing_primary_keys_to_delete_and_rollback_syn_variant_wrapper,
                _ => panic!("operation is not UpdateMany or DeleteMany")
            },
            file!(),
            line!(),
            column!()
        );
        quote::quote! {
            let #error_0_token_stream = #expected_primary_keys_token_stream.into_iter().fold(std::vec::Vec::new(),|mut #acc_snake_case, #element_snake_case| {
                if let false = #value_snake_case.contains(&#element_snake_case) {
                    #acc_snake_case.push(#element_snake_case);
                }
                #acc_snake_case
            });
            if let false = #error_0_token_stream.is_empty() {
                match #executor_snake_case.#rollback_snake_case().await {
                    Ok(_) => {
                        #non_existing_primary_keys_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    }
                    Err(#error_1_token_stream) => {
                        #non_existing_primary_keys_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    }
                }
            }
        }
    };
    let not_unique_fields_syn_variants_wrappers = fields_without_primary_key.iter().map(|element| {
        new_syn_variant_wrapper(
            &naming::parameter::NotUniqueSelfUpperCamelCase::from_tokens(&element.field_ident),
            Some(not_unique_primary_key_syn_variant_wrapper.get_option_status_code().unwrap()),
            vec![(
                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                &element.field_ident.to_string(),
                {
                    if let syn::Type::Path(value) = &element.syn_field.ty {
                        value.path.segments.clone()
                    }
                    else {
                        panic!("primary key syn::Type in not syn::Type::Path");
                    }
                    // let mut value = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                    // value.push_value(syn::PathSegment {
                    //     ident: proc_macro2::Ident::new(&postgresql_crud_snake_case_stringified.to_string(), proc_macro2::Span::call_site()),
                    //     arguments: syn::PathArguments::None,
                    // });
                    // value.push_punct(syn::token::PathSep {
                    //     spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                    // });
                    // value.push_value(syn::PathSegment {
                    //     ident: proc_macro2::Ident::new(
                    //         // &postgresql_crud_common::SqlxPostgresType::from_supported_sqlx_postgres_type_removing_option(&postgresql_crud_common::SupportedSqlxPostgresType::from(&element.rust_sqlx_map_to_postgres_type_variant)).to_string(),
                    //         // &naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(&element.syn_field.ty).to_string()
                    //         "todo"
                    //         ,
                    //         proc_macro2::Span::call_site(),
                    //     ),
                    //     arguments: 
                    //     // match &element.option_generic {
                    //     //     Some(value) => syn::PathArguments::AngleBracketed((*value.syn_angle_bracketed_generic_arguments).clone()),
                    //     //     None => syn::PathArguments::None,
                    //     // }
                    //     syn::PathArguments::None
                    //     ,
                    // });
                    // value
                }
            )],
        )
    }).collect::<std::vec::Vec<SynVariantWrapper>>();
    let wrap_into_value_token_stream = |content_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            let #value_snake_case = {
                #content_token_stream
            };
        }
    };
    let generate_fetch_token_stream = |value_handle_token_stream: &dyn quote::ToTokens, try_next_error_initialization_token_stream: &dyn quote::ToTokens| {
        wrap_into_value_token_stream(&quote::quote! {
            let mut #rows_snake_case = #binded_query_snake_case.fetch(#executor_snake_case.as_mut());
            let mut #acc_snake_case = std::vec::Vec::new();
            while let Some(#value_snake_case) = match {
                #use_postgresql_crud_try_stream_ext_token_stream;
                #rows_snake_case.try_next()
            }.await
            {
                Ok(#value_snake_case) => match #value_snake_case {
                    Some(#value_snake_case) => #value_handle_token_stream,
                    None => None,
                },
                Err(#error_0_token_stream) => {
                    #try_next_error_initialization_token_stream
                }
            }
            {
                #acc_snake_case.push(#value_snake_case);
            }
            #acc_snake_case
        })
    };
    let generate_fetch_one_token_stream = |value_handle_token_stream: &dyn quote::ToTokens, fetch_one_error_initialization_token_stream: &dyn quote::ToTokens| {
        wrap_into_value_token_stream(&quote::quote! {
            match #binded_query_snake_case.fetch_one(#executor_snake_case.as_mut()).await {
                Ok(#value_snake_case) => {
                    #value_handle_token_stream
                },
                Err(#error_0_token_stream) => {
                    #fetch_one_error_initialization_token_stream
                }
            }
        })
    };
    let generate_sqlx_row_try_get_primary_key_token_stream = |
        sqlx_row_try_get_type_token_stream: &dyn quote::ToTokens,
        ok_token_stream: &dyn quote::ToTokens,
        err_token_stream: &dyn quote::ToTokens
    | {
        quote::quote! {
            match #sqlx_row::try_get::<
                // #primary_key_original_type_token_stream,
                #sqlx_row_try_get_type_token_stream,
                #ref_std_primitive_str
            >(&#value_snake_case, #primary_key_field_ident_double_quotes_token_stream) {
                Ok(#value_snake_case) => #ok_token_stream,
                Err(#error_0_token_stream) => {
                    #err_token_stream
                }
            }
        }
    };
    let wrap_content_into_postgresql_transaction_begin_commit_value_token_stream = |operation: &Operation, content_token_stream: &dyn quote::ToTokens| {
        let postgres_transaction_begin_token_stream = {
            let sqlx_acquire = token_patterns::SqlxAcquire;
            let begin_snake_case = naming::BeginSnakeCase;
            let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let mut #executor_snake_case = match #sqlx_acquire::#begin_snake_case(#executor_snake_case).await {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    }
                };
            }
        };
        let postgres_transaction_commit_token_stream = {
            let commit_snake_case = naming::CommitSnakeCase;
            let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                if let Err(#error_0_token_stream) = #executor_snake_case.#commit_snake_case().await {
                    #postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream
                }
            }
        };
        quote::quote! {
            #postgres_transaction_begin_token_stream
            #content_token_stream
            #postgres_transaction_commit_token_stream
            #value_snake_case
        }
    };
    let generate_error_occurence_variant_token_stream = |error_variant: &syn::Variant| -> proc_macro2::TokenStream {
        let variant_ident = &error_variant.ident;
        let fields_named = if let syn::Fields::Named(fields_named) = &error_variant.fields {
            fields_named
        } else {
            panic!("expected fields would be named");
        };
        let fields_mapped_into_token_stream = fields_named.named.iter().map(|field| {
            let field_ident = field.ident.as_ref().unwrap_or_else(|| panic!("{}", naming::FIELD_IDENT_IS_NONE));
            let error_occurence_attribute = if *field_ident == *naming::CodeOccurenceSnakeCase.to_string() {
                proc_macro2::TokenStream::new()
            } else {
                let mut error_occurence_attribute: Option<macros_helpers::error_occurence::ErrorOccurenceFieldAttribute> = None;
                for element in &field.attrs {
                    if element.path().segments.len() == 1 {
                        let segment = element.path().segments.first().unwrap_or_else(|| panic!("element.path().segments.get(0) is None"));
                        if let Ok(value) = {
                            use std::str::FromStr;
                            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::from_str(&segment.ident.to_string())
                        } {
                            match error_occurence_attribute {
                                Some(value) => panic!(
                                    "duplicated attributes ({}) are not supported",
                                    macros_helpers::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&value)
                                ),
                                None => {
                                    error_occurence_attribute = Some(value);
                                }
                            }
                        }
                    }
                }
                error_occurence_attribute.map_or_else(|| panic!("{variant_ident} no supported attribute"), |value| value.to_attribute_view_token_stream())
            };
            let field_type = &field.ty;
            quote::quote! {
                #error_occurence_attribute
                #field_ident: #field_type
            }
        });
        quote::quote! {
            #variant_ident {
                #(#fields_mapped_into_token_stream),*
            }
        }
    };
    let desirable_upper_camel_case = naming::DesirableUpperCamelCase;
    let generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = |
        operation: &Operation,
        desirable_type_token_stream: &dyn quote::ToTokens,
        type_variants_from_request_response_syn_variants: &std::vec::Vec<syn::Variant>
    | -> proc_macro2::TokenStream {
        let try_operation_route_logic_response_variants_upper_camel_case = naming::parameter::TrySelfRouteLogicResponseVariantsUpperCamelCase::from_display(operation);
        let try_operation_route_logic_response_variants_token_stream = {
            let variants_token_stream = type_variants_from_request_response_syn_variants
                .iter()
                .map(|element| macros_helpers::error_occurence::generate_serialize_deserialize_version_of_named_syn_variant(element));
            quote::quote! {
                #derive_debug_serde_serialize_serde_deserialize
                pub enum #try_operation_route_logic_response_variants_upper_camel_case {
                    #desirable_upper_camel_case(#desirable_type_token_stream),
                    #(#variants_token_stream),*
                }
            }
        };
        let try_operation_route_logic_error_named_upper_camel_case = naming::parameter::TrySelfRouteLogicErrorNamedUpperCamelCase::from_display(operation);
        let impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream = {
            let variants_token_stream = type_variants_from_request_response_syn_variants.iter().map(|element| {
                let variant_ident = &element.ident;
                let fields_named = if let syn::Fields::Named(fields_named) = &element.fields {
                    fields_named
                } else {
                    panic!("expected fields would be named");
                };
                let fields_mapped_into_token_stream = {
                    let fields_token_stream = fields_named.named.iter().map(|field| &field.ident);
                    quote::quote! {#(#fields_token_stream),*}
                };
                let try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case = naming::parameter::TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCase::from_display(operation);
                quote::quote! {
                    #try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case::#variant_ident {
                        #fields_mapped_into_token_stream
                    } => Self::#variant_ident {
                        #fields_mapped_into_token_stream
                    }
                }
            });
            let into_serialize_deserialize_version_snake_case = naming::IntoSerializeDeserializeVersionSnakeCase;
            macros_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                &try_operation_route_logic_error_named_upper_camel_case,
                &try_operation_route_logic_response_variants_upper_camel_case,
                &quote::quote! {
                    match #value_snake_case.#into_serialize_deserialize_version_snake_case() {
                        #(#variants_token_stream),*
                    }
                },
            )
        };
        let try_operation_route_logic_error_named_token_stream = {
            let variants_token_stream = type_variants_from_request_response_syn_variants.iter().map(generate_error_occurence_variant_token_stream);
            let derive_debug_this_error_error_occurence = token_patterns::DeriveDebugThisErrorErrorOccurence;
            quote::quote! {
                #derive_debug_this_error_error_occurence
                pub enum #try_operation_route_logic_error_named_upper_camel_case {
                    #(#variants_token_stream),*
                }
            }
        };
        quote::quote! {
            #try_operation_route_logic_response_variants_token_stream
            #impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream
            #try_operation_route_logic_error_named_token_stream
        }
    };
    let derive_debug_thiserror_error_occurence = token_patterns::DeriveDebugThiserrorErrorOccurence;
    let generate_parameters_pattern_token_stream = |operation: &Operation, payload_token_stream: proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let parameters_token_stream = {
            let operation_parameters_upper_camel_case = naming::parameter::SelfParametersUpperCamelCase::from_display(operation);
            let operation_payload_upper_camel_case = naming::parameter::SelfPayloadUpperCamelCase::from_display(operation);
            quote::quote! {
                #derive_debug
                pub struct #operation_parameters_upper_camel_case {//todo maybe not need additional info, so parameters wrapper potentially can be removed
                    pub #payload_snake_case: #operation_payload_upper_camel_case,
                }
            }
        };
        quote::quote! {
            #payload_token_stream
            #parameters_token_stream
        }
    };
    let generate_operation_payload_token_stream = |operation: &Operation, fields_token_stream: &dyn quote::ToTokens| -> proc_macro2::TokenStream {
        let operation_payload_upper_camel_case = naming::parameter::SelfPayloadUpperCamelCase::from_display(operation);
        quote::quote! {
            #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
            pub struct #operation_payload_upper_camel_case {
                #fields_token_stream
            }
        }
    };
    let generate_payload_and_payload_element_token_stream = |operation: &Operation, fields_token_stream: &dyn quote::ToTokens| -> proc_macro2::TokenStream {
        let operation_payload_element_token_stream = {
            let operation_payload_element_upper_camel_case = naming::parameter::SelfPayloadElementUpperCamelCase::from_display(operation);
            quote::quote! {
                #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
                pub struct #operation_payload_element_upper_camel_case {
                    #fields_token_stream
                }
            }
        };
        let operation_payload_token_stream = {
            let operation_payload_upper_camel_case = naming::parameter::SelfPayloadUpperCamelCase::from_display(operation);
            let std_vec_vec_operation_payload_element_token_stream = operation.std_vec_vec_self_payload_element_token_stream();
            quote::quote! {
                #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
                pub struct #operation_payload_upper_camel_case(pub #std_vec_vec_operation_payload_element_token_stream);
            }
        };
        quote::quote! {
            #operation_payload_element_token_stream
            #operation_payload_token_stream
        }
    };
    let generate_type_variants_from_request_response_syn_variants = |syn_variants: &std::vec::Vec<&syn::Variant>, operation: &Operation| -> std::vec::Vec<syn::Variant> {
        let mut type_variants_from_request_response_syn_variants = std::vec::Vec::new();
        for element in syn_variants {
            type_variants_from_request_response_syn_variants.push((*element).clone());
        }
        let operation_additional_error_variants = generate_additional_error_variants(&syn_derive_input, operation.generate_postgresql_crud_attribute_additional_error_variants());
        for element in operation_additional_error_variants {
            type_variants_from_request_response_syn_variants.push(element.clone());
        }
        type_variants_from_request_response_syn_variants
    };
    let generate_try_operation_error_named_token_stream = |operation: &Operation, common_http_request_syn_variants: &std::vec::Vec<syn::Variant>| -> proc_macro2::TokenStream {
        let try_operation_error_named_upper_camel_case = naming::parameter::TrySelfErrorNamedUpperCamelCase::from_display(operation);
        let syn_variants = {
            let mut value = std::vec::Vec::with_capacity(common_http_request_syn_variants.len() + 1);
            for element in common_http_request_syn_variants {
                value.push(element.clone());
            }
            value.push({
                let try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case = naming::parameter::TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCase::from_display(operation);
                new_syn_variant_wrapper(
                    &try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case,
                    None,
                    vec![(
                        macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString,
                        &naming::parameter::TrySelfRouteLogicErrorNamedWithSerializeDeserializeSnakeCase::from_display(operation),
                        macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case.to_string()]),
                    )],
                )
                .get_syn_variant()
                .clone()
            });
            value
        };
        let variants_token_stream = syn_variants.iter().map(generate_error_occurence_variant_token_stream);
        quote::quote! {
            #derive_debug_thiserror_error_occurence
            pub enum #try_operation_error_named_upper_camel_case {
                #(#variants_token_stream),*
            }
        }
    };
    let generate_try_operation_route_logic_token_stream = |operation: &Operation,
        common_additional_route_logic_token_stream: &dyn quote::ToTokens,
        parameters_logic_token_stream: &dyn quote::ToTokens,
        expected_updated_primary_keys_token_stream: &dyn quote::ToTokens,
        query_string_token_stream: &dyn quote::ToTokens,
        binded_query_token_stream: &dyn quote::ToTokens,
        postgresql_logic_token_stream: &dyn quote::ToTokens
    | -> proc_macro2::TokenStream {
        let try_operation_route_logic_snake_case = naming::parameter::TrySelfRouteLogicSnakeCase::from_display(operation);
        let request_snake_case = naming::RequestSnakeCase;
        let app_state_snake_case = naming::AppStateSnakeCase;
        let request_parts_preparation_token_stream = {
            let check_body_size_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = &generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &check_body_size_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let (parts, #body_snake_case) = #request_snake_case.into_parts();
                let headers = parts.headers;
                let body_bytes = match #postgresql_crud_snake_case::check_body_size::check_body_size(#body_snake_case, *#app_state_snake_case.get_maximum_size_of_http_body_in_bytes()).await {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        #check_body_size_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                    }
                };
            }
        };
        let additional_validators_token_stream = {
            let operation_additional_route_logic_token_stream = macros_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(&syn_derive_input.attrs, &operation.generate_postgresql_crud_attribute_additional_route_logic().generate_path_to_attribute());
            quote::quote! {
                #common_additional_route_logic_token_stream
                #operation_additional_route_logic_token_stream
            }
        };
        let try_operation_route_logic_response_variants_upper_camel_case = naming::parameter::TrySelfRouteLogicResponseVariantsUpperCamelCase::from_display(operation);
        let acquire_pool_and_connection_token_stream = {
            let pool_connection_snake_case = naming::PoolConnectionSnakeCase;
            let postgresql_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let mut #pool_connection_snake_case = match #app_state_snake_case.get_postgres_pool().acquire().await {//todo find out difference between acquire and try_acquire
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        #postgresql_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                    }
                };
                let #executor_snake_case = match sqlx::Acquire::acquire(&mut #pool_connection_snake_case).await {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        #postgresql_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                    }
                };
            }
        };
        let wraped_into_axum_response_token_stream = wrap_into_axum_response_token_stream(
            &quote::quote! {#try_operation_route_logic_response_variants_upper_camel_case::#desirable_upper_camel_case(#value_snake_case)},
            &operation.desirable_status_code().to_axum_http_status_code_token_stream(),
        );
        quote::quote! {
            // // #swagger_open_api_token_stream
            pub async fn #try_operation_route_logic_snake_case(
                #app_state_snake_case: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>,
                #request_snake_case: axum::extract::Request,
            ) -> axum::response::Response {
                #request_parts_preparation_token_stream
                #additional_validators_token_stream
                #parameters_logic_token_stream
                println!("{:#?}", #parameters_snake_case);
                #expected_updated_primary_keys_token_stream
                let #query_string_snake_case = #query_string_token_stream;
                println!("{}", #query_string_snake_case);
                let #binded_query_snake_case = {
                    #binded_query_token_stream
                };
                #acquire_pool_and_connection_token_stream
                let #value_snake_case = {
                    #postgresql_logic_token_stream
                };
                #wraped_into_axum_response_token_stream
            }
        }
    };
    let generate_parameters_logic_token_stream = |operation: &Operation, operation_payload_with_serialize_deserialize_check_token_stream: &dyn quote::ToTokens| -> proc_macro2::TokenStream {
        let body_bytes_snake_case = naming::BodyBytesSnakeCase;
        let operation_payload_upper_camel_case = naming::parameter::SelfPayloadUpperCamelCase::from_display(operation);
        let try_or_try_from_operation_payload_upper_camel_case_token_stream = {
            quote::quote! {#operation_payload_upper_camel_case::#from_snake_case(#value_snake_case)}
        };
        let serde_json_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &serde_json_syn_variant_wrapper, file!(), line!(), column!());
        let operation_parameters_upper_camel_case = naming::parameter::SelfParametersUpperCamelCase::from_display(operation);
        quote::quote! {
            let #parameters_snake_case = #operation_parameters_upper_camel_case {
                //todo maybe use serde json parsing instead of axum. (coz less info)
                #payload_snake_case: match serde_json::from_slice::<#operation_payload_upper_camel_case>(
                    &#body_bytes_snake_case,
                ) {
                    Ok(#value_snake_case) => {
                        let #value_snake_case = #try_or_try_from_operation_payload_upper_camel_case_token_stream;
                        #operation_payload_with_serialize_deserialize_check_token_stream
                        #value_snake_case
                    },
                    Err(#error_0_token_stream) => {
                        #serde_json_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                    }
                },
            };
        }
    };
    let pub_field_ident_field_type_fields_named_excluding_primary_key_token_stream = 
    
    generate_fields_named_token_stream
    // generate_fields_named_excluding_primary_key_token_stream
    (&|element: &SynFieldWrapper| {
        let field_ident = &element.field_ident;
        // let field_type = &element.syn_field.ty;
        let field_type_create = naming::parameter::SelfCreateUpperCamelCase::from_type_last_segment(&element.syn_field.ty);
        // let inner_type_token_stream = &element.inner_type_with_generic_token_stream;
        // let field_type_token_stream = match &element.option_generic {
        //     Some(value) => {
        //         let value_token_stream = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&{
        //             let value = &value.upper_camel_case_stringified;
        //             value
        //                 .parse::<proc_macro2::TokenStream>()
        //                 .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        //         });
        //         quote::quote!{#value_token_stream}
        //     },
        //     None => inner_type_token_stream.clone(),
        // };
        quote::quote! {
            pub #field_ident: #field_type_create
        }
    });
    let generate_try_operation_token_stream = |
        operation: &Operation,
        type_variants_from_request_response_syn_variants: &[syn::Variant],
        result_ok_type_token_stream: &dyn quote::ToTokens,
        payload_check_token_stream: &dyn quote::ToTokens,
        desirable_from_or_try_from_desirable_with_serialize_deserialize_token_stream: &dyn quote::ToTokens
    | -> proc_macro2::TokenStream {
        let try_operation_snake_case = naming::parameter::TrySelfSnakeCase::from_display(operation);
        let try_operation_error_named_upper_camel_case = naming::parameter::TrySelfErrorNamedUpperCamelCase::from_display(operation);
        let operation_parameters_upper_camel_case = naming::parameter::SelfParametersUpperCamelCase::from_display(operation);
        let payload_token_stream = {
            let serde_json_to_string_syn_variant_initialization_token_stream = generate_initialization_token_stream(&serde_json_to_string_syn_variant_wrapper, file!(), line!(), column!());
            let operation_payload_with_serialize_deserialize_initialization_token_stream = {
                let operation_payload_upper_camel_case = naming::parameter::SelfPayloadUpperCamelCase::from_display(operation);
                quote::quote! {#operation_payload_upper_camel_case::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
            };
            quote::quote! {
                let #payload_snake_case = {
                    #payload_check_token_stream
                    let #value_snake_case = #operation_payload_with_serialize_deserialize_initialization_token_stream;
                    match serde_json::to_string(&#value_snake_case) {
                        Ok(#value_snake_case) => #value_snake_case,
                        Err(#error_0_token_stream) => {
                            return Err(#try_operation_error_named_upper_camel_case::#serde_json_to_string_syn_variant_initialization_token_stream);
                        }
                    }
                };
            }
        };
        let url_snake_case = naming::UrlSnakeCase;
        let server_location_snake_case = naming::ServerLocationSnakeCase;
        let url_token_stream = {
            let url_handle_token_stream = naming::UrlHandleSelfSnakeCaseTokenStream::url_handle_self_snake_case_token_stream(operation, &ident_snake_case_stringified);
            quote::quote! {
                let #url_snake_case = format!(
                    #url_handle_token_stream,
                    #server_location_snake_case,
                );
            }
        };
        let future_snake_case = naming::FutureSnakeCase;
        let future_token_stream = {
            let operation_http_method_snake_case_token_stream = naming::AsRefStrToSnakeCaseTokenStream::case_or_panic(&operation.http_method());
            let commit_header_addition_token_stream = quote::quote! {
                .header(
                    &#postgresql_crud_snake_case::CommitSnakeCase.to_string(),//todo remove it
                    git_info::PROJECT_GIT_INFO.commit,
                )
            };
            let application_json_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&"application/json");
            let content_type_application_json_header_addition_token_stream = quote::quote! {
                .header(reqwest::header::CONTENT_TYPE, #application_json_double_quotes_token_stream)
            };
            quote::quote! {
                let #future_snake_case = reqwest::Client::new()
                    .#operation_http_method_snake_case_token_stream(&#url_snake_case)
                    #commit_header_addition_token_stream
                    #content_type_application_json_header_addition_token_stream
                    .#body_snake_case(#payload_snake_case)
                    .send();
            }
        };
        let response_token_stream = {
            let reqwest_syn_variant_initialization_token_stream = generate_initialization_token_stream(&reqwest_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let #response_snake_case = match #future_snake_case.await {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_0_token_stream) => {
                        return Err(#try_operation_error_named_upper_camel_case::#reqwest_syn_variant_initialization_token_stream);
                    }
                };
            }
        };
        let status_code_token_stream = quote::quote! {
            let #error_0_token_stream = #response_snake_case.status();
        };
        let headers_token_stream = quote::quote! {
            let #error_1_token_stream = #response_snake_case.headers().clone();
        };
        let response_text_token_stream = {
            let failed_to_get_response_text_syn_variant_initialization_token_stream = generate_initialization_token_stream(&failed_to_get_response_text_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let #error_2_token_stream = match #response_snake_case.text().await {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_2_token_stream) => {
                        return Err(#try_operation_error_named_upper_camel_case::#failed_to_get_response_text_syn_variant_initialization_token_stream);
                    }
                };
            }
        };
        let try_operation_route_logic_response_variants_upper_camel_case = naming::parameter::TrySelfRouteLogicResponseVariantsUpperCamelCase::from_display(operation);
        let expected_response_snake_case = naming::ExpectedResponseSnakeCase;
        let expected_response_token_stream = {
            let deserialize_response_syn_variant_initialization_token_stream = generate_initialization_token_stream(&deserialize_response_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let #expected_response_snake_case = match serde_json::from_str::<#try_operation_route_logic_response_variants_upper_camel_case>(&#error_2_token_stream) {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_3_token_stream) => {
                        return Err(#try_operation_error_named_upper_camel_case::#deserialize_response_syn_variant_initialization_token_stream);
                    }
                };
            }
        };
        let try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case = naming::parameter::TrySelfRouteLogicErrorNamedWithSerializeDeserializeUpperCamelCase::from_display(operation);
        let try_operation_route_logic_error_named_with_serialize_deserialize_snake_case = naming::parameter::TrySelfRouteLogicErrorNamedWithSerializeDeserializeSnakeCase::from_display(operation);
        let try_operation_route_logic_error_named_with_serialize_deserialize_token_stream = {
            let try_operation_route_logic_response_variants_to_try_operation_route_logic_error_named_with_serialize_deserialize = type_variants_from_request_response_syn_variants.iter().map(|element| {
                let variant_ident = &element.ident;
                let fields_idents_token_stream = if let syn::Fields::Named(fields_named) = &element.fields {
                    let fields_idents = fields_named.named.iter().map(|field| &field.ident);
                    quote::quote! {#(#fields_idents),*}
                } else {
                    panic!("expected fields would be named");
                };
                quote::quote! {
                    #try_operation_route_logic_response_variants_upper_camel_case::#variant_ident {
                        #fields_idents_token_stream
                    } => #try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case::#variant_ident { #fields_idents_token_stream }
                }
            });
            quote::quote! {
                let #try_operation_route_logic_error_named_with_serialize_deserialize_snake_case = match #expected_response_snake_case {
                    #try_operation_route_logic_response_variants_upper_camel_case::#desirable_upper_camel_case(#value_snake_case) => {
                        let #value_snake_case = #desirable_from_or_try_from_desirable_with_serialize_deserialize_token_stream;
                        return Ok(#value_snake_case);
                    },
                    #(#try_operation_route_logic_response_variants_to_try_operation_route_logic_error_named_with_serialize_deserialize),*
                };
            }
        };
        let return_error_token_stream = {
            let field_code_occurence_new_6ac7b78e_da5d_4274_b58c_67bb9625d008_token_stream = macros_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(file!(), line!(), column!());
            quote::quote! {
                Err(#try_operation_error_named_upper_camel_case::#try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case {
                    #try_operation_route_logic_error_named_with_serialize_deserialize_snake_case,
                    #field_code_occurence_new_6ac7b78e_da5d_4274_b58c_67bb9625d008_token_stream,
                })
            }
        };
        quote::quote! {
            pub async fn #try_operation_snake_case(
                #server_location_snake_case: #ref_std_primitive_str,//todo rename as endpoint location
                #parameters_snake_case: #operation_parameters_upper_camel_case,
            ) -> Result<#result_ok_type_token_stream, #try_operation_error_named_upper_camel_case> {
                #payload_token_stream
                #url_token_stream
                #future_token_stream
                #response_token_stream
                #status_code_token_stream
                #headers_token_stream
                #response_text_token_stream
                #expected_response_token_stream
                #try_operation_route_logic_error_named_with_serialize_deserialize_token_stream
                #return_error_token_stream
            }
        }
    };
    let generate_filter_not_unique_token_stream = |
        iterable_token_stream: &dyn quote::ToTokens,
        contains_token_stream: &dyn quote::ToTokens,
        push_token_stream: &dyn quote::ToTokens,
        error_token_stream: &dyn quote::ToTokens,
        return_error_token_stream: &dyn quote::ToTokens
    | {
        quote::quote! {
            let mut #acc_snake_case = std::vec::Vec::new();
            for #element_snake_case in #iterable_token_stream {
                if !#acc_snake_case.contains(&#contains_token_stream) {
                    #acc_snake_case.push(&#push_token_stream);
                }
                else {
                    let #error_0_token_stream = #error_token_stream;
                    #return_error_token_stream
                }
            }
        }
    };
    // let generate_contains_check_token_stream = |contains_token_stream: &dyn quote::ToTokens, push_token_stream: &dyn quote::ToTokens, error_token_stream: &dyn quote::ToTokens, error_initialization_token_stream: &dyn quote::ToTokens| {
    //     quote::quote! {
    //         if !#acc_snake_case.contains(#contains_token_stream) {
    //             #acc_snake_case.push(#push_token_stream);
    //         }
    //         else {
    //             let #error_0_token_stream = #error_token_stream;
    //             #error_initialization_token_stream
    //         }
    //     }
    // };
    // let generate_filter_not_unique_fields_token_stream = |operation: &Operation, fields_without_primary_key: &std::vec::Vec<SynFieldWrapper>| {
    //     let filter_not_unique_primary_key_token_stream = {
    //         let filter_not_unique_token_stream = generate_filter_not_unique_token_stream(
    //             &value_snake_case,
    //             &element_snake_case,
    //             &element_snake_case,
    //             &quote::quote! {#element_snake_case.clone()},
    //             &generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &not_unique_primary_key_syn_variant_wrapper, file!(), line!(), column!()),
    //         );
    //         quote::quote! {
    //             if let Some(#value_snake_case) = &#value_snake_case.#primary_key_field_ident {
    //                 #filter_not_unique_token_stream
    //             }
    //         }
    //     };
    //     let filter_not_unique_fields_named_excluding_primary_key_token_stream = fields_without_primary_key.iter().map(|element| {
    //         let operation_clone = operation.clone();
    //         let element_field_ident = &element.field_ident;
    //         let not_unique_fields_syn_variant_wrapper = new_syn_variant_wrapper(
    //             &naming::parameter::NotUniqueSelfUpperCamelCase::from_tokens(&element.field_ident),
    //             Some(not_unique_primary_key_syn_variant_wrapper.get_option_status_code().unwrap()),
    //             vec![(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &element.field_ident.to_string(), {
    //                 if let syn::Type::Path(value) = &element.syn_field.ty {
    //                     value.path.segments.clone()
    //                 }
    //                 else {
    //                     panic!("primary key syn::Type in not syn::Type::Path");
    //                 }
    //                 // let mut value = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
    //                 // value.push_value(syn::PathSegment {
    //                 //     ident: proc_macro2::Ident::new(&postgresql_crud_snake_case_stringified, proc_macro2::Span::call_site()),
    //                 //     arguments: syn::PathArguments::None,
    //                 // });
    //                 // value.push_punct(syn::token::PathSep {
    //                 //     spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
    //                 // });
    //                 // value.push_value(syn::PathSegment {
    //                 //     ident: proc_macro2::Ident::new(
    //                 //         &postgresql_crud_common::SqlxPostgresType::from_supported_sqlx_postgres_type_removing_option(&postgresql_crud_common::SupportedSqlxPostgresType::from(&element.rust_sqlx_map_to_postgres_type_variant)).to_string(),
    //                 //         proc_macro2::Span::call_site(),
    //                 //     ),
    //                 //     arguments: syn::PathArguments::None,
    //                 // });
    //                 // value
    //             })],
    //         );
    //         let not_unique_fields_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(
    //             &operation_clone,
    //             &not_unique_fields_syn_variant_wrapper,
    //             file!(),
    //             line!(),
    //             column!()
    //         );
    //         let content_token_stream = 
    //         // match postgresql_crud_common::SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::from(
    //         //     &postgresql_crud_common::SupportedSqlxPostgresType::from(&element.rust_sqlx_map_to_postgres_type_variant)
    //         // ) {
    //         //     postgresql_crud_common::SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::SqlxPostgresType(_) => generate_contains_check_token_stream(
    //         //         &quote::quote! {&&#element_snake_case.#value_snake_case.0},
    //         //         &quote::quote! {&#element_snake_case.#value_snake_case.0},
    //         //         &quote::quote! {#element_snake_case.#value_snake_case.clone()},
    //         //         &not_unique_fields_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream,
    //         //     ),
    //         //     postgresql_crud_common::SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::OptionSupportedSqlxPostgresType(_) => {
    //         //         let content_token_stream = generate_contains_check_token_stream(
    //         //             &quote::quote! {&#value_snake_case},
    //         //             &quote::quote! {&#value_snake_case},
    //         //             &quote::quote! {#value_snake_case.clone()},
    //         //             &not_unique_fields_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream,
    //         //         );
    //         //         quote::quote! {
    //         //             if let Some(#value_snake_case) = &#element_snake_case.#value_snake_case.0 {
    //         //                 #content_token_stream
    //         //             }
    //         //         }
    //         //     }
    //         // }
    //         generate_contains_check_token_stream(
    //             // &quote::quote! {&&#element_snake_case.#value_snake_case.0},
    //             // &quote::quote! {&#element_snake_case.#value_snake_case.0},
    //             // &quote::quote! {#element_snake_case.#value_snake_case.clone()},
    //             &quote::quote! {&#element_snake_case},
    //             &quote::quote! {&#element_snake_case},
    //             &quote::quote! {#element_snake_case.#value_snake_case.clone()},
    //             &not_unique_fields_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream,
    //         )
    //         ;
    //         quote::quote! {
    //             if let Some(#value_snake_case) = &#value_snake_case.#element_field_ident {
    //                 let mut #acc_snake_case = std::vec::Vec::new();
    //                 for #element_snake_case in #value_snake_case {
    //                     #content_token_stream
    //                 }
    //             }
    //         }
    //     });
    //     // let f = quote::quote! {#(#filter_not_unique_fields_named_excluding_primary_key_token_stream)*};
    //     // println!("{f}");
    //     quote::quote! {
    //         #filter_not_unique_primary_key_token_stream
    //         #(#filter_not_unique_fields_named_excluding_primary_key_token_stream)*
    //         // #f
    //     }
    // };
    let generate_create_update_delete_many_fetch_token_stream = |operation: &Operation| {
        generate_fetch_token_stream(
            &generate_sqlx_row_try_get_primary_key_token_stream(
                &match operation {
                    Operation::CreateMany => quote::quote!{#primary_key_field_type_read_upper_camel_case},//todo maybe other parameters must be read too?
                    Operation::UpdateMany => quote::quote!{#primary_key_field_type_update_upper_camel_case},
                    Operation::DeleteMany => quote::quote!{#primary_key_field_type_to_delete_upper_camel_case},
                    _ => panic!("supported only CreateMany, UpdateMany, DeleteMany")
                },
                // &quote::quote! {Some(#primary_key_inner_type_token_stream(#value_snake_case))},
                &quote::quote! {Some(#value_snake_case)},
                &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(operation, file!(), line!(), column!(), file!(), line!(), column!()),
            ),
            &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(operation, file!(), line!(), column!(), file!(), line!(), column!()),
        )
    };
    let generate_create_update_delete_one_fetch_token_stream = |operation: &Operation| {
        generate_fetch_one_token_stream(
            &generate_sqlx_row_try_get_primary_key_token_stream(
                &match operation {
                    Operation::CreateOne => quote::quote!{#primary_key_field_type_read_upper_camel_case},//todo maybe other must also be read
                    Operation::UpdateOne => quote::quote!{#primary_key_field_type_update_upper_camel_case},
                    Operation::DeleteOne => quote::quote!{#primary_key_field_type_to_delete_upper_camel_case},
                    _ => panic!("supported only CreateOne, UpdateOne, DeleteOne")
                },
                // &quote::quote! {#primary_key_inner_type_token_stream(#value_snake_case)},
                &value_snake_case,
                &generate_match_postgres_transaction_rollback_await_token_stream(operation, file!(), line!(), column!(), file!(), line!(), column!()),
            ),
            &generate_match_postgres_transaction_rollback_await_token_stream(operation, file!(), line!(), column!(), file!(), line!(), column!()),
        )
    };
    let space_and_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!(" {and_snake_case}"));
    let generate_operation_payload_example_route_logic_token_stream = |operation: &Operation| {
        let operation_payload_example_route_logic_snake_case = naming::parameter::SelfPayloadExampleRouteLogicSnakeCase::from_display(operation);
        let wraped_into_axum_response_token_stream = wrap_into_axum_response_token_stream(
            &{
                let std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case = naming::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElementUpperCamelCase;
                let operation_payload_upper_camel_case = naming::parameter::SelfPayloadUpperCamelCase::from_display(operation);
                quote::quote! {<#operation_payload_upper_camel_case as postgresql_crud::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_upper_camel_case>::#std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_snake_case()}
            },
            &quote::quote! {axum::http::StatusCode::OK},
        );
        quote::quote! {
            pub async fn #operation_payload_example_route_logic_snake_case() -> axum::response::Response {
                #wraped_into_axum_response_token_stream
            }
        }
    };
    let fields_initialiation_excluding_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_curly_braces_token_stream = {
        let fields_initialiation_excluding_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
            let value = fields.iter().map(|element| {
                let field_ident = &element.field_ident;
                quote::quote! {
                    #field_ident: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                }
            });
            quote::quote! {#(#value),*}
        };
        quote::quote! {
            Self{#fields_initialiation_excluding_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream}
        }
    };
    // // let generics_generate_postgresql_json_type_from_vec_error_named_syn_variants_wrappers = syn_field_with_additional_info_fields_named_excluding_primary_key.iter().fold(vec![], |mut acc, element| {
    // //     if let Some(value) = &element.option_generic {
    // //         acc.push(new_syn_variant_wrapper(
    // //             &value.generate_postgresql_json_type_from_vec_error_named_upper_camel_case_stringified,
    // //             Some(macros_helpers::status_code::StatusCode::InternalServerError500),
    // //             vec![(
    // //                 macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize,
    // //                 &value.generate_postgresql_json_type_from_vec_error_named_snake_case_stringified,
    // //                 macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&value.generate_postgresql_json_type_from_vec_error_named_upper_camel_case_stringified]),
    // //             )],
    // //         ));
    // //     }
    // //     acc
    // // });
    //todo generate test_token_stream
    let create_many_token_stream = {
        let operation = Operation::CreateMany;
        let expected_length_snake_case = naming::ExpectedLengthSnakeCase;
        let got_length_snake_case = naming::GotLengthSnakeCase;
        let std_primitive_usize_syn_punctuated_punctuated = macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["std", "primitive", "usize"]);
        let unexpected_rows_length_syn_variant_wrapper = new_syn_variant_wrapper(
            &naming::UnexpectedRowsLengthUpperCamelCase,
            Some(macros_helpers::status_code::StatusCode::InternalServerError500),
            vec![
                (macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &expected_length_snake_case, std_primitive_usize_syn_punctuated_punctuated.clone()),
                (macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &got_length_snake_case, std_primitive_usize_syn_punctuated_punctuated.clone()),
            ],
        );
        let unexpected_rows_length_and_rollback_syn_variant_wrapper = new_syn_variant_wrapper(
            &naming::UnexpectedRowsLengthAndRollbackUpperCamelCase,
            Some(macros_helpers::status_code::StatusCode::InternalServerError500),
            vec![
                (macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &expected_length_snake_case, std_primitive_usize_syn_punctuated_punctuated.clone()),
                (macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &got_length_snake_case, std_primitive_usize_syn_punctuated_punctuated.clone()),
                //todo reuse vec elements
                (macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString, &rollback_snake_case, sqlx_error_syn_punctuated_punctuated.clone()),
            ],
        );
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 4);
                common_route_syn_variants.iter().for_each(|element| {
                    value.push(*element);
                });
                value.push(bind_query_syn_variant_wrapper.get_syn_variant());
                value.push(checked_add_syn_variant_wrapper.get_syn_variant());
                value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(unexpected_rows_length_syn_variant_wrapper.get_syn_variant());
                value.push(unexpected_rows_length_and_rollback_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(&operation, generate_payload_and_payload_element_token_stream(&operation, &pub_field_ident_field_type_fields_named_excluding_primary_key_token_stream));
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream =
                generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                    &operation,
                    &std_vec_vec_primary_key_field_type_read_token_stream,
                    &type_variants_from_request_response_syn_variants,
                );
            // println!("{try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream}");
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let mut column_names = fields.iter().fold(std::string::String::default(), |mut acc, element| {
                        // let incremented_index = index.checked_add(1).unwrap_or_else(|| panic!("{index} {}", constants::CHECKED_ADD_NONE_OVERFLOW_MESSAGE));
                        acc.push_str(&format!("{}", &element.field_ident));
                        // if incremented_index != fields_without_primary_key_len {
                            acc.push_str(",");
                        // }
                        acc
                    });
                    let _ = column_names.pop();
                    let column_increments_token_stream = fields.iter().map(|element| {
                        let element_field_ident = &element.field_ident;
                        // if element.option_generic.is_some() {
                        //     let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &bind_query_syn_variant_wrapper, file!(), line!(), column!());
                        //     quote::quote! {
                        //         match postgresql_crud::BindQuery::try_generate_bind_increments(&#element_snake_case.#element_field_ident, &mut #increment_snake_case) {
                        //             Ok(#value_snake_case) => {
                        //                 #acc_snake_case.push_str(&format!("{value},"));
                        //             },
                        //             Err(#error_0_token_stream) => {
                        //                 #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                        //             }
                        //         }
                        //     }
                        // } else {
                        //     let checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &checked_add_syn_variant_wrapper, file!(), line!(), column!());
                        //     quote::quote! {
                        //         match #postgresql_crud_bind_query_bind_query_try_increment_token_stream(
                        //             &#element_snake_case.#element_field_ident,
                        //             &mut #increment_snake_case,
                        //         ) {
                        //             Ok(_) => {
                        //                 #acc_snake_case.push_str(&format!("${},", #increment_snake_case));
                        //             }
                        //             Err(_) => {//todo try_increment has own error. is it must be used? or no?
                        //                 #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
                        //             }
                        //         }
                        //     }
                        // }
                        let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &bind_query_syn_variant_wrapper, file!(), line!(), column!());
                        quote::quote! {
                            match postgresql_crud::BindQuery::try_generate_bind_increments(&#element_snake_case.#element_field_ident, &mut #increment_snake_case) {
                                Ok(#value_snake_case) => {
                                    #acc_snake_case.push_str(&format!("{value},"));
                                },
                                Err(#error_0_token_stream) => {
                                    #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        }
                    });
                    let query_token_stream = generate_quotes::double_quotes_token_stream(
                        &format!("{insert_snake_case} {into_snake_case} {ident_snake_case_stringified} ({column_names}) {values_snake_case} {{values}} {returning_primary_key_stringified}")
                    );
                    quote::quote! {
                        {
                            #increment_initialization_token_stream
                            let mut values = #std_string_string::default();
                            for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                let mut #acc_snake_case = #std_string_string::default();
                                #(#column_increments_token_stream)*
                                let _ = #acc_snake_case.pop();
                                values.push_str(&format!("({acc}),"));
                            }
                            let _ = values.pop();
                            format!(#query_token_stream)
                        }
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let query_string_snake_case = naming::QueryStringSnakeCase;
                    let query_bind_token_stream = fields.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        quote::quote! {
                            #query_snake_case = #postgresql_crud_snake_case::BindQuery::bind_value_to_query(#element_snake_case.#field_ident, #query_snake_case);
                        }
                    });
                    quote::quote! {
                        let mut #query_snake_case = sqlx::query::<sqlx::Postgres>(&#query_string_snake_case);
                        for #element_snake_case in #parameters_snake_case.#payload_snake_case.0 {
                            #(#query_bind_token_stream)*
                        }
                        #query_snake_case
                    }
                };
                // println!("{binded_query_token_stream}");
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(&operation, &{
                    let fetch_token_stream = generate_create_update_delete_many_fetch_token_stream(&operation);
                    let unexpected_rows_length_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &unexpected_rows_length_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!()
                    );
                    let unexpected_rows_length_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(
                        &operation,
                        &unexpected_rows_length_and_rollback_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!()
                    );
                    quote::quote! {
                        #fetch_token_stream
                        {
                            let #error_1_token_stream = #value_snake_case.len();
                            if #error_0_token_stream != #error_1_token_stream {
                                match #executor_snake_case.#rollback_snake_case().await {
                                    Ok(_) => {
                                        #unexpected_rows_length_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                    Err(#error_2_token_stream) => {
                                        #unexpected_rows_length_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                }
                            }
                        }
                    }
                });
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &ident_snake_case_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &quote::quote! {
                        let #error_0_token_stream = #parameters_snake_case.#payload_snake_case.0.len();
                    },
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!("{try_operation_route_logic_token_stream}");
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                // &std_vec_vec_primary_key_inner_type_token_stream,
                &std_vec_vec_primary_key_field_type_read_token_stream,
                &proc_macro2::TokenStream::new(),
                // &quote::quote! {
                //     #value_snake_case
                //     .into_iter()
                //     .map(|#element_snake_case| #primary_key_inner_type_token_stream::#from_snake_case(#element_snake_case))
                //     .collect()
                // },
                &value_snake_case
            );
            // let try_operation_test_token_stream = {
            //     let element_fields_initialization_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
            //         let field_ident = &element.field_ident;
            //         let field_type = &element.field.ty;
            //         quote::quote!{
            //             #field_ident: #field_type::default()
            //         }
            //     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            //     let test_content_token_stream = quote::quote! {
            //         let #primary_keys_token_stream = match #try_operation_snake_case_token_stream(
            //             #reference_api_location_test_token_stream,
            //             #operation_parameters_upper_camel_case_token_stream {
            //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream(vec![
            //                     #operation_payload_element_upper_camel_case_token_stream{
            //                         #(#element_fields_initialization_token_stream),*
            //                     }
            //                 ])
            //             },
            //         )
            //         .await
            //         {
            //             Ok(value) => {
            //                 println!("{value:#?}");
            //                 value
            //             },
            //             Err(#error_snake_case) => panic!(
            //                 "{}",
            //                 #error_snake_case
            //             ),//todo remove panic maybe?
            //         };
            //     };
            //     naming::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // };
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        // println!("{try_operation_token_stream}");
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream = {
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_element_token_stream = 
            postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &naming::parameter::SelfPayloadElementUpperCamelCase::from_display(&operation),
                &fields_initialiation_excluding_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_curly_braces_token_stream,
            );
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &naming::parameter::SelfPayloadUpperCamelCase::from_display(&operation),
                &quote::quote! {Self(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream])},
            );
            quote::quote! {
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_element_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream
            }
        };
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream
            #operation_payload_example_route_logic_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &create_many_token_stream,
    // );
    let create_one_token_stream = {
        let operation = Operation::CreateOne;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_with_row_and_rollback_syn_variants.len() + 1);
                common_route_with_row_and_rollback_syn_variants.iter().for_each(|element| {
                    value.push(*element);
                });
                value.push(bind_query_syn_variant_wrapper.get_syn_variant());
                value.push(checked_add_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(&operation, generate_operation_payload_token_stream(&operation, &pub_field_ident_field_type_fields_named_excluding_primary_key_token_stream));
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream =
                generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                    &operation,
                    &primary_key_field_type_read_upper_camel_case,
                    &type_variants_from_request_response_syn_variants,
                );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let mut column_names = fields.iter().fold(std::string::String::default(), |mut acc, element| {
                        acc.push_str(&format!("{}", &element.field_ident));
                        acc.push_str({
                            // let incremented_index = index.checked_add(1).unwrap_or_else(|| panic!("{index} {}", constants::CHECKED_ADD_NONE_OVERFLOW_MESSAGE));
                            // if incremented_index == fields_without_primary_key_len {
                            //     ""
                            // } else {
                            //     ","
                            // }
                            ","
                        });
                        acc
                    });
                    let _ = column_names.pop();
                    let mut column_increments = fields.iter().fold(std::string::String::default(), |mut acc, _| {
                        acc.push_str("{}");
                        acc.push_str({
                            // let incremented_index = index.checked_add(1).unwrap_or_else(|| panic!("{index} {}", constants::CHECKED_ADD_NONE_OVERFLOW_MESSAGE));
                            // if incremented_index == fields_without_primary_key_len {
                            //     ""
                            // } else {
                            //     ","
                            // }
                            ","
                        });
                        acc
                    });
                    let _ = column_increments.pop();
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(
                        &format!("{insert_snake_case} {into_snake_case} {ident_snake_case_stringified} ({column_names}) {values_snake_case} ({column_increments}) {returning_primary_key_stringified}")
                    );
                    let try_generate_bind_increments = fields.iter().map(|element| {
                        let element_field_ident = &element.field_ident;
                        // if element.option_generic.is_some() {
                        //     let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &bind_query_syn_variant_wrapper, file!(), line!(), column!());
                        //     quote::quote! {
                        //         match postgresql_crud::BindQuery::try_generate_bind_increments(&#parameters_snake_case.#payload_snake_case.#element_field_ident, &mut #increment_snake_case) {
                        //             Ok(#value_snake_case) => #value_snake_case,
                        //             Err(#error_0_token_stream) => {
                        //                 #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                        //             }
                        //         }
                        //     }
                        // } else {
                        //     let checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &checked_add_syn_variant_wrapper, file!(), line!(), column!());
                        //     quote::quote! {
                        //         {
                        //             match increment.checked_add(1) {
                        //                 Some(incr) => {
                        //                     increment = incr;
                        //                     format!("${increment}")
                        //                 }
                        //                 None => {
                        //                     #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
                        //                 }
                        //             }
                        //         }
                        //     }
                        // }
                        let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &bind_query_syn_variant_wrapper, file!(), line!(), column!());
                        quote::quote! {
                            match postgresql_crud::BindQuery::try_generate_bind_increments(&#parameters_snake_case.#payload_snake_case.#element_field_ident, &mut #increment_snake_case) {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_0_token_stream) => {
                                    #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        }
                    });
                    quote::quote! {
                        {
                            let mut increment: std::primitive::u64 = 0;
                            format!(
                                #format_handle_token_stream,
                                #(#try_generate_bind_increments),*
                            )
                        }
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = fields.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        quote::quote! {
                            query = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(#parameters_snake_case.#payload_snake_case.#field_ident, query);
                        }
                    });
                    quote::quote! {
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#binded_query_modifications_token_stream)*
                        query
                    }
                };
                // println!("{binded_query_token_stream}");
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(&operation, &generate_create_update_delete_one_fetch_token_stream(&operation));
                // // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                // //     &ident_snake_case_stringified,
                // //     &unique_status_codes,
                // //     &application_json_quotes_token_stream,
                // //     &table_name_quotes_token_stream,
                // //     &operation_payload_upper_camel_case_token_stream,
                // //     &operation,
                // // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!("{try_operation_route_logic_token_stream}");
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &primary_key_field_type_read_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                // &quote::quote! {#primary_key_inner_type_token_stream::#from_snake_case(#value_snake_case)},
                &value_snake_case
            );
            // println!(" {try_operation_token_stream}");
            // let try_operation_test_token_stream = {
            //     let element_fields_initialization_token_stream = fields.iter().map(|element|{
            //         let field_ident = &element.field_ident;
            //         let field_type = &element.field.ty;
            //         quote::quote!{
            //             #field_ident: #field_type::default()
            //         }
            //     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            //     let test_content_token_stream = quote::quote! {
            //         let #primary_key_token_stream = match #try_operation_snake_case_token_stream(
            //             #reference_api_location_test_token_stream,
            //             #operation_parameters_upper_camel_case_token_stream {
            //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
            //                     #(#element_fields_initialization_token_stream),*
            //                 }
            //             },
            //         )
            //         .await
            //         {
            //             Ok(value) => {
            //                 println!("{value:#?}");
            //                 value
            //             },
            //             Err(#error_snake_case) => panic!(
            //                 "{}",
            //                 #error_snake_case
            //             )
            //         };
            //     };
            //     naming::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // };
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        // println!("{try_operation_token_stream}");
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &naming::parameter::SelfPayloadUpperCamelCase::from_display(&operation),
            &fields_initialiation_excluding_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_curly_braces_token_stream,
        );
        // println!("{impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream}");
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream
            #operation_payload_example_route_logic_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &create_one_token_stream,
    // );
    //todo add additional filters
    let read_many_token_stream = {
        let operation = Operation::ReadMany;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 4 + not_unique_fields_syn_variants_wrappers.len());
                common_route_syn_variants.iter().for_each(|element| {
                    value.push(*element);
                });
                value.push(checked_add_syn_variant_wrapper.get_syn_variant());
                value.push(bind_query_syn_variant_wrapper.get_syn_variant());
                value.push(not_unique_primary_key_syn_variant_wrapper.get_syn_variant());
                value.push(not_unique_column_syn_variant_wrapper.get_syn_variant());
                not_unique_fields_syn_variants_wrappers.iter().for_each(|element| {
                    value.push(element.get_syn_variant());
                });
                // if contains_generic_json {
                //     value.push(&empty_column_json_reader_syn_variant_wrapper.get_syn_variant());
                //     value.push(&not_unique_column_json_reader_syn_variant_wrapper.get_syn_variant());
                // }
                // generics_generate_postgresql_json_type_from_vec_error_named_syn_variants_wrappers.iter().for_each(|element| {
                //     value.push(&element.get_syn_variant());
                // });
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            generate_operation_payload_token_stream(
                &operation,
                &quote::quote! {    
                    #pub_fields_idents_std_option_option_std_vec_vec_where_inner_type_token_stream,
                    #pub_handle_select_snake_case_std_vec_vec_ident_column_upper_camel_case_token_stream,
                    pub #order_by_snake_case: #postgresql_crud_order_by_token_stream<#ident_column_upper_camel_case>,
                    pub pagination: postgresql_crud::Pagination,
                },
            ),
        );
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream =
                generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                    &operation,
                    &std_vec_vec_struct_options_ident_token_stream,
                    &type_variants_from_request_response_syn_variants,
                );
            let try_operation_route_logic_token_stream = {
                //todo maybe but checks into constructor function and use it inside deserilizaton serde impl
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &{
                    //todo where logic was commented. untill rewriting better where logic there is not point to add it again
                    // let filter_not_unique_fields_token_stream = generate_filter_not_unique_fields_token_stream(&operation, &fields_without_primary_key);
                    // let filter_not_unique_column_token_stream = generate_filter_not_unique_column_route_logic_token_stream(&operation);
                    quote::quote! {
                        // #filter_not_unique_fields_token_stream
                        // #filter_not_unique_column_token_stream
                    }
                });
                let query_string_token_stream = {
                    let additional_parameters_modification_token_stream = fields.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let field_type = &element.syn_field.ty;
                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                        quote::quote! {
                            if let Some(#value_snake_case) = &#parameters_snake_case.#payload_snake_case.#field_ident {
                                match <#field_type as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::where_query_part(
                                    value,
                                    &mut increment,
                                    &#field_ident_double_quotes_token_stream,
                                    is_first_push_to_additional_parameters_already_happend,//todo generate is in proc macro (first element ignore)
                                ) {
                                    Ok(value) => {
                                        additional_parameters.push_str(&value);
                                        is_first_push_to_additional_parameters_already_happend = true;
                                    }
                                    Err(error_0) => {
                                        // let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                                        //     bind_query: error_0,
                                        //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        //         file!().to_owned(),
                                        //         line!(),
                                        //         column!(),
                                        //         Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        //             file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        //             line: 3625,
                                        //             column: 266,
                                        //         }),
                                        //     ),
                                        // };
                                        // eprintln!("{error}");
                                        // let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                        // *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        // return response;
                                        todo!()
                                    }
                                }
                            }
                        }
                    });
                    let handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{select_snake_case} {{}} {from_snake_case} {ident_snake_case_stringified} {{}}"));
                    let by_snake_case = naming::BySnakeCase;
                    let additional_parameters_order_by_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}{order_snake_case} {by_snake_case} {{}} {{}}"));
                    // let bind_query_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream_e6e820dd_ec74_4bc5_b482_2aa9cd6b3793 = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &bind_query_syn_variant_wrapper, file!(), line!(), column!());
                    let bind_query_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream_6d384a1b_d37a_4fd3_9ed3_c160afbb74fc =
                        generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &bind_query_syn_variant_wrapper, file!(), line!(), column!());
                    let prefix_snake_case = naming::PrefixSnakeCase;
                    let prefix_to_additional_parameters_token_stream = quote::quote! {
                        let #prefix_snake_case = match additional_parameters.is_empty() {
                            true => "",
                            false => " ",
                        };
                    };
                    let query_vec_column_token_stream = generate_query_vec_column_token_stream(
                        // &operation
                    );
                    quote::quote! {
                        {
                            format!(
                                #handle_token_stream,
                                #query_vec_column_token_stream,
                                {
                                    #increment_initialization_token_stream
                                    //todo is all None then do not start with "where"
                                    let mut additional_parameters = #std_string_string::from("where");
                                    let mut is_first_push_to_additional_parameters_already_happend = false;
                                    #(#additional_parameters_modification_token_stream)*
                                    {
                                        #prefix_to_additional_parameters_token_stream
                                        let #value_snake_case = &#parameters_snake_case.#payload_snake_case.#order_by_snake_case;
                                        let #order_snake_case = match &#value_snake_case.#order_snake_case {
                                            Some(#value_snake_case) => #value_snake_case.to_snake_case_stringified(),
                                            None => #postgresql_crud_order_token_stream::default().to_snake_case_stringified(),
                                        };
                                        additional_parameters.push_str(&format!(
                                            #additional_parameters_order_by_handle_token_stream,
                                            #prefix_snake_case,
                                            #value_snake_case.#column_snake_case.pick_column(),//todo refactor pick_column
                                            #order_snake_case,
                                        ));
                                    }
                                    {
                                        #prefix_to_additional_parameters_token_stream
                                        let #value_snake_case = match #crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream(
                                            &#parameters_snake_case.#payload_snake_case.pagination,
                                            &mut #increment_snake_case
                                        ) {
                                            Ok(#value_snake_case) => #value_snake_case,
                                            Err(#error_0_token_stream) => {
                                                #bind_query_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream_6d384a1b_d37a_4fd3_9ed3_c160afbb74fc
                                            },
                                        };
                                        additional_parameters.push_str(&format!(
                                            "{}{}",
                                            #prefix_snake_case,
                                            #value_snake_case
                                        ));
                                    }
                                    additional_parameters
                                }
                            )
                        }
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = fields.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let field_type = &element.syn_field.ty;
                        quote::quote! {
                            if let Some(#value_snake_case) = #parameters_snake_case.#payload_snake_case.#field_ident {
                                query = <#field_type as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::where_query_bind(
                                    value,
                                    query
                                );
                            }
                        }
                    });
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#binded_query_modifications_token_stream)*
                        #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(
                            #parameters_snake_case.#payload_snake_case.pagination,
                            #query_snake_case,
                        );
                        #query_snake_case
                    }
                };
                // println!("{binded_query_token_stream}");
                let postgresql_logic_token_stream = {
                    let fetch_token_stream = generate_fetch_token_stream(
                        &{
                            let options_try_from_sqlx_row_token_stream = generate_options_try_from_sqlx_row_token_stream(&operation);
                            quote::quote! {Some({#options_try_from_sqlx_row_token_stream})}
                        },
                        &generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!()),
                    );
                    quote::quote! {
                        #fetch_token_stream
                        #value_snake_case
                    }
                };
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &ident_snake_case_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            // println!("{try_operation_route_logic_token_stream}");
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!("{try_operation_route_logic_token_stream}");
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(&operation, &{
                let mut value = common_http_request_syn_variants.clone();
                value.push(not_unique_primary_key_syn_variant_wrapper.get_syn_variant().clone());
                not_unique_fields_syn_variants_wrappers.iter().for_each(|element| {
                    value.push(element.get_syn_variant().clone());
                });
                value.push(not_unique_column_syn_variant_wrapper.get_syn_variant().clone());
                value
            });
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_struct_options_ident_token_stream,
                &{
                    // let try_operation_error_named_upper_camel_case = naming::parameter::TrySelfErrorNamedUpperCamelCase::from_display(&operation);
                    //todo where logic was commented. untill rewriting better where logic there is not point to add it again
                    // let filter_not_unique_fields_token_stream = {
                    //     let filter_not_unique_primary_key_token_stream = {
                    //         let filter_not_unique_token_stream = generate_filter_not_unique_token_stream(&value_snake_case, &element_snake_case, &element_snake_case, &quote::quote! {#element_snake_case.clone()}, &{
                    //             let not_unique_primary_key_syn_variant_initialization_token_stream = generate_initialization_token_stream(&not_unique_primary_key_syn_variant_wrapper, file!(), line!(), column!());
                    //             quote::quote! {
                    //                 return Err(#try_operation_error_named_upper_camel_case::#not_unique_primary_key_syn_variant_initialization_token_stream);
                    //             }
                    //         });
                    //         quote::quote! {
                    //             if let Some(#value_snake_case) = &#parameters_snake_case.#payload_snake_case.#primary_key_field_ident {
                    //                 #filter_not_unique_token_stream
                    //             }
                    //         }
                    //     };
                    //     let filter_not_unique_fields_named_excluding_primary_key_token_stream = fields_without_primary_key.iter().map(|element| {
                    //         let element_field_ident = &element.field_ident;
                    //         let not_unique_fields_syn_variant_initialization_token_stream = {
                    //             let not_unique_field_ident_upper_camel_case = naming::parameter::NotUniqueSelfUpperCamelCase::from_tokens(&element_field_ident);
                    //             let field_code_occurence_new_eb1a9553_449e_4767_9e5c_c1856b77bd4e_token_stream = macros_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
                    //                 file!(),
                    //                 line!(),
                    //                 column!()
                    //             );
                    //             quote::quote! {
                    //                 #not_unique_field_ident_upper_camel_case {
                    //                     #element_field_ident: #error_0_token_stream,
                    //                     #field_code_occurence_new_eb1a9553_449e_4767_9e5c_c1856b77bd4e_token_stream,
                    //                 }
                    //             }
                    //         };
                    //         //todo maybe reuse
                    //         let content_token_stream = {
                    //             let error_initialization_token_stream = quote::quote! {return Err(#try_operation_error_named_upper_camel_case::#not_unique_fields_syn_variant_initialization_token_stream);};
                    //             // match postgresql_crud_common::SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::from(&postgresql_crud_common::SupportedSqlxPostgresType::from(&element.rust_sqlx_map_to_postgres_type_variant)) {
                    //             //     postgresql_crud_common::SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::SqlxPostgresType(_) => generate_contains_check_token_stream(
                    //             //         &quote::quote! {&&#element_snake_case.#value_snake_case.0},
                    //             //         &quote::quote! {&#element_snake_case.#value_snake_case.0},
                    //             //         &quote::quote! {#element_snake_case.#value_snake_case.clone()},
                    //             //         &error_initialization_token_stream,
                    //             //     ),
                    //             //     postgresql_crud_common::SqlxPostgresTypeOrOptionSupportedSqlxPostgresType::OptionSupportedSqlxPostgresType(_) => {
                    //             //         let content_token_stream = generate_contains_check_token_stream(&quote::quote! {&#value_snake_case}, &quote::quote! {&#value_snake_case}, &quote::quote! {#value_snake_case.clone()}, &error_initialization_token_stream);
                    //             //         quote::quote! {
                    //             //             if let Some(#value_snake_case) = &#element_snake_case.#value_snake_case.0 {
                    //             //                 #content_token_stream
                    //             //             }
                    //             //         }
                    //             //     }
                    //             // }
                    //             generate_contains_check_token_stream(
                    //                 // &quote::quote! {&&#element_snake_case.#value_snake_case.0},
                    //                 // &quote::quote! {&#element_snake_case.#value_snake_case.0},
                    //                 // &quote::quote! {#element_snake_case.#value_snake_case.clone()},
                    //                 &quote::quote! {&&#element_snake_case.#value_snake_case},
                    //                 &quote::quote! {&#element_snake_case.#value_snake_case},
                    //                 &quote::quote! {#element_snake_case.#value_snake_case.clone()},
                    //                 &error_initialization_token_stream,
                    //             )
                    //         };
                    //         quote::quote! {
                    //             if let Some(#value_snake_case) = &#parameters_snake_case.#payload_snake_case.#element_field_ident {
                    //                 let mut #acc_snake_case = std::vec::Vec::new();
                    //                 for #element_snake_case in #value_snake_case {
                    //                     #content_token_stream
                    //                 }
                    //             }
                    //         }
                    //     });
                    //     quote::quote! {
                    //         #filter_not_unique_primary_key_token_stream
                    //         #(#filter_not_unique_fields_named_excluding_primary_key_token_stream)*
                    //     }
                    // };
                    // let filter_not_unique_column_token_stream = generate_filter_not_unique_column_http_request_token_stream(&operation);
                    quote::quote! {
                        // #filter_not_unique_fields_token_stream
                        // #filter_not_unique_column_token_stream
                    }
                },
                &
                // match fields_named_from_or_try_from {
                //     postgresql_crud_common::FromOrTryFrom::From => quote::quote! {
                //         #value_snake_case
                //         .into_iter()
                //         .map(|#element_snake_case| #ident_options_upper_camel_case::#from_snake_case(#element_snake_case))
                //         .collect()
                //     },
                //     postgresql_crud_common::FromOrTryFrom::TryFrom => quote::quote! {
                //         #value_snake_case
                //         .into_iter()
                //         .fold(std::vec::Vec::new(), |mut #acc_snake_case, #element_snake_case| {
                //             #acc_snake_case.push(#element_snake_case);
                //             #acc_snake_case
                //         })
                //     },
                // }
                quote::quote! {
                    #value_snake_case
                    .into_iter()
                    .fold(std::vec::Vec::new(), |mut #acc_snake_case, #element_snake_case| {
                        #acc_snake_case.push(#element_snake_case);
                        #acc_snake_case
                    })
                }
            );
            // println!("{try_operation_token_stream}");
            // let try_operation_test_token_stream = {
            //     let element_fields_initialization_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
            //         let field_ident = &element.field_ident;
            //         let field_type = &element.field.ty;
            //         quote::quote!{
            //             #field_ident: #field_type::default()
            //         }
            //     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            //     let test_content_token_stream = quote::quote! {
            //         let #primary_key_token_stream = match #try_operation_snake_case_token_stream(
            //             #reference_api_location_test_token_stream,
            //             #operation_parameters_upper_camel_case_token_stream {
            //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
            //                     #(#element_fields_initialization_token_stream),*
            //                 }
            //             },
            //         )
            //         .await
            //         {
            //             Ok(value) => {
            //                 println!("{value:#?}");
            //                 value
            //             },
            //             Err(#error_snake_case) => panic!(
            //                 "{}",
            //                 #error_snake_case
            //             )
            //         };
            //     };
            //     naming::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // };
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        // println!("{try_operation_token_stream}");
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream = 
        postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &naming::parameter::SelfPayloadUpperCamelCase::from_display(&operation),
            &{
                let fields_token_stream = fields.iter().map(|element|{
                    let field_ident = &element.field_ident;
                    quote::quote!{
                        #field_ident: Some(
                            #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                        ),
                    }
                });
                quote::quote!{
                    Self {
                        #(#fields_token_stream)*
                        #select_snake_case: #postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                        #order_by_snake_case: postgresql_crud::OrderBy {
                            #column_snake_case: #ident_column_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(
                                #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                            ),
                            #order_snake_case: Some(
                                #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                            ),
                        },
                        pagination: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                    }
                }
            },
        );
        // println!("{impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream}");
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        // println!("{operation_payload_example_route_logic_token_stream}");
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream
            #operation_payload_example_route_logic_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     // &proc_macro_name_upper_camel_case,
    //     "read_many",
    //     &read_many_token_stream,
    // );
    let read_one_token_stream = {
        let operation = Operation::ReadOne;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 1);
                common_route_syn_variants.iter().for_each(|element| {
                    value.push(*element);
                });
                value.push(not_unique_column_syn_variant_wrapper.get_syn_variant());
                // if contains_generic_json {
                //     value.push(&empty_column_json_reader_syn_variant_wrapper.get_syn_variant());
                //     value.push(&not_unique_column_json_reader_syn_variant_wrapper.get_syn_variant());
                // }
                // generics_generate_postgresql_json_type_from_vec_error_named_syn_variants_wrappers.iter().for_each(|element| {
                //     value.push(&element.get_syn_variant());
                // });
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            generate_operation_payload_token_stream(
                &operation,
                &{
                    let pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream = generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream(
                        &naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(&primary_key_field.syn_field.ty)
                    );
                    quote::quote! {
                        #pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream,
                        #pub_handle_select_snake_case_std_vec_vec_ident_column_upper_camel_case_token_stream,
                    }
                },
            ),
        );
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                &operation,
                &ident_options_upper_camel_case,
                &type_variants_from_request_response_syn_variants,
            );
            // println!("{try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream}");
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &{
                    //todo where logic was commented. untill rewriting better where logic there is not point to add it again
                    // let filter_not_unique_column_token_stream = generate_filter_not_unique_column_route_logic_token_stream(&operation);
                    quote::quote! {
                        // #filter_not_unique_column_token_stream
                    }
                });
                // println!("{parameters_logic_token_stream}");
                let query_string_token_stream = {
                    let query_token_stream = generate_quotes::double_quotes_token_stream(
                        &format!("{select_snake_case} {{}} {from_snake_case} {ident_snake_case_stringified} {where_snake_case} {primary_key_field_ident} = $1")
                    );
                    let query_vec_column_token_stream = generate_query_vec_column_token_stream(
                        // &operation
                    );
                    // println!("{query_vec_column_token_stream}");
                    quote::quote! {
                        format!(
                            #query_token_stream,
                            #query_vec_column_token_stream,
                        )
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = quote::quote! {
                        let #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(#parameters_snake_case.#payload_snake_case.#primary_key_field_ident, #query_snake_case);
                    };
                    quote::quote! {
                        let #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #binded_query_modifications_token_stream
                        #query_snake_case
                    }
                };
                // println!("{binded_query_token_stream}");
                let postgresql_logic_token_stream = {
                    let fetch_one_token_stream = generate_fetch_one_token_stream(
                        &generate_options_try_from_sqlx_row_token_stream(&operation),
                        &generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!()),
                    );
                    quote::quote! {
                        #fetch_one_token_stream
                        #value_snake_case
                    }
                };
                // println!("{postgresql_logic_token_stream}");
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &ident_snake_case_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!("{try_operation_route_logic_token_stream}");
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(&operation, &{
                let mut value = common_http_request_syn_variants.clone();
                value.push(not_unique_column_syn_variant_wrapper.get_syn_variant().clone());
                value
            });
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &ident_options_upper_camel_case,
                &{
                    //todo where logic was commented. untill rewriting better where logic there is not point to add it again
                    // let filter_not_unique_column_token_stream = generate_filter_not_unique_column_http_request_token_stream(&operation);
                    quote::quote! {
                        // #filter_not_unique_column_token_stream
                    }
                },
                // &match fields_named_from_or_try_from {
                //     postgresql_crud_common::FromOrTryFrom::From => quote::quote! {#ident_options_upper_camel_case::#from_snake_case(#value_snake_case)},
                //     postgresql_crud_common::FromOrTryFrom::TryFrom => quote::quote! {#value_snake_case},
                // }
                &quote::quote! {#value_snake_case},
            );
            // println!("{try_operation_token_stream}");
            // let try_operation_test_token_stream = {
            //     let element_fields_initialization_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
            //         let field_ident = &element.field_ident;
            //         let field_type = &element.field.ty;
            //         quote::quote!{
            //             #field_ident: #field_type::default()
            //         }
            //     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            //     let test_content_token_stream = quote::quote! {
            //         let #primary_key_token_stream = match #try_operation_snake_case_token_stream(
            //             #reference_api_location_test_token_stream,
            //             #operation_parameters_upper_camel_case_token_stream {
            //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
            //                     #(#element_fields_initialization_token_stream),*
            //                 }
            //             },
            //         )
            //         .await
            //         {
            //             Ok(value) => {
            //                 println!("{value:#?}");
            //                 value
            //             },
            //             Err(#error_snake_case) => panic!(
            //                 "{}",
            //                 #error_snake_case
            //             )
            //         };
            //     };
            //     naming::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // };
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        // println!("{try_operation_token_stream}");
        //     let http_request_test_expect_success_token_stream = {
        //         let test_content_token_stream = quote::quote! {
        //             match #try_operation_snake_case_token_stream(
        //                 #reference_api_location_test_token_stream,
        //                 #operation_parameters_upper_camel_case_token_stream {
        //                     #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
        //                         #primary_key_field_ident: #primary_key_token_stream.clone(),//todo
        //                         #select_snake_case_token_stream: #ident_column_select_upper_camel_case_token_stream::#select_full_variant_token_stream
        //                     }
        //                 },
        //             )
        //             .await
        //             {
        //                 Ok(value) => println!("{value:#?}"),
        //                 Err(#error_snake_case) => panic!("{}", #error_snake_case)
        //             };
        //         };
        //         naming::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
        //     };
        //     let http_request_test_expect_fail_token_stream = {
        //         let test_content_token_stream = quote::quote! {
        //             match #try_operation_snake_case_token_stream(
        //                 #reference_api_location_test_token_stream,
        //                 #operation_parameters_upper_camel_case_token_stream {
        //                     #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
        //                         #primary_key_field_ident: #primary_key_token_stream.clone(),//todo
        //                         #select_snake_case_token_stream: #ident_column_select_upper_camel_case_token_stream::#select_full_variant_token_stream
        //                     }
        //                 },
        //             )
        //             .await
        //             {
        //                 Ok(value) => panic!("{value:#?}"),
        //                 Err(#error_snake_case) => println!("{}", #error_snake_case)
        //             };
        //         };
        //         naming::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
        //     };
        //     (
        //         quote::quote! {
        //             #try_operation_error_named_token_stream
        //             #try_operation_token_stream
        //         },
        //         http_request_test_expect_success_token_stream,
        //         http_request_test_expect_fail_token_stream,
        //     )
        // println!("{try_operation_token_stream}");
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &naming::parameter::SelfPayloadUpperCamelCase::from_display(&operation),
            &quote::quote! {
                Self {
                    #primary_key_field_ident: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream,
                    #select_snake_case: #postgresql_crud_all_enum_variants_array_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                }
            },
        );
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream
            #operation_payload_example_route_logic_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     // &proc_macro_name_upper_camel_case,
    //     "read_one",
    //     &read_one_token_stream,
    // );
    // //todo update where ...?
    let update_many_token_stream = {
        let operation = Operation::UpdateMany;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 6);
                common_route_syn_variants.iter().for_each(|element| {
                    value.push(*element);
                });
                value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(non_existing_primary_keys_to_update_syn_variant_wrapper.get_syn_variant());
                value.push(non_existing_primary_keys_to_update_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(not_unique_primary_key_to_update_syn_variant_wrapper.get_syn_variant());
                value.push(bind_query_syn_variant_wrapper.get_syn_variant());
                value.push(no_payload_fields_primary_key_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(&operation, generate_payload_and_payload_element_token_stream(&operation, &update_fields_token_stream));
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream =
                generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                    &operation,
                    &std_vec_vec_primary_key_field_type_to_update_token_stream,
                    &type_variants_from_request_response_syn_variants,
                );
            // println!("{try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream}");
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &{
                    let filter_not_unique_primary_key_token_stream = {
                        let filter_not_unique_token_stream = generate_filter_not_unique_token_stream(
                            &quote::quote! {&#value_snake_case.0},
                            &quote::quote! {&#element_snake_case.#primary_key_field_ident},
                            &quote::quote! {#element_snake_case.#primary_key_field_ident},
                            &quote::quote! {#element_snake_case.#primary_key_field_ident.clone()},
                            &generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &not_unique_primary_key_syn_variant_wrapper, file!(), line!(), column!()),
                        );
                        quote::quote! {{ #filter_not_unique_token_stream }}
                    };
                    let filter_no_payload_fields_token_stream = {
                        let filter_no_payload_fields_element_token_stream = generate_filter_no_payload_fields_token_stream(&operation, &quote::quote! {#element_snake_case});
                        quote::quote! {
                            for #element_snake_case in &#value_snake_case.0 {
                                #filter_no_payload_fields_element_token_stream
                            }
                        }
                    };
                    quote::quote! {
                        #filter_not_unique_primary_key_token_stream
                        #filter_no_payload_fields_token_stream
                    }
                });
                let expected_primary_keys_token_stream = quote::quote! {
                    let #expected_primary_keys_snake_case = #parameters_snake_case
                        .#payload_snake_case
                        .0
                        .iter()
                        .map(|#element_snake_case| #element_snake_case.#primary_key_field_ident.clone()) //todo - maybe its not a good idea to remove .clone here coz in macro dont know what type
                        .collect::<#std_vec_vec_primary_key_field_type_to_update_token_stream>();
                };
                let query_string_token_stream = {
                    let query_start_token_stream = generate_quotes::double_quotes_token_stream(&format!("{update_snake_case} {ident_snake_case_stringified} {set_snake_case} "));
                    let query_snake_case = naming::QuerySnakeCase;
                    // let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &bind_query_syn_variant_wrapper, file!(), line!(), column!());
                    let fields_named_excluding_primary_key_update_assignment_token_stream = fields_without_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let field_ident_double_quotes_token_stream =  generate_quotes::double_quotes_token_stream(&field_ident);
                        let is_field_ident_update_exists_snake_case = naming::parameter::IsSelfUpdateExistSnakeCase::from_tokens(&field_ident);
                        let case_snake_case = naming::CaseSnakeCase;
                        let field_ident_equals_case_token_stream = generate_quotes::double_quotes_token_stream(&format!("{field_ident} = {case_snake_case} "));
                        let else_snake_case = naming::ElseSnakeCase;
                        let end_snake_case = naming::EndSnakeCase;
                        let else_field_ident_end_token_stream = generate_quotes::double_quotes_token_stream(&format!("{else_snake_case} {field_ident} {end_snake_case},"));
                        let when_primary_key_field_ident_equals_then_token_stream = generate_quotes::double_quotes_token_stream(
                            &format!("{} {primary_key_field_ident} = {{}} {} {{}} ", naming::WhenSnakeCase, naming::ThenSnakeCase)
                        );
                        let field_type = &element.syn_field.ty;
                        let primary_key_field_type = &primary_key_field.syn_field.ty;
                        let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
                        quote::quote! {
                            {
                                let mut #is_field_ident_update_exists_snake_case = false;
                                for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                    if #element_snake_case.#field_ident.is_some() {
                                        #is_field_ident_update_exists_snake_case = true;
                                        break;
                                    }
                                }
                                if #is_field_ident_update_exists_snake_case {
                                    let mut #acc_snake_case = #std_string_string::#from_snake_case(#field_ident_equals_case_token_stream);
                                    for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                        if let Some(#value_snake_case) = &#element_snake_case.#field_ident {
                                            #acc_snake_case.push_str(&format!(
                                                #when_primary_key_field_ident_equals_then_token_stream,
                                                match <#primary_key_field_type as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::#update_query_part_snake_case(
                                                    &#element_snake_case.#primary_key_field_ident,
                                                    // &#primary_key_field_ident_double_quotes_token_stream,
                                                    &"",
                                                    &#primary_key_field_ident_double_quotes_token_stream,
                                                    &"",
                                                    &mut #increment_snake_case,

                                                    // update: &Self::Update,
                                                    // jsonb_set_accumulator: &std::primitive::str,
                                                    // jsonb_set_target: &std::primitive::str,
                                                    // jsonb_set_path: &std::primitive::str,
                                                    // increment: &mut std::primitive::u64
                                                ) {
                                                    Ok(#value_snake_case) => #value_snake_case,
                                                    Err(#error_0_token_stream) => {
                                                        // #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                        todo!()
                                                    }
                                                }
                                                // match #postgresql_crud_snake_case::BindQuery::try_generate_bind_increments(&#element_snake_case.#primary_key_field_ident, &mut #increment_snake_case) {
                                                //     Ok(#value_snake_case) => #value_snake_case,
                                                //     Err(#error_0_token_stream) => {
                                                //         #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                //     }
                                                // }
                                                
                                                ,

                                               match <#field_type as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::#update_query_part_snake_case(
                                                    &#value_snake_case.#value_snake_case,
                                                    // &#field_ident_double_quotes_token_stream,
                                                    &"",
                                                    &#field_ident_double_quotes_token_stream,
                                                    &"",
                                                    &mut #increment_snake_case,

                                                    // update: &Self::Update,
                                                    // jsonb_set_accumulator: &std::primitive::str,
                                                    // jsonb_set_target: &std::primitive::str,
                                                    // jsonb_set_path: &std::primitive::str,
                                                    // increment: &mut std::primitive::u64
                                                ) {
                                                    Ok(#value_snake_case) => #value_snake_case,
                                                    Err(#error_0_token_stream) => {
                                                        // #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                        todo!()
                                                    }
                                                }
                                                // match #postgresql_crud_snake_case::BindQuery::try_generate_bind_increments(&#value_snake_case.#value_snake_case, &mut #increment_snake_case) {
                                                //     Ok(#value_snake_case) => #value_snake_case,
                                                //     Err(#error_0_token_stream) => {
                                                //         #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                //     }
                                                // }
                                            ));
                                        }
                                    }
                                    #query_snake_case.push_str(&format!("{}{}",
                                        #acc_snake_case,
                                        #else_field_ident_end_token_stream
                                    ));
                                }
                            }
                        }
                    });
                    let where_primary_key_field_ident_in_primary_keys_returning_primary_key_field_ident_token_stream = {
                        let where_primary_key_field_ident_in_primary_keys_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(
                            &format!(" {where_snake_case} {primary_key_field_ident} {} ({{}}) {returning_snake_case} {primary_key_field_ident};", naming::InSnakeCase)
                        );
                        let primary_key_field_type = &primary_key_field.syn_field.ty;
                        let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
                        quote::quote! {
                            #query_snake_case.push_str(&format!(
                                #where_primary_key_field_ident_in_primary_keys_double_quotes_token_stream,
                                {
                                    let mut #acc_snake_case = #std_string_string::default();
                                    for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                        // match #postgresql_crud_snake_case::BindQuery::try_generate_bind_increments(&#element_snake_case.#primary_key_field_ident, &mut #increment_snake_case) {
                                        //     Ok(#value_snake_case) => {
                                        //         #acc_snake_case.push_str(&format!("{value},"));
                                        //     },
                                        //     Err(#error_0_token_stream) => {
                                        //         #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                        //     }
                                        // }
                                        //
                                        match <#primary_key_field_type as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::#update_query_part_snake_case(
                                            &#element_snake_case.#primary_key_field_ident,
                                            // &#primary_key_field_ident_double_quotes_token_stream,
                                            &"",
                                            &#primary_key_field_ident_double_quotes_token_stream,
                                            &"",
                                            &mut #increment_snake_case,

                                            // update: &Self::Update,
                                            // jsonb_set_accumulator: &std::primitive::str,
                                            // jsonb_set_target: &std::primitive::str,
                                            // jsonb_set_path: &std::primitive::str,
                                            // increment: &mut std::primitive::u64
                                        ) {
                                            Ok(#value_snake_case) => {
                                                #acc_snake_case.push_str(&format!("{value},"));
                                            },
                                            Err(#error_0_token_stream) => {
                                                // #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                todo!()
                                            }
                                        }
                                    }
                                    let _ = #acc_snake_case.pop();
                                    #acc_snake_case
                                }
                            ));
                        }
                    };
                    quote::quote! {
                        {
                            let mut #query_snake_case = #std_string_string::#from_snake_case(#query_start_token_stream);
                            #increment_initialization_token_stream
                            #(#fields_named_excluding_primary_key_update_assignment_token_stream)*
                            let _ = #query_snake_case.pop();
                            #where_primary_key_field_ident_in_primary_keys_returning_primary_key_field_ident_token_stream
                            #query_snake_case
                        }
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
                    let fields_named_excluding_primary_key_update_assignment_token_stream = fields_without_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let field_type = &element.syn_field.ty;
                        let is_field_ident_update_exists_snake_case = naming::parameter::IsSelfUpdateExistSnakeCase::from_tokens(&field_ident);
                        quote::quote! {
                            {
                                let mut #is_field_ident_update_exists_snake_case = false;
                                for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                    if #element_snake_case.#field_ident.is_some() {
                                        #is_field_ident_update_exists_snake_case = true;
                                        break;
                                    }
                                }
                                if #is_field_ident_update_exists_snake_case {
                                    for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                        if let Some(#value_snake_case) = &#element_snake_case.#field_ident {
                                            #query_snake_case = #query_snake_case.bind(#element_snake_case.#primary_key_field_ident.clone());
                                            #query_snake_case = 
                                            // #postgresql_crud_snake_case::BindQuery::bind_value_to_query(#value_snake_case.#value_snake_case.clone(), #query_snake_case)
                                            <#field_type as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::#update_query_bind_snake_case(
                                                #value_snake_case.#value_snake_case.clone(),
                                                #query_snake_case,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    });
                    let primary_key_field_type = &primary_key_field.syn_field.ty;
                    let primary_key_update_assignment_token_stream = quote::quote! {
                        {
                            for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                #query_snake_case = 
                                // #query_snake_case.bind(#element_snake_case.#primary_key_field_ident.clone())
                                <#primary_key_field_type as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::#update_query_bind_snake_case(
                                    #element_snake_case.#primary_key_field_ident.clone(),
                                    #query_snake_case,
                                )

                                ;
                            }
                        }
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#fields_named_excluding_primary_key_update_assignment_token_stream)*
                        #primary_key_update_assignment_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(&operation, &{
                    let fetch_token_stream = generate_create_update_delete_many_fetch_token_stream(&operation);
                    let non_existing_primary_keys_check_token_stream = generate_non_existing_primary_keys_check_token_stream(&operation, &quote::quote!{#expected_primary_keys_snake_case});
                    quote::quote! {
                        #fetch_token_stream
                        {
                            #non_existing_primary_keys_check_token_stream
                        }
                    }
                });
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &ident_snake_case_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &expected_primary_keys_token_stream,
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            // println!("{try_operation_route_logic_token_stream}");
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!(" {try_operation_route_logic_token_stream}");
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(&operation, &{
                let mut value = common_http_request_syn_variants.clone();
                value.push(not_unique_primary_key_to_update_syn_variant_wrapper.get_syn_variant().clone());
                value
            });
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_primary_key_field_type_to_update_token_stream,
                &{
                    let filter_not_unique_primary_key_token_stream = generate_filter_not_unique_token_stream(
                        &quote::quote! {&#parameters_snake_case.#payload_snake_case.0},
                        &quote::quote! {&#element_snake_case.#primary_key_field_ident},
                        &quote::quote! {#element_snake_case.#primary_key_field_ident},
                        &quote::quote! {#element_snake_case.#primary_key_field_ident.clone()},
                        &{
                            let try_operation_error_named_upper_camel_case = naming::parameter::TrySelfErrorNamedUpperCamelCase::from_display(&operation);
                            let not_unique_primary_key_syn_variant_initialization_token_stream = generate_initialization_token_stream(&not_unique_primary_key_to_update_syn_variant_wrapper, file!(), line!(), column!());
                            quote::quote! {
                                return Err(#try_operation_error_named_upper_camel_case::#not_unique_primary_key_syn_variant_initialization_token_stream);
                            }
                        },
                    );
                    quote::quote! {
                        #filter_not_unique_primary_key_token_stream
                    }
                },
                &quote::quote! {
                    #value_snake_case
                    // .into_iter()
                    // .map(|#element_snake_case| #primary_key_inner_type_token_stream::#from_snake_case(#element_snake_case))
                    // .collect()
                },
            );
            // println!("{try_operation_token_stream}");
            // let try_operation_test_token_stream = {
            //     let fields_initialization_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
            //         let field_ident = &element.field_ident;
            //         let field_type = &element.field.ty;
            //         quote::quote!{
            //             #field_ident: #field_type::default()
            //         }
            //     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            //     let test_content_token_stream = quote::quote! {
            //         match #try_operation_snake_case_token_stream(
            //             &api_location,
            //             #operation_parameters_upper_camel_case_token_stream {
            //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream (
            //                     #primary_keys_token_stream.clone().into_iter().map(|element| {
            //                         #operation_payload_element_upper_camel_case_token_stream {
            //                             #primary_key_field_ident: element,
            //                             #(#fields_initialization_excluding_primary_key_token_stream),*//todo make sure name and color both are not None(make it option<value>, not just a value)
            //                         }
            //                     }).collect()
            //                 )
            //             }
            //         )
            //         .await
            //         {
            //             Ok(value) => println!("{value:#?}"),
            //             Err(#error_snake_case) => panic!("{}", #error_snake_case)
            //         }
            //     };
            //     naming::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // };
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        // println!("{try_operation_token_stream}");
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream = {
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_element_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &naming::parameter::SelfPayloadElementUpperCamelCase::from_display(&operation),
                &{
                    let primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                        quote::quote! {
                            #primary_key_field_ident: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                        }
                    };
                    let fields_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = fields_without_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        quote::quote! {
                            #field_ident: Some(postgresql_crud::Value{
                                #value_snake_case: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                            })
                        }
                    });
                    quote::quote! {
                        Self{
                            #primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream,
                            #(#fields_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream),*
                        }
                    }
                },
            );
            let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
                &naming::parameter::SelfPayloadUpperCamelCase::from_display(&operation),
                &quote::quote! {
                    Self(vec![#postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream])
                },
            );
            quote::quote! {
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_element_token_stream
                #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream
            }
        };
        // println!("{impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream}");
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream
            #operation_payload_example_route_logic_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     // &proc_macro_name_upper_camel_case,
    //     "update_many",
    //     &update_many_token_stream,
    // );
    // //todo WHY ITS RETURN SUCCESS EVEN IF ROW DOES NOT EXISTS?
    let update_one_token_stream = {
        let operation = Operation::UpdateOne;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 3);
                common_route_syn_variants.iter().for_each(|element| {
                    value.push(*element);
                });
                value.push(bind_query_syn_variant_wrapper.get_syn_variant());
                value.push(no_payload_fields_primary_key_syn_variant_wrapper.get_syn_variant());
                value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(&operation, generate_operation_payload_token_stream(&operation, &update_fields_token_stream));
        //  println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                &operation,
                &primary_key_field_type_update_upper_camel_case,
                &type_variants_from_request_response_syn_variants,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &{
                    let filter_no_payload_fields_token_stream = generate_filter_no_payload_fields_token_stream(&operation, &quote::quote! {#value_snake_case});
                    quote::quote! {
                        #filter_no_payload_fields_token_stream
                    }
                });
                let query_string_token_stream = {
                    let query_start_token_stream = generate_quotes::double_quotes_token_stream(&format!("{update_snake_case} {ident_snake_case_stringified} {set_snake_case} "));
                    // let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &bind_query_syn_variant_wrapper, file!(), line!(), column!());
                    let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
                    let additional_parameters_modification_token_stream = fields_without_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let field_ident_double_quotes_token_stream =  generate_quotes::double_quotes_token_stream(&field_ident);
                        let field_ident_equals_dollar_increment_token_stream = generate_quotes::double_quotes_token_stream(&format!("{field_ident} = {{{value_snake_case}}},"));
                        let field_type = &element.syn_field.ty;
                        quote::quote! {
                            // match #crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream(
                            //     &#value_snake_case.#value_snake_case,
                            //     &mut #increment_snake_case,
                            // ) {
                            //     Ok(#value_snake_case) => {
                            //         #query_snake_case.push_str(&format!(#field_ident_equals_dollar_increment_token_stream));
                            //     }
                            //     Err(#error_0_token_stream) => {
                            //         #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                            //     }
                            // }
                            if let Some(#value_snake_case) = &#parameters_snake_case.#payload_snake_case.#field_ident {
                                match <#field_type as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::#update_query_part_snake_case(
                                    &#value_snake_case.#value_snake_case,
                                    // &#field_ident_double_quotes_token_stream,
                                    &"",
                                    &#field_ident_double_quotes_token_stream,
                                    &"",
                                    &mut #increment_snake_case,

                                    // update: &Self::Update,
                                    // jsonb_set_accumulator: &std::primitive::str,
                                    // jsonb_set_target: &std::primitive::str,
                                    // jsonb_set_path: &std::primitive::str,
                                    // increment: &mut std::primitive::u64
                                ) {
                                    Ok(#value_snake_case) => {
                                        //todo fix it. its incorrect
                                        #query_snake_case.push_str(&format!(#field_ident_equals_dollar_increment_token_stream));
                                    }
                                    Err(#error_0_token_stream) => {
                                        todo!()
                                    }
                                }
                            }
                        }
                    }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                    let additional_parameters_primary_key_modification_token_stream = {
                        let query_part_token_stream = generate_quotes::double_quotes_token_stream(&format!(" {where_snake_case} {primary_key_field_ident} = {{{value_snake_case}}}"));
                        let primary_key_field_type = &primary_key_field.syn_field.ty;
                        quote::quote! {
                            // match #crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream(&#parameters_snake_case.#payload_snake_case.#primary_key_field_ident, &mut #increment_snake_case) {
                            //     Ok(#value_snake_case) => {
                            //         #query_snake_case.push_str(&format!(#query_part_token_stream));
                            //     },
                            //     Err(#error_0_token_stream) => {
                            //         #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                            //     },
                            // }
                            match <#primary_key_field_type as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::#update_query_part_snake_case(
                                &#parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                                // &#primary_key_field_ident_double_quotes_token_stream,
                                &"",
                                &#primary_key_field_ident_double_quotes_token_stream,
                                &"",
                                &mut #increment_snake_case,

                                // update: &Self::Update,
                                // jsonb_set_accumulator: &std::primitive::str,
                                // jsonb_set_target: &std::primitive::str,
                                // jsonb_set_path: &std::primitive::str,
                                // increment: &mut std::primitive::u64
                            ) {
                                Ok(#value_snake_case) => {
                                    //todo fix it. its incorrect
                                    #query_snake_case.push_str(&format!(#query_part_token_stream));
                                },
                                Err(#error_0_token_stream) => {
                                    todo!()
                                },
                            }
                        }
                    };
                    quote::quote! {
                        {
                            #increment_initialization_token_stream
                            let mut #query_snake_case = #std_string_string::#from_snake_case(#query_start_token_stream);
                            #(#additional_parameters_modification_token_stream)*
                            let _ = #query_snake_case.pop();
                            #additional_parameters_primary_key_modification_token_stream
                            #query_snake_case.push_str(&format!(#returning_primary_key_double_quotes_token_stream));
                            #query_snake_case
                        }
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
                    let binded_query_modifications_token_stream = fields_without_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let field_type = &element.syn_field.ty;
                        quote::quote! {
                            if let Some(#value_snake_case) = #parameters_snake_case.#payload_snake_case.#field_ident {
                                // #query_snake_case = #postgresql_crud_snake_case::BindQuery::bind_value_to_query(
                                //     #value_snake_case.#value_snake_case,
                                //     #query_snake_case,
                                // );
                                #query_snake_case = <#field_type as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::#update_query_bind_snake_case(
                                    #value_snake_case.#value_snake_case,
                                    #query_snake_case,
                                );
                            }
                        }
                    });
                    let binded_query_primary_key_modification_token_stream = {
                        let primary_key_field_type = &primary_key_field.syn_field.ty;
                        quote::quote! {
                            // #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(
                            //     #parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                            //     #query_snake_case,
                            // );
                            #query_snake_case = <#primary_key_field_type as postgresql_crud::postgresql_type::postgresql_type_trait::PostgresqlType>::#update_query_bind_snake_case(
                                #parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                                #query_snake_case,
                            );
                        }
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#binded_query_modifications_token_stream)*
                        #binded_query_primary_key_modification_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(&operation, &generate_create_update_delete_one_fetch_token_stream(&operation));
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &ident_snake_case_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!("{try_operation_route_logic_token_stream}");
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &primary_key_field_type_update_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &value_snake_case,
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        // println!("{try_operation_token_stream}");
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &naming::parameter::SelfPayloadUpperCamelCase::from_display(&operation),
            &{
                let primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                    quote::quote! {
                        #primary_key_field_ident: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                    }
                };
                let fields_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = fields_without_primary_key.iter().map(|element| {
                    let field_ident = &element.field_ident;
                    quote::quote! {
                        #field_ident: Some(postgresql_crud::Value{
                            #value_snake_case: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                        })
                    }
                });
                quote::quote! {
                    Self {
                        #primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream,
                        #(#fields_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream),*
                    }
                }
            },
        );
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream
            #operation_payload_example_route_logic_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     // &proc_macro_name_upper_camel_case,
    //     "uupdate_one",
    //     &update_one_token_stream,
    // );
    let delete_many_token_stream = {
        let operation = Operation::DeleteMany;
        let no_payload_fields_syn_variant_wrapper = new_syn_variant_wrapper(
            &naming::NoPayloadFieldsUpperCamelCase,
            Some(macros_helpers::status_code::StatusCode::BadRequest400),
            std::vec::Vec::<(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute, &'static dyn std::fmt::Display, syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>)>::default(),
        );
        let no_primary_keys_syn_variant_wrapper = new_syn_variant_wrapper(
            &naming::NoPrimaryKeysUpperCamelCase,
            Some(macros_helpers::status_code::StatusCode::BadRequest400),
            std::vec::Vec::<(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute, &'static dyn std::fmt::Display, syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>)>::default(),
        );
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = std::vec::Vec::with_capacity(common_route_syn_variants.len() + 7 + not_unique_fields_syn_variants_wrappers.len());
                common_route_syn_variants.iter().for_each(|element| {
                    value.push(*element);
                });
                value.push(bind_query_syn_variant_wrapper.get_syn_variant());
                value.push(no_payload_fields_syn_variant_wrapper.get_syn_variant());
                value.push(no_primary_keys_syn_variant_wrapper.get_syn_variant());
                value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(non_existing_primary_keys_to_delete_syn_variant_wrapper.get_syn_variant());
                value.push(non_existing_primary_keys_to_delete_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(not_unique_primary_key_to_delete_syn_variant_wrapper.get_syn_variant());
                not_unique_fields_syn_variants_wrappers.iter().for_each(|element| {
                    value.push(element.get_syn_variant());
                });
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_token_stream(
            &operation,
            generate_operation_payload_token_stream(
                &operation,
                &pub_fields_idents_std_option_option_std_vec_vec_where_inner_type_token_stream
            ),
        );
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream =
                generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                    &operation,
                    &std_vec_vec_primary_key_field_type_to_delete_token_stream,
                    &type_variants_from_request_response_syn_variants,
                );
            // println!("{try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream}");
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &{
                    //todo where logic was commented. untill rewriting better where logic there is not point to add it again
                    // let filter_no_payload_fields_token_stream = {
                    //     let none_fields_named_token_stream = fields.iter().map(|_| {
                    //         let none_upper_camel_case = naming::NoneUpperCamelCase;
                    //         quote::quote! {#none_upper_camel_case}
                    //     });
                    //     let value_fields_named_token_stream = fields.iter().map(|element| {
                    //         let field_ident = &element.field_ident;
                    //         quote::quote! {&#value_snake_case.#field_ident}
                    //     });
                    //     let no_payload_fields_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &no_payload_fields_syn_variant_wrapper, file!(), line!(), column!());
                    //     quote::quote! {
                    //         if let (#(#none_fields_named_token_stream),*) = (#(#value_fields_named_token_stream),*) {
                    //             #no_payload_fields_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    //         }
                    //     }
                    // };
                    // let filter_no_primary_keys_token_stream = {
                    //     let no_primary_keys_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &no_primary_keys_syn_variant_wrapper, file!(), line!(), column!());
                    //     quote::quote! {
                    //         if let Some(#value_snake_case) = &#value_snake_case.#primary_key_field_ident {
                    //             if #value_snake_case.is_empty() {
                    //                 #no_primary_keys_syn_variant_error_initialization_eprintln_response_creation_token_stream
                    //             }
                    //         }
                    //     }
                    // };
                    // let filter_not_unique_fields_token_stream = generate_filter_not_unique_fields_token_stream(&operation, &fields_without_primary_key);
                    quote::quote! {
                        // #filter_no_payload_fields_token_stream
                        // #filter_no_primary_keys_token_stream
                        // #filter_not_unique_fields_token_stream
                    }
                });
                let expected_primary_keys_token_stream = quote::quote! {
                    let #expected_primary_keys_snake_case = #parameters_snake_case.#payload_snake_case.#primary_key_field_ident.clone();
                };
                let query_string_token_stream = {
                    let additional_parameters_modification_token_stream = fields_without_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{field_ident} = {{{value_snake_case}}}"));
                        let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &bind_query_syn_variant_wrapper, file!(), line!(), column!());
                        quote::quote! {
                            if let Some(#value_snake_case) = &#parameters_snake_case.#payload_snake_case.#field_ident {
                                for #element_snake_case in #value_snake_case {
                                    match #crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream(
                                        #element_snake_case,
                                        &mut #increment_snake_case
                                    ) {
                                        Ok(#value_snake_case) => {
                                            let handle = format!(#handle_token_stream);
                                            match additional_parameters.is_empty() {
                                                true => {
                                                    additional_parameters.push_str(&handle);
                                                },
                                                false => {
                                                    additional_parameters.push_str(&format!(" AND {handle}"));//todo
                                                },
                                            }
                                        },
                                        Err(#error_0_token_stream) => {
                                            #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                        },
                                    }
                                }
                            }
                        }
                    });
                    let additional_parameters_primary_key_modification_token_stream = {
                        let handle_token_stream = {
                            let handle_stringified = format!("\" {primary_key_field_ident} {in_snake_case} ({{}})\"");
                            handle_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{handle_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                        };
                        let bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &bind_query_syn_variant_wrapper, file!(), line!(), column!());
                        quote::quote! {
                            if let Some(#primary_key_field_ident) = &#parameters_snake_case.#payload_snake_case.#primary_key_field_ident {
                                if let false = additional_parameters.is_empty() {
                                    additional_parameters.push_str(#space_and_double_quotes_token_stream);
                                }
                                additional_parameters.push_str(&format!(
                                    #handle_token_stream,
                                    {
                                        let mut additional_parameters = #std_string_string::default();
                                        for #element_snake_case in #primary_key_field_ident {
                                            match #crate_server_postgres_bind_query_bind_query_try_generate_bind_increments_token_stream(
                                                #element_snake_case,
                                                &mut #increment_snake_case,
                                            ) {
                                                Ok(#value_snake_case) => {
                                                    additional_parameters.push_str(&format!("{value},"));
                                                }
                                                Err(#error_0_token_stream) => {
                                                    #bind_query_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                }
                                            }
                                        }
                                        let _ = additional_parameters.pop();
                                        additional_parameters
                                    }
                                ));
                            }
                        }
                    };
                    let handle_token_stream = generate_quotes::double_quotes_token_stream(
                        &format!("{delete_snake_case} {from_snake_case} {ident_snake_case_stringified} {where_snake_case} {{}} {returning_primary_key_stringified}")
                    );
                    quote::quote! {
                        format!(
                            #handle_token_stream,
                            {
                                #increment_initialization_token_stream
                                let mut additional_parameters = #std_string_string::default();
                                #(#additional_parameters_modification_token_stream)*
                                #additional_parameters_primary_key_modification_token_stream
                                additional_parameters
                            }
                        )
                    }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = fields_without_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        quote::quote! {
                            if let Some(#value_snake_case) = #parameters_snake_case.#payload_snake_case.#field_ident {
                                for #element_snake_case in #value_snake_case {
                                    #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(#element_snake_case, #query_snake_case);
                                }
                            }
                        }
                    });
                    let binded_query_primary_key_modifications_token_stream = quote::quote! {
                        if let Some(#primary_key_field_ident) = #parameters_snake_case.#payload_snake_case.#primary_key_field_ident {
                            for #element_snake_case in #primary_key_field_ident {
                                #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(#element_snake_case, #query_snake_case);
                            }
                        }
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#binded_query_modifications_token_stream)*
                        #binded_query_primary_key_modifications_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(&operation, &{
                    let fetch_token_stream = generate_create_update_delete_many_fetch_token_stream(&operation);
                    let non_existing_primary_keys_check_token_stream = generate_non_existing_primary_keys_check_token_stream(&operation, &error_snake_case);
                    quote::quote! {
                        #fetch_token_stream
                        {
                            if let Some(#error_snake_case) = #expected_primary_keys_snake_case {
                                #non_existing_primary_keys_check_token_stream
                            }
                        }
                    }
                });
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &ident_snake_case_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &expected_primary_keys_token_stream,
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            // println!("{try_operation_route_logic_token_stream}");
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        // println!("{try_operation_route_logic_token_stream}");
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_primary_key_field_type_to_delete_token_stream,
                &proc_macro2::TokenStream::new(), //todo maybe add filter on not unique primary key like in read_many ?
                &value_snake_case
            );
            // println!("{try_operation_token_stream}");
            // let try_operation_test_token_stream = {
            //     let fields_initialization_excluding_primary_key_token_stream = fields_named_excluding_primary_key.iter().map(|element|{
            //         let field_ident = &element.field_ident;
            //         quote::quote!{
            //             //todo or and support where filter
            //             #field_ident: None
            //         }
            //     }).collect::<std::vec::Vec<proc_macro2::TokenStream>>();
            //     let test_content_token_stream = quote::quote! {
            //         match #try_operation_snake_case_token_stream(
            //             #reference_api_location_test_token_stream,
            //             //todo - builder pattern?
            //             #operation_parameters_upper_camel_case_token_stream {
            //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
            //                     #primary_key_field_ident: Some(
            //                         #primary_keys_token_stream.clone()
            //                     ),
            //                     #(#fields_initialization_excluding_primary_key_token_stream),*
            //                 }
            //             },
            //         )
            //         .await
            //         {
            //             Ok(value) => println!("{value:#?}"),
            //             // let vec_cat_id: Vec<
            //             //     super::DogId,
            //             // > = value
            //             //     .into_iter()
            //             //     .filter_map(|value| match value.id {
            //             //         Some(id) => Some(
            //             //             super::DogId {
            //             //                 id,
            //             //             },
            //             //         ),
            //             //         None => None,
            //             //     })
            //             //     .collect();
            //             // println!("{vec_cat_id:#?}");
            //             Err(#error_snake_case) => panic!("{}", #error_snake_case)
            //         }
            //     };
            //     naming::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // };
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        // println!("{try_operation_token_stream}");
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &naming::parameter::SelfPayloadUpperCamelCase::from_display(&operation),
            &{
                let primary_key_token_stream = {
                    quote::quote! {
                        #primary_key_field_ident: Some(vec![
                            #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                        ])
                    }
                };
                let fields_without_primary_key_token_stream = fields_without_primary_key.iter().map(|element| {
                    let field_ident = &element.field_ident;
                    quote::quote! {
                        #field_ident: Some(vec![
                            #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                        ])
                    }
                });
                quote::quote! {
                    Self {
                        #primary_key_token_stream,
                        #(#fields_without_primary_key_token_stream),*
                    }
                }
            },
        );
        // println!("{impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream}");
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream
            #operation_payload_example_route_logic_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &delete_many_token_stream,
    // );
    let delete_one_token_stream = {
        let operation = Operation::DeleteOne;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(&common_route_with_row_and_rollback_syn_variants, &operation);
        let parameters_token_stream = generate_parameters_pattern_token_stream(&operation, generate_operation_payload_token_stream(
            &operation,
            &generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream(
                &naming::parameter::SelfToDeleteUpperCamelCase::from_type_last_segment(&primary_key_field.syn_field.ty)
            )
        ));
        // println!("{parameters_token_stream}");
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream =
                generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                    &operation,
                    &primary_key_field_type_to_delete_upper_camel_case,
                    &type_variants_from_request_response_syn_variants,
                );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let query_token_stream = generate_quotes::double_quotes_token_stream(
                        &format!("{delete_snake_case} {from_snake_case} {ident_snake_case_stringified} {where_snake_case} {primary_key_field_ident} = $1 {returning_primary_key_stringified}")
                    );
                    quote::quote! {format!(#query_token_stream) }
                };
                // println!("{query_string_token_stream}");
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = quote::quote! {
                        #query_snake_case = #postgresql_crud_bind_query_bind_query_bind_value_to_query_token_stream(
                            #parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                            #query_snake_case
                        );
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #binded_query_modifications_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(&operation, &generate_create_update_delete_one_fetch_token_stream(&operation));
                // let swagger_open_api_token_stream = generate_swagger_open_api_token_stream(
                //     &ident_snake_case_stringified,
                //     &unique_status_codes,
                //     &application_json_quotes_token_stream,
                //     &table_name_quotes_token_stream,
                //     &operation_payload_upper_camel_case_token_stream,
                //     &operation,
                // );
                generate_try_operation_route_logic_token_stream(
                    &operation,
                    &common_additional_route_logic_token_stream,
                    &parameters_logic_token_stream,
                    &proc_macro2::TokenStream::new(),
                    &query_string_token_stream,
                    &binded_query_token_stream,
                    &postgresql_logic_token_stream,
                )
            };
            // println!("{try_operation_route_logic_token_stream}");
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            // println!("{try_operation_error_named_token_stream}");
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &primary_key_field_type_to_delete_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &value_snake_case
            );
            // let try_operation_test_token_stream = {
            //     let test_content_token_stream = quote::quote! {
            //         match #try_operation_snake_case_token_stream(
            //             #reference_api_location_test_token_stream,
            //             #operation_parameters_upper_camel_case_token_stream {
            //                 #payload_snake_case_token_stream: #operation_payload_upper_camel_case_token_stream {
            //                     #primary_key_field_ident: #primary_key_token_stream.clone()
            //                 }
            //             },
            //         )
            //         .await
            //         {
            //             Ok(value) => println!("{value:#?}"),
            //             Err(#error_snake_case) => panic!("{}", #error_snake_case)
            //         }
            //     };
            //     naming::WrapIntoStartEndPrintlnSelfTokenStream::wrap_into_start_end_println_self_token_stream(&operation, &test_content_token_stream)
            // };
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        // println!("{try_operation_token_stream}");
        let impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream = postgresql_crud_macros_common::generate_impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_tokens_token_stream(
            &naming::parameter::SelfPayloadUpperCamelCase::from_display(&operation),
            &{
                let primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                    quote::quote! {
                        #primary_key_field_ident: #postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_call_token_stream
                    }
                };
                quote::quote! {
                    Self {
                        #primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream,
                    }
                }
            },
        );
        // println!("{impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream}");
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_for_operation_payload_token_stream
            #operation_payload_example_route_logic_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &delete_one_token_stream,
    // );
    // // let emulate_crud_api_usage_test_token_stream = {
    // //     let ident_emulate_crud_api_usage_test_snake_case_token_stream = {
    // //         let ident_emulate_crud_api_usage_test_snake_case_stringified =
    // //             format!("{ident_snake_case_stringified}_emulate_crud_api_usage_test");
    // //         ident_emulate_crud_api_usage_test_snake_case_stringified.parse::<proc_macro2::TokenStream>()
    // //         .unwrap_or_else(|_| panic!("{ident_emulate_crud_api_usage_test_snake_case_stringified} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    // //     };
    // //     quote::quote! {
    // //         #[test]
    // //         fn #ident_emulate_crud_api_usage_test_snake_case_token_stream() {
    // //             async fn find_out_if_it_works() {
    // //                 let api_location = #std_string_string_token_stream::#from_snake_case("http://127.0.0.1:8080");//todo port from env or config maybe?
    // //                 let limit = 1000;
    // //                 let offset = 0;
    // //                 #create_many_test_token_stream
    // //                 #read_many_test_token_stream
    // //                 #update_many_test_token_stream
    // //                 #read_many_test_token_stream
    // //                 #delete_many_test_token_stream
    // //                 #read_many_test_token_stream
    // //                 #create_one_test_token_stream
    // //                 #read_one_test_token_stream
    // //                 #update_one_test_token_stream
    // //                 #read_one_test_token_stream
    // //                 #delete_one_test_token_stream
    // //                 #read_one_test_token_stream
    // //             }
    // //             match tokio::runtime::Builder::new_multi_thread()
    // //                 .worker_threads(num_cpus::get())
    // //                 .enable_all()
    // //                 .build()
    // //             {
    // //                 Err(#error_snake_case) => {
    // //                     panic!("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {:#?}", #error_snake_case)
    // //                 }
    // //                 Ok(runtime) => {
    // //                     runtime.block_on(find_out_if_it_works());
    // //                 }
    // //             }
    // //         }
    // //     }
    // // };
    // // println!("{emulate_crud_api_usage_test_token_stream}");
    // println!("{create_table_if_not_exists_function_token_stream}");
    let common_token_stream = {
        quote::quote! {
            #impl_ident_table_name_token_stream
            #ident_options_token_stream

            // // #from_ident_for_ident_postgresql_json_type_options_to_read_token_stream
            #column_token_stream
            #allow_methods_token_stream
            #ident_column_read_permission_token_stream
            // #(#reexport_postgresql_sqlx_column_types_token_stream)*
            // #create_table_if_not_exists_function_token_stream

            // #[cfg(test)]
            // mod test_try_create_many {
                // #emulate_crud_api_usage_test_token_stream
            // }
        }
    };
    // // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    // //     &proc_macro_name_upper_camel_case,
    // //     &common_token_stream,
    // // );
    // //comment out coz its impossible to correctly generate tokens for debug purposes
    // // let _mod_name_snake_case_token_stream = {
    // //     let value = format!("{proc_macro_name_snake_case}_{ident_snake_case_stringified}");
    // //     value.parse::<proc_macro2::TokenStream>()
    // //     .unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    // // };
    // //todo pub and private impl quote group
    let generated = quote::quote! {
        // //comment out coz its impossible to correctly generate tokens
        // // pub mod #mod_name_snake_case_token_stream {/
            #common_token_stream

            #create_many_token_stream
            // #create_one_token_stream
            #read_many_token_stream
            // #read_one_token_stream
            //todo fix trait calls in update many comparing with update_one
            #update_many_token_stream
            // #update_one_token_stream
            //// #delete_many_token_stream
            // #delete_one_token_stream
        // // }
    };
    // println!("{generated}");
    // if ident == "" {
        // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //     "GeneratePostgresqlCrud",
        //     &generated,
        // );
    // }
    generated.into()
}

// //todo maybe refactor or remove later
// #[derive(Debug, Clone, Copy, naming::AsRefStrEnumWithUnitFieldsToSnakeCaseStringified)]
// enum TestOperationPrintlnInfo {
//     Start,
//     End,
// }
// // impl std::fmt::Display for TestOperationPrintlnInfo {
// //     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
// //         match self {
// //             Self::Start => write!(f, "start"),
// //             Self::End => write!(f, "end")
// //         }
// //     }
// // }
// trait TrySelfSnakeCasePrintlnStringified {
//     fn try_self_snake_case_println_stringified(&self, test_operation_print_in_info: &crate::TestOperationPrintlnInfo) -> std::string::String;
// }

// impl<T> TrySelfSnakeCasePrintlnStringified for T
// where
//     T: naming::ToSnakeCaseStringified,
// {
//     fn try_self_snake_case_println_stringified(&self, test_operation_print_in_info: &crate::TestOperationPrintlnInfo) -> std::string::String {
//         let slashes = "-------";
//         format!(
//             "\"{}{}{} {}{}\"",
//             slashes,
//             naming::TrySnakeCase,
//             self.to_snake_case_stringified(),
//             naming::AsRefStrToSnakeCaseStringified::new(test_operation_print_in_info),
//             slashes,
//         )
//     }
// }
// trait TrySelfSnakeCasePrintlnTokenStream {
//     fn try_self_snake_case_println_token_stream(&self, test_operation_print_in_info: &TestOperationPrintlnInfo) -> proc_macro2::TokenStream;
// }

// impl<T> TrySelfSnakeCasePrintlnTokenStream for T
// where
//     T: TrySelfSnakeCasePrintlnStringified,
// {
//     fn try_self_snake_case_println_token_stream(&self, test_operation_print_in_info: &TestOperationPrintlnInfo) -> proc_macro2::TokenStream {
//         let value = self.try_self_snake_case_println_stringified(test_operation_print_in_info);
//         let value_token_stream = value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
//         quote::quote! {println!(#value_token_stream);}
//     }
// }

// trait WrapIntoStartEndPrintlnSelfTokenStream {
//     fn wrap_into_start_end_println_self_token_stream(&self, test_content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream;
// }

// impl<T> WrapIntoStartEndPrintlnSelfTokenStream for T
// where
//     T: TrySelfSnakeCasePrintlnTokenStream,
// {
//     fn wrap_into_start_end_println_self_token_stream(&self, test_content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
//         let start_println_token_stream = self.try_self_snake_case_println_token_stream(&TestOperationPrintlnInfo::Start);
//         let end_println_token_stream = self.try_self_snake_case_println_token_stream(&TestOperationPrintlnInfo::End);
//         quote::quote! {
//             #start_println_token_stream
//             #test_content_token_stream
//             #end_println_token_stream
//         }
//     }
// }