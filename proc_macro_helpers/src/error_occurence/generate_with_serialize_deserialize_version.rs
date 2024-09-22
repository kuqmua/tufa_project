// pub fn generate_field_type_with_serialize_deserialize_version(
//     attribute: crate::error_occurence::ErrorOccurenceFieldAttribute,
//     supported_container: crate::error_occurence::supported_container::SupportedContainer,
//     proc_macro_name_ident_stringified: &str,
// ) -> proc_macro2::TokenStream {
//     let with_serialize_deserialize_upper_camel_case =
//         crate::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified();
//     let supports_only_supported_container_stringified =
//         crate::naming_conventions::supports_only_supported_container_stringified();
//     let does_not_support_stringified = "does not support";
//     let str_stringified = "str";
//     let std_string_string_stringified = format!(
//         "{}::{}::{}",
//         naming_conventions::STD_STRINGIFIED,
//         <naming_conventions::String as naming_conventions::Naming>::snake_case_stringified(),
//         <naming_conventions::String as naming_conventions::Naming>::upper_camel_case_stringified()
//     );
//     let std_string_string_token_stream = proc_macro_common::std_string_string_token_stream();
//     let as_std_collections_hashmap_key_type_stringified = format!(
//         "as {}::collections::{} key type",
//         naming_conventions::STD_STRINGIFIED,
//         <naming_conventions::HashMap as naming_conventions::Naming>::upper_camel_case_stringified(),
//     );
//     let type_upper_camel_case = "Type";
//     let hashmap_value_type_stringified = format!(
//         "{}{}{type_upper_camel_case}",
//         <naming_conventions::HashMap as naming_conventions::Naming>::upper_camel_case_stringified(),
//         <naming_conventions::Value as naming_conventions::Naming>::upper_camel_case_stringified(),
//     );
//     let hashmap_value_type_path_stringified = format!(
//         "{hashmap_value_type_stringified}::{}",
//         <naming_conventions::Path as naming_conventions::Naming>::upper_camel_case_stringified(),
//     );
//     let hashmap_value_type_reference_stringified = format!(
//         "{hashmap_value_type_stringified}::{}",
//         <naming_conventions::Reference as naming_conventions::Naming>::upper_camel_case_stringified(),
//     );
//     let hashmap_key_type_stringified = format!(
//         "{}{}{type_upper_camel_case}",
//         <naming_conventions::Key as naming_conventions::Naming>::upper_camel_case_stringified(),
//         <naming_conventions::HashMap as naming_conventions::Naming>::upper_camel_case_stringified(),
//     );
//     let hashmap_key_type_path_stringified = format!(
//         "{hashmap_key_type_stringified}::{}",
//         <naming_conventions::Path as naming_conventions::Naming>::upper_camel_case_stringified(),
//     );
//     let hashmap_key_type_reference_stringified = format!(
//         "{hashmap_key_type_stringified}::{}",
//         <naming_conventions::Reference as naming_conventions::Naming>::upper_camel_case_stringified(),
//     );
//     let vec_element_type_path_stringified = format!(
//         "crate::error_occurence::vec_element_type::VecElementType::{}",
//         <naming_conventions::Path as naming_conventions::Naming>::upper_camel_case_stringified(),
//     );
//     match &attribute {
//         crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString => {
//             if let crate::error_occurence::supported_container::SupportedContainer::Path { .. } = supported_container {
//                 quote::quote! {
//                     #std_string_string_token_stream
//                 }
//             }
//             else {
//                 panic!(
//                     "{proc_macro_name_ident_stringified} {} {} {}{}",
//                     proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
//                     naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                     naming_conventions::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT,
//                     <naming_conventions::Path as naming_conventions::Naming>::upper_camel_case_stringified(),
//                 )
//             }
//         },
//         crate::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize => {
//             match supported_container {
//                 crate::error_occurence::supported_container::SupportedContainer::Path { path, vec_lifetime } => {
//                     {
//                         let type_stringified = format!("{path}{}",  crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
//                         type_stringified
//                         .parse::<proc_macro2::TokenStream>()
//                         .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//                     }
//                 },
//                 crate::error_occurence::supported_container::SupportedContainer::Reference{ reference_ident, .. } => {
//                     crate::error_occurence::panic_if_not_str::panic_if_not_str(
//                         &reference_ident,
//                         str_stringified,
//                         proc_macro_name_ident_stringified,
//                         naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                         &attribute
//                     );
//                     quote::quote!{#std_string_string_token_stream}
//                 },
//                 crate::error_occurence::supported_container::SupportedContainer::Vec{ .. } |
//                 crate::error_occurence::supported_container::SupportedContainer::HashMap{ .. } => panic!(
//                     "{proc_macro_name_ident_stringified} {} only supports {}{} and {}{}",
//                     proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
//                     naming_conventions::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT,
//                     <naming_conventions::Path as naming_conventions::Naming>::upper_camel_case_stringified(),
//                     naming_conventions::SUPPORTED_CONTAINER_DOUBLE_DOT_DOUBLE_DOT,
//                     <naming_conventions::Reference as naming_conventions::Naming>::upper_camel_case_stringified(),
//                 ),
//             }
//         },
//         crate::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence => {
//             if let crate::error_occurence::supported_container::SupportedContainer::Path { path, ..} = &supported_container {
//                 {
//                     let type_stringified = format!("{path}{with_serialize_deserialize_upper_camel_case}");
//                     type_stringified
//                     .parse::<proc_macro2::TokenStream>()
//                     .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//                 }
//             }
//             else {
//                 panic!(
//                     "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}",
//                     proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
//                     <naming_conventions::Path as naming_conventions::Naming>::upper_camel_case_stringified(),
//                 );
//             }
//         },
//         crate::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString => {
//             if let crate::error_occurence::supported_container::SupportedContainer::Vec {
//                 path,
//                 vec_element_type
//             } = supported_container {
//                 if let crate::error_occurence::vec_element_type::VecElementType::Path { .. } = vec_element_type {
//                     let type_stringified = format!("{path}<{std_string_string_stringified}>");
//                     type_stringified
//                     .parse::<proc_macro2::TokenStream>()
//                     .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//                 }
//                 else {
//                     panic!(
//                         "{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}",
//                         proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
//                         naming_conventions::SUPPORTS_ONLY_STRINGIFIED
//                     );
//                 }
//             }
//             else {
//                 panic!(
//                     "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}",
//                     proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
//                     <naming_conventions::Vec as naming_conventions::Naming>::upper_camel_case_stringified(),
//                 );
//             }
//         },
//         crate::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize => {
//             if let crate::error_occurence::supported_container::SupportedContainer::Vec {
//                 path,
//                 vec_element_type
//             } = supported_container {
//                 match vec_element_type {
//                     crate::error_occurence::vec_element_type::VecElementType::Path { element_path, vec_lifetime } => {
//                         let type_stringified = format!("{path}<{element_path}{}>",  crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
//                         type_stringified
//                         .parse::<proc_macro2::TokenStream>()
//                         .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//                     },
//                     crate::error_occurence::vec_element_type::VecElementType::Reference { reference_ident, .. } => {
//                         crate::error_occurence::panic_if_not_str::panic_if_not_str(
//                             &reference_ident,
//                             str_stringified,
//                             proc_macro_name_ident_stringified,
//                             naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                             &attribute
//                         );
//                         {
//                             let type_stringified = format!("{path}<{std_string_string_stringified}>");
//                             type_stringified
//                             .parse::<proc_macro2::TokenStream>()
//                             .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//                         }
//                     },
//                 }
//             }
//             else {
//                 panic!(
//                     "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}",
//                     proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
//                     <naming_conventions::Vec as naming_conventions::Naming>::upper_camel_case_stringified(),
//                 );
//             }
//         },
//         crate::error_occurence::ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
//             if let crate::error_occurence::supported_container::SupportedContainer::Vec {
//                 path,
//                 vec_element_type
//             } = supported_container {
//                 if let crate::error_occurence::vec_element_type::VecElementType::Path { element_path, vec_lifetime } = vec_element_type  {
//                     {
//                         let type_stringified = format!("{path}<{element_path}{with_serialize_deserialize_upper_camel_case}{}>",  crate::error_occurence::vec_lifetime_to_string::vec_lifetime_to_string(&vec_lifetime));
//                         type_stringified
//                         .parse::<proc_macro2::TokenStream>()
//                         .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//                     }
//                 }
//                 else {
//                     panic!(
//                         "{proc_macro_name_ident_stringified} {} {} {vec_element_type_path_stringified}",
//                         proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
//                         naming_conventions::SUPPORTS_ONLY_STRINGIFIED
//                     );
//                 }
//             }
//             else {
//                 panic!(
//                     "{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{}",
//                     proc_macro_common::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&attribute),
//                     <naming_conventions::Vec as naming_conventions::Naming>::upper_camel_case_stringified(),
//                 );
//             }
//         },
//         crate::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString => todo!(),
//         crate::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => todo!(),
//         crate::error_occurence::ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => todo!(),
//     }
// }

// pub fn generate_supported_container(
//     field: &syn::Field,
//     proc_macro_name_ident_stringified: &str,
// ) -> crate::error_occurence::supported_container::SupportedContainer {
//     let syn_type_reference = format!(
//         "syn::Type::{}",
//         <naming_conventions::Reference as naming_conventions::Naming>::upper_camel_case_stringified(),
//     );
//     let error_message = format!(
//         "{} {} and {syn_type_reference}",
//         naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//         naming_conventions::SYN_TYPE_PATH
//     );
//     match &field.ty {
//         syn::Type::Path(type_path) => {
//             let path = crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments);
//             let vec_lifetime = crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
//                 &type_path.path.segments,
//                 proc_macro_name_ident_stringified
//             );
//             let path_segment = type_path.path.segments.iter().last()
//             .unwrap_or_else(|| panic!(
//                 "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().last() {}",
//                 naming_conventions::IS_NONE_STRINGIFIED
//             ));
//             if path_segment.ident == <naming_conventions::Vec as naming_conventions::Naming>::upper_camel_case_stringified()
//             {
//                 let vec_element_type = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = &path_segment.arguments {
//                     if angle_brackets_generic_arguments.args.len() == 1 {
//                         if let syn::GenericArgument::Type(type_handle) =
//                             angle_brackets_generic_arguments.args
//                             .iter().next()
//                             .unwrap_or_else(|| panic!(
//                                 "{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.into_iter().nth(0) {}",
//                                 naming_conventions::IS_NONE_STRINGIFIED
//                             ))
//                         {
//                             match type_handle {
//                                 syn::Type::Path(type_path) => crate::error_occurence::vec_element_type::VecElementType::Path{
//                                     element_path: crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
//                                     vec_lifetime: crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
//                                         &type_path.path.segments,
//                                         proc_macro_name_ident_stringified
//                                     )
//                                 },
//                                 syn::Type::Reference(type_reference) => {
//                                     let reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem.clone() {
//                                         if type_path.path.segments.len() == 1 {
//                                             type_path.path.segments
//                                             .into_iter().next()
//                                             .unwrap_or_else(|| panic!(
//                                                 "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
//                                                 naming_conventions::IS_NONE_STRINGIFIED
//                                             ))
//                                             .ident
//                                         }
//                                         else {
//                                             panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
//                                         }
//                                     }
//                                     else {
//                                         panic!(
//                                             "{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {}",
//                                             naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                                             naming_conventions::SYN_TYPE_PATH
//                                         );
//                                     };
//                                     crate::error_occurence::vec_element_type::VecElementType::Reference {
//                                         reference_ident,
//                                         lifetime_ident: type_reference.lifetime.clone().unwrap_or_else(|| panic!(
//                                             "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
//                                             naming_conventions::IS_NONE_STRINGIFIED
//                                         )).ident
//                                     }
//                                 },
//                                 syn::Type::Array(_) |
//                                 syn::Type::BareFn(_) |
//                                 syn::Type::Group(_) |
//                                 syn::Type::ImplTrait(_) |
//                                 syn::Type::Infer(_) |
//                                 syn::Type::Macro(_) |
//                                 syn::Type::Never(_) |
//                                 syn::Type::Paren(_) |
//                                 syn::Type::Ptr(_) |
//                                 syn::Type::Slice(_) |
//                                 syn::Type::TraitObject(_) |
//                                 syn::Type::Tuple(_) |
//                                 syn::Type::Verbatim(_) => panic!(
//                                     "{proc_macro_name_ident_stringified} type_handle {} {} and {syn_type_reference}",
//                                     naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                                     naming_conventions::SYN_TYPE_PATH
//                                 ),
//                                 _ => panic!(
//                                     "{proc_macro_name_ident_stringified} type_handle {} {} and {syn_type_reference} (exhaustive)",
//                                     naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                                     naming_conventions::SYN_TYPE_PATH
//                                 ),
//                             }
//                         }
//                         else {
//                             panic!(
//                                 "{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args[0] {} {}",
//                                 naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                                 naming_conventions::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
//                             );
//                         }
//                     }
//                     else {
//                         panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() == 1");
//                     }
//                 }
//                 else {
//                     panic!(
//                         "{proc_macro_name_ident_stringified} path_segment.arguments {} syn::PathArguments::AngleBracketed",
//                         naming_conventions::SUPPORTS_ONLY_STRINGIFIED
//                     );
//                 };
//                 crate::error_occurence::supported_container::SupportedContainer::Vec{
//                     path,
//                     vec_element_type
//                 }
//             }
//             else if path_segment.ident == <naming_conventions::HashMap as naming_conventions::Naming>::upper_camel_case_stringified() {
//                 let (
//                     hashmap_key_type,
//                     hashmap_value_type
//                 ) = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = &path_segment.arguments {
//                     if angle_brackets_generic_arguments.args.len() == 2 {
//                         let (
//                             key_generic_argument,
//                             value_generic_argument
//                         ) = {
//                             let mut key_generic_argument_option = None;
//                             let mut value_generic_argument_option = None;
//                             angle_brackets_generic_arguments.args
//                             .iter()
//                             .enumerate()
//                             .for_each(|(index, generic_argument)|{
//                                 match index {
//                                     0 => {
//                                         key_generic_argument_option = Some(generic_argument);
//                                     }
//                                     1 => {
//                                         value_generic_argument_option = Some(generic_argument);
//                                     }
//                                     _ => panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() != 2")
//                                 }
//                             });
//                             (
//                                 key_generic_argument_option.unwrap_or_else(|| panic!(
//                                     "{proc_macro_name_ident_stringified} key_generic_argument_option {}",
//                                     naming_conventions::IS_NONE_STRINGIFIED
//                                 )),
//                                 value_generic_argument_option.unwrap_or_else(|| panic!(
//                                     "{proc_macro_name_ident_stringified} value_generic_argument_option {}",
//                                     naming_conventions::IS_NONE_STRINGIFIED
//                                 ))
//                             )
//                         };
//                         let hashmap_key_type
//                         = if let syn::GenericArgument::Type(type_handle) =
//                             key_generic_argument
//                         {
//                             match type_handle {
//                                 syn::Type::Path(type_path) => {
//                                     crate::error_occurence::hashmap_value_type::HashMapKeyType::Path{
//                                         key_segments_stringified: crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
//                                         key_vec_lifetime: crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
//                                             &type_path.path.segments,
//                                             proc_macro_name_ident_stringified
//                                         )
//                                     }
//                                 },
//                                 syn::Type::Reference(type_reference) => {
//                                     let key_reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem.clone() {
//                                         if type_path.path.segments.len() == 1 {
//                                             type_path.path.segments
//                                             .into_iter().next()
//                                             .unwrap_or_else(|| panic!(
//                                                 "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
//                                                 naming_conventions::IS_NONE_STRINGIFIED
//                                             ))
//                                             .ident
//                                         }
//                                         else {
//                                             panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
//                                         }
//                                     }
//                                     else {
//                                         panic!(
//                                             "{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {}",
//                                             naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                                             naming_conventions::SYN_TYPE_PATH
//                                         );
//                                     };
//                                     crate::error_occurence::hashmap_value_type::HashMapKeyType::Reference {
//                                         key_reference_ident,
//                                         key_lifetime_ident: type_reference.lifetime.clone().unwrap_or_else(|| panic!(
//                                             "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
//                                             naming_conventions::IS_NONE_STRINGIFIED
//                                         )).ident
//                                     }
//                                 },
//                                 syn::Type::Array(_) |
//                                 syn::Type::BareFn(_) |
//                                 syn::Type::Group(_) |
//                                 syn::Type::ImplTrait(_) |
//                                 syn::Type::Infer(_) |
//                                 syn::Type::Macro(_) |
//                                 syn::Type::Never(_) |
//                                 syn::Type::Paren(_) |
//                                 syn::Type::Ptr(_) |
//                                 syn::Type::Slice(_) |
//                                 syn::Type::TraitObject(_) |
//                                 syn::Type::Tuple(_) |
//                                 syn::Type::Verbatim(_) => panic!(
//                                     "{proc_macro_name_ident_stringified} type_handle {} {} and {syn_type_reference}",
//                                     naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                                     naming_conventions::SYN_TYPE_PATH
//                                 ),
//                                 _ => panic!(
//                                     "{proc_macro_name_ident_stringified} type_handle {} {} and {syn_type_reference} (exhaustive)",
//                                     naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                                     naming_conventions::SYN_TYPE_PATH
//                                 ),
//                             }
//                         }
//                         else {
//                             panic!(
//                                 "{proc_macro_name_ident_stringified} key_generic_argument {} {}",
//                                 naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                                 naming_conventions::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
//                             );
//                         };
//                         let hashmap_value_type = if let syn::GenericArgument::Type(type_handle) = value_generic_argument {
//                             match type_handle {
//                                 syn::Type::Path(type_path) => {
//                                    crate::error_occurence::hashmap_key_type::HashMapValueType::Path{
//                                         value_segments_stringified: crate::error_occurence::generate_path_from_segments::generate_path_from_segments(&type_path.path.segments),
//                                         value_vec_lifetime: crate::error_occurence::form_last_arg_lifetime_vec::form_last_arg_lifetime_vec(
//                                             &type_path.path.segments,
//                                             proc_macro_name_ident_stringified
//                                         )
//                                     }
//                                 },
//                                 syn::Type::Reference(type_reference) => {
//                                     let value_reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem.clone() {
//                                         if type_path.path.segments.len() == 1 {
//                                             type_path.path.segments
//                                             .into_iter().next()
//                                             .unwrap_or_else(|| panic!(
//                                                 "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
//                                                 naming_conventions::IS_NONE_STRINGIFIED
//                                             ))
//                                             .ident
//                                         }
//                                         else {
//                                             panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
//                                         }
//                                     }
//                                     else {
//                                         panic!(
//                                             "{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {}",
//                                             naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                                             naming_conventions::SYN_TYPE_PATH
//                                         );
//                                     };
//                                    crate::error_occurence::hashmap_key_type::HashMapValueType::Reference {
//                                         value_reference_ident,
//                                         value_lifetime_ident: type_reference.lifetime.clone().unwrap_or_else(|| panic!(
//                                             "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
//                                             naming_conventions::IS_NONE_STRINGIFIED
//                                         )).ident
//                                     }
//                                 },
//                                 syn::Type::Array(_) |
//                                 syn::Type::BareFn(_) |
//                                 syn::Type::Group(_) |
//                                 syn::Type::ImplTrait(_) |
//                                 syn::Type::Infer(_) |
//                                 syn::Type::Macro(_) |
//                                 syn::Type::Never(_) |
//                                 syn::Type::Paren(_) |
//                                 syn::Type::Ptr(_) |
//                                 syn::Type::Slice(_) |
//                                 syn::Type::TraitObject(_) |
//                                 syn::Type::Tuple(_) |
//                                 syn::Type::Verbatim(_) => panic!(
//                                     "{proc_macro_name_ident_stringified} type_handle {} {} and syn::Type::Reference",
//                                     naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                                     naming_conventions::SYN_TYPE_PATH
//                                 ),
//                                 _ => panic!(
//                                     "{proc_macro_name_ident_stringified} type_handle {} {} and syn::Type::Reference (exhaustive)",
//                                     naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                                     naming_conventions::SYN_TYPE_PATH
//                                 ),
//                             }
//                         }
//                         else {
//                             panic!(
//                                 "{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args[0] {} {}",
//                                 naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                                 naming_conventions::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
//                             );
//                         };
//                         (
//                             hashmap_key_type,
//                             hashmap_value_type,
//                         )
//                     }
//                     else {
//                         panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() == 2");
//                     }
//                 }
//                 else {
//                     panic!(
//                         "{proc_macro_name_ident_stringified} path_segment.arguments {} syn::PathArguments::AngleBracketed",
//                         naming_conventions::SUPPORTS_ONLY_STRINGIFIED
//                     );
//                 };
//                 crate::error_occurence::supported_container::SupportedContainer::HashMap{
//                     path,
//                     hashmap_key_type,
//                     hashmap_value_type
//                 }
//             }
//             else {
//                 crate::error_occurence::supported_container::SupportedContainer::Path{
//                     path,
//                     vec_lifetime,
//                 }
//             }
//         },
//         syn::Type::Reference(type_reference) => {
//             let reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem.clone() {
//                 if type_path.path.segments.len() == 1 {
//                     type_path.path.segments
//                     .into_iter().next()
//                     .unwrap_or_else(|| panic!(
//                         "{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {}",
//                         naming_conventions::IS_NONE_STRINGIFIED
//                     ))
//                     .ident
//                 }
//                 else {
//                     panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
//                 }
//             }
//             else {
//                 panic!(
//                     "{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {} {}",
//                     naming_conventions::SUPPORTS_ONLY_STRINGIFIED,
//                     naming_conventions::SYN_TYPE_PATH
//                 );
//             };
//             crate::error_occurence::supported_container::SupportedContainer::Reference{
//                 reference_ident,
//                 lifetime_ident: type_reference.lifetime.clone().unwrap_or_else(|| panic!(
//                     "{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {}",
//                     naming_conventions::IS_NONE_STRINGIFIED
//                 )).ident,
//             }
//         },
//         syn::Type::Array(_) |
//         syn::Type::BareFn(_) |
//         syn::Type::Group(_) |
//         syn::Type::ImplTrait(_) |
//         syn::Type::Infer(_) |
//         syn::Type::Macro(_) |
//         syn::Type::Never(_) |
//         syn::Type::Paren(_) |
//         syn::Type::Ptr(_) |
//         syn::Type::Slice(_) |
//         syn::Type::TraitObject(_) |
//         syn::Type::Tuple(_) |
//         syn::Type::Verbatim(_) => panic!("{proc_macro_name_ident_stringified} field.ty is not syn::Type::Path or syn::Type::Reference {error_message}"),
//         _ => panic!("{proc_macro_name_ident_stringified} field.ty is not syn::Type::Path or syn::Type::Reference {error_message} (exhaustive)"),
//     }
// }

// pub fn generate_field_type_with_serialize_deserialize_version(
//     attribute: crate::error_occurence::ErrorOccurenceFieldAttribute,
//     supported_container: crate::error_occurence::supported_container::SupportedContainer,
//     proc_macro_name_ident_stringified: &str,
// ) -> proc_macro2::TokenStream {
