#[derive(Debug)]
enum SupportedPredefinedType {
    JsonStdPrimitiveI8, 
    JsonStdPrimitiveI16, 
    JsonStdPrimitiveI32, 
    JsonStdPrimitiveI64, 
    JsonStdPrimitiveI128,
    JsonStdPrimitiveU8, 
    JsonStdPrimitiveU16, 
    JsonStdPrimitiveU32, 
    JsonStdPrimitiveU64, 
    JsonStdPrimitiveU128, 
    JsonStdPrimitiveF32, 
    JsonStdPrimitiveF64, 
    JsonStdPrimitiveBool, 
    JsonStdStringString, 

    JsonStdOptionOptionStdPrimitiveI8, 
    JsonStdOptionOptionStdPrimitiveI16, 
    JsonStdOptionOptionStdPrimitiveI32, 
    JsonStdOptionOptionStdPrimitiveI64, 
    JsonStdOptionOptionStdPrimitiveI128, 
    JsonStdOptionOptionStdPrimitiveU8, 
    JsonStdOptionOptionStdPrimitiveU16, 
    JsonStdOptionOptionStdPrimitiveU32, 
    JsonStdOptionOptionStdPrimitiveU64, 
    JsonStdOptionOptionStdPrimitiveU128, 
    JsonStdOptionOptionStdPrimitiveF32, 
    JsonStdOptionOptionStdPrimitiveF64, 
    JsonStdOptionOptionStdPrimitiveBool, 
    JsonStdOptionOptionStdStringString, 

    // JsonStdVecVecStdPrimitiveI8, 
    // JsonStdVecVecStdPrimitiveI16, 
    // JsonStdVecVecStdPrimitiveI32, 
    // JsonStdVecVecStdPrimitiveI64, 
    // JsonStdVecVecStdPrimitiveI128, 
    // JsonStdVecVecStdPrimitiveU8, 
    // JsonStdVecVecStdPrimitiveU16, 
    // JsonStdVecVecStdPrimitiveU32, 
    // JsonStdVecVecStdPrimitiveU64, 
    // JsonStdVecVecStdPrimitiveU128, 
    // JsonStdVecVecStdPrimitiveF32, 
    // JsonStdVecVecStdPrimitiveF64, 
    // JsonStdVecVecStdPrimitiveBool, 
    // JsonStdVecVecStdStringString, 

    // JsonStdOptionOptionStdVecVecStdPrimitiveI8, 
    // JsonStdOptionOptionStdVecVecStdPrimitiveI16, 
    // JsonStdOptionOptionStdVecVecStdPrimitiveI32, 
    // JsonStdOptionOptionStdVecVecStdPrimitiveI64, 
    // JsonStdOptionOptionStdVecVecStdPrimitiveI128, 
    // JsonStdOptionOptionStdVecVecStdPrimitiveU8, 
    // JsonStdOptionOptionStdVecVecStdPrimitiveU16, 
    // JsonStdOptionOptionStdVecVecStdPrimitiveU32, 
    // JsonStdOptionOptionStdVecVecStdPrimitiveU64, 
    // JsonStdOptionOptionStdVecVecStdPrimitiveU128, 
    // JsonStdOptionOptionStdVecVecStdPrimitiveF32, 
    // JsonStdOptionOptionStdVecVecStdPrimitiveF64, 
    // JsonStdOptionOptionStdVecVecStdPrimitiveBool, 
    // JsonStdOptionOptionStdVecVecStdStringString, 

    // JsonStdVecVecStdOptionOptionStdPrimitiveI8, 
    // JsonStdVecVecStdOptionOptionStdPrimitiveI16, 
    // JsonStdVecVecStdOptionOptionStdPrimitiveI32, 
    // JsonStdVecVecStdOptionOptionStdPrimitiveI64, 
    // JsonStdVecVecStdOptionOptionStdPrimitiveI128, 
    // JsonStdVecVecStdOptionOptionStdPrimitiveU8, 
    // JsonStdVecVecStdOptionOptionStdPrimitiveU16, 
    // JsonStdVecVecStdOptionOptionStdPrimitiveU32, 
    // JsonStdVecVecStdOptionOptionStdPrimitiveU64, 
    // JsonStdVecVecStdOptionOptionStdPrimitiveU128, 
    // JsonStdVecVecStdOptionOptionStdPrimitiveF32, 
    // JsonStdVecVecStdOptionOptionStdPrimitiveF64, 
    // JsonStdVecVecStdOptionOptionStdPrimitiveBool, 
    // JsonStdVecVecStdOptionOptionStdStringString, 

    // JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8, 
    // JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16, 
    // JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32, 
    // JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64, 
    // JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128, 
    // JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8, 
    // JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16, 
    // JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32, 
    // JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64, 
    // JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128, 
    // JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32, 
    // JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64, 
    // JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool, 
    // JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString, 

    JsonGeneric(syn::TypePath), 
    JsonStdOptionOptionGeneric(syn::TypePath), 
    JsonStdVecVecGeneric(syn::TypePath), 
    JsonStdOptionOptionStdVecVecGeneric(syn::TypePath), 
    JsonStdVecVecStdOptionOptionGeneric(syn::TypePath), 
    JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(syn::TypePath),

    JsonUuid,
}
impl SupportedPredefinedType {
    fn to_original_type(&self) -> SupportedPredefinedOriginalType {
        match self {
            Self::JsonStdPrimitiveI8 => SupportedPredefinedOriginalType::I8,
            Self::JsonStdPrimitiveI16 => SupportedPredefinedOriginalType::I16,
            Self::JsonStdPrimitiveI32 => SupportedPredefinedOriginalType::I32,
            Self::JsonStdPrimitiveI64 => SupportedPredefinedOriginalType::I64,
            Self::JsonStdPrimitiveI128 => SupportedPredefinedOriginalType::I128,
            Self::JsonStdPrimitiveU8 => SupportedPredefinedOriginalType::U8,
            Self::JsonStdPrimitiveU16 => SupportedPredefinedOriginalType::U16,
            Self::JsonStdPrimitiveU32 => SupportedPredefinedOriginalType::U32,
            Self::JsonStdPrimitiveU64 => SupportedPredefinedOriginalType::U64,
            Self::JsonStdPrimitiveU128 => SupportedPredefinedOriginalType::U128,
            Self::JsonStdPrimitiveF32 => SupportedPredefinedOriginalType::F32,
            Self::JsonStdPrimitiveF64 => SupportedPredefinedOriginalType::F64,
            Self::JsonStdPrimitiveBool => SupportedPredefinedOriginalType::Bool,
            Self::JsonStdStringString => SupportedPredefinedOriginalType::String,

            Self::JsonStdOptionOptionStdPrimitiveI8 => SupportedPredefinedOriginalType::I8,
            Self::JsonStdOptionOptionStdPrimitiveI16 => SupportedPredefinedOriginalType::I16,
            Self::JsonStdOptionOptionStdPrimitiveI32 => SupportedPredefinedOriginalType::I32,
            Self::JsonStdOptionOptionStdPrimitiveI64 => SupportedPredefinedOriginalType::I64,
            Self::JsonStdOptionOptionStdPrimitiveI128 => SupportedPredefinedOriginalType::I128,
            Self::JsonStdOptionOptionStdPrimitiveU8 => SupportedPredefinedOriginalType::U8,
            Self::JsonStdOptionOptionStdPrimitiveU16 => SupportedPredefinedOriginalType::U16,
            Self::JsonStdOptionOptionStdPrimitiveU32 => SupportedPredefinedOriginalType::U32,
            Self::JsonStdOptionOptionStdPrimitiveU64 => SupportedPredefinedOriginalType::U64,
            Self::JsonStdOptionOptionStdPrimitiveU128 => SupportedPredefinedOriginalType::U128,
            Self::JsonStdOptionOptionStdPrimitiveF32 => SupportedPredefinedOriginalType::F32,
            Self::JsonStdOptionOptionStdPrimitiveF64 => SupportedPredefinedOriginalType::F64,
            Self::JsonStdOptionOptionStdPrimitiveBool => SupportedPredefinedOriginalType::Bool,
            Self::JsonStdOptionOptionStdStringString => SupportedPredefinedOriginalType::String,

            // Self::JsonStdVecVecStdPrimitiveI8 => SupportedPredefinedOriginalType::I8,
            // Self::JsonStdVecVecStdPrimitiveI16 => SupportedPredefinedOriginalType::I16,
            // Self::JsonStdVecVecStdPrimitiveI32 => SupportedPredefinedOriginalType::I32,
            // Self::JsonStdVecVecStdPrimitiveI64 => SupportedPredefinedOriginalType::I64,
            // Self::JsonStdVecVecStdPrimitiveI128 => SupportedPredefinedOriginalType::I128,
            // Self::JsonStdVecVecStdPrimitiveU8 => SupportedPredefinedOriginalType::U8,
            // Self::JsonStdVecVecStdPrimitiveU16 => SupportedPredefinedOriginalType::U16,
            // Self::JsonStdVecVecStdPrimitiveU32 => SupportedPredefinedOriginalType::U32,
            // Self::JsonStdVecVecStdPrimitiveU64 => SupportedPredefinedOriginalType::U64,
            // Self::JsonStdVecVecStdPrimitiveU128 => SupportedPredefinedOriginalType::U128,
            // Self::JsonStdVecVecStdPrimitiveF32 => SupportedPredefinedOriginalType::F32,
            // Self::JsonStdVecVecStdPrimitiveF64 => SupportedPredefinedOriginalType::F64,
            // Self::JsonStdVecVecStdPrimitiveBool => SupportedPredefinedOriginalType::Bool,
            // Self::JsonStdVecVecStdStringString => SupportedPredefinedOriginalType::String,

            // Self::JsonStdOptionOptionStdVecVecStdPrimitiveI8 => SupportedPredefinedOriginalType::I8,
            // Self::JsonStdOptionOptionStdVecVecStdPrimitiveI16 => SupportedPredefinedOriginalType::I16,
            // Self::JsonStdOptionOptionStdVecVecStdPrimitiveI32 => SupportedPredefinedOriginalType::I32,
            // Self::JsonStdOptionOptionStdVecVecStdPrimitiveI64 => SupportedPredefinedOriginalType::I64,
            // Self::JsonStdOptionOptionStdVecVecStdPrimitiveI128 => SupportedPredefinedOriginalType::I128,
            // Self::JsonStdOptionOptionStdVecVecStdPrimitiveU8 => SupportedPredefinedOriginalType::U8,
            // Self::JsonStdOptionOptionStdVecVecStdPrimitiveU16 => SupportedPredefinedOriginalType::U16,
            // Self::JsonStdOptionOptionStdVecVecStdPrimitiveU32 => SupportedPredefinedOriginalType::U32,
            // Self::JsonStdOptionOptionStdVecVecStdPrimitiveU64 => SupportedPredefinedOriginalType::U64,
            // Self::JsonStdOptionOptionStdVecVecStdPrimitiveU128 => SupportedPredefinedOriginalType::U128,
            // Self::JsonStdOptionOptionStdVecVecStdPrimitiveF32 => SupportedPredefinedOriginalType::F32,
            // Self::JsonStdOptionOptionStdVecVecStdPrimitiveF64 => SupportedPredefinedOriginalType::F64,
            // Self::JsonStdOptionOptionStdVecVecStdPrimitiveBool => SupportedPredefinedOriginalType::Bool,
            // Self::JsonStdOptionOptionStdVecVecStdStringString => SupportedPredefinedOriginalType::String,

            // Self::JsonStdVecVecStdOptionOptionStdPrimitiveI8 => SupportedPredefinedOriginalType::I8,
            // Self::JsonStdVecVecStdOptionOptionStdPrimitiveI16 => SupportedPredefinedOriginalType::I16,
            // Self::JsonStdVecVecStdOptionOptionStdPrimitiveI32 => SupportedPredefinedOriginalType::I32,
            // Self::JsonStdVecVecStdOptionOptionStdPrimitiveI64 => SupportedPredefinedOriginalType::I64,
            // Self::JsonStdVecVecStdOptionOptionStdPrimitiveI128 => SupportedPredefinedOriginalType::I128,
            // Self::JsonStdVecVecStdOptionOptionStdPrimitiveU8 => SupportedPredefinedOriginalType::U8,
            // Self::JsonStdVecVecStdOptionOptionStdPrimitiveU16 => SupportedPredefinedOriginalType::U16,
            // Self::JsonStdVecVecStdOptionOptionStdPrimitiveU32 => SupportedPredefinedOriginalType::U32,
            // Self::JsonStdVecVecStdOptionOptionStdPrimitiveU64 => SupportedPredefinedOriginalType::U64,
            // Self::JsonStdVecVecStdOptionOptionStdPrimitiveU128 => SupportedPredefinedOriginalType::U128,
            // Self::JsonStdVecVecStdOptionOptionStdPrimitiveF32 => SupportedPredefinedOriginalType::F32,
            // Self::JsonStdVecVecStdOptionOptionStdPrimitiveF64 => SupportedPredefinedOriginalType::F64,
            // Self::JsonStdVecVecStdOptionOptionStdPrimitiveBool => SupportedPredefinedOriginalType::Bool,
            // Self::JsonStdVecVecStdOptionOptionStdStringString => SupportedPredefinedOriginalType::String,

            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 => SupportedPredefinedOriginalType::I8,
            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 => SupportedPredefinedOriginalType::I16,
            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 => SupportedPredefinedOriginalType::I32,
            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 => SupportedPredefinedOriginalType::I64,
            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 => SupportedPredefinedOriginalType::I128,
            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 => SupportedPredefinedOriginalType::U8,
            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 => SupportedPredefinedOriginalType::U16,
            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 => SupportedPredefinedOriginalType::U32,
            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 => SupportedPredefinedOriginalType::U64,
            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 => SupportedPredefinedOriginalType::U128,
            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 => SupportedPredefinedOriginalType::F32,
            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => SupportedPredefinedOriginalType::F64,
            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => SupportedPredefinedOriginalType::Bool,
            // Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => SupportedPredefinedOriginalType::String,

            Self::JsonGeneric(value) => SupportedPredefinedOriginalType::Generic(value.clone()),
            Self::JsonStdOptionOptionGeneric(value) => SupportedPredefinedOriginalType::Generic(value.clone()),
            Self::JsonStdVecVecGeneric(value) => SupportedPredefinedOriginalType::Generic(value.clone()),
            Self::JsonStdOptionOptionStdVecVecGeneric(value) => SupportedPredefinedOriginalType::Generic(value.clone()),
            Self::JsonStdVecVecStdOptionOptionGeneric(value) => SupportedPredefinedOriginalType::Generic(value.clone()),
            Self::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(value) => SupportedPredefinedOriginalType::Generic(value.clone()),

            Self::JsonUuid => SupportedPredefinedOriginalType::Uuid,
        }
    }
}
#[derive(Debug)]
enum SupportedPredefinedOriginalType {
    I8, 
    I16, 
    I32, 
    I64, 
    I128,
    U8, 
    U16, 
    U32, 
    U64, 
    U128, 
    F32, 
    F64, 
    Bool, 
    String, 
    Generic(syn::TypePath),
    Uuid,
}

fn generate_ident_options_to_read_upper_camel_case_token_stream(value: &std::primitive::str) -> proc_macro2::TokenStream {
    let value = format!("{value}{}", naming_conventions::OptionsToReadUpperCamelCase);
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}

impl SupportedPredefinedOriginalType {
    fn to_read_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Self::I8 => quote::quote!{i8},
            Self::I16 => quote::quote!{i16},
            Self::I32 => quote::quote!{i32},
            Self::I64 => quote::quote!{i64},
            Self::I128 => quote::quote!{i128},
            Self::U8 => quote::quote!{u8},
            Self::U16 => quote::quote!{u16},
            Self::U32 => quote::quote!{u32},
            Self::U64 => quote::quote!{u64},
            Self::U128 => quote::quote!{u128},
            Self::F32 => quote::quote!{f32},
            Self::F64 => quote::quote!{f64},
            Self::Bool => quote::quote!{bool},
            Self::String => quote::quote!{String},
            Self::Generic(type_path) => generate_ident_options_to_read_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string()),
            Self::Uuid => quote::quote!{Uuid},
        }
    }
    fn full_type_path_token_stream(&self) -> proc_macro2::TokenStream {
        let wrap_into_std_primitive_token_stream = |value: proc_macro2::TokenStream|{
            quote::quote!{std::primitive::#value}
        };
        match self {
            Self::I8 |
            Self::I16 |
            Self::I32 |
            Self::I64 |
            Self::I128 |
            Self::U8 |
            Self::U16 |
            Self::U32 |
            Self::U64 |
            Self::U128 |
            Self::F32 |
            Self::F64 |
            Self::Bool => wrap_into_std_primitive_token_stream(self.to_read_token_stream()), 
            Self::String => {
                let value = self.to_read_token_stream();
                quote::quote!{std::string::#value}
            },
            Self::Generic(_) => self.to_read_token_stream(),
            Self::Uuid => quote::quote!{uuid::Uuid},
        }
    }
    fn std_option_option_full_type_path_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote!{std::option::Option<#value>}
    }
    fn std_vec_vec_full_type_path_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote!{std::vec::Vec<#value>}
    }
    fn std_option_option_std_vec_vec_full_type_path_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote!{std::option::Option<std::vec::Vec<#value>>}
    }
    fn std_vec_vec_std_option_option_full_type_path_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote!{std::vec::Vec<std::option::Option<#value>>}
    }
    fn std_option_option_std_vec_vec_std_option_option_full_type_path_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote!{std::option::Option<std::vec::Vec<std::option::Option<#value>>>}
    }
    fn std_vec_vec_std_result_result_full_path_type_std_string_string_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote!{std::vec::Vec<std::result::Result<#value,std::string::String>>}
    }
    fn std_option_option_std_vec_vec_std_result_result_full_path_type_std_string_string_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<#value,std::string::String>>>}
    }
    fn std_vec_vec_std_result_result_std_option_option_full_path_type_std_string_string_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote!{std::vec::Vec<std::result::Result<std::option::Option<#value>,std::string::String>>}
    }
    fn std_option_option_std_vec_vec_std_result_result_std_option_option_full_path_type_std_string_string_token_stream(&self) -> proc_macro2::TokenStream {
        let value = self.full_type_path_token_stream();
        quote::quote!{std::option::Option<std::vec::Vec<std::result::Result<std::option::Option<#value>,std::string::String>>>}
    }
}
#[derive(Debug)]
enum SupportedPredefinedTypeTryFromSynField {
    TypePathPathSegmentsIsNotSynTypePath,
    TypePathPathSegmentsLastIsNone,
    PathSegmentArgumentsIsNotSynPathArgumentsNone,
    UnsupportedPredefinedTypeWrapper,
    SynPathArgumentsAngleBracketedArgsLengthNotEqualOne,
    SynPathArgumentsAngleBracketedFirstSynGenericArgumentTypeSynTypePathSynPathArgumentsNone,
    SynPathArgumentsAngleBracketedFirstSynGenericArgumentTypeIsNotSynTypePath,
    SynPathArgumentsAngleBracketedFirstIsNotSynGenericArgumentType,
    SynPathArgumentsAngleBracketedFirstIsNone,
    SynPathArgumentsIsNotAngleBracketed,
}
impl std::convert::TryFrom<&syn::Field> for SupportedPredefinedType {
    type Error = SupportedPredefinedTypeTryFromSynField;
    fn try_from(value: &syn::Field) -> Result<Self, Self::Error> {
        match &value.ty {
            syn::Type::Path(type_path) => match type_path.path.segments.last() {
                Some(path_segment) => {
                    let try_generate_generic_ident_upper_camel_case_token_stream = |path_segment: &syn::PathSegment|{
                        match &path_segment.arguments {
                            syn::PathArguments::AngleBracketed(value) => {
                                if value.args.len() != 1 {
                                    return Err(Self::Error::SynPathArgumentsAngleBracketedArgsLengthNotEqualOne);
                                }
                                match value.args.first() {
                                    Some(value) => if let syn::GenericArgument::Type(value) = value {
                                        match &value {
                                            syn::Type::Path(type_path) => {
                                                for element in &type_path.path.segments {
                                                    if let syn::PathArguments::None = element.arguments {}
                                                    else {
                                                        return Err(Self::Error::SynPathArgumentsAngleBracketedFirstSynGenericArgumentTypeSynTypePathSynPathArgumentsNone);
                                                    }
                                                }
                                                return Ok(type_path.clone());
                                            }
                                            _ => {
                                                return Err(Self::Error::SynPathArgumentsAngleBracketedFirstSynGenericArgumentTypeIsNotSynTypePath);
                                            }
                                        }
                                    }
                                    else {
                                        return Err(Self::Error::SynPathArgumentsAngleBracketedFirstIsNotSynGenericArgumentType);
                                    },
                                    None => {
                                        return Err(Self::Error::SynPathArgumentsAngleBracketedFirstIsNone);
                                    }
                                }
                            },
                            _ => {
                                return Err(Self::Error::SynPathArgumentsIsNotAngleBracketed);
                            }
                        }
                    };
                    let supported_predefined_type = match path_segment.ident.to_string().as_str() {
                        "JsonStdPrimitiveI8" => Self::JsonStdPrimitiveI8,
                        "JsonStdPrimitiveI16" => Self::JsonStdPrimitiveI16,
                        "JsonStdPrimitiveI32" => Self::JsonStdPrimitiveI32,
                        "JsonStdPrimitiveI64" => Self::JsonStdPrimitiveI64,
                        "JsonStdPrimitiveI128" => Self::JsonStdPrimitiveI128,
                        "JsonStdPrimitiveU8" => Self::JsonStdPrimitiveU8,
                        "JsonStdPrimitiveU16" => Self::JsonStdPrimitiveU16,
                        "JsonStdPrimitiveU32" => Self::JsonStdPrimitiveU32,
                        "JsonStdPrimitiveU64" => Self::JsonStdPrimitiveU64,
                        "JsonStdPrimitiveU128" => Self::JsonStdPrimitiveU128,
                        "JsonStdPrimitiveF32" => Self::JsonStdPrimitiveF32,
                        "JsonStdPrimitiveF64" => Self::JsonStdPrimitiveF64,
                        "JsonStdPrimitiveBool" => Self::JsonStdPrimitiveBool,
                        "JsonStdStringString" => Self::JsonStdStringString,

                        "JsonStdOptionOptionStdPrimitiveI8" => Self::JsonStdOptionOptionStdPrimitiveI8,
                        "JsonStdOptionOptionStdPrimitiveI16" => Self::JsonStdOptionOptionStdPrimitiveI16,
                        "JsonStdOptionOptionStdPrimitiveI32" => Self::JsonStdOptionOptionStdPrimitiveI32,
                        "JsonStdOptionOptionStdPrimitiveI64" => Self::JsonStdOptionOptionStdPrimitiveI64,
                        "JsonStdOptionOptionStdPrimitiveI128" => Self::JsonStdOptionOptionStdPrimitiveI128,
                        "JsonStdOptionOptionStdPrimitiveU8" => Self::JsonStdOptionOptionStdPrimitiveU8,
                        "JsonStdOptionOptionStdPrimitiveU16" => Self::JsonStdOptionOptionStdPrimitiveU16,
                        "JsonStdOptionOptionStdPrimitiveU32" => Self::JsonStdOptionOptionStdPrimitiveU32,
                        "JsonStdOptionOptionStdPrimitiveU64" => Self::JsonStdOptionOptionStdPrimitiveU64,
                        "JsonStdOptionOptionStdPrimitiveU128" => Self::JsonStdOptionOptionStdPrimitiveU128,
                        "JsonStdOptionOptionStdPrimitiveF32" => Self::JsonStdOptionOptionStdPrimitiveF32,
                        "JsonStdOptionOptionStdPrimitiveF64" => Self::JsonStdOptionOptionStdPrimitiveF64,
                        "JsonStdOptionOptionStdPrimitiveBool" => Self::JsonStdOptionOptionStdPrimitiveBool,
                        "JsonStdOptionOptionStdStringString" => Self::JsonStdOptionOptionStdStringString,

                        // "JsonStdVecVecStdPrimitiveI8" => Self::JsonStdVecVecStdPrimitiveI8,
                        // "JsonStdVecVecStdPrimitiveI16" => Self::JsonStdVecVecStdPrimitiveI16,
                        // "JsonStdVecVecStdPrimitiveI32" => Self::JsonStdVecVecStdPrimitiveI32,
                        // "JsonStdVecVecStdPrimitiveI64" => Self::JsonStdVecVecStdPrimitiveI64,
                        // "JsonStdVecVecStdPrimitiveI128" => Self::JsonStdVecVecStdPrimitiveI128,
                        // "JsonStdVecVecStdPrimitiveU8" => Self::JsonStdVecVecStdPrimitiveU8,
                        // "JsonStdVecVecStdPrimitiveU16" => Self::JsonStdVecVecStdPrimitiveU16,
                        // "JsonStdVecVecStdPrimitiveU32" => Self::JsonStdVecVecStdPrimitiveU32,
                        // "JsonStdVecVecStdPrimitiveU64" => Self::JsonStdVecVecStdPrimitiveU64,
                        // "JsonStdVecVecStdPrimitiveU128" => Self::JsonStdVecVecStdPrimitiveU128,
                        // "JsonStdVecVecStdPrimitiveF32" => Self::JsonStdVecVecStdPrimitiveF32,
                        // "JsonStdVecVecStdPrimitiveF64" => Self::JsonStdVecVecStdPrimitiveF64,
                        // "JsonStdVecVecStdPrimitiveBool" => Self::JsonStdVecVecStdPrimitiveBool,
                        // "JsonStdVecVecStdStringString" => Self::JsonStdVecVecStdStringString,

                        // "JsonStdOptionOptionStdVecVecStdPrimitiveI8" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveI8,
                        // "JsonStdOptionOptionStdVecVecStdPrimitiveI16" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveI16,
                        // "JsonStdOptionOptionStdVecVecStdPrimitiveI32" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveI32,
                        // "JsonStdOptionOptionStdVecVecStdPrimitiveI64" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveI64,
                        // "JsonStdOptionOptionStdVecVecStdPrimitiveI128" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveI128,
                        // "JsonStdOptionOptionStdVecVecStdPrimitiveU8" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveU8,
                        // "JsonStdOptionOptionStdVecVecStdPrimitiveU16" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveU16,
                        // "JsonStdOptionOptionStdVecVecStdPrimitiveU32" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveU32,
                        // "JsonStdOptionOptionStdVecVecStdPrimitiveU64" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveU64,
                        // "JsonStdOptionOptionStdVecVecStdPrimitiveU128" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveU128,
                        // "JsonStdOptionOptionStdVecVecStdPrimitiveF32" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveF32,
                        // "JsonStdOptionOptionStdVecVecStdPrimitiveF64" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveF64,
                        // "JsonStdOptionOptionStdVecVecStdPrimitiveBool" => Self::JsonStdOptionOptionStdVecVecStdPrimitiveBool,
                        // "JsonStdOptionOptionStdVecVecStdStringString" => Self::JsonStdOptionOptionStdVecVecStdStringString,

                        // "JsonStdVecVecStdOptionOptionStdPrimitiveI8" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveI8,
                        // "JsonStdVecVecStdOptionOptionStdPrimitiveI16" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveI16,
                        // "JsonStdVecVecStdOptionOptionStdPrimitiveI32" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveI32,
                        // "JsonStdVecVecStdOptionOptionStdPrimitiveI64" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveI64,
                        // "JsonStdVecVecStdOptionOptionStdPrimitiveI128" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveI128,
                        // "JsonStdVecVecStdOptionOptionStdPrimitiveU8" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveU8,
                        // "JsonStdVecVecStdOptionOptionStdPrimitiveU16" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveU16,
                        // "JsonStdVecVecStdOptionOptionStdPrimitiveU32" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveU32,
                        // "JsonStdVecVecStdOptionOptionStdPrimitiveU64" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveU64,
                        // "JsonStdVecVecStdOptionOptionStdPrimitiveU128" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveU128,
                        // "JsonStdVecVecStdOptionOptionStdPrimitiveF32" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveF32,
                        // "JsonStdVecVecStdOptionOptionStdPrimitiveF64" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveF64,
                        // "JsonStdVecVecStdOptionOptionStdPrimitiveBool" => Self::JsonStdVecVecStdOptionOptionStdPrimitiveBool,
                        // "JsonStdVecVecStdOptionOptionStdStringString" => Self::JsonStdVecVecStdOptionOptionStdStringString,

                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128,
                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128,
                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
                        // "JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString,

                        "JsonGeneric" => Self::JsonGeneric(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)?),
                        "JsonStdOptionOptionGeneric" => Self::JsonStdOptionOptionGeneric(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)?),
                        "JsonStdVecVecGeneric" => Self::JsonStdVecVecGeneric(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)?),
                        "JsonStdOptionOptionStdVecVecGeneric" => Self::JsonStdOptionOptionStdVecVecGeneric(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)?),
                        "JsonStdVecVecStdOptionOptionGeneric" => Self::JsonStdVecVecStdOptionOptionGeneric(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)?),
                        "JsonStdOptionOptionStdVecVecStdOptionOptionGeneric" => Self::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(try_generate_generic_ident_upper_camel_case_token_stream(&path_segment)?),

                        "JsonUuid" => Self::JsonUuid,
                        _ => {
                            return Err(Self::Error::UnsupportedPredefinedTypeWrapper);
                        }
                    };
                    match &supported_predefined_type {
                        SupportedPredefinedType::JsonStdPrimitiveI8 |
                        SupportedPredefinedType::JsonStdPrimitiveI16 |
                        SupportedPredefinedType::JsonStdPrimitiveI32 |
                        SupportedPredefinedType::JsonStdPrimitiveI64 |
                        SupportedPredefinedType::JsonStdPrimitiveI128 |
                        SupportedPredefinedType::JsonStdPrimitiveU8 |
                        SupportedPredefinedType::JsonStdPrimitiveU16 |
                        SupportedPredefinedType::JsonStdPrimitiveU32 |
                        SupportedPredefinedType::JsonStdPrimitiveU64 |
                        SupportedPredefinedType::JsonStdPrimitiveU128 |
                        SupportedPredefinedType::JsonStdPrimitiveF32 |
                        SupportedPredefinedType::JsonStdPrimitiveF64 |
                        SupportedPredefinedType::JsonStdPrimitiveBool |
                        SupportedPredefinedType::JsonStdStringString |

                        SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                        SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                        SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                        SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                        SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                        SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                        SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                        SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                        SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                        SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                        SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                        SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                        SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                        SupportedPredefinedType::JsonStdOptionOptionStdStringString |

                        // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                        // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                        // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                        // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                        // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                        // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                        // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                        // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                        // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                        // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                        // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                        // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                        // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                        // SupportedPredefinedType::JsonStdVecVecStdStringString |

                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString |

                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                        // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString |

                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                        // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString |

                        SupportedPredefinedType::JsonUuid
                        => {
                            match path_segment.arguments{
                                syn::PathArguments::None => (),
                                _ => {
                                    return Err(Self::Error::PathSegmentArgumentsIsNotSynPathArgumentsNone);
                                },
                            }
                        }
                        _ => (),
                    }
                    Ok(supported_predefined_type)
                }
                None => Err(Self::Error::TypePathPathSegmentsLastIsNone),
            }
            _ => Err(Self::Error::TypePathPathSegmentsIsNotSynTypePath)
        }
    }
}
enum PrimitiveJsonType {
    Boolean,
    Number,
    String,
    Array,
    Object,
    Null,
}
impl std::fmt::Display for PrimitiveJsonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean => write!(f, "boolean"),
            Self::Number => write!(f, "number"),
            Self::String => write!(f, "string"),
            Self::Array => write!(f, "array"),
            Self::Object => write!(f, "object"),
            Self::Null => write!(f, "null"),
        }
    }
}
//todo maybe generate example of valid json to create - maybe with serde_json::to_string() adn #[derive(Default)} then println or write into file 
//todo maybe in many few dimantional array error message would be wrong. test it
//todo generate authorization rights enum for json fields
#[proc_macro_derive(GeneratePostgresqlQueryPart)]
pub fn generate_postgresql_query_part(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "GeneratePostgresqlQueryPart";
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    // println!("{:#?}", syn_derive_input.data);
    let ident = &syn_derive_input.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let vec_syn_field = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            fields_named.named.iter().map(|element|element).collect::<std::vec::Vec<&syn::Field>>()
        } else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Named");
        }
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    // println!("{vec_syn_field:#?}");
    let id_snake_case = naming_conventions::IdSnakeCase;
    {
        let mut is_id_field_exists = false;
        for element in &vec_syn_field {
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            if element_ident == &id_snake_case.to_string() {
                if let SupportedPredefinedType::JsonUuid = SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}")) {
                    is_id_field_exists = true;
                    break;
                }
                else {
                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} field {id_snake_case} is not SupportedPredefinedType::JsonUuid");
                };
            }
        }
        if !is_id_field_exists {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} field {id_snake_case} does not exist");
        }
    }
    let vec_syn_field_filtered_id_iter = vec_syn_field.iter().filter(|element|{
        let element_ident = element.ident.as_ref().unwrap_or_else(|| {
            panic!(
                "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                naming_conventions::FIELD_IDENT_IS_NONE
            );
        });
        element_ident != &id_snake_case.to_string()
    }).collect::<std::vec::Vec<&&syn::Field>>();
    let generate_ident_field_to_read_upper_camel_case_token_stream = |value: &std::primitive::str|{
        let value = format!("{value}{}", naming_conventions::FieldToReadUpperCamelCase);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generate_ident_options_to_read_upper_camel_case_token_stream = |value: &std::primitive::str|{
        let value = format!("{value}{}", naming_conventions::OptionsToReadUpperCamelCase);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_field_to_read_upper_camel_case_token_stream = generate_ident_field_to_read_upper_camel_case_token_stream(&ident.to_string());
    let ident_options_to_read_upper_camel_case_stringified = format!("{ident}{}", naming_conventions::OptionsToReadUpperCamelCase);
    let ident_options_to_read_upper_camel_case_token_stream = {
        ident_options_to_read_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_options_to_read_upper_camel_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_options_to_read_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
        &ident_options_to_read_upper_camel_case_stringified,
        &proc_macro_name_upper_camel_case_ident_stringified
    );
    let ident_reader_upper_camel_case_stringified = format!("{ident}{}", naming_conventions::ReaderUpperCamelCase);
    let ident_reader_upper_camel_case_token_stream = {
        ident_reader_upper_camel_case_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {ident_reader_upper_camel_case_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_option_to_update_upper_camel_case_token_stream = {
        let value = format!("{ident}{}", naming_conventions::OptionToUpdateUpperCamelCase);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_options_to_update_upper_camel_case_token_stream = {
        let value = format!("{ident}{}", naming_conventions::OptionsToUpdateUpperCamelCase);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_field_to_update_upper_camel_case_token_stream = {
        let value = format!("{ident}{}", naming_conventions::FieldToUpdateUpperCamelCase);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream = {
        let value = format!("{ident}{}", naming_conventions::OptionsToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_to_create_upper_camel_case_token_stream = naming_conventions::tokens_to_create_upper_camel_case_token_stream(&ident);
    let ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream = {
        let value = format!("{ident}{}", naming_conventions::GeneratePostgresqlQueryPartToReadErrorNamedUpperCamelCase);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let add_postfix_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_stringified = |value: &std::primitive::str|{
        format!("{value}{}", naming_conventions::GeneratePostgresqlQueryPartToReadFromSelfVecErrorNamedUpperCamelCase)
    };
    let ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream = {
        let value = add_postfix_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_stringified(&ident.to_string());
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let add_postfix_generate_postgresql_query_part_from_self_vec_upper_camel_case_stringified = |value: &std::primitive::str|{
        format!("{value}{}", naming_conventions::GeneratePostgresqlQueryPartToReadFromSelfVecUpperCamelCase)
    };
    let generate_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = |value: &std::primitive::str|{
        let value = add_postfix_generate_postgresql_query_part_from_self_vec_upper_camel_case_stringified(value);
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generate_ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream = |value: &syn::Field|{
        let value = format!(
            "{}OffsetPlusLimitIsIntOverflow",
            proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&value.ident.as_ref().unwrap_or_else(|| {
               panic!(
                   "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                   naming_conventions::FIELD_IDENT_IS_NONE
               );
            }).to_string()),
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generate_field_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = |value: &std::primitive::str|{
        let value = add_postfix_generate_postgresql_query_part_from_self_vec_upper_camel_case_stringified(
            &proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&value)
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let generate_field_ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream = |value: &std::primitive::str|{
        let value = add_postfix_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_stringified(&proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&value));
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let update_snake_case = naming_conventions::UpdateSnakeCase;
    let uuid_uuid_token_stream = quote::quote!{uuid::Uuid};
    let offset_plus_limit_is_int_overflow_variants_token_stream = vec_syn_field.iter().fold(vec![], |mut acc, element| {
        let ident_offset_plus_limit_is_int_overflow_token_stream = {
            let ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream = generate_ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream(&element);
            quote::quote!{
                #ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream {
                    #[eo_to_std_string_string_serialize_deserialize]
                    limit: std::primitive::u64,
                    #[eo_to_std_string_string_serialize_deserialize]
                    offset: std::primitive::u64,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
            }
        };
        let supported_predefined_type = SupportedPredefinedType::try_from(*element)
            .unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
        match supported_predefined_type {
            SupportedPredefinedType::JsonStdPrimitiveI8 |
            SupportedPredefinedType::JsonStdPrimitiveI16 |
            SupportedPredefinedType::JsonStdPrimitiveI32 |
            SupportedPredefinedType::JsonStdPrimitiveI64 |
            SupportedPredefinedType::JsonStdPrimitiveI128 |
            SupportedPredefinedType::JsonStdPrimitiveU8 |
            SupportedPredefinedType::JsonStdPrimitiveU16 |
            SupportedPredefinedType::JsonStdPrimitiveU32 |
            SupportedPredefinedType::JsonStdPrimitiveU64 |
            SupportedPredefinedType::JsonStdPrimitiveU128 |
            SupportedPredefinedType::JsonStdPrimitiveF32 |
            SupportedPredefinedType::JsonStdPrimitiveF64 |
            SupportedPredefinedType::JsonStdPrimitiveBool |
            SupportedPredefinedType::JsonStdStringString |

            SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
            SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
            SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
            SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
            SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
            SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
            SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
            SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
            SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
            SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
            SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
            SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
            SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
            SupportedPredefinedType::JsonStdOptionOptionStdStringString
            => (),

            // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
            // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
            // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
            // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
            // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
            // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
            // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
            // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
            // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
            // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
            // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
            // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
            // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
            // SupportedPredefinedType::JsonStdVecVecStdStringString |

            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString |

            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
            // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString |

            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
            // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString
            // => {
            //     acc.push(ident_offset_plus_limit_is_int_overflow_token_stream);
            // },

            SupportedPredefinedType::JsonGeneric(_) |
            SupportedPredefinedType::JsonStdOptionOptionGeneric(_) => (),
            SupportedPredefinedType::JsonStdVecVecGeneric(_) |
            SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(_) |
            SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(_) |
            SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(_) => {
                acc.push(ident_offset_plus_limit_is_int_overflow_token_stream);
            },

            SupportedPredefinedType::JsonUuid => (),
        };
        acc
    });
    let field_ident_generate_postgresql_query_part_from_self_vec_variants_token_stream = {
        let unique_type_path = vec_syn_field.iter().fold(vec![], |mut acc, element| {
            let supported_predefined_type = SupportedPredefinedType::try_from(*element)
                .unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
            let option_type_path = match supported_predefined_type {
                SupportedPredefinedType::JsonStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdPrimitiveBool |
                SupportedPredefinedType::JsonStdStringString |
    
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::JsonStdOptionOptionStdStringString
                => None,
    
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdStringString |
    
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString |
    
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString |
    
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString 
                // => None,
    
                SupportedPredefinedType::JsonGeneric(type_path) => Some(type_path),
                SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => Some(type_path),
                SupportedPredefinedType::JsonStdVecVecGeneric(type_path) => Some(type_path),
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(type_path) => Some(type_path),
                SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(type_path) => Some(type_path),
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => Some(type_path),

                SupportedPredefinedType::JsonUuid => None,
            };
            if let Some(value) = option_type_path {
                if !acc.contains(&value) {
                    acc.push(value)
                }
            }
            acc
        });
        unique_type_path.iter().fold(vec![], |mut acc, element| {
            acc.push({
                let field_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = generate_field_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream(&quote::quote!{#element}.to_string());
                let field_ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream = generate_field_ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream(&quote::quote!{#element}.to_string());
                quote::quote!{
                    #field_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream {
                        #[eo_error_occurence]
                        field: #field_ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    }
                }
            });
            acc
        })
    };
    let should_generate_ident_generate_postgresql_query_part_error_named_enum = match (offset_plus_limit_is_int_overflow_variants_token_stream.is_empty(),field_ident_generate_postgresql_query_part_from_self_vec_variants_token_stream.is_empty()) {
        (true,true) => false,
        (true,false) => true,
        (false,true) => true,
        (false,false) =>  true,
    };
    let impl_std_fmt_display_for_ident_token_stream = {
        quote::quote!{
            impl std::fmt::Display for #ident {
                fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(formatter, "{:?}", &self)
                }
            }
        }
    };
    let pub_enum_ident_field_token_stream = {
        let variants_token_stream = vec_syn_field.iter().map(|element|{
            let field_ident_stringified = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            }).to_string();
            let serialize_deserialize_field_ident_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
                &field_ident_stringified,
                &proc_macro_name_upper_camel_case_ident_stringified,
            );
            let variant_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&field_ident_stringified);
            let maybe_additional_token_stream = {
                let supported_predefined_type = SupportedPredefinedType::try_from(*element)
                    .unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
                let generate_std_vec_vec_generic_ident_field_token_stream = |type_path: &syn::TypePath|{
                    let generic_ident_field_to_read_upper_camel_case_token_stream = generate_ident_field_to_read_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        (std::vec::Vec<#generic_ident_field_to_read_upper_camel_case_token_stream>)
                    }
                };
                let generate_field_vec_limit_offset_token_stream = |type_path: &syn::TypePath|{
                    let generic_ident_field_to_read_upper_camel_case_token_stream = generate_ident_field_to_read_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        {
                            field_vec: std::vec::Vec<#generic_ident_field_to_read_upper_camel_case_token_stream>,
                            limit: std::primitive::u64,
                            offset: std::primitive::u64,
                        }
                    }
                };
                match supported_predefined_type {
                    SupportedPredefinedType::JsonStdPrimitiveI8 |
                    SupportedPredefinedType::JsonStdPrimitiveI16 |
                    SupportedPredefinedType::JsonStdPrimitiveI32 |
                    SupportedPredefinedType::JsonStdPrimitiveI64 |
                    SupportedPredefinedType::JsonStdPrimitiveI128 |
                    SupportedPredefinedType::JsonStdPrimitiveU8 |
                    SupportedPredefinedType::JsonStdPrimitiveU16 |
                    SupportedPredefinedType::JsonStdPrimitiveU32 |
                    SupportedPredefinedType::JsonStdPrimitiveU64 |
                    SupportedPredefinedType::JsonStdPrimitiveU128 |
                    SupportedPredefinedType::JsonStdPrimitiveF32 |
                    SupportedPredefinedType::JsonStdPrimitiveF64 |
                    SupportedPredefinedType::JsonStdPrimitiveBool |
                    SupportedPredefinedType::JsonStdStringString |

                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                    SupportedPredefinedType::JsonStdOptionOptionStdStringString
                    => proc_macro2::TokenStream::new(),

                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                    // SupportedPredefinedType::JsonStdVecVecStdStringString |

                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString |

                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString |

                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString
                    // => quote::quote!{
                    //     {
                    //         limit: std::primitive::u64,
                    //         offset: std::primitive::u64,
                    //     }
                    // },

                    SupportedPredefinedType::JsonGeneric(type_path) => generate_std_vec_vec_generic_ident_field_token_stream(&type_path),
                    SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => generate_std_vec_vec_generic_ident_field_token_stream(&type_path),
                    SupportedPredefinedType::JsonStdVecVecGeneric(type_path) => generate_field_vec_limit_offset_token_stream(&type_path),
                    SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(type_path) => generate_field_vec_limit_offset_token_stream(&type_path),
                    SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(type_path) => generate_field_vec_limit_offset_token_stream(&type_path),
                    SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => generate_field_vec_limit_offset_token_stream(&type_path),

                    SupportedPredefinedType::JsonUuid => proc_macro2::TokenStream::new(),
                }
            };
            quote::quote!{
                #[serde(rename(
                   serialize = #serialize_deserialize_field_ident_double_quotes_token_stream,
                   deserialize = #serialize_deserialize_field_ident_double_quotes_token_stream
                ))]
                #variant_ident_upper_camel_case_token_stream #maybe_additional_token_stream
            }
        });
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]//Eq, 
            pub enum #ident_field_to_read_upper_camel_case_token_stream {
                #(#variants_token_stream),*
            }
        }
    };
    let impl_error_occurence_lib_to_std_string_string_for_ident_field_token_stream = {
        quote::quote!{
            impl error_occurence_lib::ToStdStringString for #ident_field_to_read_upper_camel_case_token_stream {
                fn to_std_string_string(&self) -> std::string::String {
                    format!("{self:?}")
                }
            }
        }
    };
    let pub_enum_field_generate_postgresql_query_part_error_named_token_stream = {
        let ident_generate_postgresql_query_part_from_self_vec_error_named_token_stream = {
            let maybe_generate_postgresql_query_part_variant_token_stream = match should_generate_ident_generate_postgresql_query_part_error_named_enum {
                true => quote::quote!{
                    GeneratePostgresqlQueryPart {
                        #[eo_error_occurence]
                        error: #ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                },
                false => proc_macro2::TokenStream::new()
            };
            quote::quote!{
                #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                pub enum #ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream {
                    FieldsFilterIsEmpty {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    NotUniqueFieldFilter {
                        #[eo_to_std_string_string_serialize_deserialize]
                        field: #ident_field_to_read_upper_camel_case_token_stream,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                    },
                    #maybe_generate_postgresql_query_part_variant_token_stream
                }
            }
        };
        let impl_error_occurence_lib_to_std_string_string_for_ident_generate_postgresql_query_part_from_self_vec_error_named_token_stream = {
            quote::quote!{
                impl error_occurence_lib::ToStdStringString for #ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream {
                    fn to_std_string_string(&self) -> std::string::String {
                        format!("{self:?}")
                    }
                }
            }
        };
        let ident_generate_postgresql_query_part_error_named_token_stream = {
            match should_generate_ident_generate_postgresql_query_part_error_named_enum {
                true => {
                    quote::quote!{
                        #[derive(Debug, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
                        pub enum #ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream {
                            #(#offset_plus_limit_is_int_overflow_variants_token_stream)*
                            #(#field_ident_generate_postgresql_query_part_from_self_vec_variants_token_stream),*
                        }
                    }
                },
                false => proc_macro2::TokenStream::new()
            }
        };
        quote::quote!{
            #ident_generate_postgresql_query_part_from_self_vec_error_named_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_ident_generate_postgresql_query_part_from_self_vec_error_named_token_stream
            #ident_generate_postgresql_query_part_error_named_token_stream
        }
    };
    let impl_generate_postgresql_query_part_to_read_for_ident_field_token_stream = {
        let type_of_space_stringified = "type of ";
        let space_and_not_null_stringified = " and not null";
        let space_is_not_space_stringified = " is not ";
        let wrap_into_jsonb_object_build = |value: &std::primitive::str|{
            format!("jsonb_build_object({value})")
        };
        let wrap_into_jsonb_build_object_ok_stringified = |value: &std::primitive::str|{
            wrap_into_jsonb_object_build(&format!("'Ok',{value}"))
        };
        let wrap_into_jsonb_typeof_stringified = |value: &std::primitive::str|{
            format!("jsonb_typeof({value})")
        };
        let wrap_into_when_space_value_space_equals_space_null_stringified = |value: &std::primitive::str, primitive_json_type: &PrimitiveJsonType|{
            format!("when {value} = '{primitive_json_type}'")
        };
        let add_then_space_prefix_stringified = |value: &std::primitive::str|{
            format!("then {value}")
        };
        let generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified = |value: &std::primitive::str|{
            let add_then_space_prefix_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&wrap_into_jsonb_build_object_ok_stringified("null"));
            let wraped_into_when_space_value_space_equals_space_null_stringified = wrap_into_when_space_value_space_equals_space_null_stringified(
                &wrap_into_jsonb_typeof_stringified(value),
                &PrimitiveJsonType::Null
            );
            format!("{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_wraped_into_jsonb_build_object_ok_stringified}")
        };
        let generate_space_else_space_jsonb_build_object_err_stringified = |value: &std::primitive::str|{
            let wraped_into_jsonb_object_build = wrap_into_jsonb_object_build(&format!("'Err','{value}'"));
            format!(" else jsonb_build_object({wraped_into_jsonb_object_build})")
        };
        let generate_vec_wrong_type_error_message_stringified = |is_optional: std::primitive::bool, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str|{
            format!(
                "{type_of_space_stringified}{column_name_and_maybe_field_getter_for_error_message}{space_is_not_space_stringified}array{}",
                match is_optional {
                    true => space_and_not_null_stringified,
                    false => ""
                }
            )
        };
        let wrap_into_case_end_stringified = |value: &std::primitive::str|{
            format!("case {value} end")
        };
        let wraped_into_jsonb_typeof_stringified = wrap_into_jsonb_typeof_stringified("value");
        let wraped_into_when_space_value_space_equals_space_null_wraped_into_jsonb_typeof_stringified = |json_type: &PrimitiveJsonType|{
            wrap_into_when_space_value_space_equals_space_null_stringified(
                &wraped_into_jsonb_typeof_stringified,
                &json_type
            )
        };
        let generate_postgresql_query_part_content = |match_value_token_stream: &proc_macro2::TokenStream, wrap_in_ok_token_stream: std::primitive::bool|{
            let generate_postgresql_query_part_match_variants_token_stream = vec_syn_field.iter().map(|element|{
                let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                   panic!(
                       "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                       naming_conventions::FIELD_IDENT_IS_NONE
                   );
                });
                let el_ident_str = element_ident.to_string();
                let element_ident_upper_camel_case_token_stream = {
                    let value = proc_macro_common::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&el_ident_str);
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                };
                let gen_maybe_wrap_in_ok_token_stream = |value: &proc_macro2::TokenStream|{
                    match wrap_in_ok_token_stream {
                        true => quote::quote!{Ok(#value)},
                        false => quote::quote!{#value}
                    }
                };
                let gen_simple_types_token_stream = |query_string_token_stream: &proc_macro2::TokenStream|{
                    gen_maybe_wrap_in_ok_token_stream(&quote::quote!{format!(#query_string_token_stream)})
                };
                let gen_vec_simple_types_token_stream = |query_string_token_stream: &proc_macro2::TokenStream|{
                    let ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream = generate_ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream(&element);
                    let handle_return_token_stream = gen_maybe_wrap_in_ok_token_stream(&quote::quote!{format!(#query_string_token_stream)});
                    quote::quote!{
                        {
                            let start = offset;
                            let end = match offset.checked_add(*limit) {
                                Some(value) => value,
                                None => {
                                    return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::#ident_offset_plus_limit_is_int_overflow_upper_camel_case_token_stream {
                                        limit: *limit,
                                        offset: *offset,
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    });
                                }
                            };
                            #handle_return_token_stream
                        }
                    }
                };
                let column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified = format!("{{column_name_and_maybe_field_getter_for_error_message}}.{el_ident_str}");
                let generate_wrong_type_error_message_stringified = |is_optional: std::primitive::bool, json_type: &PrimitiveJsonType|{
                    format!(
                        "{type_of_space_stringified}{column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified}{space_is_not_space_stringified}{json_type}{}",
                        match is_optional {
                            true => " and not null",
                            false => ""
                        }
                    )
                };
                let array_element_stringified = "[array element]";
                let generate_vec_element_wrong_type_error_message_stringified = |is_optional: std::primitive::bool, json_type: &PrimitiveJsonType|{
                    format!(
                        "{type_of_space_stringified}{column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified}{array_element_stringified}{space_is_not_space_stringified}{json_type}{}",
                        match is_optional {
                            true => space_and_not_null_stringified,
                            false => ""
                        }
                    )
                };
                let el_ident_str_single_quotes_stringified = format!("'{el_ident_str}'");
                let column_name_and_maybe_field_getter_el_ident_str_stringified = format!("{{column_name_and_maybe_field_getter}}->{el_ident_str_single_quotes_stringified}");
                let column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified = wrap_into_jsonb_typeof_stringified(&column_name_and_maybe_field_getter_el_ident_str_stringified);
                let add_el_ident_str_comma_prefix_stringified = |value: &std::primitive::str|{
                    format!("{el_ident_str_single_quotes_stringified},{value}")
                };
                let wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified = |json_type: &PrimitiveJsonType|{
                    wrap_into_when_space_value_space_equals_space_null_stringified(
                        &column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified,
                        json_type
                    )
                };
                let add_then_space_prefix_wraped_into_jsonb_build_object_ok_column_name_and_maybe_field_getter_el_ident_str_stringified = add_then_space_prefix_stringified(&wrap_into_jsonb_build_object_ok_stringified(&column_name_and_maybe_field_getter_el_ident_str_stringified));
                let generate_simple_json_type = |json_type: PrimitiveJsonType|{
                    let query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &wrap_into_jsonb_object_build(&{
                            let space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_wrong_type_error_message_stringified(false, &json_type));
                            let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&json_type);
                            add_el_ident_str_comma_prefix_stringified(&format!(
                                "{} ",
                                wrap_into_case_end_stringified(&format!(
                                    "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_wraped_into_jsonb_build_object_ok_column_name_and_maybe_field_getter_el_ident_str_stringified}{space_else_space_jsonb_build_object_err_stringified}"
                                )) 
                            ))
                        }),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    gen_simple_types_token_stream(&query_string_token_stream)
                };
                let generate_optional_simple_json_type = |json_type: PrimitiveJsonType|{
                    let query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &wrap_into_jsonb_object_build(&{
                            let space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_wrong_type_error_message_stringified(true, &json_type));
                            let when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified = generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified(&column_name_and_maybe_field_getter_el_ident_str_stringified);
                            let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&json_type);
                            add_el_ident_str_comma_prefix_stringified(&format!(
                                "{} ",
                                wrap_into_case_end_stringified(&format!(
                                    "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_wraped_into_jsonb_build_object_ok_column_name_and_maybe_field_getter_el_ident_str_stringified} {when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified}{space_else_space_jsonb_build_object_err_stringified}"
                                ))
                            ))
                        }),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    gen_simple_types_token_stream(&query_string_token_stream)
                };
                let generate_jsonb_array_elements_stringified = |value: &std::primitive::str|{
                    format!("jsonb_array_elements({value})")
                };
                let jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified = generate_jsonb_array_elements_stringified(&format!("(select {column_name_and_maybe_field_getter_el_ident_str_stringified})"));
                let jsonb_build_object_ok_value_stringified = wrap_into_jsonb_build_object_ok_stringified("value");
                let space_with_ordinality_where_ordinality_between_start_and_end_stringified = " with ordinality where ordinality between {start} and {end}";
                let wrap_into_jsonb_agg_stringified = |value: &std::primitive::str|{
                    format!("jsonb_agg({value})")
                };
                let generate_select_from_stringified = |select_value: &std::primitive::str, from_value: &std::primitive::str|{
                    format!("select {select_value} from {from_value}")
                };
                let wrap_into_scopes_stringified = |value: &std::primitive::str|{
                    format!("({value})")
                };
                let jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified_space_with_ordinality_where_ordinality_between_start_and_end_stringified = format!(
                    "{jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified}{space_with_ordinality_where_ordinality_between_start_and_end_stringified}"
                );
                let add_then_space_prefix_jsonb_build_object_ok_value_stringified = add_then_space_prefix_stringified(&jsonb_build_object_ok_value_stringified);
                let generate_vec_simple_json_type = |json_type: PrimitiveJsonType|{
                    gen_vec_simple_types_token_stream(&proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &wrap_into_jsonb_object_build(&{
                            let vec_wraped_into_jsonb_build_object_ok_stringified = wrap_into_jsonb_build_object_ok_stringified(&{
                                let vec_element_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_element_wrong_type_error_message_stringified(false, &json_type));
                                let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_wraped_into_jsonb_typeof_stringified(&json_type);
                                let wraped_into_jsonb_agg_stringified = wrap_into_jsonb_agg_stringified(&
                                    wrap_into_case_end_stringified(&format!("{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_jsonb_build_object_ok_value_stringified}{vec_element_space_else_space_jsonb_build_object_err_stringified}"))
                                );
                                wrap_into_scopes_stringified(&generate_select_from_stringified(
                                    &wraped_into_jsonb_agg_stringified,
                                    &jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified_space_with_ordinality_where_ordinality_between_start_and_end_stringified
                                ))
                            });
                            let vec_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_wrong_type_error_message_stringified(false, &column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified));
                            let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&PrimitiveJsonType::Array);
                            let add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&vec_wraped_into_jsonb_build_object_ok_stringified);
                            add_el_ident_str_comma_prefix_stringified(&wrap_into_case_end_stringified(&format!(
                                "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified}{vec_space_else_space_jsonb_build_object_err_stringified}"
                            )))
                        }),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ))
                };
                let generate_optional_vec_simple_json_type = |json_type: PrimitiveJsonType|{
                    gen_vec_simple_types_token_stream(&proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &wrap_into_jsonb_object_build(&{
                            let vec_wraped_into_jsonb_build_object_ok_stringified = wrap_into_jsonb_build_object_ok_stringified(&{
                                let vec_element_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_element_wrong_type_error_message_stringified(false, &json_type));
                                let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_wraped_into_jsonb_typeof_stringified(&json_type);
                                let wraped_into_jsonb_agg_stringified = wrap_into_jsonb_agg_stringified(&wrap_into_case_end_stringified(
                                    &format!("{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_jsonb_build_object_ok_value_stringified}{vec_element_space_else_space_jsonb_build_object_err_stringified}")
                                ));
                                wrap_into_scopes_stringified(&generate_select_from_stringified(
                                    &wraped_into_jsonb_agg_stringified,
                                    &jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified_space_with_ordinality_where_ordinality_between_start_and_end_stringified
                                ))
                            });
                            let vec_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_wrong_type_error_message_stringified(true, &column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified));
                            let when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified = generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified(&column_name_and_maybe_field_getter_el_ident_str_stringified);
                            let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&PrimitiveJsonType::Array);
                            let add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&vec_wraped_into_jsonb_build_object_ok_stringified);
                            add_el_ident_str_comma_prefix_stringified(&wrap_into_case_end_stringified(&format!(
                                "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified} {when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified}{vec_space_else_space_jsonb_build_object_err_stringified}"
                            )))
                        }),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ))
                };
                let generate_vec_optional_simple_json_type = |json_type: PrimitiveJsonType|{
                    gen_vec_simple_types_token_stream(&proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &wrap_into_jsonb_object_build(&{
                            let vec_wraped_into_jsonb_build_object_ok_stringified = wrap_into_jsonb_build_object_ok_stringified(&{
                                let when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified = generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified("value");
                                let vec_element_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_element_wrong_type_error_message_stringified(true, &json_type));
                                let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_wraped_into_jsonb_typeof_stringified(&json_type);
                                let wraped_into_jsonb_agg_stringified = wrap_into_jsonb_agg_stringified(
                                    &wrap_into_case_end_stringified(
                                        &format!("{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_jsonb_build_object_ok_value_stringified} {when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified}{vec_element_space_else_space_jsonb_build_object_err_stringified}")
                                    )
                                );
                                wrap_into_scopes_stringified(&generate_select_from_stringified(
                                    &wraped_into_jsonb_agg_stringified,
                                    &jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified_space_with_ordinality_where_ordinality_between_start_and_end_stringified
                                ))
                            });
                            let vec_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_wrong_type_error_message_stringified(false, &column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified));
                            let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&PrimitiveJsonType::Array);
                            let add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&vec_wraped_into_jsonb_build_object_ok_stringified);
                            add_el_ident_str_comma_prefix_stringified(
                                &wrap_into_case_end_stringified(
                                    &format!("{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified}{vec_space_else_space_jsonb_build_object_err_stringified}")
                                )
                            )
                        }),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ))
                };
                let generate_optional_vec_optional_simple_json_type = |json_type: PrimitiveJsonType|{
                    gen_vec_simple_types_token_stream(&proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &wrap_into_jsonb_object_build(&{
                            let vec_wraped_into_jsonb_build_object_ok_stringified = wrap_into_jsonb_build_object_ok_stringified(&{
                                let when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_vec_element_stringified = generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified("value");
                                let vec_element_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_element_wrong_type_error_message_stringified(true, &json_type));
                                let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_wraped_into_jsonb_typeof_stringified(&json_type);
                                let wraped_into_jsonb_agg_stringified = wrap_into_jsonb_agg_stringified(
                                    &wrap_into_case_end_stringified(&format!(
                                        "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_jsonb_build_object_ok_value_stringified} {when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_vec_element_stringified}{vec_element_space_else_space_jsonb_build_object_err_stringified}"
                                    ))
                                );
                                wrap_into_scopes_stringified(&generate_select_from_stringified(
                                    &wraped_into_jsonb_agg_stringified,
                                    &jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified_space_with_ordinality_where_ordinality_between_start_and_end_stringified
                                ))
                            });
                            let vec_space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_wrong_type_error_message_stringified(true, &column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified));
                            let when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_vec_stringified = generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified(&column_name_and_maybe_field_getter_el_ident_str_stringified);
                            let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&PrimitiveJsonType::Array);
                            let add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&vec_wraped_into_jsonb_build_object_ok_stringified);
                            add_el_ident_str_comma_prefix_stringified(
                                &wrap_into_case_end_stringified(&format!(
                                    "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified}{when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_vec_stringified}{vec_space_else_space_jsonb_build_object_err_stringified}"
                                ))
                            )
                        }),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ))
                };
                let generic_and_std_option_option_generic_logic_token_stream = |type_path: &syn::TypePath, is_optional: std::primitive::bool|{
                    let column_name_and_maybe_field_getter_query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &column_name_and_maybe_field_getter_el_ident_str_stringified,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    let column_name_and_maybe_field_getter_for_error_message_query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified,
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    let simple_types_token_stream = gen_simple_types_token_stream(&proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &wrap_into_jsonb_object_build(&add_el_ident_str_comma_prefix_stringified("{value}")),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    let ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = generate_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    let is_optional_token_stream = if is_optional {
                        quote::quote!{true}
                    }
                    else {
                        quote::quote!{false}
                    };
                    quote::quote!{
                        (fields_vec) => match postgresql_crud::GeneratePostgresqlQueryPartToRead::generate_postgresql_query_part_to_read_from_self_vec(
                            fields_vec,
                            &format!(#column_name_and_maybe_field_getter_query_string_token_stream),
                            &format!(#column_name_and_maybe_field_getter_for_error_message_query_string_token_stream),
                            #is_optional_token_stream
                        ) {
                            Ok(value) => #simple_types_token_stream,
                            Err(error) => {
                                return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::#ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream  {
                                    field: error,
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        }
                    }
                };
                let maybe_std_option_option_std_vec_vec_maybe_std_option_option_generic_logic_token_stream = |
                    type_path: &syn::TypePath, 
                    is_optional: std::primitive::bool,
                    is_std_vec_vec_optional: std::primitive::bool,
                |{
                    let column_name_and_maybe_field_getter_for_error_message_query_string_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!("{column_name_and_maybe_field_getter_for_error_message_el_ident_str_stringified}{array_element_stringified}"),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    let vec_simple_types_token_stream = gen_vec_simple_types_token_stream(&proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &wrap_into_jsonb_object_build(&{
                            let vec_wraped_into_jsonb_build_object_ok_stringified = wrap_into_jsonb_build_object_ok_stringified(&{
                                let wraped_into_jsonb_agg_stringified = wrap_into_jsonb_agg_stringified("{value}");
                                wrap_into_scopes_stringified(&generate_select_from_stringified(
                                    &wraped_into_jsonb_agg_stringified,
                                    &jsonb_array_elements_select_column_name_and_maybe_field_getter_el_ident_str_stringified_space_with_ordinality_where_ordinality_between_start_and_end_stringified
                                ))
                            });
                            let maybe_check_on_null_stringified = if is_std_vec_vec_optional {
                                generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified(&column_name_and_maybe_field_getter_el_ident_str_stringified)
                            }
                            else {
                                std::string::String::default()
                            };
                            let space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&generate_vec_wrong_type_error_message_stringified(is_std_vec_vec_optional, "{{column_name_and_maybe_field_getter_for_error_message}}"));
                            let wraped_into_when_space_value_space_equals_space_null_stringified = wraped_into_when_space_value_space_equals_space_null_column_name_and_maybe_field_getter_el_ident_str_wraped_into_jsonb_typeof_stringified(&PrimitiveJsonType::Array);
                            let add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&vec_wraped_into_jsonb_build_object_ok_stringified);
                            add_el_ident_str_comma_prefix_stringified(&wrap_into_case_end_stringified(&format!(
                                "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_vec_wraped_into_jsonb_build_object_ok_stringified}{maybe_check_on_null_stringified}{space_else_space_jsonb_build_object_err_stringified}"
                            )))
                        }),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    ));
                    let ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream = generate_ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    let is_optional_token_stream = if is_optional {
                        quote::quote!{true}
                    }
                    else {
                        quote::quote!{false}
                    };
                    quote::quote!{
                        {
                            field_vec,
                            limit,
                            offset
                        } => match postgresql_crud::GeneratePostgresqlQueryPartToRead::generate_postgresql_query_part_to_read_from_self_vec(
                            field_vec,
                            &format!("value"),
                            &format!(#column_name_and_maybe_field_getter_for_error_message_query_string_token_stream),
                            #is_optional_token_stream
                        ) {
                            Ok(value) => #vec_simple_types_token_stream,
                            Err(error) => {
                                return Err(#ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream::#ident_generate_postgresql_query_part_from_self_vec_upper_camel_case_token_stream {
                                    field: error,
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        }
                    }
                };
                let variant_logic_token_stream = match SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}")) 
                {
                    SupportedPredefinedType::JsonStdPrimitiveI8 |
                    SupportedPredefinedType::JsonStdPrimitiveI16 |
                    SupportedPredefinedType::JsonStdPrimitiveI32 |
                    SupportedPredefinedType::JsonStdPrimitiveI64 |
                    SupportedPredefinedType::JsonStdPrimitiveI128 |
                    SupportedPredefinedType::JsonStdPrimitiveU8 |
                    SupportedPredefinedType::JsonStdPrimitiveU16 |
                    SupportedPredefinedType::JsonStdPrimitiveU32 |
                    SupportedPredefinedType::JsonStdPrimitiveU64 |
                    SupportedPredefinedType::JsonStdPrimitiveU128 |
                    SupportedPredefinedType::JsonStdPrimitiveF32 |
                    SupportedPredefinedType::JsonStdPrimitiveF64 => {
                        let query_part_token_stream = generate_simple_json_type(PrimitiveJsonType::Number);
                        quote::quote!{ => #query_part_token_stream}
                    },
                    SupportedPredefinedType::JsonStdPrimitiveBool => {
                        let query_part_token_stream = generate_simple_json_type(PrimitiveJsonType::Boolean);
                        quote::quote!{ => #query_part_token_stream}
                    },
                    SupportedPredefinedType::JsonStdStringString => {
                        let query_part_token_stream = generate_simple_json_type(PrimitiveJsonType::String);
                        quote::quote!{ => #query_part_token_stream}
                    },

                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 => {
                        let query_part_token_stream = generate_optional_simple_json_type(PrimitiveJsonType::Number);
                        quote::quote!{ => #query_part_token_stream}
                    },
                    SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool => {
                        let query_part_token_stream = generate_optional_simple_json_type(PrimitiveJsonType::Boolean);
                        quote::quote!{ => #query_part_token_stream}
                    },
                    SupportedPredefinedType::JsonStdOptionOptionStdStringString => {
                        let query_part_token_stream = generate_optional_simple_json_type(PrimitiveJsonType::String);
                        quote::quote!{ => #query_part_token_stream}
                    },

                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 => {
                    //     let query_part_token_stream = generate_vec_simple_json_type(PrimitiveJsonType::Number);
                    //     quote::quote!{
                    //         {
                    //             limit,
                    //             offset
                    //         } => #query_part_token_stream
                    //     }
                    // },
                    // // generate_vec_imple_json_types
                    // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool => {
                    //     let query_part_token_stream = generate_vec_simple_json_type(PrimitiveJsonType::Boolean);
                    //     quote::quote!{
                    //         {
                    //             limit,
                    //             offset
                    //         } => #query_part_token_stream
                    //     }
                    // },
                    // SupportedPredefinedType::JsonStdVecVecStdStringString => {
                    //     let query_part_token_stream = generate_vec_simple_json_type(PrimitiveJsonType::String);
                    //     quote::quote!{
                    //         {
                    //             limit,
                    //             offset
                    //         } => #query_part_token_stream
                    //     }
                    // },

                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 => {
                    //     let query_part_token_stream = generate_optional_vec_simple_json_type(PrimitiveJsonType::Number);
                    //     quote::quote!{
                    //         {
                    //             limit,
                    //             offset
                    //         } => #query_part_token_stream
                    //     }
                    // },
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool => {
                    //     let query_part_token_stream = generate_optional_vec_simple_json_type(PrimitiveJsonType::Boolean);
                    //     quote::quote!{
                    //         {
                    //             limit,
                    //             offset
                    //         } => #query_part_token_stream
                    //     }
                    // },
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString => {
                    //     let query_part_token_stream = generate_optional_vec_simple_json_type(PrimitiveJsonType::String);
                    //     quote::quote!{
                    //         {
                    //             limit,
                    //             offset
                    //         } => #query_part_token_stream
                    //     }
                    // },

                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 => {
                    //     let query_part_token_stream = generate_vec_optional_simple_json_type(PrimitiveJsonType::Number);
                    //     quote::quote!{
                    //         {
                    //             limit,
                    //             offset
                    //         } => #query_part_token_stream
                    //     }
                    // },
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool => {
                    //     let query_part_token_stream = generate_vec_optional_simple_json_type(PrimitiveJsonType::Boolean);
                    //     quote::quote!{
                    //         {
                    //             limit,
                    //             offset
                    //         } => #query_part_token_stream
                    //     }
                    // },
                    // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString => {
                    //     let query_part_token_stream = generate_vec_optional_simple_json_type(PrimitiveJsonType::String);
                    //     quote::quote!{
                    //         {
                    //             limit,
                    //             offset
                    //         } => #query_part_token_stream
                    //     }
                    // },

                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 => {
                    //     let query_part_token_stream = generate_optional_vec_optional_simple_json_type(PrimitiveJsonType::Number);
                    //     quote::quote!{
                    //         {
                    //             limit,
                    //             offset
                    //         } => #query_part_token_stream
                    //     }
                    // },
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool => {
                    //     let query_part_token_stream = generate_optional_vec_optional_simple_json_type(PrimitiveJsonType::Boolean);
                    //     quote::quote!{
                    //         {
                    //             limit,
                    //             offset
                    //         } => #query_part_token_stream
                    //     }
                    // },
                    // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => {
                    //     let query_part_token_stream = generate_optional_vec_optional_simple_json_type(PrimitiveJsonType::String);
                    //     quote::quote!{
                    //         {
                    //             limit,
                    //             offset
                    //         } => #query_part_token_stream
                    //     }
                    // },

                    SupportedPredefinedType::JsonGeneric(type_path) => generic_and_std_option_option_generic_logic_token_stream(&type_path, false),
                    SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => generic_and_std_option_option_generic_logic_token_stream(&type_path, true),
                    SupportedPredefinedType::JsonStdVecVecGeneric(type_path) => maybe_std_option_option_std_vec_vec_maybe_std_option_option_generic_logic_token_stream(
                        &type_path, 
                        false,
                        false,
                    ),
                    SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(type_path) => maybe_std_option_option_std_vec_vec_maybe_std_option_option_generic_logic_token_stream(
                        &type_path, 
                        false,
                        true,
                    ),
                    SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(type_path) => maybe_std_option_option_std_vec_vec_maybe_std_option_option_generic_logic_token_stream(
                        &type_path, 
                        true,
                        false,
                    ),
                    SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => maybe_std_option_option_std_vec_vec_maybe_std_option_option_generic_logic_token_stream(
                        &type_path, 
                        true,
                        true,
                    ),

                    SupportedPredefinedType::JsonUuid => {
                        let query_part_token_stream = generate_simple_json_type(PrimitiveJsonType::String);
                        quote::quote!{ => #query_part_token_stream}
                    },
                };
                quote::quote!{Self::#element_ident_upper_camel_case_token_stream #variant_logic_token_stream}
            });
            quote::quote!{
                match #match_value_token_stream {
                    #(#generate_postgresql_query_part_match_variants_token_stream),*
                }
            }
        };
        let acc_push_token_stream = {
            let jsonb_build_object_concatenation_stringified = "||";
            match should_generate_ident_generate_postgresql_query_part_error_named_enum {
                true => {
                    let format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!("{{value}}{jsonb_build_object_concatenation_stringified}"),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    quote::quote!{
                        match element.generate_postgresql_query_part_to_read(
                            column_name_and_maybe_field_getter,
                            column_name_and_maybe_field_getter_for_error_message,
                        ) {
                            Ok(value) => {
                                acc.push_str(&format!(#format_handle_token_stream));
                            }
                            Err(error) => {
                                return Err(#ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream::GeneratePostgresqlQueryPart {
                                    error,
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        }
                    }
                },
                false => {
                    let format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!("{{}}{jsonb_build_object_concatenation_stringified}"),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    let postgresql_query_part_content_token_stream = generate_postgresql_query_part_content(&quote::quote!{element}, false);
                    quote::quote!{
                        acc.push_str(&format!(#format_handle_token_stream, #postgresql_query_part_content_token_stream));
                    }
                }
            }
        };
        let second_generic_token_stream = match should_generate_ident_generate_postgresql_query_part_error_named_enum {
            true => &ident_generate_postgresql_query_part_error_named_upper_camel_case_token_stream,
            false => &quote::quote!{()}
        };
        let postgresql_query_part_content_token_stream = generate_postgresql_query_part_content(&quote::quote!{self}, true);
        let space_and_not_null_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
            &space_and_not_null_stringified,
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let query_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
            &{
                let space_else_space_jsonb_build_object_err_stringified = generate_space_else_space_jsonb_build_object_err_stringified(&format!("{type_of_space_stringified}{{column_name_and_maybe_field_getter_for_error_message}}{space_is_not_space_stringified}object{{space_and_not_null}}"));
                let wraped_into_jsonb_build_object_ok_stringified = wrap_into_jsonb_build_object_ok_stringified("{acc}");
                let wraped_into_jsonb_typeof_stringified = wrap_into_jsonb_typeof_stringified("{column_name_and_maybe_field_getter}");
                let wraped_into_when_space_value_space_equals_space_null_stringified = wrap_into_when_space_value_space_equals_space_null_stringified(
                    &wraped_into_jsonb_typeof_stringified,
                    &PrimitiveJsonType::Object
                );
                let add_then_space_prefix_wraped_into_jsonb_build_object_ok_stringified = add_then_space_prefix_stringified(&wraped_into_jsonb_build_object_ok_stringified);
                wrap_into_case_end_stringified(&format!(
                    "{wraped_into_when_space_value_space_equals_space_null_stringified} {add_then_space_prefix_wraped_into_jsonb_build_object_ok_stringified}{{is_optional_query_part}}{space_else_space_jsonb_build_object_err_stringified}"
                ))
            },
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let check_optional_query_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
            &generate_when_jsonb_typeof_value_equal_null_then_jsob_build_object_ok_null_stringified("{column_name_and_maybe_field_getter}"),
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        quote::quote!{
            impl postgresql_crud::GeneratePostgresqlQueryPartToRead<
                #ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream,
                #second_generic_token_stream
            > for #ident_field_to_read_upper_camel_case_token_stream {
                fn generate_postgresql_query_part_to_read_from_self_vec(
                    value: &std::vec::Vec<Self>,
                    column_name_and_maybe_field_getter: &std::primitive::str,
                    column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
                    is_optional: std::primitive::bool,
                ) -> Result<std::string::String, #ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream> {
                    if value.is_empty() {
                        return Err(#ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream::FieldsFilterIsEmpty {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                    let mut unique = vec![];
                    for element in value {
                        if unique.contains(&element) {
                            return Err(#ident_generate_postgresql_query_part_from_self_vec_error_named_upper_camel_case_token_stream::NotUniqueFieldFilter {
                                field: element.clone(),
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                        else {
                            unique.push(&element);
                        }
                    }
                    let mut acc = std::string::String::default();
                    for element in value {
                        #acc_push_token_stream
                    }
                    let _ = acc.pop();
                    let _ = acc.pop();
                    let is_optional_query_part = match is_optional {
                        true => format!(#check_optional_query_token_stream),
                        false => std::string::String::default()
                    };
                    Ok({
                        let space_and_not_null = if is_optional {
                            #space_and_not_null_quotes_token_stream
                        }
                        else {
                            ""
                        };
                        format!(
                            #query_token_stream
                        )
                    })
                }
                fn generate_postgresql_query_part_to_read(
                    &self,
                    column_name_and_maybe_field_getter: &std::primitive::str,
                    column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
                ) -> Result<std::string::String, #second_generic_token_stream> {
                    #postgresql_query_part_content_token_stream
                }
            }
        }
    };
    let pub_struct_ident_options_to_read_token_stream = {
        let fields_token_stream = vec_syn_field.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let supported_predefined_type = SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
            let type_token_stream = match supported_predefined_type
            {
                SupportedPredefinedType::JsonStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdPrimitiveBool |
                SupportedPredefinedType::JsonStdStringString => supported_predefined_type.to_original_type().full_type_path_token_stream(),

                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::JsonStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_option_option_full_type_path_token_stream(),
                
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdStringString => supported_predefined_type.to_original_type().std_vec_vec_full_type_path_token_stream(),

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_full_type_path_token_stream(),

                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_vec_vec_std_option_option_full_type_path_token_stream(),

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_std_option_option_full_type_path_token_stream(),

                SupportedPredefinedType::JsonGeneric(_) => supported_predefined_type.to_original_type().full_type_path_token_stream(),
                SupportedPredefinedType::JsonStdOptionOptionGeneric(_) => supported_predefined_type.to_original_type().std_option_option_full_type_path_token_stream(),
                SupportedPredefinedType::JsonStdVecVecGeneric(_) => supported_predefined_type.to_original_type().std_vec_vec_full_type_path_token_stream(),
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(_) => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_full_type_path_token_stream(),
                SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(_) => supported_predefined_type.to_original_type().std_vec_vec_std_option_option_full_type_path_token_stream(),
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(_) => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_std_option_option_full_type_path_token_stream(),

                SupportedPredefinedType::JsonUuid => supported_predefined_type.to_original_type().full_type_path_token_stream(),
            };
            let serde_skip_serializing_if_value_attribute_token_stream = proc_macro_helpers::generate_serde_skip_serializing_if_value_attribute_token_stream::generate_serde_skip_serializing_if_value_attribute_token_stream();
            quote::quote!{
                #serde_skip_serializing_if_value_attribute_token_stream
                #element_ident: std::option::Option<postgresql_crud::Value<#type_token_stream>>
            }
        });
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait//Eq, 
            pub struct #ident_options_to_read_upper_camel_case_token_stream {
                #(#fields_token_stream),*

                // std_string_string: std::option::Option<postgresql_crud::Value<std::string::String>>,
                // std_vec_vec_std_primitive_bool: std::option::Option<postgresql_crud::Value<std::vec::Vec<std::primitive::bool>>>,
                // generic: std::option::Option<postgresql_crud::Value<DoggieOptions>>,
                // std_option_option_generic: std::option::Option<postgresql_crud::Value<std::option::Option<DoggieOptions>>>,
                // std_vec_vec_generic: std::option::Option<postgresql_crud::Value<std::vec::Vec<DoggieOptions>>>,
                // std_option_option_std_vec_vec_generic: std::option::Option<postgresql_crud::Value<std::option::Option<std::vec::Vec<DoggieOptions>>>>,
                // std_vec_vec_std_option_option_generic: std::option::Option<postgresql_crud::Value<std::vec::Vec<std::option::Option<DoggieOptions>>>>,
                // std_option_option_std_vec_vec_std_option_option_generic: std::option::Option<postgresql_crud::Value<std::option::Option<std::vec::Vec<std::option::Option<DoggieOptions>>>>>,

                // std_string_string: std::option::Option<std::result::Result<std::string::String, std::string::String>>,
                // std_vec_vec_std_primitive_bool: std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>, std::string::String>>,
                // generic: std::option::Option<std::result::Result<DoggieOptions,std::string::String>>,
                // std_option_option_generic: std::option::Option<std::result::Result<std::option::Option<DoggieOptions>,std::string::String>>,
                // std_vec_vec_generic: std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<DoggieOptions,std::string::String>>,std::string::String>>,
                // std_option_option_std_vec_vec_generic: std::option::Option<std::result::Result<
                //     std::option::Option<std::vec::Vec<std::result::Result<DoggieOptions,std::string::String>>>,
                //     std::string::String
                // >>,
                // std_vec_vec_std_option_option_generic: std::option::Option<std::result::Result<std::vec::Vec<std::result::Result<std::option::Option<DoggieOptions>,std::string::String>>,std::string::String>>,
                // std_option_option_std_vec_vec_std_option_option_generic: std::option::Option<
                //     std::result::Result<
                //         std::option::Option<
                //             std::vec::Vec<
                //                 std::result::Result<
                //                     std::option::Option<DoggieOptions>,
                //                     std::string::String
                //                 >
                //             >
                //         >,
                //         std::string::String
                //     >
                // >,
            }
        }
    };
    let impl_std_convert_from_ident_for_ident_options_token_stream = {
        let fields_token_stream = vec_syn_field.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let conversion_token_stream = match SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}")) {
                SupportedPredefinedType::JsonStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdPrimitiveBool |
                SupportedPredefinedType::JsonStdStringString
                => quote::quote!{value.#element_ident.0},

                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::JsonStdOptionOptionStdStringString
                => quote::quote!{
                    match value.#element_ident.0 {
                        Some(value) => Some(value.0),
                        None => None,
                    }
                },

                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdStringString
                // => quote::quote!{value.#element_ident.0.into_iter().map(|element|element.0).collect()},

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
                // => quote::quote!{
                //     match value.#element_ident.0 {
                //         Some(value) => Some(value.into_iter().map(|element|element.0).collect()),
                //         None => None
                //     }
                // },

                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
                // => quote::quote!{
                //     value.#element_ident.0.into_iter().map(|element|match element {
                //         Some(value) => Some(value.0),
                //         None => None
                //     }).collect()
                // },

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString
                // => quote::quote!{
                //     match value.#element_ident.0 {
                //         Some(value) => Some(value.into_iter().map(|element|match element {
                //             Some(value) => Some(value.0),
                //             None => None
                //         }).collect()),
                //         None => None
                //     }
                // },

                SupportedPredefinedType::JsonGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_to_read_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        #generic_ident_options_upper_camel_case_token_stream::from(value.#element_ident.0)
                    }
                }
                SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_to_read_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        match value.#element_ident.0 {
                            Some(value) => Some(#generic_ident_options_upper_camel_case_token_stream::from(value)),
                            None => None,
                        }
                    }
                }
                SupportedPredefinedType::JsonStdVecVecGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_to_read_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        value.#element_ident.0.into_iter().map(|element|#generic_ident_options_upper_camel_case_token_stream::from(element)).collect::<std::vec::Vec<#generic_ident_options_upper_camel_case_token_stream>>()
                    }
                }
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_to_read_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        match value.#element_ident.0 {
                            Some(value) => Some(value.into_iter().map(|element|#generic_ident_options_upper_camel_case_token_stream::from(element)).collect::<std::vec::Vec<#generic_ident_options_upper_camel_case_token_stream>>()),
                            None => None
                        }
                    }
                }
                SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_to_read_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        value.#element_ident.0.into_iter().map(|element|match element {
                            Some(value) => Some(#generic_ident_options_upper_camel_case_token_stream::from(value)),
                            None => None
                        }).collect::<std::vec::Vec<std::option::Option<#generic_ident_options_upper_camel_case_token_stream>>>()
                    }
                }
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => {
                    let generic_ident_options_upper_camel_case_token_stream = generate_ident_options_to_read_upper_camel_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        match value.#element_ident.0 {
                            Some(value) => Some(value.into_iter().map(|element|match element {
                                Some(value) => Some(#generic_ident_options_upper_camel_case_token_stream::from(value)),
                                None => None
                            }).collect::<std::vec::Vec<std::option::Option<#generic_ident_options_upper_camel_case_token_stream>>>()),
                            None => None
                        }
                    }
                }

                SupportedPredefinedType::JsonUuid => quote::quote!{value.#element_ident.0},
            };
            quote::quote!{
                #element_ident: Some(postgresql_crud::Value{ value: #conversion_token_stream })
            }
        });
        quote::quote!{
            impl std::convert::From<#ident> for #ident_options_to_read_upper_camel_case_token_stream {
                fn from(value: #ident) -> Self {
                    Self {
                        #(#fields_token_stream),*

                        //just for case if must return result impl
                        // // std_string_string: Some(std::result::Result::Ok(value.std_string_string.0)),
                        // // std_vec_vec_std_primitive_bool: Some(std::result::Result::Ok(
                        // //     value.std_vec_vec_std_primitive_bool.0.into_iter().map(|element|
                        // //         std::result::Result::Ok(element)
                        // //     ).collect::<std::vec::Vec<std::result::Result<std::primitive::bool,std::string::String>>>()
                        // // )),
                        // // generic: Some(std::result::Result::Ok(DoggieOptions::from(value.generic.0))),
                        // // std_option_option_generic: Some(std::result::Result::Ok(Some(match value.std_option_option_generic.0 {
                        // //     Some(value) => DoggieOptions {
                        // //         std_string_string: Some(postgresql_crud::Value{ value: value.std_string_string.0 }),
                        // //     },
                        // //     None => DoggieOptions {
                        // //         std_string_string: None,
                        // //     },
                        // // }))),
                        // // std_vec_vec_generic: Some(std::result::Result::Ok(value.std_vec_vec_generic.0.into_iter().map(|element|std::result::Result::Ok(DoggieOptions::from(element))).collect::<std::vec::Vec<std::result::Result<DoggieOptions,std::string::String>>>())),
                        // // std_option_option_std_vec_vec_generic: Some(std::result::Result::Ok(match value.std_option_option_std_vec_vec_generic.0 {
                        // //         Some(value) => Some(value.into_iter().map(|element|std::result::Result::Ok(DoggieOptions::from(element))).collect::<std::vec::Vec<std::result::Result<DoggieOptions,std::string::String>>>()),
                        // //         None => None
                        // // })),
                        // // std_vec_vec_std_option_option_generic: Some(std::result::Result::Ok(value.std_vec_vec_std_option_option_generic.0.into_iter().map(|element|std::result::Result::Ok(match element {
                        // //     Some(value) => Some(DoggieOptions::from(value)),
                        // //     None => None
                        // // })).collect::<std::vec::Vec<std::result::Result<std::option::Option<DoggieOptions>,std::string::String>>>())),
                        // // std_option_option_std_vec_vec_std_option_option_generic: Some(
                        // //     std::result::Result::Ok(
                        // //         match value.std_option_option_std_vec_vec_std_option_option_generic.0 {
                        // //             Some(value) => Some(value.into_iter().map(|element|std::result::Result::Ok(match element {
                        // //                 Some(value) => Some(DoggieOptions::from(value)),
                        // //                 None => None
                        // //             })).collect::<std::vec::Vec<std::result::Result<std::option::Option<DoggieOptions>,std::string::String>>>()),
                        // //             None => None
                        // //         }
                        // //     )
                        // // ),
                    }
                }
            }
        }
    };
    let impl_serde_deserialize_for_ident_options_token_stream = {
        let field_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, _)|{
            let value = format!(
                "__{}{index}",
                naming_conventions::FieldSnakeCase,
            );
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        });
        let generate_field_index_token_stream = |index: std::primitive::usize|{
            let value = format!("__field{index}");
            value.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let visit_u64_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, _)|{
            let index_u64_token_stream = {
                let value = format!("{index}u64");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let field_index_token_stream = generate_field_index_token_stream(index);
            quote::quote!{
                #index_u64_token_stream => serde::__private::Ok(__Field::#field_index_token_stream)
            }
        });
        let visit_str_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let field_name_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
                &{
                     element.ident.as_ref().unwrap_or_else(|| {
                        panic!(
                            "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                            naming_conventions::FIELD_IDENT_IS_NONE
                        );
                    }).to_string()
                },
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let field_index_token_stream = generate_field_index_token_stream(index);
            quote::quote!{
                #field_name_double_quotes_token_stream=> serde::__private::Ok(__Field::#field_index_token_stream)
            }
        });
        let visit_bytes_value_enum_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let b_field_name_double_quotes_token_stream= {
                let element_ident_double_quotes_stringified = proc_macro_common::generate_quotes::double_quotes_stringified(
                    &element.ident.as_ref().unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE);
                    }).to_string()
                );
                let value = format!("b{element_ident_double_quotes_stringified}");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let field_index_token_stream = generate_field_index_token_stream(index);
            quote::quote!{
                #b_field_name_double_quotes_token_stream=> serde::__private::Ok(__Field::#field_index_token_stream)
            }
        });
        let struct_ident_options_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
            &format!("struct {ident_options_to_read_upper_camel_case_stringified}"),
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let struct_ident_options_with_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
            &format!(
                "struct {ident_options_to_read_upper_camel_case_stringified} with {} elements",
                vec_syn_field.len()
            ),
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let generate_type_token_stream = |value: &syn::Field|{
            let supported_predefined_type = SupportedPredefinedType::try_from(value).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
            match supported_predefined_type {
                SupportedPredefinedType::JsonStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdPrimitiveBool |
                SupportedPredefinedType::JsonStdStringString => supported_predefined_type.to_original_type().full_type_path_token_stream(),
    
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::JsonStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_option_option_full_type_path_token_stream(),
    
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdStringString => supported_predefined_type.to_original_type().std_vec_vec_std_result_result_full_path_type_std_string_string_token_stream(),
    
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_std_result_result_full_path_type_std_string_string_token_stream(),
    
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_vec_vec_std_result_result_std_option_option_full_path_type_std_string_string_token_stream(),
    
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_std_result_result_std_option_option_full_path_type_std_string_string_token_stream(),
    
                SupportedPredefinedType::JsonGeneric(_) => supported_predefined_type.to_original_type().full_type_path_token_stream(),
                SupportedPredefinedType::JsonStdOptionOptionGeneric(_) => supported_predefined_type.to_original_type().std_option_option_full_type_path_token_stream(),
                SupportedPredefinedType::JsonStdVecVecGeneric(_) => supported_predefined_type.to_original_type().std_vec_vec_std_result_result_full_path_type_std_string_string_token_stream(),
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(_) => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_std_result_result_full_path_type_std_string_string_token_stream(),
                SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(_) => supported_predefined_type.to_original_type().std_vec_vec_std_result_result_std_option_option_full_path_type_std_string_string_token_stream(),
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(_) => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_std_result_result_std_option_option_full_path_type_std_string_string_token_stream(),

                SupportedPredefinedType::JsonUuid => supported_predefined_type.to_original_type().full_type_path_token_stream(),
            }
        };
        let visit_seq_fields_initialization_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let field_index_token_stream = generate_field_index_token_stream(index);
            let index_usize_token_stream = {
                let value = format!("{index}usize");
                value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let type_token_stream = generate_type_token_stream(element);
            quote::quote!{
                let #field_index_token_stream = match serde::de::SeqAccess::next_element::<std::option::Option<std::result::Result<#type_token_stream, std::string::String>>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(
                            serde::de::Error::invalid_length(
                                #index_usize_token_stream,
                                &#struct_ident_options_with_double_quotes_token_stream,
                            ),
                        );
                    }
                };
            }
        });
        let visit_seq_fields_assignment_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let field_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let field_index_token_stream = generate_field_index_token_stream(index);
            let conversion_logic_token_stream = match SupportedPredefinedType::try_from(*element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}")) 
            {
                SupportedPredefinedType::JsonStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdPrimitiveBool |
                SupportedPredefinedType::JsonStdStringString
                => quote::quote!{value},

                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::JsonStdOptionOptionStdStringString
                => quote::quote!{value},

                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdStringString
                // => quote::quote!{
                //     {
                //         let mut acc = vec![];
                //         for element in value {
                //             match element {
                //                 Ok(value) => {
                //                     acc.push(value);
                //                 }
                //                 Err(error) => {
                //                     return Err(serde::de::Error::custom(error));
                //                 }
                //             }
                //         }
                //         acc
                //     }
                // },

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString
                // => quote::quote!{
                //     match value {
                //         Some(value) => {
                //             let mut acc = vec![];
                //             for element in value {
                //                 match element {
                //                     Ok(value) => {
                //                         acc.push(value);
                //                     }
                //                     Err(error) => {
                //                         return Err(serde::de::Error::custom(error));
                //                     }
                //                 }
                //             }
                //             Some(acc)
                //         }
                //         None => None
                //     }
                // },

                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString
                // => quote::quote!{
                //     {
                //         let mut acc = vec![];
                //         for element in value {
                //             match element {
                //                 Ok(value) => {
                //                     acc.push(value);
                //                 }
                //                 Err(error) => {
                //                     return Err(serde::de::Error::custom(error));
                //                 }
                //             }
                //         }
                //         acc
                //     }
                // },

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString
                // => quote::quote!{
                //     match value {
                //         Some(value) => {
                //             let mut acc = vec![];
                //             for element in value {
                //                 match element {
                //                     Ok(value) => {
                //                         acc.push(value);
                //                     }
                //                     Err(error) => {
                //                         return Err(serde::de::Error::custom(error));
                //                     }
                //                 }
                //             }
                //             Some(acc)
                //         }
                //         None => None
                //     }
                // },

                SupportedPredefinedType::JsonGeneric(_) => quote::quote!{value},
                SupportedPredefinedType::JsonStdOptionOptionGeneric(_) => quote::quote!{value},
                SupportedPredefinedType::JsonStdVecVecGeneric(_) => quote::quote!{
                    {
                        let mut acc = vec![];
                        for element in value {
                            match element {
                                Ok(value) => {
                                    acc.push(value);
                                }
                                Err(error) => {
                                    return Err(serde::de::Error::custom(error));
                                }
                            }
                        }
                        acc
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(_) => quote::quote!{
                    match value {
                        Some(value) => {
                            let mut acc = vec![];
                            for element in value {
                                match element {
                                    Ok(value) => {
                                        acc.push(value);
                                    }
                                    Err(error) => {
                                        return Err(serde::de::Error::custom(error));
                                    }
                                }
                            }
                            Some(acc)
                        }
                        None => None
                    }
                },
                SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(_) => quote::quote!{
                    {
                        let mut acc = vec![];
                        for element in value {
                            match element {
                                Ok(value) => {
                                    acc.push(value);
                                }
                                Err(error) => {
                                    return Err(serde::de::Error::custom(error));
                                }
                            }
                        }
                        acc
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(_) => quote::quote!{
                    match value {
                        Some(value) => {
                            let mut acc = vec![];
                            for element in value {
                                match element {
                                    Ok(value) => {
                                        acc.push(value);
                                    }
                                    Err(error) => {
                                        return Err(serde::de::Error::custom(error));
                                    }
                                }
                            }
                            Some(acc)
                        }
                        None => None
                    }
                },

                SupportedPredefinedType::JsonUuid => quote::quote!{value},
            };
            quote::quote!{
                #field_ident: match #field_index_token_stream {
                    Some(value) => match value {
                        Ok(value) => Some(postgresql_crud::Value{ value: #conversion_logic_token_stream }),
                        Err(error) => {
                            return Err(serde::de::Error::custom(error));
                        }
                    },
                    None => None
                }
            }
        });
        //its just #(#visit_seq_fields_assignment_token_stream),* reusage making move error 
        let visit_seq_fields_assignment_handle_token_stream = quote::quote!{#(#visit_seq_fields_assignment_token_stream),*};
        let visit_map_fields_initialization_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let field_index_token_stream = generate_field_index_token_stream(index);
            let type_token_stream = generate_type_token_stream(element);
            quote::quote!{
                let mut #field_index_token_stream: serde::__private::Option<
                    std::option::Option<
                        std::result::Result<#type_token_stream, std::string::String>,
                    >,
                > = serde::__private::None;
            }
        });
        let generate_field_ident_double_quotes_token_stream = |value: &syn::Field|{
            proc_macro_common::generate_quotes::double_quotes_token_stream(
                &value.ident.as_ref().unwrap_or_else(|| {
                    panic!(
                        "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                        naming_conventions::FIELD_IDENT_IS_NONE
                    );
                }).to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified
            )
        };
        let visit_map_match_variants_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let field_index_token_stream = generate_field_index_token_stream(index);
            let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(&element);
            let type_token_stream = generate_type_token_stream(element);
            quote::quote!{
                __Field::#field_index_token_stream => {
                    if serde::__private::Option::is_some(&#field_index_token_stream) {
                        return serde::__private::Err(
                            <__A::Error as serde::de::Error>::duplicate_field(
                                #field_ident_double_quotes_token_stream,
                            ),
                        );
                    }
                    #field_index_token_stream = serde::__private::Some(
                        serde::de::MapAccess::next_value::<
                            std::option::Option<
                                std::result::Result<
                                    #type_token_stream,
                                    std::string::String,
                                >,
                            >,
                        >(&mut __map)?,
                    );
                }
            }
        });
        let visit_map_missing_fields_check_token_stream = vec_syn_field.iter().enumerate().map(|(index, element)|{
            let field_index_token_stream = generate_field_index_token_stream(index);
            let field_ident_double_quotes_token_stream = generate_field_ident_double_quotes_token_stream(&element);
            quote::quote!{
                let #field_index_token_stream = match #field_index_token_stream {
                    serde::__private::Some(#field_index_token_stream) => #field_index_token_stream,
                    serde::__private::None => {
                        serde::__private::de::missing_field(#field_ident_double_quotes_token_stream)?
                    }
                };
            }
        });
        let fields_array_elements_token_stream = vec_syn_field.iter().map(|element|generate_field_ident_double_quotes_token_stream(&element));
        quote::quote!{
            impl<'de> serde::Deserialize<'de> for #ident_options_to_read_upper_camel_case_token_stream {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> serde::__private::Result<Self, __D::Error>
                where
                    __D: serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        #(#field_enum_variants_token_stream),*,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl serde::de::Visitor<'_> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut serde::__private::Formatter<'_>,
                        ) -> serde::__private::fmt::Result {
                            serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> serde::__private::Result<Self::Value, __E>
                        where
                            __E: serde::de::Error,
                        {
                            match __value {
                                #(#visit_u64_value_enum_variants_token_stream),*,
                                _ => serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> serde::__private::Result<Self::Value, __E>
                        where
                            __E: serde::de::Error,
                        {
                            match __value {
                                #(#visit_str_value_enum_variants_token_stream),*,
                                _ => serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> serde::__private::Result<Self::Value, __E>
                        where
                            __E: serde::de::Error,
                        {
                            match __value {
                                #(#visit_bytes_value_enum_variants_token_stream),*,
                                _ => serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> serde::__private::Result<Self, __D::Error>
                        where
                            __D: serde::Deserializer<'de>,
                        {
                            serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: serde::__private::PhantomData<#ident_options_to_read_upper_camel_case_token_stream>,
                        lifetime: serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #ident_options_to_read_upper_camel_case_token_stream;
                        fn expecting(
                            &self,
                            __formatter: &mut serde::__private::Formatter<'_>,
                        ) -> serde::__private::fmt::Result {
                            serde::__private::Formatter::write_str(
                                __formatter,
                                #struct_ident_options_double_quotes_token_stream,
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: serde::de::SeqAccess<'de>,
                        {
                            #(#visit_seq_fields_initialization_token_stream)*
                            serde::__private::Ok(#ident_options_to_read_upper_camel_case_token_stream {
                                #visit_seq_fields_assignment_handle_token_stream
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: serde::de::MapAccess<'de>,
                        {
                            #(#visit_map_fields_initialization_token_stream)*
                            while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    #(#visit_map_match_variants_token_stream),*
                                    _ => {
                                        let _ = serde::de::MapAccess::next_value::<
                                            serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            #(#visit_map_missing_fields_check_token_stream)*
                            serde::__private::Ok(#ident_options_to_read_upper_camel_case_token_stream {
                                #visit_seq_fields_assignment_handle_token_stream
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        #(#fields_array_elements_token_stream),*
                    ];
                    serde::Deserializer::deserialize_struct(
                        __deserializer,
                        #ident_options_to_read_double_quotes_token_stream,
                        FIELDS,
                        __Visitor {
                            marker: serde::__private::PhantomData::<#ident_options_to_read_upper_camel_case_token_stream>,
                            lifetime: serde::__private::PhantomData,
                        },
                    )
                }
            }
        }
    };
    let ident_reader_token_stream = {
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait //, serde::Deserialize//Eq, 
            pub struct #ident_reader_upper_camel_case_token_stream(pub #ident_options_to_read_upper_camel_case_token_stream);//pub Result<SomethingOptions,std::string::String>
        }
    };
    let impl_serde_deserialize_for_ident_reader_token_stream = {
        let tuple_struct_space_stringified = "tuple struct ";
        let tuple_struct_ident_reader_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
            &format!("{tuple_struct_space_stringified}{ident_reader_upper_camel_case_stringified}"),
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let tuple_struct_ident_reader_with_1_element_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
            &format!("{tuple_struct_space_stringified}{ident_reader_upper_camel_case_stringified} with 1 element"),
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        let ident_wrapper_double_quotes_token_stream= proc_macro_common::generate_quotes::double_quotes_token_stream(
            &ident_reader_upper_camel_case_stringified,
            &proc_macro_name_upper_camel_case_ident_stringified
        );
        quote::quote!{
            impl<'de> serde::Deserialize<'de> for #ident_reader_upper_camel_case_token_stream {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> serde::__private::Result<Self, __D::Error>
                where
                    __D: serde::Deserializer<'de>,
                {
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: serde::__private::PhantomData<#ident_reader_upper_camel_case_token_stream>,
                        lifetime: serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #ident_reader_upper_camel_case_token_stream;
                        fn expecting(
                            &self,
                            __formatter: &mut serde::__private::Formatter<'_>,
                        ) -> serde::__private::fmt::Result {
                            serde::__private::Formatter::write_str(
                                __formatter,
                                #tuple_struct_ident_reader_double_quotes_token_stream,
                            )
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(
                            self,
                            __e: __E,
                        ) -> serde::__private::Result<Self::Value, __E::Error>
                        where
                            __E: serde::Deserializer<'de>,
                        {
                            let __field0: Result<#ident_options_to_read_upper_camel_case_token_stream, std::string::String> = <Result<
                                #ident_options_to_read_upper_camel_case_token_stream,
                                std::string::String,
                            > as serde::Deserialize>::deserialize(__e)?;
                            serde::__private::Ok(#ident_reader_upper_camel_case_token_stream(match __field0 {
                                Ok(value) => value,
                                Err(error) => {
                                    return Err(serde::de::Error::custom(error));
                                }
                            }))
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match serde::de::SeqAccess::next_element::<
                                Result<#ident_options_to_read_upper_camel_case_token_stream, std::string::String>,
                            >(&mut __seq)? {
                                serde::__private::Some(__value) => __value,
                                serde::__private::None => {
                                    return serde::__private::Err(
                                        serde::de::Error::invalid_length(
                                            0usize,
                                            &#tuple_struct_ident_reader_with_1_element_double_quotes_token_stream,
                                        ),
                                    );
                                }
                            };
                            serde::__private::Ok(#ident_reader_upper_camel_case_token_stream(match __field0 {
                                Ok(value) => value,
                                Err(error) => {
                                    return Err(serde::de::Error::custom(error));
                                }
                            }))
                        }
                    }
                    serde::Deserializer::deserialize_newtype_struct(
                        __deserializer,
                        #ident_wrapper_double_quotes_token_stream,
                        __Visitor {
                            marker: serde::__private::PhantomData::<#ident_reader_upper_camel_case_token_stream>,
                            lifetime: serde::__private::PhantomData,
                        },
                    )
                }
            }
        }
    };
    let impl_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_ident_token_stream = {
        let fields_initialization_token_stream = vec_syn_field.iter().map(|element|{
            let field_ident = element.ident.as_ref().unwrap_or_else(||panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE));
            quote::quote!{
                #field_ident: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
            }
        });
        quote::quote!{
            impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident {
                #[inline]
                fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                    Self {
                        #(#fields_initialization_token_stream),*
                    }
                }
            }
        }
    };
    let pub_enum_ident_option_to_update_token_stream = {
        let variants_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &element_ident.to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
            let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
            let type_token_stream = match &supported_predefined_type
            {
                SupportedPredefinedType::JsonStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdPrimitiveBool |
                SupportedPredefinedType::JsonStdStringString => supported_predefined_type.to_original_type().full_type_path_token_stream(),

                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::JsonStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_option_option_full_type_path_token_stream(),
                
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdStringString => supported_predefined_type.to_original_type().std_vec_vec_full_type_path_token_stream(),

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_full_type_path_token_stream(),

                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_vec_vec_std_option_option_full_type_path_token_stream(),

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_std_option_option_full_type_path_token_stream(),

                SupportedPredefinedType::JsonGeneric(type_path) => {
                    let value = format!("{}{}", quote::quote!{#type_path}.to_string(), naming_conventions::OptionsToUpdateUpperCamelCase);
                    value.parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                },
                SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
                    let type_path_options_to_update_upper_camel_case = {
                        let value = format!("{}{}", quote::quote!{#type_path}.to_string(), naming_conventions::OptionsToUpdateUpperCamelCase);
                        value.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote!{std::option::Option<#type_path_options_to_update_upper_camel_case>}
                },
                SupportedPredefinedType::JsonStdVecVecGeneric(type_path) => {
                    let type_path_options_to_update_upper_camel_case = {
                        let value = format!("{}{}", quote::quote!{#type_path}.to_string(), naming_conventions::OptionsToUpdateUpperCamelCase);
                        value.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                    };
                    quote::quote!{std::vec::Vec<#type_path_options_to_update_upper_camel_case>}
                },
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(_) => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_full_type_path_token_stream(),
                SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(_) => supported_predefined_type.to_original_type().std_vec_vec_std_option_option_full_type_path_token_stream(),
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(_) => supported_predefined_type.to_original_type().std_option_option_std_vec_vec_std_option_option_full_type_path_token_stream(),

                SupportedPredefinedType::JsonUuid => supported_predefined_type.to_original_type().full_type_path_token_stream(),
            };
            quote::quote!{
                #[serde(rename(serialize = #element_ident_double_quotes_token_stream, deserialize = #element_ident_double_quotes_token_stream))]
                #element_ident_upper_camel_case_token_stream(postgresql_crud::Value<#type_token_stream>)
            }
        });
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
            enum #ident_option_to_update_upper_camel_case_token_stream {
                #(#variants_token_stream),*
            }
        }
    };
    let pub_struct_ident_options_to_update_token_stream = {
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
            pub struct #ident_options_to_update_upper_camel_case_token_stream{
                #id_snake_case: #uuid_uuid_token_stream,
                #update_snake_case: std::vec::Vec<#ident_option_to_update_upper_camel_case_token_stream>,
            }
        }
    };
    let pub_enum_ident_field_to_update_token_stream = {
        let variants_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &element_ident.to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
            quote::quote!{
                #[serde(rename(serialize = #element_ident_double_quotes_token_stream, deserialize = #element_ident_double_quotes_token_stream))]
                #element_ident_upper_camel_case_token_stream
            }
        });
        quote::quote!{
            #[derive(
                Debug,
                Clone,
                PartialEq,
                serde::Serialize,
                serde::Deserialize,
                utoipa::ToSchema,
                schemars::JsonSchema,
            )]
            pub enum #ident_field_to_update_upper_camel_case_token_stream {
                #(#variants_token_stream),*
            }
        }
    };
    let impl_error_occurence_lib_to_std_string_string_for_ident_field_to_update_token_stream = {
        let variants_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                &element_ident.to_string(),
                &proc_macro_name_upper_camel_case_ident_stringified
            );
            let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
            quote::quote!{
                Self::#element_ident_upper_camel_case_token_stream => #element_ident_double_quotes_token_stream.to_owned()
            }
        });
        quote::quote!{
            impl error_occurence_lib::ToStdStringString for #ident_field_to_update_upper_camel_case_token_stream {
                fn to_std_string_string(&self) -> std::string::String {
                    match &self {
                        #(#variants_token_stream),*
                    } 
                }
            }
        }
    };
    let pub_enum_ident_options_to_update_try_generate_bind_increments_error_named_token_stream = {
        let mut acc = vec![];
        vec_syn_field_filtered_id_iter.iter().for_each(|element|{
            let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
            match &supported_predefined_type {
                SupportedPredefinedType::JsonStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdPrimitiveBool |
                SupportedPredefinedType::JsonStdStringString |

                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::JsonStdOptionOptionStdStringString 
                
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdStringString |

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString |

                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString |

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString 
                => (),

                SupportedPredefinedType::JsonGeneric(type_path) => {
                    if !acc.contains(type_path) {
                        acc.push(type_path.clone());
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
                    if !acc.contains(type_path) {
                        acc.push(type_path.clone());
                    }
                },
                SupportedPredefinedType::JsonStdVecVecGeneric(type_path) => {
                    if !acc.contains(type_path) {
                        acc.push(type_path.clone());
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(type_path) => {
                    if !acc.contains(type_path) {
                        acc.push(type_path.clone());
                    }
                },
                SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(type_path) => {
                    if !acc.contains(type_path) {
                        acc.push(type_path.clone());
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => {
                    if !acc.contains(type_path) {
                        acc.push(type_path.clone());
                    }
                },

                SupportedPredefinedType::JsonUuid => (),
            }
        });
        let additional_generic_variants = acc.iter().map(|element| {
            let value = quote::quote!{#element}.to_string();
            let generic_ident_token_stream = value.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
            let generic_ident_snake_case_token_stream = proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&value);
            let generic_ident_options_to_update_try_generate_bind_increments_error_named_token_stream = {
                let value = format!("{value}{}", naming_conventions::OptionsToUpdateTryGenerateBindIncrementsErrorNamedUpperCamelCase);
                value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            quote::quote!{
                #generic_ident_token_stream {
                    #[eo_error_occurence]
                    #generic_ident_snake_case_token_stream: #generic_ident_options_to_update_try_generate_bind_increments_error_named_token_stream,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                }
            }
        });
        quote::quote!{
            #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
            pub enum #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream {
                FieldsIsEmpty {
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
                NotUniqueField {
                    #[eo_to_std_string_string_serialize_deserialize]
                    field: #ident_field_to_update_upper_camel_case_token_stream,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
                CheckedAdd {
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
                #(#additional_generic_variants),*
            }
        }
    };
    let impl_postgresql_crud_generate_postgresql_query_part_to_update_ident_options_to_update_try_generate_bind_increments_error_named_for_ident_options_to_update_token_stream = {
        let check_not_unique_field_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
            quote::quote!{
                #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(_) => {
                    let value = #ident_field_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream;
                    if acc.contains(&value) {
                        return Err(
                            #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::NotUniqueField {
                                field: value,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            },
                        );
                    }
                    else {
                        acc.push(value);
                    }
                }
            }
        });
        let query_part_generation_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
            let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
            let content_token_stream = match &supported_predefined_type {
                SupportedPredefinedType::JsonStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdPrimitiveBool |
                SupportedPredefinedType::JsonStdStringString |

                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::JsonStdOptionOptionStdStringString
                
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdStringString |

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString |

                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString |

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString
                => {
                    let format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!("jsonb_set({{acc}},'{{{{{{previous_path}}{element_ident}}}}}',${{increment}})"),//element_ident.to_string(),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    quote::quote!{
                        (_) => {
                            match increment.checked_add(1) {
                                Some(value) => {
                                    *increment = value;
                                    acc = format!(#format_handle_token_stream);
                                }
                                None => {
                                    return Err(
                                        #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::CheckedAdd {
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }

                SupportedPredefinedType::JsonGeneric(type_path) => {
                    let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &element_ident.to_string(),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    let element_ident_snake_case_token_stream = proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        (value) => {
                            match value.value.try_generate_bind_increments(
                                &acc,
                                Some(#element_ident_double_quotes_token_stream),
                                increment,
                            ) {
                                Ok(value) => {
                                    acc = value;
                                }
                                Err(error) => {
                                    return Err(
                                        #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#type_path {
                                            #element_ident_snake_case_token_stream: error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
                    let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &element_ident.to_string(),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    let element_ident_snake_case_token_stream = proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        (value) => {
                            match &value.value {
                                Some(value) => {
                                    match value.try_generate_bind_increments(
                                        &acc,
                                        Some(#element_ident_double_quotes_token_stream),
                                        increment,
                                    ) {
                                        Ok(value) => {
                                            acc = value;
                                        }
                                        Err(error) => {
                                            return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#type_path {
                                                #element_ident_snake_case_token_stream: error,
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            });
                                        }
                                    }
                                }
                                None => {
                                    match increment.checked_add(1) {
                                        Some(value) => {
                                            *increment = value;
                                            acc = format!("jsonb_set({acc},'{{{previous_path}std_option_option_generic}}',${increment})");
                                        }
                                        None => {
                                            return Err(#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::CheckedAdd {
                                                code_occurence: error_occurence_lib::code_occurence!(),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                SupportedPredefinedType::JsonStdVecVecGeneric(type_path) => {
                    let element_ident_double_quotes_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &element_ident.to_string(),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    let element_ident_snake_case_token_stream = proc_macro_common::naming_conventions::ToSnakeCaseTokenStream::to_snake_case_token_stream(&quote::quote!{#type_path}.to_string());
                    quote::quote!{
                        (value) => {
                            match value.value.try_generate_bind_increments(
                                &acc,
                                Some(#element_ident_double_quotes_token_stream),
                                increment,
                            ) {
                                Ok(value) => {
                                    acc = value;
                                }
                                Err(error) => {
                                    return Err(
                                        #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::#type_path {
                                            #element_ident_snake_case_token_stream: error,
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(_) => todo!(),
                SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(_) => todo!(),
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(_) => todo!(),

                SupportedPredefinedType::JsonUuid => {
                    let format_handle_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!("jsonb_set({{acc}},'{{{{{{previous_path}}{element_ident}}}}}',${{increment}})"),//element_ident.to_string(),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    quote::quote!{
                        (_) => {
                            match increment.checked_add(1) {
                                Some(value) => {
                                    *increment = value;
                                    acc = format!(#format_handle_token_stream);
                                }
                                None => {
                                    return Err(
                                        #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::CheckedAdd {
                                            code_occurence: error_occurence_lib::code_occurence!(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            };
            quote::quote!{
                #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream #content_token_stream
            }
        });
        let bind_query_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let element_ident_upper_camel_case_token_stream = proc_macro_common::naming_conventions::ToUpperCamelCaseTokenStream::to_upper_camel_case_token_stream(&element_ident.to_string());
            let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
            let content_token_stream = match &supported_predefined_type {
                SupportedPredefinedType::JsonStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdPrimitiveBool |
                SupportedPredefinedType::JsonStdStringString |

                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::JsonStdOptionOptionStdStringString |
                
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdStringString |

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString |

                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString |

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString => {
                //     quote::quote!{
                //         query = query.bind(sqlx::types::Json(value.value));
                //     }
                // }

                SupportedPredefinedType::JsonGeneric(_) => {
                    quote::quote!{
                        query = value.value.bind_value_to_query(query);
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionGeneric(_) => {
                    quote::quote!{
                        match value.value {
                            Some(value) => {
                                query = value.bind_value_to_query(query);
                            }
                            None => {
                                query = query.bind(sqlx::types::Json(None::<std::option::Option<#ident_options_to_update_upper_camel_case_token_stream>>));
                            }
                        }
                    }
                },
                SupportedPredefinedType::JsonStdVecVecGeneric(_) => {
                    quote::quote!{
                        query = value.value.bind_value_to_query(query);
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(_) => todo!(),
                SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(_) => todo!(),
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(_) => todo!(),

                SupportedPredefinedType::JsonUuid => {
                    quote::quote!{
                        query = query.bind(sqlx::types::Json(value.value));
                    }
                }
            };
            quote::quote!{
                #ident_option_to_update_upper_camel_case_token_stream::#element_ident_upper_camel_case_token_stream(value) => {
                    #content_token_stream
                }
            }
        });
        quote::quote!{
            impl postgresql_crud::GeneratePostgresqlQueryPartToUpdate<#ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream> for #ident_options_to_update_upper_camel_case_token_stream {
                fn try_generate_bind_increments(
                    &self,
                    jsonb_set_acc: &std::primitive::str,
                    option_path: std::option::Option<&std::primitive::str>,
                    increment: &mut std::primitive::u64,
                ) -> Result<std::string::String, #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream> {
                    if self.0.is_empty() {
                        return Err(
                            #ident_options_to_update_try_generate_bind_increments_error_named_upper_camel_case_token_stream::FieldsIsEmpty {
                                code_occurence: error_occurence_lib::code_occurence!(),
                            },
                        );
                    }
                    {
                        let mut acc = vec![];
                        for element in &self.0 {
                            match element {
                                #(#check_not_unique_field_token_stream),*
                            }
                        }
                    }
                    let mut acc = std::string::String::from(jsonb_set_acc);
                    let previous_path = match &option_path {
                        Some(value) => format!("{value},"),
                        None => std::string::String::default()
                    };
                    for element in &self.0 {
                        match &element {
                            #(#query_part_generation_token_stream),*
                        }
                    }
                    Ok(acc)
                }
                fn bind_value_to_query<'a>(
                    self,
                    mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
                ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    for element in self.0 {
                        match element {
                            #(#bind_query_token_stream),*
                        }
                    }
                    query
                }
            }
        }
    };
    // println!("{}");
    let f = quote::quote!{
        #impl_postgresql_crud_generate_postgresql_query_part_to_update_ident_options_to_update_try_generate_bind_increments_error_named_for_ident_options_to_update_token_stream
    };
    // if ident == "" {
    //     println!("{f}");
    // }

    let pub_struct_ident_to_create_token_stream = {
        let variants_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let element_type = &element.ty;
            let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
            match &supported_predefined_type {
                SupportedPredefinedType::JsonStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdPrimitiveBool |
                SupportedPredefinedType::JsonStdStringString |

                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::JsonStdOptionOptionStdStringString
                
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdStringString |

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString |

                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString |

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString
                => {
                    quote::quote!{
                        pub #element_ident: #element_type
                    }
                },

                SupportedPredefinedType::JsonGeneric(type_path) => {
                    let element_type = naming_conventions::tokens_to_create_upper_camel_case_token_stream(&type_path);
                    quote::quote!{
                        pub #element_ident: postgresql_crud::JsonGeneric<#element_type>
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
                    let element_type = naming_conventions::tokens_to_create_upper_camel_case_token_stream(&type_path);
                    quote::quote!{
                        pub #element_ident: postgresql_crud::JsonStdOptionOptionGeneric<#element_type>
                    }
                },
                SupportedPredefinedType::JsonStdVecVecGeneric(type_path) => {
                    let element_type = naming_conventions::tokens_to_create_upper_camel_case_token_stream(&type_path);
                    quote::quote!{
                        pub #element_ident: postgresql_crud::JsonStdVecVecGeneric<#element_type>
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(type_path) => {
                    let element_type = naming_conventions::tokens_to_create_upper_camel_case_token_stream(&type_path);
                    quote::quote!{
                        pub #element_ident: postgresql_crud::JsonStdOptionOptionStdVecVecGeneric<#element_type>
                    }
                },
                SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(type_path) => {
                    let element_type = naming_conventions::tokens_to_create_upper_camel_case_token_stream(&type_path);
                    quote::quote!{
                        pub #element_ident: postgresql_crud::JsonStdVecVecStdOptionOptionGeneric<#element_type>
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => {
                    let element_type = naming_conventions::tokens_to_create_upper_camel_case_token_stream(&type_path);
                    quote::quote!{
                        pub #element_ident: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric<#element_type>
                    }
                },

                SupportedPredefinedType::JsonUuid => panic!("{proc_macro_name_upper_camel_case_ident_stringified} cannot be JsonUuid"),
            }
        });
        quote::quote!{
            #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
           pub struct #ident_to_create_upper_camel_case_token_stream {
               #(#variants_token_stream),*
           }
        }
    };
    let impl_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_ident_to_create_token_stream = {
        let fields_initialization_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element|{
            let field_ident = element.ident.as_ref().unwrap_or_else(||panic!("{proc_macro_name_upper_camel_case_ident_stringified} {}", naming_conventions::FIELD_IDENT_IS_NONE));
            quote::quote!{
                #field_ident: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
            }
        });
        quote::quote!{
            impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for #ident_to_create_upper_camel_case_token_stream {
                #[inline]
                fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
                    Self {
                        #(#fields_initialization_token_stream),*
                    }
                }
            }
        }
    };
    let impl_postgresql_crud_bind_query_for_ident_to_create_token_stream = {
        let try_generate_bind_increments_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
            match &supported_predefined_type {
                SupportedPredefinedType::JsonStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdPrimitiveBool |
                SupportedPredefinedType::JsonStdStringString |

                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::JsonStdOptionOptionStdStringString
                
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdStringString |

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString |

                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString |

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString
                 => {
                    let element_ident_double_quotes_increment_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!("'{}',${{increment}},", element_ident),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    quote::quote!{
                        match increment.checked_add(1) {
                            Some(incr) => {
                                *increment = incr;
                                increments.push_str(&format!(#element_ident_double_quotes_increment_token_stream));
                            }
                            None => {
                                return Err(postgresql_crud::TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                                    code_occurence: error_occurence_lib::code_occurence!(),
                                });
                            }
                        }
                    }
                },

                SupportedPredefinedType::JsonGeneric(type_path) => {
                    let element_ident_double_quotes_value_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!("'{}',{{value}},", element_ident),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    quote::quote!{
                        match self.#element_ident.0.try_generate_bind_increments(increment) {
                            Ok(value) => {
                                increments.push_str(&format!(#element_ident_double_quotes_value_token_stream));
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        }
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
                    quote::quote!{
                        todo!()
                    }
                },
                SupportedPredefinedType::JsonStdVecVecGeneric(type_path) => {
                    let element_ident_double_quotes_jsonb_build_array_token_stream = proc_macro_common::generate_quotes::double_quotes_token_stream(
                        &format!("'{}',jsonb_build_array({{acc}}),", element_ident),
                        &proc_macro_name_upper_camel_case_ident_stringified
                    );
                    quote::quote!{
                        {
                            let mut acc = std::string::String::default();
                            for element in &self.#element_ident.0 {
                                match element.try_generate_bind_increments(increment) {
                                    Ok(value) => {
                                        acc.push_str(&format!("{value},"));
                                    }
                                    Err(error) => {
                                        return Err(error);
                                    }
                                }
                            }
                            let _ = acc.pop();
                            increments.push_str(&format!(#element_ident_double_quotes_jsonb_build_array_token_stream));
                        }
                        //
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(type_path) => {
                    quote::quote!{
                        todo!()
                    }
                },
                SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(type_path) => {
                    quote::quote!{
                        todo!()
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => {
                    quote::quote!{
                        todo!()
                    }
                },

                SupportedPredefinedType::JsonUuid => panic!("{proc_macro_name_upper_camel_case_ident_stringified} cannot be JsonUuid"),
            }
        });
        let bind_value_to_query_token_stream = vec_syn_field_filtered_id_iter.iter().map(|element|{
            let element_ident = element.ident.as_ref().unwrap_or_else(|| {
                panic!(
                    "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                    naming_conventions::FIELD_IDENT_IS_NONE
                );
            });
            let supported_predefined_type = SupportedPredefinedType::try_from(**element).unwrap_or_else(|error| panic!("{proc_macro_name_upper_camel_case_ident_stringified} failed to convert into SupportedPredefinedType: {error:#?}"));
            match &supported_predefined_type {
                SupportedPredefinedType::JsonStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdPrimitiveBool |
                SupportedPredefinedType::JsonStdStringString |

                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveI128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU8 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU16 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveU128 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF32 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveF64 |
                SupportedPredefinedType::JsonStdOptionOptionStdPrimitiveBool |
                SupportedPredefinedType::JsonStdOptionOptionStdStringString
                
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdStringString |

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdStringString |

                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdVecVecStdOptionOptionStdStringString |

                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU128 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64 |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool |
                // SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionStdStringString
                 => {
                    quote::quote!{
                        query = query.bind(sqlx::types::Json(self.#element_ident.0));
                    }
                },

                SupportedPredefinedType::JsonGeneric(type_path) => {
                    quote::quote!{
                        query = self.#element_ident.0.bind_value_to_query(query);
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionGeneric(type_path) => {
                    quote::quote!{
                        todo!()
                    }
                },
                SupportedPredefinedType::JsonStdVecVecGeneric(type_path) => {
                    quote::quote!{
                        for element in self.#element_ident.0 {
                            query = element.bind_value_to_query(query);
                        }
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecGeneric(type_path) => {
                    quote::quote!{
                        todo!()
                    }
                },
                SupportedPredefinedType::JsonStdVecVecStdOptionOptionGeneric(type_path) => {
                    quote::quote!{
                        todo!()
                    }
                },
                SupportedPredefinedType::JsonStdOptionOptionStdVecVecStdOptionOptionGeneric(type_path) => {
                    quote::quote!{
                        todo!()
                    }
                },

                SupportedPredefinedType::JsonUuid => panic!("{proc_macro_name_upper_camel_case_ident_stringified} cannot be JsonUuid"),
            }
        });
        quote::quote!{
            impl<'a> postgresql_crud::BindQuery<'a> for #ident_to_create_upper_camel_case_token_stream {
                fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
                    todo!()//not usefull here - refactor later
                }
                fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::TryGenerateBindIncrementsErrorNamed> {
                    let mut increments = std::string::String::from("'id', to_jsonb(gen_random_uuid()),");
                    #(#try_generate_bind_increments_token_stream)*
                    let _ = increments.pop();
                    Ok(format!("jsonb_build_object({increments})"))
                }
                fn bind_value_to_query(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                    #(#bind_value_to_query_token_stream)*
                    query
                }
            }
        }
    };
    // if ident == "" {
    //     println!("{impl_postgresql_crud_bind_query_for_ident_to_create_token_stream}");
    // }
    let generated = quote::quote!{
        #impl_std_fmt_display_for_ident_token_stream
        #pub_enum_ident_field_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_field_token_stream
        #pub_enum_field_generate_postgresql_query_part_error_named_token_stream
        #impl_generate_postgresql_query_part_to_read_for_ident_field_token_stream
        #pub_struct_ident_options_to_read_token_stream
        #impl_std_convert_from_ident_for_ident_options_token_stream
        #impl_serde_deserialize_for_ident_options_token_stream
        #ident_reader_token_stream
        #impl_serde_deserialize_for_ident_reader_token_stream
        #impl_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_ident_token_stream

        #pub_enum_ident_field_to_update_token_stream
        #impl_error_occurence_lib_to_std_string_string_for_ident_field_to_update_token_stream
        #pub_enum_ident_option_to_update_token_stream
        #pub_struct_ident_options_to_update_token_stream
        #pub_enum_ident_options_to_update_try_generate_bind_increments_error_named_token_stream

        // #impl_postgresql_crud_generate_postgresql_query_part_to_update_ident_options_to_update_try_generate_bind_increments_error_named_for_ident_options_to_update_token_stream

        #pub_struct_ident_to_create_token_stream
        #impl_std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_ident_to_create_token_stream
        #impl_postgresql_crud_bind_query_for_ident_to_create_token_stream
    };
    // if ident == "" {
    //     proc_macro_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //         "www",
    //         &generated,
    //         "www",
    //     );
    // }
    generated.into()
}    
