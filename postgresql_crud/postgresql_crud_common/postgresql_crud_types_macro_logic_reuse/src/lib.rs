#[proc_macro_derive(FieldTypeImplementsSerializeDeserializeWithEqImpl)]
pub fn field_type_implements_serialize_deserialize_with_eq_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let proc_macro_name_upper_camel_case = "FieldTypeImplementsSerializeDeserializeWithEqImpl";
    field_type_implements_serialize_deserialize_handle(
        input,
        proc_macro_name_upper_camel_case,
        true,
    )
}

#[proc_macro_derive(FieldTypeImplementsSerializeDeserializeWithoutEqImpl)]
pub fn field_type_implements_serialize_deserialize_without_eq_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let proc_macro_name_upper_camel_case = "FieldTypeImplementsSerializeDeserializeWithoutEqImpl";
    field_type_implements_serialize_deserialize_handle(
        input,
        proc_macro_name_upper_camel_case,
        false,
    )
}


fn field_type_implements_serialize_deserialize_handle(
    input: proc_macro::TokenStream,
    proc_macro_name_upper_camel_case: &str,
    should_implement_eq: std::primitive::bool,
) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let field = if let syn::Data::Struct(data_struct) = &ast.data {
        if let syn::Fields::Unnamed(fields_unnamed) = &data_struct.fields {
            match fields_unnamed.unnamed.len() {
                1 => &fields_unnamed.unnamed[0],
                _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Unnamed with one field")
            }
        } else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Unnamed");
        }
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let field_type = &field.ty;
    let ident_with_serialize_deserialize_token_stream = {
        let value = format!("{ident}{}", proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified());
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_option_option_ident_upper_camel_case_token_stream = {
        let value = format!(
            "{}{ident}",
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream = {
        let value = format!(
            "{}{ident}{}",
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified(),
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let possible_eq_impl_token_stream = if should_implement_eq {
        quote::quote!{Eq,}
    }
    else {
        proc_macro2::TokenStream::new()
    };
    let gen = quote::quote!{
        //todo maybe some of them will not be needed in the future
        #[derive(Debug, PartialEq, #possible_eq_impl_token_stream serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
        pub struct #ident_with_serialize_deserialize_token_stream(#field_type);
        impl std::fmt::Display for #ident_with_serialize_deserialize_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{:?}", self.0)
            }
        }
        impl std::convert::From<#ident_with_serialize_deserialize_token_stream> for #ident {
            fn from(value: #ident_with_serialize_deserialize_token_stream) -> Self {
                Self(value.0)
            }
        }
        impl std::convert::From<#ident> for #ident_with_serialize_deserialize_token_stream {
            fn from(value: #ident) -> Self {
                Self(value.0)
            }
        }
        ///////////////////////////////////////
        #[derive(
            Debug,
            PartialEq,
            #possible_eq_impl_token_stream
            serde::Serialize,
            serde::Deserialize,
            utoipa::ToSchema,
        )]
        pub struct #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream(std::option::Option<#field_type>);
        impl std::fmt::Display for #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{:?}", self.0)
            }
        }
        impl std::convert::From<#std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream> for #std_option_option_ident_upper_camel_case_token_stream {
            fn from(value: #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream) -> Self {
                Self(value.0)
            }
        }
        impl std::convert::From<#std_option_option_ident_upper_camel_case_token_stream> for #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream {
            fn from(value: #std_option_option_ident_upper_camel_case_token_stream) -> Self {
                Self(value.0)
            }
        }
    };
    // if ident == "" {
    //     println!("{gen}");
    //     println!("----------");//todo for some reason gen duplicates for few times - find out why and fix
    // }
    gen.into()
}


#[proc_macro_derive(CommonFrom)]
pub fn common_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "CommonFrom";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let where_ident_token_stream = {
        let value = format!("{}{ident}", <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified());
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_ident_with_serialize_deserialize_token_stream = {
        let value = format!(
            "{}{ident}{}", 
            <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_option_option_ident_token_stream = {
        let value = format!(
            "{}{ident}",
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified(),
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_std_option_option_ident_token_stream = {
        let value = format!(
            "{}{}{ident}",
            <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified(),
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_std_option_option_ident_with_serialize_deserialize_token_stream = {
        let value = format!(
            "{}{}{ident}{}", 
            <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let gen = quote::quote!{
        impl std::convert::From<#where_ident_with_serialize_deserialize_token_stream> for #where_ident_token_stream {
            fn from(value: #where_ident_with_serialize_deserialize_token_stream) -> Self {
                Self {
                    value: #ident::from(value.value),
                    conjuctive_operator: value.conjuctive_operator
                }
            }
        }
        impl std::convert::From<#where_std_option_option_ident_with_serialize_deserialize_token_stream> for #where_std_option_option_ident_token_stream {
            fn from(value: #where_std_option_option_ident_with_serialize_deserialize_token_stream) -> Self {
                Self {
                    value: #std_option_option_ident_token_stream::from(value.value),
                    conjuctive_operator: value.conjuctive_operator,
                }
            }
        }
    };
    // if ident == "" {
    //     println!("{gen}");
    //     println!("----------");//todo for some reason gen duplicates for few times - find out why and fix
    // }
    gen.into()
}

#[proc_macro_derive(CommonTryFrom)]
pub fn common_try_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "CommonTryFrom";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let ident_snake_case_token_stream = {
        let value = proc_macro_common::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&ident.to_string());
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_with_serialize_deserialize_error_named_token_stream = {
        let value = format!(
            "{ident}{}{}", 
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::error_named_upper_camel_case_stringified(),
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_option_option_ident_token_stream = {
        let value = format!(
            "{}{ident}",
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified(),
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_ident_token_stream = {
        let value = format!("{}{ident}", <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified());
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_ident_with_serialize_deserialize_token_stream = {
        let value = format!(
            "{}{ident}{}", 
            <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_ident_with_serialize_deserialize_error_named_token_stream = {
        let value = format!(
            "{}{ident}{}{}", 
            <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::error_named_upper_camel_case_stringified(),
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_std_option_option_ident_with_serialize_deserialize_error_named_token_stream = {
        let value = format!(
            "{}{}{ident}{}{}", 
            <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::error_named_upper_camel_case_stringified(),
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_std_option_option_ident_token_stream = {
        let value = format!(
            "{}{}{ident}", 
            <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified(),
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_std_option_option_ident_with_serialize_deserialize_token_stream = {
        let value = format!(
            "{}{}{ident}{}", 
            <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let gen = quote::quote!{
        #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
        pub enum #where_ident_with_serialize_deserialize_error_named_token_stream {
            #ident {
                #[eo_error_occurence]
                #ident_snake_case_token_stream: #ident_with_serialize_deserialize_error_named_token_stream,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence//todo reuse path to error_occurence_lib
            },
        }
        impl std::convert::TryFrom<#where_ident_with_serialize_deserialize_token_stream> for #where_ident_token_stream {
            type Error = #where_ident_with_serialize_deserialize_error_named_token_stream;
            fn try_from(value: #where_ident_with_serialize_deserialize_token_stream) -> Result<Self, Self::Error> {
                Ok(Self {
                    value: match #ident::try_from(value.value) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Self::Error::#ident {
                                #ident_snake_case_token_stream: error,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }  
                    },
                    conjuctive_operator: value.conjuctive_operator
                })
            }
        }
        //
        #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
        pub enum #where_std_option_option_ident_with_serialize_deserialize_error_named_token_stream {
            #ident {
                #[eo_error_occurence]
                #ident_snake_case_token_stream: #ident_with_serialize_deserialize_error_named_token_stream,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence//todo reuse path to error_occurence_lib
            },
        }
        impl std::convert::TryFrom<#where_std_option_option_ident_with_serialize_deserialize_token_stream> for #where_std_option_option_ident_token_stream {
            type Error = #where_std_option_option_ident_with_serialize_deserialize_error_named_token_stream;
            fn try_from(value: #where_std_option_option_ident_with_serialize_deserialize_token_stream) -> Result<Self, Self::Error> {
                match value.value.0 {
                    Some(value_one) => match #ident::try_from(value_one) {
                        Ok(value_two) => Ok(Self{
                            value: #std_option_option_ident_token_stream(Some(value_two.0)),
                            conjuctive_operator: value.conjuctive_operator,
                        }),
                        Err(error) => Err(Self::Error::#ident {
                            #ident_snake_case_token_stream: error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    },
                    None => Ok(Self {
                        value: #std_option_option_ident_token_stream(None),
                        conjuctive_operator: value.conjuctive_operator,
                    })
                }
            }
        }
        //
    };
    // if ident == "" {
        // println!("{gen}");
    // }
    gen.into()
}

#[proc_macro_derive(CommonWithEqImpl)] //todo check on postgresql max length value of type
pub fn common_with_eq_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    common_handle(
        input,
        "CommonWithEqImpl",
        true,
    )
}

#[proc_macro_derive(CommonWithoutEqImpl)] //todo check on postgresql max length value of type
pub fn common_without_eq_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    common_handle(
        input,
        "CommonWithoutEqImpl",
        false,
    )
}

fn common_handle(
    input: proc_macro::TokenStream,
    proc_macro_name_upper_camel_case: &str,
    should_implement_eq: std::primitive::bool,
) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    // println!("{:#?}", ast.data);
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let field = if let syn::Data::Struct(data_struct) = &ast.data {
        if let syn::Fields::Unnamed(fields_unnamed) = &data_struct.fields {
            match fields_unnamed.unnamed.len() {
                1 => &fields_unnamed.unnamed[0],
                _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Unnamed with one field")
            }
        } else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Unnamed");
        }
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let field_type = &field.ty;
    let ident_with_serialize_deserialize_token_stream = {
        let value = format!("{ident}{}", proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified());
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_ident_token_stream = {
        let value = format!("{}{ident}", <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified());
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_ident_with_serialize_deserialize_token_stream = {
        let value = format!(
            "{}{ident}{}", 
            <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_option_option_ident_upper_camel_case_token_stream = {
        let value = format!(
            "{}{ident}",
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_std_option_option_ident_upper_camel_case_token_stream = {
        let value = format!(
            "{}{}{ident}",
            <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    //
    let std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream = {
        let value = format!(
            "{}{ident}{}",
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let where_std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream = {
        let value = format!(
            "{}{}{ident}{}",
            <naming_constants::Where as naming_constants::Naming>::upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let possible_eq_impl_token_stream = if should_implement_eq {
        quote::quote!{Eq,}
    }
    else {
        proc_macro2::TokenStream::new()
    };
    let gen = quote::quote!{
        impl std::fmt::Display for #ident {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{:?}", self.0)
            }
        }
        impl #ident {
            pub fn into_inner(self) -> #field_type {
                self.0
            }
        }
        impl std::convert::From<#ident> for #field_type {
            fn from(value: #ident) -> Self {
                value.0
            }
        }
        impl sqlx::Type<sqlx::Postgres> for #ident {
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                <#field_type as sqlx::Type<sqlx::Postgres>>::type_info()
            }
            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
                <#field_type as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
        }
        // impl sqlx::Encode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
        //     fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        //         sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
        //     }
        //     fn encode(
        //         self,
        //         buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
        //     ) -> sqlx::encode::IsNull
        //     where
        //         Self: Sized,
        //     {
        //         sqlx::Encode::<sqlx::Postgres>::encode(self.0, buf)
        //     }
        //     fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        //         sqlx::Encode::<sqlx::Postgres>::produces(&self.0)
        //     }
        //     fn size_hint(&self) -> std::primitive::usize {
        //         sqlx::Encode::<sqlx::Postgres>::size_hint(&self.0)
        //     }
        // }
        // impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgRangeStdPrimitiveI32 {
        //     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        //         match sqlx::Decode::<sqlx::Postgres>::decode(value) {
        //             Ok(value) => Ok(Self(value)),
        //             Err(error) => Err(error),
        //         }
        //     }
        // }
        impl CheckSupportedPostgresqlColumnType for #ident {
            fn check_supported_postgresql_column_type() {}
        }
        // impl AsPostgresqlInt4Range for SqlxPostgresTypesPgRangeStdPrimitiveI32 {}
        impl std::convert::From<#ident> for SupportedSqlxPostgresType {
            fn from(_value: #ident) -> Self {
                Self::#ident
            }
        }
        impl #ident {
            pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<#field_type> {
                value
                    .into_iter()
                    .map(Self::into_inner)
                    .collect()
            }
        }
        impl BindQuery for #ident {
            fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
                increment.checked_add(1).map_or_else(|| Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                    checked_add: std::string::String::from(CHECKED_ADD_IS_NONE),
                    code_occurence: error_occurence_lib::code_occurence!(),
                }), |incr| {
                    *increment = incr;
                    Ok(())
                })
            }
            fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
                let mut increments = std::string::String::default();
                match increment.checked_add(1) {
                    Some(incr) => {
                        *increment = incr;
                        increments.push_str(&format!("${increment}"));
                    }
                    None => {
                        return Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                            checked_add: std::string::String::from(CHECKED_ADD_IS_NONE),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
                Ok(increments)
            }
            fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                query = query.bind(self.0);
                query
            }
        }
        #[derive(Debug, PartialEq, #possible_eq_impl_token_stream)]
        pub struct #where_ident_token_stream {
            pub value: #ident,
            pub conjuctive_operator: ConjunctiveOperator,
        }
        impl std::fmt::Display for #where_ident_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "value: {}, conjuctive_operator: {}", self.value, self.conjuctive_operator)
            }
        }
        impl BindQuery for #where_ident_token_stream {
            fn try_increment(&self, increment: &mut std::primitive::u64) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
                increment.checked_add(1).map_or_else(|| Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                    checked_add: std::string::String::from("checked_add is None"),
                    code_occurence: error_occurence_lib::code_occurence!(),
                }), |incr| {
                    *increment = incr;
                    Ok(())
                })
            }
            fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<
                std::string::String,
                TryGenerateBindIncrementsErrorNamed,
            > {
                increment.checked_add(1).map_or_else(|| Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                    checked_add: std::string::String::from("checked_add is None"),
                    code_occurence: error_occurence_lib::code_occurence!(),
                }), |incr| {
                    *increment = incr;
                    Ok(format!("${increment}"))
                })
            }
            fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                query = query.bind(self.value.0);
                query
            }
        }
        #[derive(Debug, PartialEq, #possible_eq_impl_token_stream serde::Serialize, serde::Deserialize)]
        pub struct #where_ident_with_serialize_deserialize_token_stream {
            pub value: #ident_with_serialize_deserialize_token_stream,
            pub conjuctive_operator: ConjunctiveOperator,
        }
        impl std::fmt::Display for #where_ident_with_serialize_deserialize_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "value: {}, conjuctive_operator: {}", self.value, self.conjuctive_operator)
            }
        }
        impl std::convert::From<#where_ident_token_stream> for #where_ident_with_serialize_deserialize_token_stream {
            fn from(value: #where_ident_token_stream) -> Self {
                Self {
                    value: #ident_with_serialize_deserialize_token_stream::from(value.value),
                    conjuctive_operator: value.conjuctive_operator,
                }
            }
        }
        ///////////////////////////////////////
        #[derive(Debug, PartialEq, #possible_eq_impl_token_stream)]
        pub struct #std_option_option_ident_upper_camel_case_token_stream(pub std::option::Option<#field_type>);
        ////////////////////////////
        impl std::fmt::Display for #std_option_option_ident_upper_camel_case_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{:?}", self.0)
            }
        }
        impl #std_option_option_ident_upper_camel_case_token_stream {
            pub fn into_inner(self) -> std::option::Option<#field_type> {
                self.0
            }
        }
        impl std::convert::From<#std_option_option_ident_upper_camel_case_token_stream> for std::option::Option<#field_type> {
            fn from(value: #std_option_option_ident_upper_camel_case_token_stream) -> Self {
                value.0
            }
        }
        impl sqlx::Type<sqlx::Postgres> for #std_option_option_ident_upper_camel_case_token_stream {
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                <std::option::Option<#field_type> as sqlx::Type<sqlx::Postgres>>::type_info()
            }
            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
                <std::option::Option<#field_type> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
        }
        impl CheckSupportedPostgresqlColumnType for #std_option_option_ident_upper_camel_case_token_stream {
            fn check_supported_postgresql_column_type() {}
        }
        impl std::convert::From<#std_option_option_ident_upper_camel_case_token_stream> for SupportedSqlxPostgresType {
            fn from(_value: #std_option_option_ident_upper_camel_case_token_stream) -> Self {
                SupportedSqlxPostgresType::#ident
            }
        }
        impl #std_option_option_ident_upper_camel_case_token_stream {
            pub fn into_inner_type_vec(value: std::vec::Vec<Self>) -> std::vec::Vec<std::option::Option<#field_type>> {
                value
                    .into_iter()
                    .map(|element| element.into_inner())
                    .collect()
            }
        }
        impl BindQuery for #std_option_option_ident_upper_camel_case_token_stream {
            fn try_increment(
                &self,
                increment: &mut std::primitive::u64,
            ) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
                match increment.checked_add(1) {
                    Some(incr) => {
                        *increment = incr;
                        Ok(())
                    }
                    None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                        checked_add: std::string::String::from(CHECKED_ADD_IS_NONE),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                }
            }
            fn try_generate_bind_increments(
                &self,
                increment: &mut std::primitive::u64,
            ) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
                let mut increments = std::string::String::default();
                match increment.checked_add(1) {
                    Some(incr) => {
                        *increment = incr;
                        increments.push_str(&format!("${increment}"));
                    }
                    None => {
                        return Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                            checked_add: std::string::String::from(CHECKED_ADD_IS_NONE),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
                Ok(increments)
            }
            fn bind_value_to_query(
                self,
                mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
            ) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                query = query.bind(self.0);
                query
            }
        }
        #[derive(Debug, PartialEq)]
        pub struct #where_std_option_option_ident_upper_camel_case_token_stream {
            pub value: #std_option_option_ident_upper_camel_case_token_stream ,
            pub conjuctive_operator: ConjunctiveOperator,
        }
        impl std::fmt::Display for #where_std_option_option_ident_upper_camel_case_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    formatter,
                    "value: {}, conjuctive_operator: {}",
                    self.value, self.conjuctive_operator
                )
            }
        }
        impl BindQuery for #where_std_option_option_ident_upper_camel_case_token_stream {
            fn try_increment(
                &self,
                increment: &mut std::primitive::u64,
            ) -> Result<(), TryGenerateBindIncrementsErrorNamed> {
                increment.checked_add(1).map_or_else(|| Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                    checked_add: std::string::String::from("checked_add is None"),
                    code_occurence: error_occurence_lib::code_occurence!(),
                }), |incr| {
                    *increment = incr;
                    Ok(())
                })
            }
            fn try_generate_bind_increments(
                &self,
                increment: &mut std::primitive::u64,
            ) -> Result<std::string::String, TryGenerateBindIncrementsErrorNamed> {
                match increment.checked_add(1) {
                    Some(incr) => {
                        *increment = incr;
                        Ok(format!("${increment}"))
                    }
                    None => Err(TryGenerateBindIncrementsErrorNamed::CheckedAdd {
                        checked_add: std::string::String::from("checked_add is None"),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }),
                }
            }
            fn bind_value_to_query(
                self,
                mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
            ) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
                query = query.bind(self.value.0);
                query
            }
        }
        #[derive(Debug, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct #where_std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream {
            pub value: #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream,
            pub conjuctive_operator: ConjunctiveOperator,
        }
        impl std::fmt::Display for #where_std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    formatter,
                    "value: {}, conjuctive_operator: {}",
                    self.value, self.conjuctive_operator
                )
            }
        }
        impl std::convert::From<#where_std_option_option_ident_upper_camel_case_token_stream> for #where_std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream {
            fn from(value: #where_std_option_option_ident_upper_camel_case_token_stream) -> Self {
                Self {
                    value: #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream::from(value.value),
                    conjuctive_operator: value.conjuctive_operator,
                }
            }
        }
    };
    // if ident == "StdPrimitiveBool" {
    //     println!("{gen}");
    //     println!("----------");//todo for some reason gen duplicates for few times - find out why and fix
    // }
    gen.into()
}
///////////////
#[proc_macro_derive(CommonSpecificFrom)]
pub fn common_specific_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "CommonSpecificFrom";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let ident_with_serialize_deserialize_token_stream = {
        let value = format!("{ident}{}", proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified());
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_option_option_ident_upper_camel_case_token_stream = {
        let value = format!(
            "{}{ident}",
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream = {
        let value = format!(
            "{}{ident}{}",
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let gen = quote::quote!{
        #[derive(
            Debug,
            PartialEq,
            serde::Serialize,
            serde::Deserialize,
            utoipa::ToSchema,
        )]
        pub struct #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream(std::option::Option<#ident_with_serialize_deserialize_token_stream>);
        impl std::fmt::Display for #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{:?}", self.0)
            }
        }
        impl std::convert::From<#std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream> for #std_option_option_ident_upper_camel_case_token_stream {
            fn from(value: #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream) -> Self {
                match value.0 {
                    Some(value) => Self(Some(#ident::from(value).0)),
                    None => Self(None)
                }
            }
        }
        impl std::convert::From<#std_option_option_ident_upper_camel_case_token_stream> for #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream {
            fn from(value: #std_option_option_ident_upper_camel_case_token_stream) -> Self {
                match value.0 {
                    Some(value) => Self(Some(#ident_with_serialize_deserialize_token_stream::from(#ident(value)))),
                    None => Self(None)
                }
            }
        }
    };
    // if ident == "" {
    //     println!("{gen}");
    //     println!("----------");//todo for some reason gen duplicates for few times - find out why and fix
    // }
    gen.into()
}
////////////////
#[proc_macro_derive(CommonSpecificTryFrom)]
pub fn common_specific_try_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "CommonSpecificTryFrom";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let ident_with_serialize_deserialize_token_stream = {
        let value = format!("{ident}{}", proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified());
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_option_option_ident_upper_camel_case_token_stream = {
        let value = format!(
            "{}{ident}",
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream = {
        let value = format!(
            "{}{ident}{}",
            proc_macro_helpers::naming_conventions::std_option_option_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified()
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let ident_with_serialize_deserialize_error_named_token_stream = {
        let value = format!(
            "{ident}{}{}", 
            proc_macro_helpers::naming_conventions::with_serialize_deserialize_upper_camel_case_stringified(),
            proc_macro_helpers::naming_conventions::error_named_upper_camel_case_stringified(),
        );
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    };
    let gen = quote::quote!{
        #[derive(
            Debug,
            PartialEq,
            serde::Serialize,
            serde::Deserialize,
            utoipa::ToSchema,
        )]
        pub struct #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream(std::option::Option<#ident_with_serialize_deserialize_token_stream>);
        impl std::fmt::Display for #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{:?}", self.0)
            }
        }
        impl std::convert::TryFrom<#std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream> for #std_option_option_ident_upper_camel_case_token_stream {
            type Error = #ident_with_serialize_deserialize_error_named_token_stream;
            fn try_from(value: #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream) -> Result<Self, Self::Error> {
                match value.0 {
                    Some(value) => match #ident::try_from(value) {
                        Ok(value) => Ok(Self(Some(value.0))),
                        Err(error) => Err(error)
                    },
                    None => Ok(Self(None))
                }
            }
        }
        impl std::convert::From<#std_option_option_ident_upper_camel_case_token_stream> for #std_option_option_ident_with_serialize_deserialize_upper_camel_case_token_stream {
            fn from(value: #std_option_option_ident_upper_camel_case_token_stream) -> Self {
                match value.0 {
                    Some(value) => Self(Some(#ident_with_serialize_deserialize_token_stream::from(#ident(value)))),
                    None => Self(None)
                }
            }
        }
    };
    // if ident == "" {
        // println!("{gen}");
    // }
    gen.into()
}
////////////////

#[proc_macro_derive(AsPostgresqlCommon)] //todo check on postgresql max length value of type
pub fn as_postgresql_common(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
    proc_macro_common::panic_location::panic_location();
    let proc_macro_name_upper_camel_case = "AsPostgresqlCommon";
    let ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| {
        panic!(
            "{proc_macro_name_upper_camel_case} {}: {error}",
            proc_macro_common::constants::AST_PARSE_FAILED
        )
    });
    // println!("{:#?}", ast.data);
    let ident = &ast.ident;
    let proc_macro_name_upper_camel_case_ident_stringified = format!("{proc_macro_name_upper_camel_case} {ident}");
    let _field = if let syn::Data::Struct(data_struct) = &ast.data {
        if let syn::Fields::Unnamed(fields_unnamed) = &data_struct.fields {
            match fields_unnamed.unnamed.len() {
                1 => &fields_unnamed.unnamed[0],
                _ => panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Unnamed with one field")
            }
        } else {
            panic!("{proc_macro_name_upper_camel_case_ident_stringified} supports only syn::Fields::Unnamed");
        }
    } else {
        panic!("{proc_macro_name_upper_camel_case_ident_stringified} does work only on structs!");
    };
    let gen = quote::quote!{
        impl CheckSupportedRustAndPostgresqlColumnType for #ident {
            fn check_supported_rust_and_postgresql_column_type() {}
        }
    };
    gen.into()
}