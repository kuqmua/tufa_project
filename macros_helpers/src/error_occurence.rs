pub fn form_last_arg_lifetime_vec(segments: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>, proc_macro_name_ident_stringified: &str) -> Vec<Lifetime> {
    segments.last().map_or_else(
        || panic!("{proc_macro_name_ident_stringified} type_path.path.segments.last() {}", naming::IS_NONE_STRINGIFIED),
        |path_segment| match &path_segment.arguments {
            syn::PathArguments::None => Vec::new(),
            syn::PathArguments::AngleBracketed(angle_bracketed_generic_argument) => angle_bracketed_generic_argument
                .args
                .iter()
                .map(|generic_argument| match generic_argument {
                    syn::GenericArgument::Lifetime(lfmt) => Lifetime::Specified(lfmt.ident.to_string()),
                    syn::GenericArgument::Type(_) => Lifetime::NotSpecified,
                    syn::GenericArgument::Const(_) | syn::GenericArgument::AssocType(_) | syn::GenericArgument::AssocConst(_) | syn::GenericArgument::Constraint(_) => {
                        panic!("{proc_macro_name_ident_stringified} type_path.path.segments.last() angle_bracketed_generic_argument.args[0] {} syn::GenericArgument::Lifetime and {}", naming::SUPPORTS_ONLY_STRINGIFIED, naming::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED)
                    }
                    _ => panic!(
                        "{proc_macro_name_ident_stringified} type_path.path.segments.last() angle_bracketed_generic_argument.args[0] {} syn::GenericArgument::Lifetime and {} (exhaustive)",
                        naming::SUPPORTS_ONLY_STRINGIFIED,
                        naming::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
                    ),
                })
                .collect(),
            syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name_ident_stringified} type_path.path.segments.last() is unexpected syn::PathArguments::Parenthesized"),
        },
    )
}

pub fn generate_path_from_segments(segments: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>) -> String {
    let mut segments_stringified = segments.iter().fold(String::new(), |mut acc, elem| {
        use std::fmt::Write as _;
        assert!(write!(acc, "{}::", elem.ident).is_ok(), "error 87a234a5-8086-474d-b780-57e77c4add43");
        acc
    });
    let _: Option<char> = segments_stringified.pop();
    let _: Option<char> = segments_stringified.pop();
    segments_stringified
}

#[derive(Debug)]
pub enum HashMapValueType {
    Path { value_segments_stringified: String, value_vec_lifetime: Vec<Lifetime> },
    Reference { value_reference_ident: proc_macro2::Ident, value_lifetime_ident: proc_macro2::Ident },
}

#[derive(Debug)]
pub enum HashMapKeyType {
    Path { key_segments_stringified: String, key_vec_lifetime: Vec<Lifetime> },
    Reference { key_reference_ident: proc_macro2::Ident, key_lifetime_ident: proc_macro2::Ident },
}

#[derive(Debug)]
pub enum Lifetime {
    Specified(String),
    NotSpecified,
}
impl std::fmt::Display for Lifetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Specified(value) => {
                write!(f, "'{value}")
            }
            Self::NotSpecified => write!(f, ""),
        }
    }
}

pub fn panic_if_not_str(reference_ident: &proc_macro2::Ident, str_stringified: &str, proc_macro_name_ident_stringified: &str, supports_only_stringified: &str, attribute: &ErrorOccurenceFieldAttribute) {
    assert!(
        reference_ident == str_stringified,
        "{proc_macro_name_ident_stringified} {} {supports_only_stringified} {str_stringified}, but got {reference_ident}",
        crate::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(attribute)
    );
}

pub fn panic_if_not_string(segments_stringified: &str, std_string_string_stringified: &str, proc_macro_name_ident_stringified: &str, supports_only_stringified: &str, as_std_collections_hashmap_key_type_stringified: &str, attribute: &ErrorOccurenceFieldAttribute) {
    assert!(
        segments_stringified == std_string_string_stringified,
        "{proc_macro_name_ident_stringified} {} {supports_only_stringified} {std_string_string_stringified} {as_std_collections_hashmap_key_type_stringified} (hashmap key must be string for deserialization)",
        crate::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(attribute)
    );
}

pub fn possible_lifetime_addition(lifetime: String, lifetimes_for_serialize_deserialize: &mut Vec<String>) {
    if !lifetimes_for_serialize_deserialize.contains(&lifetime) {
        lifetimes_for_serialize_deserialize.push(lifetime);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuportedEnumVariant {
    Named,
    Unnamed,
}
impl SuportedEnumVariant {
    pub fn new_or_panic(data_enum: &syn::DataEnum) -> Self {
        let mut all_equal: Option<Self> = None;
        assert!(!data_enum.variants.is_empty(), "enum variants are empty");
        let error_message = format!("{} enums where all variants are {}::{} or all variants are {}::{}", naming::SUPPORTS_ONLY_STRINGIFIED, naming::SYN_FIELDS, naming::SYN_FIELDS, naming::NamedUpperCamelCase, naming::UnnamedUpperCamelCase,);
        data_enum.variants.iter().for_each(|variant| match &variant.fields {
            syn::Fields::Named(_) => match &all_equal {
                Some(supported_variant) => {
                    assert!(!(*supported_variant == Self::Unnamed), "{error_message}");
                }
                None => {
                    all_equal = Some(Self::Named);
                }
            },
            syn::Fields::Unnamed(_) => match &all_equal {
                Some(supported_variant) => {
                    assert!(!(*supported_variant == Self::Named), "{error_message}");
                }
                None => {
                    all_equal = Some(Self::Unnamed);
                }
            },
            syn::Fields::Unit => panic!("{error_message}"),
        });
        all_equal.unwrap_or_else(|| {
            panic!("{} with enums where all variants are named or unnamed", naming::SUPPORTS_ONLY_STRINGIFIED);
        })
    }
}

#[derive(Debug)]
pub enum VecElementType {
    Path { element_path: String, vec_lifetime: Vec<Lifetime> },
    Reference { reference_ident: proc_macro2::Ident, lifetime_ident: proc_macro2::Ident },
}

pub fn vec_lifetime_to_string(vec: &[Lifetime]) -> String {
    let mut lifetimes_stringified_handle = vec.iter().fold(String::new(), |mut acc, path_segment| {
        acc.push_str(&path_segment.to_string());
        acc
    });
    let _: Option<char> = lifetimes_stringified_handle.pop();
    format!("<{lifetimes_stringified_handle}>")
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorOccurenceFieldAttribute {
    EoToStdStringString,
    EoToStdStringStringSerializeDeserialize,
    EoErrorOccurence,
    EoVecToStdStringString,
    EoVecToStdStringStringSerializeDeserialize,
    EoVecErrorOccurence,
    EoHashMapKeyStdStringStringValueToStdStringString,
    EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize,
    EoHashMapKeyStdStringStringValueErrorOccurence,
}
impl std::str::FromStr for ErrorOccurenceFieldAttribute {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "eo_to_std_string_string" {
            Ok(Self::EoToStdStringString)
        } else if value == "eo_to_std_string_string_serialize_deserialize" {
            Ok(Self::EoToStdStringStringSerializeDeserialize)
        } else if value == "eo_error_occurence" {
            Ok(Self::EoErrorOccurence)
        } else if value == "eo_vec_to_std_string_string" {
            Ok(Self::EoVecToStdStringString)
        } else if value == "eo_vec_to_std_string_string_serialize_deserialize" {
            Ok(Self::EoVecToStdStringStringSerializeDeserialize)
        } else if value == "eo_vec_error_occurence" {
            Ok(Self::EoVecErrorOccurence)
        } else if value == "eo_hashmap_key_std_string_string_value_to_std_string_string" {
            Ok(Self::EoHashMapKeyStdStringStringValueToStdStringString)
        } else if value == "eo_hashmap_key_std_string_string_value_to_std_string_string_serialize_deserialize" {
            Ok(Self::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize)
        } else if value == "eo_hashmap_key_std_string_string_value_error_occurence" {
            Ok(Self::EoHashMapKeyStdStringStringValueErrorOccurence)
        } else {
            Err(())
        }
    }
}
impl TryFrom<&syn::Field> for ErrorOccurenceFieldAttribute {
    type Error = String;
    fn try_from(syn_field: &syn::Field) -> Result<Self, Self::Error> {
        let mut option_attribute = None;
        for attr in &syn_field.attrs {
            if attr.path().segments.len() == 1 {
                let first_segment_ident = match &attr.path().segments.first() {
                    Some(value) => &value.ident,
                    None => {
                        return Err("no first value in punctuated".to_owned());
                    }
                };
                if let Ok(value) = std::str::FromStr::from_str(&first_segment_ident.to_string()) {
                    if option_attribute.is_some() {
                        return Err("two or more supported attributes!".to_owned());
                    }
                    option_attribute = Some(value);
                }
            } //other attributes are not for this proc_macro
        }
        option_attribute.map_or_else(|| Err("option attribute is None".to_owned()), Ok)
    }
}
impl crate::attribute_ident_stringified::AttributeIdentStringified for ErrorOccurenceFieldAttribute {
    fn attribute_ident_stringified(&self) -> &str {
        match self {
            Self::EoToStdStringString => "eo_to_std_string_string",
            Self::EoToStdStringStringSerializeDeserialize => "eo_to_std_string_string_serialize_deserialize",
            Self::EoErrorOccurence => "eo_error_occurence",
            Self::EoVecToStdStringString => "eo_vec_to_std_string_string",
            Self::EoVecToStdStringStringSerializeDeserialize => "eo_vec_to_std_string_string_serialize_deserialize",
            Self::EoVecErrorOccurence => "eo_vec_error_occurence",
            Self::EoHashMapKeyStdStringStringValueToStdStringString => "eo_hashmap_key_std_string_string_value_to_std_string_string",
            Self::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => "eo_hashmap_key_std_string_string_value_to_std_string_string_serialize_deserialize",
            Self::EoHashMapKeyStdStringStringValueErrorOccurence => "eo_hashmap_key_std_string_string_value_error_occurence",
        }
    }
}
impl ErrorOccurenceFieldAttribute {
    pub fn to_attribute_view_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!("#[{}]", crate::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(self));
        value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

pub fn attribute_view(attribute: &str) -> String {
    format!("attribute #[{attribute}]")
}

pub fn get_type_path_third_segment_second_argument_check_if_hashmap<'value_lifetime>(value: &'value_lifetime syn::Field, std_snake_case: &naming::StdSnakeCase, std_string_string: &token_patterns::StdStringString) -> &'value_lifetime syn::GenericArgument {
    let segments = if let syn::Type::Path(syn_type_path) = &value.ty {
        &syn_type_path.path.segments
    } else {
        panic!("Type::Path(syn_type_path) != &element.ty");
    };
    assert!(segments.len() == 3, "segments.len() != 3");
    let first_segment = segments.iter().next().expect("no .next()) element");
    assert!(first_segment.ident == std_snake_case.to_string(), "first_segment.ident != {std_snake_case} {}", first_segment.ident);
    match first_segment.arguments {
        syn::PathArguments::None => (),
        syn::PathArguments::AngleBracketed(_) | syn::PathArguments::Parenthesized(_) => {
            panic!("first_segment.arguments != PathArguments::None")
        }
    }
    let second_segment = segments.iter().nth(1).expect("no .nth(1) element");
    {
        let collections_snake_case = naming::CollectionsSnakeCase;
        assert!(second_segment.ident == collections_snake_case.to_string(), "second_segment.ident != {collections_snake_case} {}", second_segment.ident);
    };
    match second_segment.arguments {
        syn::PathArguments::None => (),
        syn::PathArguments::AngleBracketed(_) | syn::PathArguments::Parenthesized(_) => {
            panic!("second_segment.arguments != PathArguments::None")
        }
    }
    let third_segment = segments.iter().nth(2).expect("no .nth(2) element");
    {
        let hashmap_upper_camel_case = naming::HashMapUpperCamelCase;
        assert!(third_segment.ident == hashmap_upper_camel_case.to_string(), "third_segment.ident != {hashmap_upper_camel_case} {}", third_segment.ident);
    };
    let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) = &third_segment.arguments else {
        panic!("third_segment.arguments != syn::PathArguments::AngleBracketed");
    };
    assert!(args.len() == 2, "args.len() != 2");
    let first_argument_stringified = {
        let first_argument = args.iter().next().expect("args.iter().next() is None");
        quote::quote! {#first_argument}.to_string()
    };
    assert!(quote::quote! {#std_string_string}.to_string() == first_argument_stringified, "{} != {first_argument_stringified}", quote::quote! {#std_string_string});
    args.iter().nth(1).expect("args.iter().nth(1) is None")
}

pub fn generate_serialize_deserialize_version_of_named_syn_variant(value: &syn::Variant) -> proc_macro2::TokenStream {
    let element_ident = &value.ident;
    let fields = if let syn::Fields::Named(fields) = &value.fields {
        &fields.named
    } else {
        panic!("{} syn::Data::Enum", naming::SUPPORTS_ONLY_STRINGIFIED);
    };
    let std_string_string = token_patterns::StdStringString;
    let fields_idents_idents_with_serialize_deserialize_excluding_code_occurence_token_stream = fields.iter().filter(|element| *element.ident.as_ref().expect(constants::IDENT_IS_NONE) != *naming::CodeOccurenceSnakeCase.to_string()).map(|element| {
        let current_element_ident = element.ident.as_ref().expect(constants::IDENT_IS_NONE);
        let element_type_token_stream = {
            let element_type = &element.ty;
            quote::quote! {#element_type}
        };
        let std_snake_case = naming::StdSnakeCase;
        let element_type_with_serialize_deserialize_token_stream = match ErrorOccurenceFieldAttribute::try_from(element).expect("error 2db209a8-2f57-4474-a9c6-9743aaaed57d") {
            ErrorOccurenceFieldAttribute::EoToStdStringString => {
                quote::quote! {
                    #std_string_string
                }
            }
            ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize | ErrorOccurenceFieldAttribute::EoVecToStdStringStringSerializeDeserialize => element_type_token_stream,
            ErrorOccurenceFieldAttribute::EoErrorOccurence => format!(
                "{}{}",
                {
                    let element_type = &element.ty;
                    quote::quote! {#element_type}
                },
                naming::WithSerializeDeserializeUpperCamelCase
            ).parse::<proc_macro2::TokenStream>().expect("error 201dc0a4-4563-4e51-a228-ba085b767775"),
            ErrorOccurenceFieldAttribute::EoVecToStdStringString => {
                quote::quote! {
                    Vec<#std_string_string>
                }
            }
            ErrorOccurenceFieldAttribute::EoVecErrorOccurence => {
                let segments = if let syn::Type::Path(path_value) = &element.ty {
                    &path_value.path.segments
                } else {
                    panic!("Type::Path(value) != &element.ty");
                };
                assert!(segments.len() == 1, "segments.len() != 1");
                let first_segment = segments.iter().next().expect("no .next() element");
                let element_vec_type_with_serialize_deserialize_token_stream = if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) = &first_segment.arguments {
                    assert!(args.len() == 1, "args.len() != 1");
                    format!(
                        "{}{}",
                        {
                            let first_arg = args.iter().next().expect("args.iter().next() is None");
                            quote::quote! {#first_arg}
                        },
                        naming::WithSerializeDeserializeUpperCamelCase,
                    ).parse::<proc_macro2::TokenStream>().expect("error 22c364b9-c645-46ec-984e-cf0b911feb84")
                } else {
                    panic!("third_segment.arguments != syn::PathArguments::AngleBracketed");
                };
                quote::quote! {
                    Vec<#element_vec_type_with_serialize_deserialize_token_stream>
                }
            }
            ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringString => {
                let _: &syn::GenericArgument = get_type_path_third_segment_second_argument_check_if_hashmap(element, &std_snake_case, &std_string_string);
                quote::quote! {
                    std::collections::HashMap<#std_string_string, #std_string_string>
                }
            }
            ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueToStdStringStringSerializeDeserialize => {
                let _: &syn::GenericArgument = get_type_path_third_segment_second_argument_check_if_hashmap(element, &std_snake_case, &std_string_string);
                element_type_token_stream
            }
            ErrorOccurenceFieldAttribute::EoHashMapKeyStdStringStringValueErrorOccurence => {
                let second_argument = get_type_path_third_segment_second_argument_check_if_hashmap(element, &std_snake_case, &std_string_string);
                let element_hashmap_value_type_with_serialize_deserialize_token_stream = format!(
                    "{}{}",
                    quote::quote! {#second_argument},
                    naming::WithSerializeDeserializeUpperCamelCase
                ).parse::<proc_macro2::TokenStream>().expect("error 86307dbc-484e-4012-ac70-2d593b1f99e6");
                quote::quote! {
                    std::collections::HashMap<#std_string_string, #element_hashmap_value_type_with_serialize_deserialize_token_stream>
                }
            }
        };
        quote::quote! {#current_element_ident: #element_type_with_serialize_deserialize_token_stream,}
    });
    let code_occurence_snake_case_token_stream = naming::CodeOccurenceSnakeCase;
    quote::quote! {
        #element_ident {
            #(#fields_idents_idents_with_serialize_deserialize_excluding_code_occurence_token_stream)*
            #code_occurence_snake_case_token_stream: error_occurence_lib::code_occurence::CodeOccurence,
        }
    }
}
