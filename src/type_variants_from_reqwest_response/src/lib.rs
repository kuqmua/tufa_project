static PATH: &str = "type_variants_from_reqwest_response";
static RESPONSE_VARIANTS: &str = "ResponseVariants";

fn get_vec_enum_paths(
    attribute: &syn::Attribute,
    ident_stringified: &std::string::String
) -> Vec<std::string::String> {
    let mut stringified_tokens = quote::ToTokens::to_token_stream(&attribute.tokens).to_string();
    stringified_tokens.retain(|c| !c.is_whitespace());
    match stringified_tokens.len() > 3 {
        true => {
            let mut chars = stringified_tokens.chars();
            match (chars.next(), chars.last()) {
                (None, None) => panic!("{ident_stringified} no first and last token attribute"),
                (None, Some(_)) => panic!("{ident_stringified} no first token attribute"),
                (Some(_), None) => panic!("{ident_stringified} no last token attribute"),
                (Some(first), Some(last)) => match (first == '(', last == ')') {
                    (true, true) => {
                        match stringified_tokens.get(1..(stringified_tokens.len()-1)) {
                            Some(inner_tokens_str) => {
                                inner_tokens_str.split(',').map(|str|{str.to_string()}).collect::<Vec<std::string::String>>()
                            },
                            None => panic!("{ident_stringified} cannot get inner_token"),
                        }
                    },
                    (true, false) => panic!("{ident_stringified} last token attribute is not )"),
                    (false, true) => panic!("{ident_stringified} first token attribute is not ("),
                    (false, false) => panic!("{ident_stringified} first token attribute is not ( and last token attribute is not )"),
                },
            }
        }
        false => panic!("{ident_stringified} stringified_tokens.len() > 3 == false"),
    }
}
fn generate_from_logic(
    ident: &syn::Ident,
    ident_response_variants_stringified: &std::string::String,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>
) -> proc_macro2::TokenStream {
    let ident_with_serialize_deserialize_stringified = format!("{ident}{}", proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified());
    let ident_with_serialize_deserialize_token_stream = ident_with_serialize_deserialize_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| {
        panic!("{ident} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE)
    });
    let enum_path_token_stream = ident_response_variants_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| {
            panic!("{ident} {ident_response_variants_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE)
        });
    let variants = {
        variants.iter().map(|variant| {
            let variant_ident = &variant.ident;
            match &variant.fields {
                syn::Fields::Named(fields_named) => {
                    let fields_generated = fields_named.named.iter().map(|field|{
                        field.ident.clone().unwrap_or_else(|| {
                            panic!("{ident} {ident_response_variants_stringified} field ident is None")
                        })
                    });
                    let fields_generated_cloned = fields_generated.clone();
                    quote::quote! {
                        #ident_with_serialize_deserialize_token_stream::#variant_ident { #(#fields_generated),* } => Self::#variant_ident { #(#fields_generated_cloned),* }
                    }
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    if let false = fields_unnamed.unnamed.len() == 1 {
                        panic!("{ident} fields_unnamed.unnamed.len() != 1");
                    }
                    quote::quote! {
                        #ident_with_serialize_deserialize_token_stream::#variant_ident(i) => Self::#variant_ident(i)
                    }
                }
                syn::Fields::Unit => panic!(
                    "{ident} works only with syn::Fields::Named and syn::Fields::Unnamed"
                ),
            }
        })
    };
    let gen = quote::quote! {
        impl std::convert::From<#ident> for #enum_path_token_stream
        {
            fn from(val: #ident) -> Self {
                match val.into_serialize_deserialize_version() {
                    #(#variants),*
                }
            }
        }
    };
    // if ident_response_variants_stringified == "" {
    //     println!("{gen}");
    // }
    gen
}

// #[proc_macro_derive(
//     TypeVariantsFromReqwestResponse,
//     attributes(
//         tvfrr_100_continue,
//         tvfrr_101_switching_protocols,
//         tvfrr_102_processing,
//         tvfrr_200_ok,
//         tvfrr_201_created,
//         tvfrr_202_accepted,
//         tvfrr_203_non_authoritative_information,
//         tvfrr_204_no_content,
//         tvfrr_205_reset_content,
//         tvfrr_206_partial_content,
//         tvfrr_207_multi_status,
//         tvfrr_208_already_reported,
//         tvfrr_226_im_used,
//         tvfrr_300_multiple_choices,
//         tvfrr_301_moved_permanently,
//         tvfrr_302_found,
//         tvfrr_303_see_other,
//         tvfrr_304_not_modified,
//         tvfrr_305_use_proxy,
//         tvfrr_307_temporary_redirect,
//         tvfrr_308_permanent_redirect,
//         tvfrr_400_bad_request,
//         tvfrr_401_unauthorized,
//         tvfrr_402_payment_required,
//         tvfrr_403_forbidden,
//         tvfrr_404_not_found,
//         tvfrr_405_method_not_allowed,
//         tvfrr_406_not_acceptable,
//         tvfrr_407_proxy_authentication_required,
//         tvfrr_408_request_timeout,
//         tvfrr_409_conflict,
//         tvfrr_410_gone,
//         tvfrr_411_length_required,
//         tvfrr_412_precondition_failed,
//         tvfrr_413_payload_too_large,
//         tvfrr_414_uri_too_long,
//         tvfrr_415_unsupported_media_type,
//         tvfrr_416_range_not_satisfiable,
//         tvfrr_417_expectation_failed,
//         tvfrr_418_im_a_teapot,
//         tvfrr_421_misdirected_request,
//         tvfrr_422_unprocessable_entity,
//         tvfrr_423_locked,
//         tvfrr_424_failed_dependency,
//         tvfrr_426_upgrade_required,
//         tvfrr_428_precondition_required,
//         tvfrr_429_too_many_requests,
//         tvfrr_431_request_header_fields_too_large,
//         tvfrr_451_unavailable_for_legal_reasons,
//         tvfrr_500_internal_server_error,
//         tvfrr_501_not_implemented,
//         tvfrr_502_bad_gateway,
//         tvfrr_503_service_unavailable,
//         tvfrr_504_gateway_timeout,
//         tvfrr_505_http_version_not_supported,
//         tvfrr_506_variant_also_negotiates,
//         tvfrr_507_insufficient_storage,
//         tvfrr_508_loop_detected,
//         tvfrr_510_not_extended,
//         tvfrr_511_network_authentication_required,
//     )
// )]
// pub fn type_variants_from_reqwest_response(
//     input: proc_macro::TokenStream,
// ) -> proc_macro::TokenStream {
//     proc_macro_common::panic_location::panic_location(); //panic_location function from https://github.com/kuqmua/proc_macro_helpers
//     let macro_name = "TypeVariantsFromReqwestResponse";
//     let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|_| {
//         panic!("{macro_name} {}", proc_macro_common::global_variables::hardcode::AST_PARSE_FAILED)
//     });
//     let ident = &ast.ident;
//     let ident_snake_case_stringified = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&ident.to_string());
//     let ident_response_variants_stringified = format!("{ident}{RESPONSE_VARIANTS}");
//     let ident_response_variants_token_stream = ident_response_variants_stringified
//     .parse::<proc_macro2::TokenStream>()
//     .unwrap_or_else(|_| panic!("{ident} {ident_response_variants_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
//     let attribute = proc_macro_helpers::get_macro_attribute::get_macro_attribute(
//         &ast.attrs,
//         &format!("{PATH}::type_variants_from_reqwest_response_attribute"),
//         &ident.to_string()
//     );
//     let (
//         desirable_token_stream,
//         desirable_status_code_token_stream,
//         desirable_enum_name,
//         desirable_attribute,
//         response_without_body
//     ) = {
//         let stringified_tokens = {
//             let mut stringified_tokens = quote::ToTokens::to_token_stream(&attribute.tokens).to_string();
//             stringified_tokens.retain(|c| !c.is_whitespace());
//             stringified_tokens
//         };
//         match stringified_tokens.len() > 3 {
//             true => {
//                 let mut chars = stringified_tokens.chars();
//                 match (chars.next(), chars.last()) {
//                         (None, None) => panic!("{ident} no first and last token attribute"),
//                         (None, Some(_)) => panic!("{ident} no first token attribute"),
//                         (Some(_), None) => panic!("{ident} no last token attribute"),
//                         (Some(first), Some(last)) => match (first == '(', last == ')') {
//                             (true, true) => {
//                                 match stringified_tokens.get(1..(stringified_tokens.len()-1)) {
//                                     Some(inner_tokens_str) => {
//                                         let vec_attr_params = inner_tokens_str.split(',').map(|str|{str.to_string()}).collect::<Vec<std::string::String>>();
//                                         if let false = vec_attr_params.len() == 2 {
//                                             panic!("{ident} vec_attr_params.len() != 2");
//                                         }
//                                         match (
//                                             vec_attr_params.get(0),
//                                             vec_attr_params.get(1)
//                                         ) {
//                                             (None, None) => panic!("{ident} failed to get vec_attr_params.get(0) or vec_attr_params.get(1)"),
//                                             (None, Some(_)) => panic!("{ident} failed to get vec_attr_params.get(0)"),
//                                             (Some(_), None) => panic!("{ident} failed to get vec_attr_params.get(1)"),
//                                             (Some(first_param), Some(second_param)) => {
//                                                 let status_code = proc_macro_helpers::status_code::StatusCode::try_from(second_param).unwrap_or_else(|_| panic!("{ident} second_param failed to StatusCode::try_from"));
//                                                 let desirable_token_stream = first_param
//                                                     .parse::<proc_macro2::TokenStream>()
//                                                     .unwrap_or_else(|_| panic!("{ident} {first_param} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
//                                                 let response_without_body = desirable_token_stream.to_string() == "()";
//                                                 (
//                                                     desirable_token_stream,
//                                                     status_code.to_http_status_code_token_stream(),
//                                                     {
//                                                         let status_code_enum_name_stingified = format!("{ident_response_variants_stringified}{status_code}");
//                                                         status_code_enum_name_stingified
//                                                         .parse::<proc_macro2::TokenStream>()
//                                                         .unwrap_or_else(|_| panic!("{ident} {status_code_enum_name_stingified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//                                                     },
//                                                     status_code,
//                                                     response_without_body
//                                                 )
//                                             },
//                                         }
//                                     },
//                                     None => panic!("{ident} cannot get inner_token"),
//                                 }
//                             },
//                             (true, false) => panic!("{ident} last token attribute is not )"),
//                             (false, true) => panic!("{ident} first token attribute is not ("),
//                             (false, false) => panic!("{ident} first token attribute is not ( and last token attribute is not )"),
//                         },
//                     }
//             }
//             false => panic!("{ident} {stringified_tokens}.len() > 3 == false"),
//         }
//     };
//     let data_enum = if let syn::Data::Enum(data_enum) = ast.data {
//         data_enum
//     } else {
//         panic!("{ident} syn::Data is not a syn::Data::Enum");
//     };
//     let variants_len = data_enum.variants.len();
//     let try_error_ident_token_stream = {
//         let try_error_ident_stringified = format!("{ident}{}", proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified());
//         try_error_ident_stringified
//         .parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("{ident} {try_error_ident_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     };
//     let (
//         unique_status_codes, 
//         variants_from_status_code,
//         desirable_try_from_ident,
//     ) = data_enum.variants.iter().fold(
//         (
//             Vec::with_capacity(variants_len),
//             Vec::with_capacity(variants_len),
//             Vec::with_capacity(variants_len),
//         ),
//         |mut acc, variant| {
//             let status_code = proc_macro_helpers::status_code::get_only_one_status_code(
//                 variant,
//                 &ident.to_string()
//             );
//             let (
//                 variants_from_status_code,
//                 desirable_try_from_ident,
//             ) = {
//                 let variant_ident = &variant.ident;
//                 let http_status_code_token_stream = status_code.to_http_status_code_token_stream();
//                 match &variant.fields {
//                     syn::Fields::Named(fields_named) => {
//                         let fields_named_named_len = fields_named.named.len();
//                         let (
//                             fields_token_stream_variants_from_status_code,
//                             fields_token_stream_desirable_try_from_ident,
//                         ) = fields_named.named.iter().fold(
//                             (
//                                 Vec::with_capacity(fields_named_named_len),
//                                 Vec::with_capacity(fields_named_named_len),
//                             ),
//                             |mut acc, field| {
//                                 let field_ident = field.ident.clone().unwrap_or_else(|| {
//                                     panic!("{ident} named field ident is None");
//                                 });
//                                 acc.0.push(quote::quote! { #field_ident: _ });
//                                 acc.1.push(field_ident);
//                                 acc
//                         });
//                         (
//                             quote::quote! {
//                                 #ident_response_variants_token_stream::#variant_ident {
//                                      #(#fields_token_stream_variants_from_status_code),*
//                                 } => #http_status_code_token_stream
//                             },
//                             quote::quote! {
//                                 #ident_response_variants_token_stream::#variant_ident {
//                                      #(#fields_token_stream_desirable_try_from_ident),*
//                                 } => Err(#try_error_ident_token_stream::#variant_ident { #(#fields_token_stream_desirable_try_from_ident),* })
//                             }
//                         )
//                     }
//                     syn::Fields::Unnamed(fields_unnamed) => {
//                         let fields_token_stream = if let true = fields_unnamed.unnamed.len() == 1 {
//                             quote::quote! { _ }
//                         }
//                         else {
//                             panic!("{ident} fields_unnamed.unnamed.len() != 1");                           
//                         };
//                         (
//                             quote::quote! {
//                                 #ident::#variant_ident(#fields_token_stream) => #http_status_code_token_stream
//                             },
//                             quote::quote! {
//                                 #ident_response_variants_token_stream::#variant_ident(i) => Ok(i)
//                             }
//                         )

//                     }
//                     syn::Fields::Unit => {
//                         panic!("{ident} syn::Data is not a syn::Data::Enum")
//                     }
//                 }
//             };
//             if !acc.0.contains(&status_code) {
//                 acc.0.push(status_code)
//             }
//             acc.1.push(variants_from_status_code);
//             acc.2.push(desirable_try_from_ident);
//             acc
//         },
//     );
//     let (
//         unique_status_codes_len,
//         unique_status_codes_len_minus_one
//      ) = {
//         let unique_status_codes_len = unique_status_codes.len();
//         if let true = unique_status_codes.is_empty() {
//             panic!("{ident} true = unique_status_codes.is_empty()");
//         }
//         (
//             unique_status_codes_len,
//             unique_status_codes_len - 1
//         )
//     };
//     let hashmap_attribute_variants = data_enum.variants.iter().fold(
//         std::collections::HashMap::<proc_macro_helpers::status_code::StatusCode, Vec<&syn::Variant>>::with_capacity(unique_status_codes_len),
//         |mut acc, variant|{
//             let status_code = proc_macro_helpers::status_code::get_only_one_status_code(
//                 variant,
//                 &ident.to_string()
//             );
//             match acc.get_mut(&status_code) {
//                 Some(i) => {
//                     i.push(variant);
//                 },
//                 None => {
//                     acc.insert(status_code, vec![variant]);
//                 },
//             }
//             acc
//     });
//     let supported_enum_variant = proc_macro_helpers::error_occurence::supported_enum_variant::create_supported_enum_variant(
//         &data_enum,
//         &ident.to_string()
//     );
//     let desirable_name_stringified = "Desirable";
//     let desirable_name_token_stream = desirable_name_stringified
//         .parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("{ident} {desirable_name_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
//     let generics_len = ast.generics.params.len();
//     let api_request_unexpected_error_module_path_token_stream = quote::quote! { crate::common::api_request_unexpected_error };
//     let api_request_unexpected_error_path_token_stream = quote::quote! { #api_request_unexpected_error_module_path_token_stream::ApiRequestUnexpectedError };
//     let ident_request_error_upper_camel_case_token_stream = proc_macro_helpers::type_variants_from_request_response::generate_ident_request_error_upper_camel_case_token_stream(
//         &ident,
//         &ident.to_string(),
//     );
//     let enum_with_serialize_deserialize_logic_token_stream = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_with_serialize_deserialize_version(
//         &supported_enum_variant,
//         &data_enum.variants.iter().collect(),
//         &ident.to_string(),
//         generics_len,
//         &ident_response_variants_token_stream,
//         Some(quote::quote!{
//             #desirable_name_token_stream(#desirable_token_stream)
//         }),
//         false,
//         true
//     );
//     let from_logic_token_stream = generate_from_logic(
//         ident,
//         &ident_response_variants_stringified,
//         &data_enum.variants
//     );
//     let impl_std_convert_from_ident_response_variants_token_stream_for_http_status_code_logic_token_stream = quote::quote! {
//         impl std::convert::From<&#ident_response_variants_token_stream> for http::StatusCode {
//             fn from(value: &#ident_response_variants_token_stream) -> Self {
//                 match value {
//                     #ident_response_variants_token_stream::#desirable_name_token_stream(_) => #desirable_status_code_token_stream,
//                     #(#variants_from_status_code),*
//                 }
//             }
//         }
//     };
//     let generated_status_code_enums_with_from_impls_logic_token_stream = {
//         let generated_status_code_enums_with_from_impls = {
//             let mut is_desirable_detected = false;
//             let mut generated_status_code_enums_with_from_impls = hashmap_attribute_variants.iter().map(|(attribute, vec_variants)|{
//                 let status_code_enum_name_token_stream = {
//                     let status_code_enum_name_stingified = format!("{ident_response_variants_stringified}{attribute}");
//                     status_code_enum_name_stingified
//                     .parse::<proc_macro2::TokenStream>()
//                     .unwrap_or_else(|_| panic!("{ident} {status_code_enum_name_stingified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//                 };
//                 let (
//                     optional_additional_named_variant, 
//                     additional_from_variant
//                 ) = match attribute == &desirable_attribute {
//                     true => {
//                         is_desirable_detected = true;
//                         (
//                             Some(quote::quote! {
//                                 #desirable_name_token_stream(#desirable_token_stream)
//                             }),
//                             quote::quote!{
//                                 #desirable_enum_name::#desirable_name_token_stream(i) => Self::#desirable_name_token_stream(i),
//                             }
//                         )
//                     },
//                     false => (
//                         None,
//                         proc_macro2::TokenStream::new()
//                     )
//                 };
//                 let status_enum = proc_macro_helpers::error_occurence::generate_with_serialize_deserialize_version::generate_with_serialize_deserialize_version(
//                     &supported_enum_variant,
//                     vec_variants,
//                     &ident.to_string(),
//                     generics_len,
//                     &status_code_enum_name_token_stream,
//                     optional_additional_named_variant,
//                     false,
//                     false
//                 );
//                 let status_enum_from = {
//                     let variants = vec_variants.iter().map(|variant|{
//                         let fields = if let syn::Fields::Named(fields_named) = &variant.fields {
//                             fields_named.named.iter().map(|field| {
//                                 let field_ident = &field.ident.clone().unwrap_or_else(|| panic!("{ident} field_ident is None {}",     proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
//                                 quote::quote! { #field_ident }
//                             })
//                         }
//                         else {
//                             panic!("{ident} variant.fields is not a if let syn::Fields::Named");
//                         };
//                         let fields_cloned =  fields.clone();
//                         let variant_ident = &variant.ident;
//                         quote::quote! {
//                             #status_code_enum_name_token_stream::#variant_ident{ 
//                                 #(#fields_cloned),*
//                             } => Self::#variant_ident{ 
//                                 #(#fields),*
//                             }
//                         }
//                     });
//                     quote::quote!{
//                         impl std::convert::From<#status_code_enum_name_token_stream> for #ident_response_variants_token_stream {
//                             fn from(value : #status_code_enum_name_token_stream) -> Self {
//                                 match value {
//                                     #additional_from_variant
//                                     #(#variants),*
//                                 }
//                             }
//                         } 
//                     }
//                 };
//                 quote::quote!{
//                     #status_enum
//                     #status_enum_from
//                 }
//             }).collect::<Vec<proc_macro2::TokenStream>>();
//             if !is_desirable_detected {
//                 if let false = response_without_body {
//                     //todo maybe for desirable status code can be few different response variants - rewrite logic
//                     generated_status_code_enums_with_from_impls.push(quote::quote!{
//                         #[derive(Debug, serde::Serialize, serde::Deserialize)] 
//                         enum #desirable_enum_name {
//                             #desirable_name_token_stream(#desirable_token_stream)
//                         } 
//                         impl std::convert::From<#desirable_enum_name> for #ident_response_variants_token_stream {
//                             fn from(value: #desirable_enum_name) -> Self {
//                                 match value { 
//                                     #desirable_enum_name::#desirable_name_token_stream(i) => Self::#desirable_name_token_stream(i) 
//                                 }
//                             }
//                         }    
//                     });
//                 }
//             }
//             generated_status_code_enums_with_from_impls
//         };
//         quote::quote! {
//             #(#generated_status_code_enums_with_from_impls)*
//         }
//     };
//     let try_from_response_ident_snake_case_token_stream = proc_macro_helpers::type_variants_from_request_response::generate_try_from_response_ident_snake_case_token_stream(
//         &ident_snake_case_stringified,
//         &ident.to_string(),
//     );
//     let try_from_response_logic_token_stream = {
//         let status_code_enums_try_from = {
//             let mut is_last_element_found = false;
//             let desirable_status_code_case_token_stream = match response_without_body {
//                 true => quote::quote! {
//                     Ok(#ident_response_variants_token_stream::#desirable_name_token_stream(()))
//                 },
//                 false => quote::quote! {
//                     match response.text().await {
//                         Ok(response_text) => match serde_json::from_str::<#desirable_enum_name>(&response_text){
//                             Ok(value) => Ok(#ident_response_variants_token_stream::from(value)), 
//                             Err(e) => Err(
//                                 #api_request_unexpected_error_path_token_stream::DeserializeBody{ 
//                                     serde: e,
//                                     status_code,
//                                     headers,response_text
//                                 }
//                             ),
//                         },
//                         Err(e) => Err(
//                             #api_request_unexpected_error_path_token_stream::FailedToGetResponseText {
//                                 reqwest: e,
//                                 status_code,
//                                 headers,
//                             }
//                         ),
//                     }
//                 },
//             };
//             let mut status_code_enums_try_from_variants = Vec::with_capacity(unique_status_codes_len + 1);
//             status_code_enums_try_from_variants.push(quote::quote! {
//                 if status_code == #desirable_status_code_token_stream {
//                     #desirable_status_code_case_token_stream
//                 }
//             });
//             unique_status_codes
//             .into_iter()
//             .enumerate()
//             .for_each(
//                 |(index, status_code_attribute)| 
//             {
//                 let status_code_enum_name_stringified = format!("{ident_response_variants_token_stream}{status_code_attribute}");
//                 let status_code_enum_name_token_stream = status_code_enum_name_stringified
//                     .parse::<proc_macro2::TokenStream>()
//                     .unwrap_or_else(|_| panic!("{ident} {status_code_enum_name_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
//                 let http_status_code_token_stream = status_code_attribute.to_http_status_code_token_stream();
//                 match index == unique_status_codes_len_minus_one{
//                     true => {
//                         is_last_element_found = true;
//                         status_code_enums_try_from_variants.push(quote::quote! {
//                             else {
//                                 match response.text().await {
//                                     Ok(response_text) => Err(
//                                         #api_request_unexpected_error_path_token_stream::StatusCode {
//                                             status_code,
//                                             headers,
//                                             response_text_result: #api_request_unexpected_error_module_path_token_stream::ResponseTextResult::ResponseText(response_text)
//                                         },
//                                     ),
//                                     Err(e) => Err(
//                                         #api_request_unexpected_error_path_token_stream::StatusCode {
//                                             status_code,
//                                             headers,
//                                             response_text_result: #api_request_unexpected_error_module_path_token_stream::ResponseTextResult::ReqwestError(e),
//                                         },
//                                     ),
//                                 }
//                             }
//                         });
//                     },
//                     false => {
//                         if let false = desirable_attribute == status_code_attribute {
//                             status_code_enums_try_from_variants.push(quote::quote! {
//                                 else if status_code == #http_status_code_token_stream {
//                                     match response.text().await {
//                                         Ok(response_text) => match serde_json::from_str::<#status_code_enum_name_token_stream>(&response_text){
//                                             Ok(value) => Ok(#ident_response_variants_token_stream::from(value)), 
//                                             Err(e) => Err(
//                                                 #api_request_unexpected_error_path_token_stream::DeserializeBody{ 
//                                                     serde: e,
//                                                     status_code,
//                                                     headers,response_text
//                                                 }
//                                             ),
//                                         },
//                                         Err(e) => Err(
//                                             #api_request_unexpected_error_path_token_stream::FailedToGetResponseText {
//                                                 reqwest: e,
//                                                 status_code,
//                                                 headers,
//                                             }
//                                         ),
//                                     }
//                                 }
//                             });
//                         }
//                     },
//                 }
//             });
//             if let false = is_last_element_found {
//                 panic!("{ident} false = is_last_element_found");
//             }
//             status_code_enums_try_from_variants
//         };
//         quote::quote! {
//             async fn #try_from_response_ident_snake_case_token_stream(response: reqwest::Response) -> Result<#ident_response_variants_token_stream, #api_request_unexpected_error_path_token_stream> {
//                 let status_code = response.status();
//                 let headers = response.headers().clone();
//                 #(#status_code_enums_try_from)*
//             }
//         }
//     };
//     let impl_try_from_ident_response_variants_token_stream_for_desirable_logic_token_stream = match response_without_body {
//         true => quote::quote! {},
//         false => quote::quote! {
//             impl TryFrom<#ident_response_variants_token_stream> for #desirable_token_stream
//             {
//                 type Error = #try_error_ident_token_stream;
//                 fn try_from(
//                     value: #ident_response_variants_token_stream,
//                 ) -> Result<Self, Self::Error> {
//                     match value {
//                         #ident_response_variants_token_stream::#desirable_name_token_stream(i) => Ok(i),
//                         #(#desirable_try_from_ident),*
//                     }
//                 }
//             }
//         },
//     };
//     let ident_request_error_logic_token_stream = quote::quote! {
//         #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
//         pub enum #ident_request_error_upper_camel_case_token_stream {
//             ExpectedType {
//                 #[eo_display_with_serialize_deserialize]
//                 expected_type: #try_error_ident_token_stream,
//                 code_occurence: crate::common::code_occurence::CodeOccurence,
//             },
//             UnexpectedStatusCode {
//                 #[eo_display]
//                 status_code: http::StatusCode,
//                 #[eo_display_foreign_type]
//                 headers: reqwest::header::HeaderMap,
//                 #[eo_display_foreign_type]
//                 response_text_result: #api_request_unexpected_error_module_path_token_stream::ResponseTextResult,
//                 code_occurence: crate::common::code_occurence::CodeOccurence,
//             },
//             FailedToGetResponseText {
//                 #[eo_display_foreign_type]
//                 reqwest: reqwest::Error,
//                 #[eo_display]
//                 status_code: http::StatusCode,
//                 #[eo_display_foreign_type]
//                 headers: reqwest::header::HeaderMap,
//                 code_occurence: crate::common::code_occurence::CodeOccurence,
//             },
//             DeserializeResponse {
//                 #[eo_display]
//                 serde: serde_json::Error,
//                 #[eo_display]
//                 status_code: http::StatusCode,
//                 #[eo_display_foreign_type]
//                 headers: reqwest::header::HeaderMap,
//                 #[eo_display_with_serialize_deserialize]
//                 response_text: std::string::String,
//                 code_occurence: crate::common::code_occurence::CodeOccurence,
//             },
//             Reqwest {
//                 #[eo_display_foreign_type]
//                 reqwest: reqwest::Error,
//                 code_occurence: crate::common::code_occurence::CodeOccurence,
//             },
//         }
//     };
//     let extraction_logic_token_stream = {
//         let response_without_body_logic_token_stream = match response_without_body {
//             true => quote::quote! {
//                 Ok(_variants) => Ok(())
//             },
//             false => quote::quote! {
//                 Ok(variants) => match #desirable_token_stream::try_from(variants)
//                 {
//                     Ok(value) => Ok(value),
//                     Err(e) => Err(#ident_request_error_upper_camel_case_token_stream::ExpectedType {
//                         expected_type: e,
//                         code_occurence: crate::code_occurence_tufa_common!(),
//                     }),
//                 }
//             },
//         };
//         let tvfrr_extraction_logic_snake_case_token_stream = proc_macro_helpers::type_variants_from_request_response::generate_tvfrr_extraction_logic_snake_case_token_stream(
//             &ident_snake_case_stringified,
//             &ident.to_string(),
//         );
//         quote::quote! {
//             async fn #tvfrr_extraction_logic_snake_case_token_stream<'a>(
//                 future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
//             ) -> Result<#desirable_token_stream, #ident_request_error_upper_camel_case_token_stream> {
//                 match future.await {
//                     Ok(response) => match #try_from_response_ident_snake_case_token_stream(response).await {
//                         #response_without_body_logic_token_stream,
//                         Err(e) => match e {
//                             #api_request_unexpected_error_path_token_stream::StatusCode { 
//                                 status_code,
//                                 headers,
//                                 response_text_result,
//                             } => Err(
//                                 #ident_request_error_upper_camel_case_token_stream::UnexpectedStatusCode {
//                                     status_code,
//                                     headers,
//                                     response_text_result,
//                                     code_occurence: crate::code_occurence_tufa_common!()
//                                 }
//                             ),
//                             #api_request_unexpected_error_path_token_stream::FailedToGetResponseText {
//                                 reqwest, 
//                                 status_code, 
//                                 headers 
//                             } => Err(
//                                 #ident_request_error_upper_camel_case_token_stream::FailedToGetResponseText {
//                                     reqwest,
//                                     status_code,
//                                     headers,
//                                     code_occurence: crate::code_occurence_tufa_common!()
//                                 }
//                             ),
//                             #api_request_unexpected_error_path_token_stream::DeserializeBody {
//                                 serde, 
//                                 status_code,
//                                 headers,
//                                 response_text,
//                             } => Err(
//                                 #ident_request_error_upper_camel_case_token_stream::DeserializeResponse {
//                                     serde, 
//                                     status_code,
//                                     headers,
//                                     response_text,
//                                     code_occurence: crate::code_occurence_tufa_common!()
//                                 }
//                             ),
//                         },
//                     },
//                     Err(e) => Err(#ident_request_error_upper_camel_case_token_stream::Reqwest {
//                         reqwest: e,
//                         code_occurence: crate::code_occurence_tufa_common!(),
//                     }),
//                 }
//             }
//         }
//     };
//     let enum_status_codes_checker_name_logic_token_stream = {
//         let enum_status_codes_checker_upper_camel_case_token_stream = proc_macro_helpers::type_variants_from_request_response::generate_enum_status_codes_checker_upper_camel_case_token_stream(
//             &ident,
//             &ident.to_string(),
//         );
//         let enum_status_codes_checker_variants = data_enum.variants.iter().map(
//             |variant| {
//                     let status_code = proc_macro_helpers::status_code::get_only_one_status_code(
//                         variant,
//                         &ident.to_string()
//                     );
//                     let check_variant_ident_stringified = format!("{}{}", variant.ident, status_code);
//                     check_variant_ident_stringified
//                         .parse::<proc_macro2::TokenStream>()
//                         .unwrap_or_else(|_| panic!("{ident} {check_variant_ident_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//                 }
//             );
//         quote::quote! {
//             pub enum #enum_status_codes_checker_upper_camel_case_token_stream {
//                 #(#enum_status_codes_checker_variants),*
//             }
//         }
//     };
//     let axum_response_into_response_logic_token_stream = {
//         let res_name_token_stream = quote::quote!{res};
//         let mut_res_axum_json_into_response = quote::quote!{let mut #res_name_token_stream = axum::Json(self).into_response()};
//         let star_res_status_mut = quote::quote!{*#res_name_token_stream.status_mut()};
//         let axum_response_into_response_variants = data_enum.variants.iter().map(
//             |variant| match &variant.fields {
//                 syn::Fields::Named(fields_named) => {
//                     let fields = fields_named.named.iter().map(|field|{
//                         let field_ident = field.ident.clone().unwrap_or_else(|| panic!(
//                         "{ident} field.ident {}",
//                         proc_macro_helpers::naming_conventions::IS_NONE_STRINGIFIED
//                     ));
//                         quote::quote! { #field_ident: _ }
//                     });
//                     let status_code_token_stream = proc_macro_helpers::status_code::get_only_one_status_code(
//                         variant,
//                         &ident.to_string()
//                     ).to_http_status_code_token_stream();
//                     let variant_ident = &variant.ident;
//                     quote::quote! {
//                         #ident_response_variants_token_stream::#variant_ident {
//                             #(#fields),*
//                         } => {
//                             #mut_res_axum_json_into_response;
//                             #star_res_status_mut = #status_code_token_stream;
//                             #res_name_token_stream
//                         }
//                     }
//                 },
//                 _ => panic!("{ident} variant.fields is not syn::Fields::Named"),
//             }
//         );
//         let desirable_variant_logic_token_stream = match response_without_body {
//             true => quote::quote! {
//                 #desirable_status_code_token_stream.into_response()
//             },
//             false => quote::quote! {
//                 #mut_res_axum_json_into_response;
//                 #star_res_status_mut = #desirable_status_code_token_stream;
//                 #res_name_token_stream
//             },
//         };
//         quote::quote! {
//             impl axum::response::IntoResponse for #ident_response_variants_token_stream {
//                 fn into_response(self) -> axum::response::Response {
//                     match &self {
//                         #ident_response_variants_token_stream::#desirable_name_token_stream(_) => {
//                             #desirable_variant_logic_token_stream
//                         }
//                         #(#axum_response_into_response_variants),*
//                     }
//                 }
//             }
//         }
//     };
//     // let f = quote::quote! {

//     // };
//     // if ident == "" {
//     //   println!("{f} ");
//     // }
//     let gen = quote::quote! {
//         #enum_with_serialize_deserialize_logic_token_stream
//         #from_logic_token_stream
//         #impl_std_convert_from_ident_response_variants_token_stream_for_http_status_code_logic_token_stream
//         #generated_status_code_enums_with_from_impls_logic_token_stream
//         #try_from_response_logic_token_stream
//         #impl_try_from_ident_response_variants_token_stream_for_desirable_logic_token_stream
//         #ident_request_error_logic_token_stream
//         #extraction_logic_token_stream
//         #enum_status_codes_checker_name_logic_token_stream
//         #axum_response_into_response_logic_token_stream
//     };
//     // if ident == "" {
//     //     proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
//     //         &macro_name,
//     //         &gen,
//     //         &ident.to_string()
//     //     );
//     // }
//     gen.into()
// }

// #[proc_macro_attribute]
// pub fn type_variants_from_reqwest_response_attribute(
//     _attr: proc_macro::TokenStream,
//     item: proc_macro::TokenStream,
// ) -> proc_macro::TokenStream {
//     item
// }

#[proc_macro_derive(
    TypeVariantsFromReqwestResponseFromChecker,
    attributes(
        tvfrr_100_continue,
        tvfrr_101_switching_protocols,
        tvfrr_102_processing,
        tvfrr_200_ok,
        tvfrr_201_created,
        tvfrr_202_accepted,
        tvfrr_203_non_authoritative_information,
        tvfrr_204_no_content,
        tvfrr_205_reset_content,
        tvfrr_206_partial_content,
        tvfrr_207_multi_status,
        tvfrr_208_already_reported,
        tvfrr_226_im_used,
        tvfrr_300_multiple_choices,
        tvfrr_301_moved_permanently,
        tvfrr_302_found,
        tvfrr_303_see_other,
        tvfrr_304_not_modified,
        tvfrr_305_use_proxy,
        tvfrr_307_temporary_redirect,
        tvfrr_308_permanent_redirect,
        tvfrr_400_bad_request,
        tvfrr_401_unauthorized,
        tvfrr_402_payment_required,
        tvfrr_403_forbidden,
        tvfrr_404_not_found,
        tvfrr_405_method_not_allowed,
        tvfrr_406_not_acceptable,
        tvfrr_407_proxy_authentication_required,
        tvfrr_408_request_timeout,
        tvfrr_409_conflict,
        tvfrr_410_gone,
        tvfrr_411_length_required,
        tvfrr_412_precondition_failed,
        tvfrr_413_payload_too_large,
        tvfrr_414_uri_too_long,
        tvfrr_415_unsupported_media_type,
        tvfrr_416_range_not_satisfiable,
        tvfrr_417_expectation_failed,
        tvfrr_418_im_a_teapot,
        tvfrr_421_misdirected_request,
        tvfrr_422_unprocessable_entity,
        tvfrr_423_locked,
        tvfrr_424_failed_dependency,
        tvfrr_426_upgrade_required,
        tvfrr_428_precondition_required,
        tvfrr_429_too_many_requests,
        tvfrr_431_request_header_fields_too_large,
        tvfrr_451_unavailable_for_legal_reasons,
        tvfrr_500_internal_server_error,
        tvfrr_501_not_implemented,
        tvfrr_502_bad_gateway,
        tvfrr_503_service_unavailable,
        tvfrr_504_gateway_timeout,
        tvfrr_505_http_version_not_supported,
        tvfrr_506_variant_also_negotiates,
        tvfrr_507_insufficient_storage,
        tvfrr_508_loop_detected,
        tvfrr_510_not_extended,
        tvfrr_511_network_authentication_required,
    )
)]
pub fn type_variants_from_reqwest_response_from_checker(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location(); //panic_location function from https://github.com/kuqmua/proc_macro_helpers
    let ast: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;
    let enum_status_codes_checker_name_token_stream = {
        let enum_status_codes_checker_stringified = format!(
            "{ident}{}",
            proc_macro_helpers::type_variants_from_request_response::STATUS_CODES_CHECKER
        );
        enum_status_codes_checker_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{ident} {enum_status_codes_checker_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let variants = if let syn::Data::Enum(data_enum) = ast.data {
        data_enum.variants
    } else {
        panic!("does work only on enums!");
    };
    let vec_enum_paths = get_vec_enum_paths(
        proc_macro_helpers::get_macro_attribute::get_macro_attribute(
            &ast.attrs,
            &format!("{PATH}::type_variants_from_reqwest_response_from_checker_paths"),
            &ident.to_string()
        ),
        &ident.to_string()
    );
    let generated_response_variants_logic_token_stream = {
        let generated_response_variants = vec_enum_paths.iter().map(
            |enum_path| generate_from_logic(
                ident,
                &format!("{enum_path}{RESPONSE_VARIANTS}"),
                &variants
            )
        );
        quote::quote! {
            #(#generated_response_variants)*
        }
    };
    let generated_with_serialize_deserialize_logic_token_stream = {
        let generated_with_serialize_deserialize = {
            vec_enum_paths.iter().map(
                |enum_path| generate_from_logic(
                    ident,
                    &format!("{enum_path}{}", proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()),
                    &variants
                )
            )
        };
        quote::quote! {
            #(#generated_with_serialize_deserialize)*
        }
    };
    let impl_from_ident_for_http_status_code_logic_token_stream = {
        let variants_from_status_code = variants.iter().map(
            |variant| {
                let http_status_code_token_stream = {
                    proc_macro_helpers::status_code::get_only_one_status_code(
                        variant,
                        &ident.to_string()
                    ).to_http_status_code_token_stream()
                };
                let variant_ident = &variant.ident;
                match &variant.fields {
                    syn::Fields::Named(fields_named) => {
                        let fields_named_named_len = fields_named.named.len();
                        let fields_token_stream_variants_from_status_code = fields_named.named.iter().fold(
                            Vec::with_capacity(fields_named_named_len),
                            |mut acc, field| {
                                let field_ident = &field.clone().ident.unwrap_or_else(|| {
                                    panic!("{ident} named field ident is None");
                                });
                                acc.push(quote::quote! { #field_ident: _ });
                                acc
                            }
                        );
                        quote::quote! {
                            #ident::#variant_ident {
                                #(#fields_token_stream_variants_from_status_code),*
                            } => #http_status_code_token_stream,
                        }
                    }
                    syn::Fields::Unnamed(fields_unnamed) => {
                        let fields_token_stream = if let true = fields_unnamed.unnamed.len() == 1 {
                            quote::quote! { _ }
                        }
                        else {
                            panic!("{ident} fields_unnamed.unnamed.len() != 1");                       
                        };
                        quote::quote! {
                            #ident::#variant_ident(#fields_token_stream) => #http_status_code_token_stream
                        }
                    }
                    syn::Fields::Unit => {
                        panic!("{ident} syn::Data is not a syn::Data::Enum")
                    }
                }
            },
        );
        quote::quote!{
            impl From<&#ident> for http::StatusCode {
                fn from(val: &#ident) -> Self {
                    match &val {
                        #(#variants_from_status_code)*
                    }
                }
            }
        }
    };
    let enum_status_codes_checker_logic_token_stream = {
        let enum_status_codes_checker_variants = variants.iter().map(|variant|{
            let attr = proc_macro_helpers::status_code::get_only_one_status_code(
                variant,
                &ident.to_string()
            );
            let check_variant_ident_stringified = format!("{}{}", variant.ident, attr);
            check_variant_ident_stringified
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{ident} {check_variant_ident_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        });
        quote::quote! {
            #[allow(clippy::enum_variant_names, dead_code)]
            enum #enum_status_codes_checker_name_token_stream {
                #(#enum_status_codes_checker_variants),*
            }
        }
    };
    let enum_status_codes_checker_from_impls_logic_token_stream = {
        let enum_status_codes_checker_from_impls = vec_enum_paths.iter().map(|enum_path| {
            let enum_path_token_stream = {
                let enum_path_stringified = format!(
                    "{enum_path}{}",
                    proc_macro_helpers::type_variants_from_request_response::STATUS_CODES_CHECKER
                );
                enum_path_stringified
                .parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| {
                    panic!("{ident} {enum_path_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE)
                })
            };
            let enum_status_codes_checker_from_impls_variants = variants.iter().map(|variant| {
                let attr = proc_macro_helpers::status_code::get_only_one_status_code(
                    variant,
                    &ident.to_string()
                );
                let check_variant_ident_token_stream = {
                    let check_variant_ident_stringified = format!("{}{}", variant.ident, attr);
                    check_variant_ident_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{ident} {check_variant_ident_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                match &variant.fields {
                    syn::Fields::Named(_fields_named) => {
                        quote::quote! {
                            #enum_status_codes_checker_name_token_stream::#check_variant_ident_token_stream => Self::#check_variant_ident_token_stream
                        }
                    }
                    syn::Fields::Unnamed(fields_unnamed) => {
                        if let false = fields_unnamed.unnamed.len() == 1 {
                            panic!("{ident} fields_unnamed.unnamed.len() != 1");
                        }
                        quote::quote! {
                            #enum_status_codes_checker_name_token_stream::#check_variant_ident_token_stream => Self::#check_variant_ident_token_stream
                        }
                    }
                    syn::Fields::Unit => panic!(
                        "{ident} works only with syn::Fields::Named and syn::Fields::Unnamed"
                    ),
                }
            });
            let variant_gen = quote::quote! {
                impl std::convert::From<#enum_status_codes_checker_name_token_stream>
                    for #enum_path_token_stream
                {
                    fn from(
                        val: #enum_status_codes_checker_name_token_stream,
                    ) -> Self {
                        match val {
                            #(#enum_status_codes_checker_from_impls_variants),*
                        }
                    }
                }
            };
            // if enum_path == "" {
            //     println!("{variant_gen}");
            // }
            variant_gen
        });
        quote::quote! {
            #(#enum_status_codes_checker_from_impls)*
        }
    };
    let gen = quote::quote! {
        #generated_response_variants_logic_token_stream
        #generated_with_serialize_deserialize_logic_token_stream
        #impl_from_ident_for_http_status_code_logic_token_stream
        #enum_status_codes_checker_logic_token_stream
        #enum_status_codes_checker_from_impls_logic_token_stream
    };
    // if ident == "" {
    //     proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         &macro_name,//todo make name standart
    //         &gen,
    //         &"something"
    //     );
    // }
    gen.into()
}

#[proc_macro_attribute]
pub fn type_variants_from_reqwest_response_from_checker_paths(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}