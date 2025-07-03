//todo decide where to do error log (maybe add in some places)
//todo generate route what will return columns of the table and their rust and postgersql types
//todo created at and updated at fields + created by + updated by
//todo attributes for activation generation crud methods(like generate create, update_one, delete_one)
//todo authorization for returning concrete error or just minimal info(user role)
//todo generate rules and roles
//todo maybe add unnest sql types?
//todo maybe add unnest to filter parameters if its array ?
//todo swagger ui https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs
//todo derive utoipa::ToSchema for what? original structs or with serialize deserialize?
//todo need to add utoipa::ToSchema annotation #[schema(value_type = YourToSchemaTraitImplStruct)] for all fields
//todo remove useless derives like useless serde::Serialize and Deserialize
//todo maybe generate compisite type for user defined type https://docs.rs/sqlx/0.7.3/sqlx/postgres/types/index.html#rust_decimal
//todo read again some interesting thoughts about sql as api https://habr.com/ru/companies/timeweb/articles/798937/
//todo reexport all crates what logic depends on (from crates.io) (use of undeclared crate or module `time`)
//todo add transaction isolation level (see postgresql docs)
//todo check on postgresql max length value of type
//todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its error? or maybe use select then update\delete?(rewrite query)
//todo postgresql json schema validation https://youtu.be/F6X60ln2VNc
//todo generate json schema from rust type https://docs.rs/schemars/latest/schemars/
//todo support read table length

//todo postgresql json:
//* write json schema in postgresql
//* validate insert json field with json schema

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

#[proc_macro_derive(GeneratePostgresqlCrud, attributes(generate_postgresql_crud_primary_key))]
pub fn generate_postgresql_crud(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let syn_derive_input: syn::DeriveInput = syn::parse(input).unwrap_or_else(|error| panic!("{}: {error}", constants::AST_PARSE_FAILED));
    let ident = &syn_derive_input.ident;
    let ident_snake_case_stringified = naming::ToTokensToSnakeCaseStringified::case(&ident);
    #[derive(Debug, Clone)]
    struct SynFieldWrapper {
        syn_field: syn::Field,
        field_ident: syn::Ident,
    }
    let (primary_key_field, fields, fields_without_primary_key) = if let syn::Data::Struct(data_struct) = &syn_derive_input.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            let mut option_primary_key_field: std::option::Option<SynFieldWrapper> = None;
            let mut fields = vec![];
            let mut fields_without_primary_key = vec![];
            for element in &fields_named.named {
                let field_ident = element.ident.clone().unwrap();
                let field_ident_len = field_ident.to_string().len();
                let max_postgresql_column_length = 63;
                //todo write runtime check
                assert!((field_ident_len <= max_postgresql_column_length), "Postgresql truncates column names to {max_postgresql_column_length} characters, this is more: {field_ident} ({field_ident_len} characters)");
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
            // assert!((fields.len() <= 100), "explicitly not supporting number of columns more than 100 so its less possibility to cause stack overflow or build process exit");
            (option_primary_key_field.unwrap_or_else(|| panic!("primary_key_field is None")), fields, fields_without_primary_key)
        } else {
            panic!("supports only syn::Fields::Named");
        }
    } else {
        panic!("does work only on structs!");
    };
    let fields_len_minus_one = fields.len() - 1;
    let primary_key_field_type = &primary_key_field.syn_field.ty;
    let generate_as_postgresql_crud_postgresql_type_postgresql_type_token_stream = |field_type: &dyn quote::ToTokens| {
        quote::quote! {<#field_type as postgresql_crud::PostgresqlType>::}
    };
    let primary_key_field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_token_stream(&primary_key_field_type);
    let generate_as_postgresql_crud_postgresql_type_postgresql_type_tokens_token_stream = |field_type: &dyn quote::ToTokens, tokens: &dyn quote::ToTokens| {
        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_token_stream(&field_type);
        quote::quote! {#as_postgresql_crud_postgresql_type_postgresql_type_token_stream #tokens}
    };
    let primary_key_field_type_as_primary_key_upper_camel_case = quote::quote! {
        <#primary_key_field_type as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey
    };
    let primary_key_field_ident = &primary_key_field.field_ident;
    let primary_key_field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&primary_key_field_ident);
    let primary_key_field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&primary_key_field_ident);
    let primary_key_field_type_update_token_stream = &naming::parameter::SelfUpdateUpperCamelCase::from_type_last_segment(&primary_key_field.syn_field.ty);
    let std_string_string = token_patterns::StdStringString;
    let impl_ident_token_stream = {
        let ident_create_table_if_not_exists_error_named_upper_camel_case = naming::parameter::SelfCreateTableIfNotExistsErrorNamedUpperCamelCase::from_tokens(&ident);
        let create_extension_if_not_exists_pg_jsonschema_upper_camel_case = naming::CreateExtensionIfNotExistsPgJsonschemaUpperCamelCase;
        let create_extension_if_not_exists_uuid_ossp_upper_camel_case = naming::CreateExtensionIfNotExistsUuidOsspUpperCamelCase;
        let create_table_if_not_exists_upper_camel_case = naming::CreateTableIfNotExistsUpperCamelCase;
        let ident_create_table_if_not_exists_error_named_token_stream = quote::quote!{
            #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
            pub enum #ident_create_table_if_not_exists_error_named_upper_camel_case {
                #create_extension_if_not_exists_pg_jsonschema_upper_camel_case {
                    #[eo_to_std_string_string]
                    error: sqlx::Error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
                #create_extension_if_not_exists_uuid_ossp_upper_camel_case {
                    #[eo_to_std_string_string]
                    error: sqlx::Error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
                #create_table_if_not_exists_upper_camel_case {
                    #[eo_to_std_string_string]
                    error: sqlx::Error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
                },
            }
        };
        let pub_fn_table_token_stream = {
            let table_name_snake_case = naming::TableNameSnakeCase;
            let ident_snake_case_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&ident_snake_case_stringified);
            quote::quote! {
                pub fn #table_name_snake_case() -> &'static str {
                    #ident_snake_case_double_quotes_token_stream
                }
            }
        };
        let pub_async_fn_create_table_if_not_exists_token_stream = {
            let pool_snake_case = naming::PoolSnakeCase;
            let create_table_if_not_exists_double_quotes_token_stream = {
                let acc = {
                    let mut acc = std::string::String::new();
                    for _ in &fields {
                        acc.push_str("{},");
                    }
                    let _: Option<char> = acc.pop();
                    acc
                };
                generate_quotes::double_quotes_token_stream(&format!("create table if not exists {ident_snake_case_stringified} ({acc})"))
            };
            let serde_json_to_string_schemars_schema_for_generic_unwrap_token_stream = {
                let generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_token_stream = |field_type: &syn::Type, field_ident: &syn::Ident, is_primary_key: std::primitive::bool| {
                    let is_primary_key_token_stream: &dyn quote::ToTokens = if is_primary_key { &naming::TrueSnakeCase } else { &naming::FalseSnakeCase };
                    let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                    quote::quote! {
                        <#field_type as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&#field_ident_double_quotes_token_stream, #is_primary_key_token_stream)
                    }
                };
                let mut acc = vec![generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_token_stream(&primary_key_field.syn_field.ty, &primary_key_field.field_ident, true)];
                for element in &fields_without_primary_key {
                    acc.push(generate_field_type_as_postgresql_crud_create_table_column_query_part_create_table_query_part_token_stream(&element.syn_field.ty, &element.field_ident, false));
                }
                acc
            };
            quote::quote! {
                pub async fn create_table_if_not_exists(#pool_snake_case: &sqlx::Pool<sqlx::Postgres>) -> Result<(), #ident_create_table_if_not_exists_error_named_upper_camel_case> {
                    let create_extension_if_not_exists_pg_jsonschema_query_stringified = "create extension if not exists pg_jsonschema";
                    println!("{create_extension_if_not_exists_pg_jsonschema_query_stringified}");
                    if let Err(error) = sqlx::query(create_extension_if_not_exists_pg_jsonschema_query_stringified).execute(#pool_snake_case).await {
                        return Err(#ident_create_table_if_not_exists_error_named_upper_camel_case::#create_extension_if_not_exists_pg_jsonschema_upper_camel_case {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!()
                        });
                    }
                    let create_extension_if_not_exists_uuid_ossp_query_stringified = "create extension if not exists \"uuid-ossp\"";
                    println!("{create_extension_if_not_exists_uuid_ossp_query_stringified}");
                    if let Err(error) = sqlx::query(create_extension_if_not_exists_uuid_ossp_query_stringified).execute(#pool_snake_case).await {
                        return Err(#ident_create_table_if_not_exists_error_named_upper_camel_case::#create_extension_if_not_exists_uuid_ossp_upper_camel_case {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!()
                        });
                    }
                    let create_table_if_not_exists_query_stringified = format!(
                        #create_table_if_not_exists_double_quotes_token_stream,
                        #(#serde_json_to_string_schemars_schema_for_generic_unwrap_token_stream),*
                    );
                    println!("{create_table_if_not_exists_query_stringified}");
                    if let Err(error) = sqlx::query(&create_table_if_not_exists_query_stringified).execute(#pool_snake_case).await {
                        return Err(#ident_create_table_if_not_exists_error_named_upper_camel_case::#create_table_if_not_exists_upper_camel_case {
                            error,
                            code_occurence: error_occurence_lib::code_occurence!()
                        });
                    }
                    Ok(())
                }
            }
        };
        let pub_fn_allow_methods_token_stream = {
            let http_method_token_stream = quote::quote! {http::Method};
            quote::quote! {
                pub fn allow_methods() -> [#http_method_token_stream;4] {[
                    #http_method_token_stream::GET,
                    #http_method_token_stream::POST,
                    #http_method_token_stream::PATCH,
                    #http_method_token_stream::DELETE
                ]}
            }
        };
        quote::quote! {
            #ident_create_table_if_not_exists_error_named_token_stream
            impl #ident {
                #pub_fn_table_token_stream
                #pub_async_fn_create_table_if_not_exists_token_stream
                #pub_fn_allow_methods_token_stream
            }
        }
    };
    let debug_upper_camel_case = naming::DebugUpperCamelCase;
    let error_snake_case = naming::ErrorSnakeCase;
    let eprintln_error_token_stream = quote::quote! {eprintln!("{error}");};
    let serde_serialize = token_patterns::SerdeSerialize;
    let serde_deserialize = token_patterns::SerdeDeserialize;
    let derive_debug_serde_serialize_serde_deserialize = token_patterns::DeriveDebugSerdeSerializeSerdeDeserialize;
    let sqlx_row = token_patterns::SqlxRow;
    let ident_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&ident);
    let postgresql_crud_snake_case = &naming::PostgresqlCrudSnakeCase;
    let value_upper_camel_case = naming::ValueUpperCamelCase;
    let value_snake_case = naming::ValueSnakeCase;
    let from_snake_case = naming::FromSnakeCase;
    let create_query_part_snake_case = naming::CreateQueryPartSnakeCase;
    let generate_postgresql_crud_value_declaration_token_stream = |content_token_stream: &dyn quote::ToTokens| {
        quote::quote! {#postgresql_crud_snake_case::#value_upper_camel_case<#content_token_stream>}
    };
    let generate_postgresql_crud_value_initialization_token_stream = |content_token_stream: &dyn quote::ToTokens| {
        quote::quote! {#postgresql_crud_snake_case::#value_upper_camel_case{#value_snake_case: #content_token_stream}}
    };
    let ident_read_token_stream = {
        let field_attribute_serde_skip_serializing_if_option_is_none_token_stream = token_patterns::FieldAttributeSerdeSkipSerializingIfOptionIsNone;
        let field_option_primary_key_token_stream = {
            let postgresql_crud_value_declaration_token_stream = generate_postgresql_crud_value_declaration_token_stream(&generate_as_postgresql_crud_postgresql_type_postgresql_type_tokens_token_stream(&primary_key_field_type, &naming::ReadUpperCamelCase));
            quote::quote! {
                #field_attribute_serde_skip_serializing_if_option_is_none_token_stream
                pub #primary_key_field_ident: std::option::Option<#postgresql_crud_value_declaration_token_stream>
            }
        };
        let fields_options_excluding_primary_key_token_stream = fields_without_primary_key.iter().map(|element| {
            let field_vis = &element.syn_field.vis;
            let field_ident = &element.field_ident;
            let postgresql_crud_value_declaration_token_stream = generate_postgresql_crud_value_declaration_token_stream(&generate_as_postgresql_crud_postgresql_type_postgresql_type_tokens_token_stream(&element.syn_field.ty, &naming::ReadUpperCamelCase));
            quote::quote! {
                #field_attribute_serde_skip_serializing_if_option_is_none_token_stream
                #field_vis #field_ident: std::option::Option<#postgresql_crud_value_declaration_token_stream>
            }
        });
        quote::quote! {
            #derive_debug_serde_serialize_serde_deserialize
            pub struct #ident_read_upper_camel_case {
                #field_option_primary_key_token_stream,
                #(#fields_options_excluding_primary_key_token_stream),*
            }
        }
    };
    let default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementSnakeCase;
    let postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
    let postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream = token_patterns::PostgresqlCrudAllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall;
    let ident_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&ident);
    let select_token_stream = {
        let ident_select_token_stream = {
            let variants = fields.iter().map(|element| {
                let serialize_deserialize_ident_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
                let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_tokens_token_stream(&element.syn_field.ty, &naming::SelectUpperCamelCase);
                quote::quote! {
                    #[serde(rename(serialize = #serialize_deserialize_ident_token_stream, deserialize = #serialize_deserialize_ident_token_stream))]
                    #field_ident_upper_camel_case_token_stream(
                        #as_postgresql_crud_postgresql_type_postgresql_type_token_stream
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
                pub enum #ident_select_upper_camel_case {
                    #(#variants),*
                }
            }
        };
        let impl_std_fmt_display_for_ident_select_token_stream = macros_helpers::generate_impl_std_fmt_display_token_stream(
            &proc_macro2::TokenStream::new(),
            &ident_select_upper_camel_case,
            &proc_macro2::TokenStream::new(),
            &quote::quote! {write!(formatter, "{}", serde_json::to_string(&self).unwrap_or_else(|e|format!("cannot serialize into json: {e:?}")))},
        );
        let impl_error_occurence_lib_to_std_string_string_for_ident_select_token_stream =
            macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_token_stream(&proc_macro2::TokenStream::new(), &ident_select_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {format!("{self}")});
        let impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream =
            postgresql_crud_macros_common::generate_impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(&ident_select_upper_camel_case, &{
                let elements_token_stream = fields.iter().map(|element| {
                    let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                    quote::quote! {
                        #ident_select_upper_camel_case::#field_ident_upper_camel_case_token_stream(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                    }
                });
                quote::quote! {vec![#(#elements_token_stream),*]}
            });
        //todo this is temporary impl. maybe should write trait and implement different logic
        let pick_select_token_stream = {
            let fields_token_stream = fields.iter().map(|element| {
                let field_ident_upper_camel_case = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
                quote::quote! {
                    Self::#field_ident_upper_camel_case(_) => #field_ident_double_quotes_token_stream.to_string()
                }
            });
            quote::quote! {
                impl #ident_select_upper_camel_case {
                    fn pick_select(&self) -> std::string::String {
                        match &self {
                            #(#fields_token_stream),*
                        }
                    }
                }
            }
        };
        quote::quote! {
            #ident_select_token_stream
            #impl_std_fmt_display_for_ident_select_token_stream
            #impl_error_occurence_lib_to_std_string_string_for_ident_select_token_stream
            #impl_postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_for_ident_select_token_stream
            //todo this is temporary impl. maybe should write trait and implement different logic
            #pick_select_token_stream
        }
    };
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
        const fn get_syn_variant(&self) -> &syn::Variant {
            &self.variant
        }
        const fn get_option_status_code(&self) -> &std::option::Option<macros_helpers::status_code::StatusCode> {
            &self.status_code
        }
    }
    let new_syn_variant_wrapper = |
        variant_name: &dyn std::fmt::Display,
        status_code: std::option::Option<macros_helpers::status_code::StatusCode>,
        fields: std::vec::Vec<(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute, &dyn std::fmt::Display, syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>)>
    | -> SynVariantWrapper {
        SynVariantWrapper {
            variant: syn::Variant {
                attrs: status_code.as_ref().map_or_else(|| vec![], |value| vec![syn::Attribute {
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
                }]),
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
            panic!("syn::Fields::Named(value) != &self.variant.fields {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE);
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
    let generate_ident_try_operation_error_named_upper_camel_case = |operation: &Operation|{
        format!("{ident}Try{operation}ErrorNamed")
        .parse::<proc_macro2::TokenStream>().unwrap()
    };
    let generate_ident_try_operation_route_logic_error_named_upper_camel_case = |operation: &Operation|{
        format!("{ident}Try{operation}RouteLogicErrorNamed")
        .parse::<proc_macro2::TokenStream>().unwrap()
    };
    let generate_ident_try_operation_route_logic_response_variants_upper_camel_case = |operation: &Operation|{
        format!("{ident}Try{operation}RouteLogicResponseVariants")
        .parse::<proc_macro2::TokenStream>().unwrap()
    };
    let generate_ident_try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case = |operation: &Operation|{
        format!("{ident}Try{operation}RouteLogicErrorNamedWithSerializeDeserialize")
        .parse::<proc_macro2::TokenStream>().unwrap()
    };
    let generate_operation_error_initialization_eprintln_response_creation_token_stream = |operation: &Operation, syn_variant_wrapper: &SynVariantWrapper, file: &'static str, line: std::primitive::u32, column: std::primitive::u32| {
        let ident_try_operation_route_logic_error_named_upper_camel_case = generate_ident_try_operation_route_logic_error_named_upper_camel_case(&operation);
        let ident_try_operation_route_logic_response_variants_upper_camel_case = generate_ident_try_operation_route_logic_response_variants_upper_camel_case(&operation);
        let syn_variant_initialization_token_stream = generate_initialization_token_stream(syn_variant_wrapper, file, line, column);
        let status_code_token_stream = syn_variant_wrapper.get_option_status_code().unwrap_or_else(|| panic!("option_status_code is None")).to_axum_http_status_code_token_stream();
        let wraped_into_axum_response_token_stream = wrap_into_axum_response_token_stream(&quote::quote! {#ident_try_operation_route_logic_response_variants_upper_camel_case::#from_snake_case(#error_snake_case)}, &status_code_token_stream);
        quote::quote! {
            let #error_snake_case = #ident_try_operation_route_logic_error_named_upper_camel_case::#syn_variant_initialization_token_stream;
            #eprintln_error_token_stream
            #wraped_into_axum_response_token_stream
        }
    };
    let error_0_token_stream = token_patterns::Error0;
    let error_1_token_stream = token_patterns::Error1;
    let error_2_token_stream = token_patterns::Error2;
    let error_3_token_stream = token_patterns::Error3;
    let select_query_part_vec_column_token_stream = {
        let variants_token_stream = fields.iter().map(|element| {
            let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
            let initialization_token_stream = {
                let field_ident_string_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
                let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_token_stream(
                    &element.syn_field.ty
                );
                quote::quote! {
                    => #as_postgresql_crud_postgresql_type_postgresql_type_token_stream select_query_part(
                        #value_snake_case,
                        #field_ident_string_double_quotes_token_stream
                    )
                }
            };
            quote::quote! {#ident_select_upper_camel_case::#field_ident_upper_camel_case_token_stream(value) #initialization_token_stream}
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
                let _: std::option::Option<char> = #value_snake_case.pop();
                #value_snake_case
            }
        }
    };
    let sqlx_error_syn_punctuated_punctuated = macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["sqlx", "Error"]);
    let macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string = macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringString;
    let macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize = macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoToStdStringStringSerializeDeserialize;
    let postgresql_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::PostgresqlUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![(macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &naming::PostgresqlSnakeCase, sqlx_error_syn_punctuated_punctuated.clone())],
    );
    // //todo find out how to declare lifetime on closures
    // //todo refactor as &[&'a SynRust...]
    let generate_self_fields_token_stream = |fields: &[&syn::Field]| -> std::vec::Vec<syn::Ident> { fields.iter().map(|field| field.ident.as_ref().unwrap_or_else(|| panic!("{}", naming::FIELD_IDENT_IS_NONE)).clone()).collect() };
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
    impl Operation {
        const fn http_method(self) -> OperationHttpMethod {
            match self {
                Self::CreateMany | Self::CreateOne | Self::ReadMany | Self::ReadOne => OperationHttpMethod::Post,
                Self::UpdateMany | Self::UpdateOne => OperationHttpMethod::Patch,
                Self::DeleteMany | Self::DeleteOne => OperationHttpMethod::Delete,
            }
        }
        const fn desirable_status_code(self) -> macros_helpers::status_code::StatusCode {
            match self {
                Self::CreateMany | Self::CreateOne => macros_helpers::status_code::StatusCode::Created201,
                Self::ReadMany | Self::ReadOne | Self::UpdateMany | Self::UpdateOne | Self::DeleteMany | Self::DeleteOne => macros_helpers::status_code::StatusCode::Ok200,
            }
        }
        const fn generate_postgresql_crud_attribute_additional_error_variants(self) -> GeneratePostgresqlCrudAttribute {
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
        const fn generate_postgresql_crud_attribute_additional_route_logic(self) -> GeneratePostgresqlCrudAttribute {
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
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                Self::CreateMany => write!(formatter, "CreateMany"),
                Self::CreateOne => write!(formatter, "CreateOne"),
                Self::ReadMany => write!(formatter, "ReadMany"),
                Self::ReadOne => write!(formatter, "ReadOne"),
                Self::UpdateMany => write!(formatter, "UpdateMany"),
                Self::UpdateOne => write!(formatter, "UpdateOne"),
                Self::DeleteMany => write!(formatter, "DeleteMany"),
                Self::DeleteOne => write!(formatter, "DeleteOne"),
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
            let postgresql_crud_value_declaration_token_stream = generate_postgresql_crud_value_declaration_token_stream(&primary_key_field_type_as_primary_key_upper_camel_case);
            quote::quote! {
                let mut #primary_key_field_ident: std::option::Option<#postgresql_crud_value_declaration_token_stream> = None;
            }
        };
        let declaration_excluding_primary_key_token_stream = fields_without_primary_key.iter().map(|element| {
            let field_ident = &element.field_ident;
            let postgresql_crud_value_declaration_token_stream = generate_postgresql_crud_value_declaration_token_stream(&{
                let element_syn_field_ty = &element.syn_field.ty;
                quote::quote! {<#element_syn_field_ty as postgresql_crud::PostgresqlType>::Read}
            });
            quote::quote! {
                let mut #field_ident: std::option::Option<#postgresql_crud_value_declaration_token_stream> = None;
            }
        });
        let assignment_variant_primary_key_token_stream = {
            let primary_key_field_ident_string_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&primary_key_field_ident);
            let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!());
            let postgresql_crud_value_initialization_token_stream = generate_postgresql_crud_value_initialization_token_stream(&value_snake_case);
            quote::quote! {
                #ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(_) => match sqlx::Row::try_get::<
                    #primary_key_field_type_as_primary_key_upper_camel_case,
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
        let assignment_variants_excluding_primary_key_token_stream = fields_without_primary_key
            .iter()
            .map(|element| {
                let field_ident = &element.field_ident;
                let field_ident_upper_camel_case_token_stream = naming::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element.field_ident);
                let maybe_generic_filter_token_stream = proc_macro2::TokenStream::new();
                let field_ident_string_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&element.field_ident);
                let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &postgresql_syn_variant_wrapper, file!(), line!(), column!());
                let postgresql_crud_value_initialization_token_stream = generate_postgresql_crud_value_initialization_token_stream(&value_snake_case);
                let element_syn_field_ty = &element.syn_field.ty;
                quote::quote! {
                    #ident_select_upper_camel_case::#field_ident_upper_camel_case_token_stream #maybe_generic_filter_token_stream(_) => match sqlx::Row::try_get::<
                        <#element_syn_field_ty as postgresql_crud::PostgresqlType>::Read,
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
            })
            .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
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
            #ident_read_upper_camel_case {
                #(#option_fields_initiation_token_stream),*
            }
        }
    };
    let column_snake_case = naming::ColumnSnakeCase;
    let order_snake_case = naming::OrderSnakeCase;
    let order_by_upper_camel_case = naming::OrderByUpperCamelCase;
    let postgresql_crud_order_by_token_stream = quote::quote! {#postgresql_crud_snake_case::#order_by_upper_camel_case};
    let postgresql_crud_order_token_stream = quote::quote! {#postgresql_crud_snake_case::Order};
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
    let derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema = token_patterns::DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema;
    let derive_debug = token_patterns::DeriveDebug;
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
    let returning_primary_key_stringified = format!(" {returning_snake_case} {primary_key_field_ident}");
    let returning_primary_key_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&returning_primary_key_stringified);
    let std_string_string_syn_punctuated_punctuated = macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["std", "string", "String"]);
    let query_part_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::QueryPartUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoErrorOccurence,
            &naming::ErrorSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&postgresql_crud_snake_case.to_string(), &naming::QueryPartErrorNamedUpperCamelCase.to_string()]),
        )],
    );
    let row_and_rollback_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::RowAndRollbackUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &naming::RowSnakeCase, sqlx_error_syn_punctuated_punctuated.clone()),
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &rollback_snake_case, sqlx_error_syn_punctuated_punctuated.clone()),
        ],
    );
    enum ReadOrUpdate {
        Read,
        Update
    }
    let generate_std_vec_vec_primary_key_field_type_syn_punctuated_punctuated = |read_or_update: &ReadOrUpdate|{
        if let syn::Type::Path(value) = &primary_key_field.syn_field.ty {
            value.path.segments.last().map_or_else(|| {
                panic!("no last path segment");
            }, |last_path_segment| {
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
                                                &{
                                                    let last_path_segment_ident = &last_path_segment.ident;
                                                    match &read_or_update {
                                                        ReadOrUpdate::Read => naming::parameter::SelfReadUpperCamelCase::from_tokens(&last_path_segment_ident).to_string(),
                                                        ReadOrUpdate::Update => naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&last_path_segment_ident).to_string(),
                                                    }
                                                },
                                                proc_macro2::Span::call_site()
                                            ),
                                            arguments: syn::PathArguments::None,
                                        });
                                        handle
                                    },
                                }
                            })));
                            handle
                        },
                        gt_token: syn::token::Gt { spans: [proc_macro2::Span::call_site()] },
                    }),
                });
                handle
            })
        } else {
            panic!("primary key syn::Type in not syn::Type::Path");
        }
    };
    let std_vec_vec_primary_key_field_type_read_syn_punctuated_punctuated = generate_std_vec_vec_primary_key_field_type_syn_punctuated_punctuated(&ReadOrUpdate::Read);
    let std_vec_vec_primary_key_field_type_update_syn_punctuated_punctuated = generate_std_vec_vec_primary_key_field_type_syn_punctuated_punctuated(&ReadOrUpdate::Update);
    let non_existing_primary_keys_update_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NonExistingPrimaryKeysUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString,
            &naming::NonExistingPrimaryKeysSnakeCase,
            std_vec_vec_primary_key_field_type_update_syn_punctuated_punctuated.clone(),
        )],
    );
    let non_existing_primary_keys_update_and_rollback_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NonExistingPrimaryKeysAndRollbackUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString,
                &naming::NonExistingPrimaryKeysSnakeCase,
                std_vec_vec_primary_key_field_type_update_syn_punctuated_punctuated.clone(),
            ),
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &rollback_snake_case, sqlx_error_syn_punctuated_punctuated.clone()),
        ],
    );
    let non_existing_primary_keys_delete_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NonExistingPrimaryKeysUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString,
            &naming::NonExistingPrimaryKeysSnakeCase,
            std_vec_vec_primary_key_field_type_read_syn_punctuated_punctuated.clone(),
        )],
    );
    let non_existing_primary_keys_delete_and_rollback_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NonExistingPrimaryKeysAndRollbackUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                macros_helpers::error_occurence::ErrorOccurenceFieldAttribute::EoVecToStdStringString,
                &naming::NonExistingPrimaryKeysSnakeCase,
                std_vec_vec_primary_key_field_type_read_syn_punctuated_punctuated,
            ),
            (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &rollback_snake_case, sqlx_error_syn_punctuated_punctuated.clone()),
        ],
    );
    let sqlx_query_sqlx_postgres_token_stream = quote::quote! {sqlx::query::<sqlx::Postgres>};

    let (postgresql_crud_postgresql_type_where_filter_query_part_token_stream, postgresql_crud_postgresql_type_where_filter_query_bind_token_stream) = {
        let postgresql_crud_postgresql_type_where_filter_token_stream = quote::quote! {#postgresql_crud_snake_case::PostgresqlTypeWhereFilter::};
        (
            {
                let query_part_snake_case = naming::QueryPartSnakeCase;
                quote::quote! {#postgresql_crud_postgresql_type_where_filter_token_stream #query_part_snake_case}
            },
            {
                let query_bind_snake_case = naming::QueryBindSnakeCase;
                quote::quote! {#postgresql_crud_postgresql_type_where_filter_token_stream #query_bind_snake_case}
            },
        )
    };

    let increment_snake_case = naming::IncrementSnakeCase;
    let increment_initialization_token_stream = quote::quote! {let mut #increment_snake_case: std::primitive::u64 = 0;};
    let order_by_snake_case = naming::OrderBySnakeCase;
    let response_snake_case = naming::ResponseSnakeCase;
    let status_code_snake_case = naming::StatusCodeSnakeCase;
    let body_snake_case = naming::BodySnakeCase;

    //todo make primarykey trait and use it here instead
    let std_vec_vec_primary_key_field_type_read_token_stream = quote::quote! {std::vec::Vec::<#primary_key_field_type_as_primary_key_upper_camel_case>};
    
    let primary_key_field_type_as_postgresql_type_update_token_stream = quote::quote!{<#primary_key_field_type as postgresql_crud::PostgresqlType>::Update};
    let std_vec_vec_primary_key_field_type_as_postgresql_type_update_token_stream = quote::quote! {
        std::vec::Vec::<#primary_key_field_type_as_postgresql_type_update_token_stream>
    };

    let std_vec_vec_struct_options_ident_token_stream = quote::quote! {std::vec::Vec::<#ident_read_upper_camel_case>};
    // //todo rename not_unique_column to something what mean json tree getter too
    let not_unique_column_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NotUniqueColumnUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize.clone(),
            &naming::NotUniqueColumnSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&ident_select_upper_camel_case.to_string()]),
        )],
    );
    let serde_json_to_string_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::SerdeJsonToStringUpperCamelCase,
        None,
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
            &naming::SerdeJsonToStringSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"]),
        )],
    );
    let failed_to_get_response_text_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::FailedToGetResponseTextUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
                &status_code_snake_case,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["http", "StatusCode"]),
            ),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
                &naming::HeadersSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "header", "HeaderMap"]),
            ),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
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
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
                &status_code_snake_case,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["http", "StatusCode"]),
            ),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
                &naming::HeadersSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["reqwest", "header", "HeaderMap"]),
            ),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string_serialize_deserialize.clone(),
                &naming::ResponseTextSnakeCase,
                std_string_string_syn_punctuated_punctuated,
            ),
            (
                macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
                &naming::SerdeSnakeCase,
                macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"]),
            ),
        ],
    );
    let reqwest_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::ReqwestUpperCamelCase,
        None,
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
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
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&postgresql_crud_snake_case.to_string(), "check_body_size", &naming::CheckBodySizeErrorNamedUpperCamelCase.to_string()]),
        )],
    );
    let serde_json_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::SerdeJsonUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(
            macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
            &naming::SerdeJsonSnakeCase,
            macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["serde_json", "Error"]),
        )],
    );
    let header_content_type_application_json_not_found_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::HeaderContentTypeApplicationJsonNotFoundUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        std::vec::Vec::<(macros_helpers::error_occurence::ErrorOccurenceFieldAttribute, &'static dyn std::fmt::Display, syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>)>::default(),
    );
    let not_unique_primary_key_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NotUniquePrimaryKeyUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &naming::NotUniquePrimaryKeySnakeCase, {
            if let syn::Type::Path(value) = &primary_key_field.syn_field.ty {
                value.path.segments.clone()
            } else {
                panic!("primary key syn::Type in not syn::Type::Path");
            }
        })],
    );
    //todo use trait primary key and its type instead
    let not_unique_primary_key_update_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NotUniquePrimaryKeyUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::BadRequest400),
        vec![(macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &naming::NotUniquePrimaryKeySnakeCase, {
            if let syn::Type::Path(value) = &primary_key_field.syn_field.ty {
                value.path.segments.last().map_or_else(|| {panic!("no last path segment");}, |last_path_segment| {
                    let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                    for element in value.path.segments.iter().rev().skip(1).rev() {
                        handle.push_value(element.clone());
                        handle.push_punct(syn::token::PathSep {
                            spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                        });
                    }
                    handle.push_value(syn::PathSegment {
                        ident: proc_macro2::Ident::new(&naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&last_path_segment.ident).to_string(), proc_macro2::Span::call_site()),
                        arguments: syn::PathArguments::None,
                    });
                    handle
                })
            } else {
                panic!("primary key syn::Type in not syn::Type::Path");
            }
        })],
    );
    let no_payload_fields_primary_key_syn_variant_wrapper = new_syn_variant_wrapper(
        &naming::NoPayloadFieldsPrimaryKeyUpperCamelCase,
        Some(macros_helpers::status_code::StatusCode::InternalServerError500),
        vec![(macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &naming::NoPayloadFieldsPrimaryKeySnakeCase, {
            if let syn::Type::Path(value) = &primary_key_field.syn_field.ty {
                value.path.segments.last().map_or_else(|| {
                    panic!("no last path segment");
                }, |last_path_segment| {
                    let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
                    for element in value.path.segments.iter().rev().skip(1).rev() {
                        handle.push_value(element.clone());
                        handle.push_punct(syn::token::PathSep {
                            spans: [proc_macro2::Span::call_site(), proc_macro2::Span::call_site()],
                        });
                    }
                    handle.push_value(syn::PathSegment {
                        ident: proc_macro2::Ident::new(&naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&last_path_segment.ident).to_string(), proc_macro2::Span::call_site()),
                        arguments: syn::PathArguments::None,
                    });
                    handle
                })
            } else {
                panic!("primary key syn::Type in not syn::Type::Path");
            }
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
        assert!(value_ident_stringified == generate_postgresql_crud_attribute_stringified, "{value_ident_stringified} is not equal to {generate_postgresql_crud_attribute_stringified}");
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
        let mut value = vec![];
        value.push(check_body_size_syn_variant_wrapper.get_syn_variant());
        value.push(postgresql_syn_variant_wrapper.get_syn_variant());
        value.push(serde_json_syn_variant_wrapper.get_syn_variant());
        value.push(header_content_type_application_json_not_found_syn_variant_wrapper.get_syn_variant());
        for element in common_additional_error_variants_vec {
            value.push(element);
        }
        value
    };
    let common_route_with_row_and_rollback_syn_variants = {
        let mut value = vec![];
        for element in &common_route_syn_variants {
            value.push(*element);
        }
        value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
        value
    };
    let common_additional_route_logic_token_stream = macros_helpers::get_macro_attribute::get_macro_attribute_meta_list_token_stream(&syn_derive_input.attrs, &GeneratePostgresqlCrudAttribute::CommonAdditionalRouteLogic.generate_path_to_attribute());
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
        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_tokens_token_stream(&element.syn_field.ty, &naming::WhereElementUpperCamelCase);
        quote::quote! {
            pub #field_ident: std::option::Option<
                postgresql_crud::PostgresqlTypeWhere<
                    #as_postgresql_crud_postgresql_type_postgresql_type_token_stream
                >
            >
        }
    });
    let generate_pub_handle_token_stream = |is_pub: bool| if is_pub { quote::quote! {pub} } else { proc_macro2::TokenStream::new() };
    //todo check if vec is not empty
    let pub_handle_select_snake_case_std_vec_vec_ident_column_upper_camel_case_token_stream = {
        quote::quote! {pub #select_snake_case: std::vec::Vec<#ident_select_upper_camel_case>}
    };
    let generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream = |primary_key_type_token_stream: &dyn quote::ToTokens| {
        let is_pub = true;
        let pub_handle_token_stream = generate_pub_handle_token_stream(is_pub);
        quote::quote! {#pub_handle_token_stream #primary_key_field_ident: #primary_key_type_token_stream}
    };
    let update_fields_token_stream = {
        let fields_named_excluding_primary_key_token_stream = generate_fields_named_excluding_primary_key_token_stream(&|element: &SynFieldWrapper| {
            let field_ident = &element.field_ident;
            let path_value_token_stream = {
                let value = format!("{}::{}", naming::PostgresqlCrudSnakeCase, naming::ValueUpperCamelCase);
                value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
            };
            let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_tokens_token_stream(&element.syn_field.ty, &naming::UpdateUpperCamelCase);
            quote::quote! {
                pub #field_ident: std::option::Option<#path_value_token_stream<
                    #as_postgresql_crud_postgresql_type_postgresql_type_token_stream
                >>
            }
        });
        quote::quote! {
            pub #primary_key_field_ident: #primary_key_field_type_update_token_stream,
            #fields_named_excluding_primary_key_token_stream
        }
    };
    let generate_filter_no_payload_fields_token_stream = |operation: &Operation, source_token_stream: &dyn quote::ToTokens| {
        let no_payload_fields_primary_key_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &no_payload_fields_primary_key_syn_variant_wrapper, file!(), line!(), column!());
        let generate_content_token_stream = |content_token_stream: proc_macro2::TokenStream|{
            if fields_without_primary_key.len() > 1 {
                quote::quote!{(#content_token_stream)}
            }
            else {
                content_token_stream
            }
        };
        let none_fields_named_excluding_primary_key_content_token_stream = {
            let none_fields_named_excluding_primary_key_token_stream = fields_without_primary_key.iter().map(|_| naming::NoneUpperCamelCase);
            generate_content_token_stream(quote::quote!{#(#none_fields_named_excluding_primary_key_token_stream),*})
        };
        let match_fields_named_excluding_primary_key_content_token_stream = {
            let match_fields_named_excluding_primary_key_token_stream = fields_without_primary_key.iter().map(|element| {
                let field_ident = &element.field_ident;
                quote::quote! {&#source_token_stream.#field_ident}
            });
            generate_content_token_stream(quote::quote!{#(#match_fields_named_excluding_primary_key_token_stream),*})
        };
        quote::quote! {
            if let #none_fields_named_excluding_primary_key_content_token_stream = #match_fields_named_excluding_primary_key_content_token_stream {
                let #error_0_token_stream = #source_token_stream.#primary_key_field_ident.clone();
                #no_payload_fields_primary_key_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
            }
        }
    };
    let executor_snake_case = naming::ExecutorSnakeCase;
    let generate_match_postgres_transaction_rollback_await_token_stream =
        |operation: &Operation, postgresql_file: &'static str, postgresql_line: std::primitive::u32, postgresql_column: std::primitive::u32, row_and_rollback_file: &'static str, row_and_rollback_line: std::primitive::u32, row_and_rollback_column: std::primitive::u32| {
            let postgresql_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &postgresql_syn_variant_wrapper, postgresql_file, postgresql_line, postgresql_column);
            let row_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream =
                generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &row_and_rollback_syn_variant_wrapper, row_and_rollback_file, row_and_rollback_line, row_and_rollback_column);
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
    let generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream =
        |operation: &Operation, postgresql_file: &'static str, postgresql_line: std::primitive::u32, postgresql_column: std::primitive::u32, row_and_rollback_file: &'static str, row_and_rollback_line: std::primitive::u32, row_and_rollback_column: std::primitive::u32| {
            let match_postgres_transaction_rollback_await_token_stream = generate_match_postgres_transaction_rollback_await_token_stream(operation, postgresql_file, postgresql_line, postgresql_column, row_and_rollback_file, row_and_rollback_line, row_and_rollback_column);
            quote::quote! {
                drop(#rows_snake_case);
                #match_postgres_transaction_rollback_await_token_stream
            }
        };
    let expected_primary_keys_snake_case = naming::ExpectedPrimaryKeysSnakeCase;
    enum UpdateManyOrDeleteMany {
        UpdateMany,
        DeleteMany
    }
    impl std::convert::From<&UpdateManyOrDeleteMany> for Operation {
        fn from(value: &UpdateManyOrDeleteMany) -> Self {
            match &value {
                UpdateManyOrDeleteMany::UpdateMany => Self::UpdateMany,
                UpdateManyOrDeleteMany::DeleteMany => Self::DeleteMany,
            }
        }
    }
    //todo use it for delete_many too intead of using it only for update_many
    let generate_non_existing_primary_keys_check_token_stream = |update_many_or_delete_many: &UpdateManyOrDeleteMany, expected_primary_keys_token_stream: &dyn quote::ToTokens| {
        let current_operation = match update_many_or_delete_many {
            UpdateManyOrDeleteMany::UpdateMany => Operation::UpdateMany,
            UpdateManyOrDeleteMany::DeleteMany => Operation::DeleteMany,
        };
        let non_existing_primary_keys_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(
            &current_operation,
            match &update_many_or_delete_many {
                UpdateManyOrDeleteMany::UpdateMany => &non_existing_primary_keys_update_syn_variant_wrapper,
                UpdateManyOrDeleteMany::DeleteMany => &non_existing_primary_keys_delete_syn_variant_wrapper,
            },
            file!(),
            line!(),
            column!(),
        );
        let non_existing_primary_keys_and_rollback_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(
            &current_operation,
            match &update_many_or_delete_many {
                UpdateManyOrDeleteMany::UpdateMany => &non_existing_primary_keys_update_and_rollback_syn_variant_wrapper,
                UpdateManyOrDeleteMany::DeleteMany => &non_existing_primary_keys_delete_and_rollback_syn_variant_wrapper,
            },
            file!(),
            line!(),
            column!(),
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
    let generate_sqlx_row_try_get_primary_key_token_stream = |sqlx_row_try_get_type_token_stream: &dyn quote::ToTokens, ok_token_stream: &dyn quote::ToTokens, err_token_stream: &dyn quote::ToTokens| {
        quote::quote! {
            match #sqlx_row::try_get::<
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
        let syn::Fields::Named(fields_named) = &error_variant.fields else {
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
                                Some(value) => panic!("duplicated attributes ({}) are not supported", macros_helpers::attribute_ident_stringified::AttributeIdentStringified::attribute_ident_stringified(&value)),
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
        let ident_try_operation_route_logic_response_variants_upper_camel_case = generate_ident_try_operation_route_logic_response_variants_upper_camel_case(&operation);
        let ident_try_operation_route_logic_response_variants_token_stream = {
            let variants_token_stream = type_variants_from_request_response_syn_variants.iter().map(|element| macros_helpers::error_occurence::generate_serialize_deserialize_version_of_named_syn_variant(element));
            quote::quote! {
                #derive_debug_serde_serialize_serde_deserialize
                pub enum #ident_try_operation_route_logic_response_variants_upper_camel_case {
                    #desirable_upper_camel_case(#desirable_type_token_stream),
                    #(#variants_token_stream),*
                }
            }
        };
        let ident_try_operation_route_logic_error_named_upper_camel_case = generate_ident_try_operation_route_logic_error_named_upper_camel_case(&operation);
        let impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream = {
            let variants_token_stream = type_variants_from_request_response_syn_variants.iter().map(|element| {
                let variant_ident = &element.ident;
                let syn::Fields::Named(fields_named) = &element.fields else {
                    panic!("expected fields would be named");
                };
                let fields_mapped_into_token_stream = {
                    let fields_token_stream = fields_named.named.iter().map(|field| &field.ident);
                    quote::quote! {#(#fields_token_stream),*}
                };
                let ident_try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case = generate_ident_try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case(&operation);
                quote::quote! {
                    #ident_try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case::#variant_ident {
                        #fields_mapped_into_token_stream
                    } => Self::#variant_ident {
                        #fields_mapped_into_token_stream
                    }
                }
            });
            let into_serialize_deserialize_version_snake_case = naming::IntoSerializeDeserializeVersionSnakeCase;
            macros_helpers::generate_impl_std_convert_from_token_stream::generate_impl_std_convert_from_token_stream(
                &ident_try_operation_route_logic_error_named_upper_camel_case,
                &ident_try_operation_route_logic_response_variants_upper_camel_case,
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
                pub enum #ident_try_operation_route_logic_error_named_upper_camel_case {
                    #(#variants_token_stream),*
                }
            }
        };
        quote::quote! {
            #ident_try_operation_route_logic_response_variants_token_stream
            #impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_token_stream
            #try_operation_route_logic_error_named_token_stream
        }
    };
    let derive_debug_thiserror_error_occurence = token_patterns::DeriveDebugThiserrorErrorOccurence;
    let generate_ident_operation_payload_element_upper_camel_case = |operation: &Operation|{
        format!("{ident}{operation}PayloadElement")
        .parse::<proc_macro2::TokenStream>().unwrap()
    };
    let generate_ident_operation_payload_upper_camel_case = |operation: &Operation|{
        format!("{ident}{operation}Payload")
        .parse::<proc_macro2::TokenStream>().unwrap()
    };
    let generate_ident_operation_parameters_upper_camel_case = |operation: &Operation|{
        format!("{ident}{operation}Parameters")
        .parse::<proc_macro2::TokenStream>().unwrap()
    };
    let generate_parameters_pattern_token_stream = |operation: &Operation, payload_token_stream: proc_macro2::TokenStream| -> proc_macro2::TokenStream {
        let parameters_token_stream = {
            let ident_operation_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&operation);
            let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
            quote::quote! {
                #derive_debug
                pub struct #ident_operation_parameters_upper_camel_case {
                    pub #payload_snake_case: #ident_operation_payload_upper_camel_case,
                }
            }
        };
        quote::quote! {
            #payload_token_stream
            #parameters_token_stream
        }
    };
    let generate_parameters_pattern_payload_token_stream = |operation: &Operation, fields_token_stream: &dyn quote::ToTokens|{
        generate_parameters_pattern_token_stream(
            &operation,
            {
                let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
                quote::quote! {
                    #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
                    pub struct #ident_operation_payload_upper_camel_case {
                        #fields_token_stream
                    }
                }
            }
        )
    };
    let generate_parameters_pattern_payload_and_payload_element_token_stream = |operation: &Operation, fields_token_stream: &dyn quote::ToTokens|{
        generate_parameters_pattern_token_stream(
            &operation,
            {
                let ident_operation_payload_element_upper_camel_case = generate_ident_operation_payload_element_upper_camel_case(&operation);
                let ident_operation_payload_element_token_stream = quote::quote! {
                    #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
                    pub struct #ident_operation_payload_element_upper_camel_case {
                        #fields_token_stream
                    }
                };
                let ident_operation_payload_token_stream = {
                    let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
                    quote::quote! {
                        #derive_debug_serde_serialize_serde_deserialize_utoipa_to_schema
                        pub struct #ident_operation_payload_upper_camel_case(pub std::vec::Vec<#ident_operation_payload_element_upper_camel_case>);
                    }
                };
                quote::quote! {
                    #ident_operation_payload_element_token_stream
                    #ident_operation_payload_token_stream
                }
            }
        )
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
    let generate_ident_try_operation_error_named_token_stream = |operation: &Operation, common_http_request_syn_variants: &std::vec::Vec<syn::Variant>| -> proc_macro2::TokenStream {
        let ident_try_operation_error_named_upper_camel_case = generate_ident_try_operation_error_named_upper_camel_case(&operation);
        let syn_variants = {
            let mut value = vec![];
            for element in common_http_request_syn_variants {
                value.push(element.clone());
            }
            value.push({
                let ident_try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case = generate_ident_try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case(&operation);
                new_syn_variant_wrapper(
                    &ident_try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case,
                    None,
                    vec![(
                        macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(),
                        &naming::parameter::TrySelfRouteLogicErrorNamedWithSerializeDeserializeSnakeCase::from_display(operation),
                        macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&[&ident_try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case.to_string()]),
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
            pub enum #ident_try_operation_error_named_upper_camel_case {
                #(#variants_token_stream),*
            }
        }
    };
    let generate_try_operation_route_logic_token_stream = |
        operation: &Operation,
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
            let header_content_type_application_json_not_found_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = &generate_operation_error_initialization_eprintln_response_creation_token_stream(
                operation,
                &header_content_type_application_json_not_found_syn_variant_wrapper,
                file!(),
                line!(),
                column!()
            );
            let check_body_size_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = &generate_operation_error_initialization_eprintln_response_creation_token_stream(
                operation,
                &check_body_size_syn_variant_wrapper,
                file!(),
                line!(),
                column!()
            );
            quote::quote! {
                let (parts, #body_snake_case) = #request_snake_case.into_parts();
                let headers = parts.headers;
                if !matches!(
                    headers.get(axum::http::header::CONTENT_TYPE),
                    Some(value) if value == axum::http::header::HeaderValue::from_static("application/json")
                ) {
                    #header_content_type_application_json_not_found_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream
                }
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
            &{
                let ident_try_operation_route_logic_response_variants_upper_camel_case = generate_ident_try_operation_route_logic_response_variants_upper_camel_case(&operation);
                quote::quote! {#ident_try_operation_route_logic_response_variants_upper_camel_case::#desirable_upper_camel_case(#value_snake_case)}
            },
            &operation.desirable_status_code().to_axum_http_status_code_token_stream(),
        );
        quote::quote! {
            impl #ident {
                pub async fn #try_operation_route_logic_snake_case(
                    #app_state_snake_case: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>,
                    #request_snake_case: axum::extract::Request,
                ) -> axum::response::Response {
                    #request_parts_preparation_token_stream
                    #additional_validators_token_stream
                    #parameters_logic_token_stream
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
        }
    };
    let generate_parameters_logic_token_stream = |operation: &Operation, operation_payload_with_serialize_deserialize_check_token_stream: &dyn quote::ToTokens| -> proc_macro2::TokenStream {
        let body_bytes_snake_case = naming::BodyBytesSnakeCase;
        let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
        let serde_json_syn_variant_wrapper_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(operation, &serde_json_syn_variant_wrapper, file!(), line!(), column!());
        let ident_operation_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&operation);
        quote::quote! {
            let #parameters_snake_case = #ident_operation_parameters_upper_camel_case {
                //todo maybe use serde json parsing instead of axum. (coz less info)
                #payload_snake_case: match serde_json::from_slice::<#ident_operation_payload_upper_camel_case>(
                    &#body_bytes_snake_case,
                ) {
                    Ok(#value_snake_case) => {
                        let #value_snake_case = #ident_operation_payload_upper_camel_case::#from_snake_case(#value_snake_case);
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
    let pub_field_ident_field_type_fields_named_excluding_primary_key_token_stream = generate_fields_named_token_stream(&|element: &SynFieldWrapper| {
        let field_ident = &element.field_ident;
        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_tokens_token_stream(
            &element.syn_field.ty,
            &naming::CreateUpperCamelCase
        );
        quote::quote! {
            pub #field_ident: #as_postgresql_crud_postgresql_type_postgresql_type_token_stream
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
        let ident_try_operation_error_named_upper_camel_case = generate_ident_try_operation_error_named_upper_camel_case(&operation);
        let ident_operation_parameters_upper_camel_case = generate_ident_operation_parameters_upper_camel_case(&operation);
        let payload_token_stream = {
            let serde_json_to_string_syn_variant_initialization_token_stream = generate_initialization_token_stream(&serde_json_to_string_syn_variant_wrapper, file!(), line!(), column!());
            let operation_payload_with_serialize_deserialize_initialization_token_stream = {
                let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
                quote::quote! {#ident_operation_payload_upper_camel_case::#from_snake_case(#parameters_snake_case.#payload_snake_case)}
            };
            quote::quote! {
                let #payload_snake_case = {
                    #payload_check_token_stream
                    let #value_snake_case = #operation_payload_with_serialize_deserialize_initialization_token_stream;
                    match serde_json::to_string(&#value_snake_case) {
                        Ok(#value_snake_case) => #value_snake_case,
                        Err(#error_0_token_stream) => {
                            return Err(#ident_try_operation_error_named_upper_camel_case::#serde_json_to_string_syn_variant_initialization_token_stream);
                        }
                    }
                };
            }
        };
        let url_snake_case = naming::UrlSnakeCase;
        let endpoint_location_snake_case = naming::EndpointLocationSnakeCase;
        let url_token_stream = {
            let url_handle_token_stream = naming::UrlHandleSelfSnakeCaseTokenStream::url_handle_self_snake_case_token_stream(operation, &ident_snake_case_stringified);
            quote::quote! {
                let #url_snake_case = format!(
                    #url_handle_token_stream,
                    #endpoint_location_snake_case,
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
                        return Err(#ident_try_operation_error_named_upper_camel_case::#reqwest_syn_variant_initialization_token_stream);
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
                        return Err(#ident_try_operation_error_named_upper_camel_case::#failed_to_get_response_text_syn_variant_initialization_token_stream);
                    }
                };
            }
        };
        let ident_try_operation_route_logic_response_variants_upper_camel_case = generate_ident_try_operation_route_logic_response_variants_upper_camel_case(&operation);
        let expected_response_snake_case = naming::ExpectedResponseSnakeCase;
        let expected_response_token_stream = {
            let deserialize_response_syn_variant_initialization_token_stream = generate_initialization_token_stream(&deserialize_response_syn_variant_wrapper, file!(), line!(), column!());
            quote::quote! {
                let #expected_response_snake_case = match serde_json::from_str::<#ident_try_operation_route_logic_response_variants_upper_camel_case>(&#error_2_token_stream) {
                    Ok(#value_snake_case) => #value_snake_case,
                    Err(#error_3_token_stream) => {
                        return Err(#ident_try_operation_error_named_upper_camel_case::#deserialize_response_syn_variant_initialization_token_stream);
                    }
                };
            }
        };
        let try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case = generate_ident_try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case(&operation);
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
                    #ident_try_operation_route_logic_response_variants_upper_camel_case::#variant_ident {
                        #fields_idents_token_stream
                    } => #try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case::#variant_ident { #fields_idents_token_stream }
                }
            });
            quote::quote! {
                let #try_operation_route_logic_error_named_with_serialize_deserialize_snake_case = match #expected_response_snake_case {
                    #ident_try_operation_route_logic_response_variants_upper_camel_case::#desirable_upper_camel_case(#value_snake_case) => {
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
                Err(#ident_try_operation_error_named_upper_camel_case::#try_operation_route_logic_error_named_with_serialize_deserialize_upper_camel_case {
                    #try_operation_route_logic_error_named_with_serialize_deserialize_snake_case,
                    #field_code_occurence_new_6ac7b78e_da5d_4274_b58c_67bb9625d008_token_stream,
                })
            }
        };
        quote::quote! {
            impl #ident {
                pub async fn #try_operation_snake_case(
                    #endpoint_location_snake_case: #ref_std_primitive_str,
                    #parameters_snake_case: #ident_operation_parameters_upper_camel_case
                ) -> Result<#result_ok_type_token_stream, #ident_try_operation_error_named_upper_camel_case> {
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
        }
    };
    let generate_filter_not_unique_token_stream = |iterable_token_stream: &dyn quote::ToTokens, contains_token_stream: &dyn quote::ToTokens, push_token_stream: &dyn quote::ToTokens, error_token_stream: &dyn quote::ToTokens, return_error_token_stream: &dyn quote::ToTokens| {
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
    enum CreateManyOrUpdateManyOrDeleteMany {
        CreateMany,
        UpdateMany,
        DeleteMany
    }
    impl std::convert::From<&CreateManyOrUpdateManyOrDeleteMany> for Operation {
        fn from(value: &CreateManyOrUpdateManyOrDeleteMany) -> Self {
            match &value {
                CreateManyOrUpdateManyOrDeleteMany::CreateMany => Self::CreateMany,
                CreateManyOrUpdateManyOrDeleteMany::UpdateMany => Self::UpdateMany,
                CreateManyOrUpdateManyOrDeleteMany::DeleteMany => Self::DeleteMany,
            }
        }
    }
    let generate_create_update_delete_many_fetch_token_stream = |create_many_or_update_many_or_delete_many: &CreateManyOrUpdateManyOrDeleteMany| {
        let current_operation = match &create_many_or_update_many_or_delete_many {
            CreateManyOrUpdateManyOrDeleteMany::CreateMany => Operation::CreateMany,
            CreateManyOrUpdateManyOrDeleteMany::UpdateMany => Operation::UpdateMany,
            CreateManyOrUpdateManyOrDeleteMany::DeleteMany => Operation::DeleteMany,
        };
        generate_fetch_token_stream(
            &generate_sqlx_row_try_get_primary_key_token_stream(
                &match &create_many_or_update_many_or_delete_many {
                    CreateManyOrUpdateManyOrDeleteMany::CreateMany |
                    CreateManyOrUpdateManyOrDeleteMany::DeleteMany => quote::quote! {#primary_key_field_type_as_primary_key_upper_camel_case},
                    CreateManyOrUpdateManyOrDeleteMany::UpdateMany => quote::quote! {#primary_key_field_type_as_postgresql_type_update_token_stream},
                },
                &quote::quote! {Some(#value_snake_case)},
                &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
            ),
            &generate_drop_rows_match_postgres_transaction_rollback_await_handle_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
        )
    };
    enum CreateOneOrUpdateOneOrDeleteOne {
        CreateOne,
        UpdateOne,
        DeleteOne
    }
    impl std::convert::From<&CreateOneOrUpdateOneOrDeleteOne> for Operation {
        fn from(value: &CreateOneOrUpdateOneOrDeleteOne) -> Self {
            match &value {
                CreateOneOrUpdateOneOrDeleteOne::CreateOne => Self::CreateOne,
                CreateOneOrUpdateOneOrDeleteOne::UpdateOne => Self::UpdateOne,
                CreateOneOrUpdateOneOrDeleteOne::DeleteOne => Self::DeleteOne,
            }
        }
    }
    let generate_create_update_delete_one_fetch_token_stream = |create_one_or_update_one_or_delete_one: &CreateOneOrUpdateOneOrDeleteOne| {
        let current_operation = match &create_one_or_update_one_or_delete_one {
            CreateOneOrUpdateOneOrDeleteOne::CreateOne => Operation::CreateOne,
            CreateOneOrUpdateOneOrDeleteOne::UpdateOne => Operation::UpdateOne,
            CreateOneOrUpdateOneOrDeleteOne::DeleteOne => Operation::DeleteOne,
        };
        generate_fetch_one_token_stream(
            &generate_sqlx_row_try_get_primary_key_token_stream(
                &quote::quote! {#primary_key_field_type_as_primary_key_upper_camel_case},
                &value_snake_case,
                &generate_match_postgres_transaction_rollback_await_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
            ),
            &generate_match_postgres_transaction_rollback_await_token_stream(&current_operation, file!(), line!(), column!(), file!(), line!(), column!()),
        )
    };
    let generate_operation_payload_example_route_logic_token_stream = |operation: &Operation| {
        let operation_payload_example_route_logic_snake_case = naming::parameter::SelfPayloadExampleRouteLogicSnakeCase::from_display(operation);
        let wraped_into_axum_response_token_stream = wrap_into_axum_response_token_stream(
            &{
                let default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case = naming::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementUpperCamelCase;
                let ident_operation_payload_upper_camel_case = generate_ident_operation_payload_upper_camel_case(&operation);
                quote::quote! {<#ident_operation_payload_upper_camel_case as postgresql_crud::#default_but_option_is_always_some_and_vec_always_contains_one_element_upper_camel_case>::#default_but_option_is_always_some_and_vec_always_contains_one_element_snake_case()}
            },
            &quote::quote! {axum::http::StatusCode::OK},
        );
        quote::quote! {
            impl #ident {
                pub async fn #operation_payload_example_route_logic_snake_case() -> axum::response::Response {
                    #wraped_into_axum_response_token_stream
                }
            }
        }
    };
    let fields_initialiation_excluding_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_curly_braces_token_stream = {
        let fields_initialiation_excluding_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
            let value = fields.iter().map(|element| {
                let field_ident = &element.field_ident;
                quote::quote! {
                    #field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                }
            });
            quote::quote! {#(#value),*}
        };
        quote::quote! {
            Self{#fields_initialiation_excluding_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream}
        }
    };
    let generate_maybe_is_first_push_to_additional_parameters_already_happend_true_token_stream = |index: std::primitive::usize|{
        if index == fields_len_minus_one {
            proc_macro2::TokenStream::new()
        }
        else {
            quote::quote!{is_first_push_to_additional_parameters_already_happend = true;}
        }
    };
    let generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream = |ident: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens|{
        postgresql_crud_macros_common::generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_token_stream(
            &ident,
            &proc_macro2::TokenStream::new(),
            &content_token_stream
        )
    };
    let create_many_token_stream = {
        let operation = Operation::CreateMany;
        let expected_length_snake_case = naming::ExpectedLengthSnakeCase;
        let got_length_snake_case = naming::GotLengthSnakeCase;
        let std_primitive_usize_syn_punctuated_punctuated = macros_helpers::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(&["std", "primitive", "usize"]);
        let unexpected_rows_length_syn_variant_wrapper = new_syn_variant_wrapper(
            &naming::UnexpectedRowsLengthUpperCamelCase,
            Some(macros_helpers::status_code::StatusCode::InternalServerError500),
            vec![
                (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &expected_length_snake_case, std_primitive_usize_syn_punctuated_punctuated.clone()),
                (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &got_length_snake_case, std_primitive_usize_syn_punctuated_punctuated.clone()),
            ],
        );
        let unexpected_rows_length_and_rollback_syn_variant_wrapper = new_syn_variant_wrapper(
            &naming::UnexpectedRowsLengthAndRollbackUpperCamelCase,
            Some(macros_helpers::status_code::StatusCode::InternalServerError500),
            vec![
                (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &expected_length_snake_case, std_primitive_usize_syn_punctuated_punctuated.clone()),
                (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &got_length_snake_case, std_primitive_usize_syn_punctuated_punctuated),
                //todo reuse vec elements
                (macros_helpers_error_occurence_error_occurence_field_attribute_eo_to_std_string_string.clone(), &rollback_snake_case, sqlx_error_syn_punctuated_punctuated),
            ],
        );
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = vec![];
                for element in &common_route_syn_variants {
                    value.push(*element);
                }
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(unexpected_rows_length_syn_variant_wrapper.get_syn_variant());
                value.push(unexpected_rows_length_and_rollback_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_payload_and_payload_element_token_stream(
            &operation,
            &pub_field_ident_field_type_fields_named_excluding_primary_key_token_stream
        );
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                &operation,
                &std_vec_vec_primary_key_field_type_read_token_stream,
                &type_variants_from_request_response_syn_variants,
            );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let mut column_names = fields.iter().fold(std::string::String::default(), |mut acc, element| {
                        acc.push_str(&format!("{}", &element.field_ident));
                        acc.push(',');
                        acc
                    });
                    let _: Option<char> = column_names.pop();
                    let column_increments_token_stream = fields.iter().map(|element| {
                        let element_field_ident = &element.field_ident;
                        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_token_stream(&element.syn_field.ty);
                        let checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                        quote::quote! {
                            match #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #create_query_part_snake_case(&#element_snake_case.#element_field_ident, &mut #increment_snake_case) {
                                Ok(#value_snake_case) => {
                                    #acc_snake_case.push_str(&format!("{value},"));
                                },
                                Err(#error_0_token_stream) => {
                                    #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        }
                    });
                    let query_token_stream = generate_quotes::double_quotes_token_stream(&format!("{insert_snake_case} {into_snake_case} {ident_snake_case_stringified} ({column_names}) {values_snake_case} {{values}} {returning_primary_key_stringified}"));
                    quote::quote! {
                        {
                            #increment_initialization_token_stream
                            let mut values = #std_string_string::default();
                            for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                let mut #acc_snake_case = #std_string_string::default();
                                #(#column_increments_token_stream)*
                                let _: Option<char> = #acc_snake_case.pop();
                                values.push_str(&format!("({acc}),"));
                            }
                            let _: Option<char> = values.pop();
                            format!(#query_token_stream)
                        }
                    }
                };
                let binded_query_token_stream = {
                    let query_string_snake_case = naming::QueryStringSnakeCase;
                    let query_bind_token_stream = fields.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_token_stream(&element.syn_field.ty);
                        quote::quote! {
                            #query_snake_case = #as_postgresql_crud_postgresql_type_postgresql_type_token_stream create_query_bind(
                                #element_snake_case.#field_ident,
                                #query_snake_case
                            );
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
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &{
                        let fetch_token_stream = generate_create_update_delete_many_fetch_token_stream(
                            &CreateManyOrUpdateManyOrDeleteMany::CreateMany
                        );
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
                    }
                );
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
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_primary_key_field_type_read_token_stream,
                &proc_macro2::TokenStream::new(),
                &value_snake_case,
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream = {
            let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_element_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
                    &generate_ident_operation_payload_element_upper_camel_case(&operation),
                    &fields_initialiation_excluding_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_curly_braces_token_stream,
                );
            let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
                &generate_ident_operation_payload_upper_camel_case(&operation),
                &quote::quote! {Self(vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream])},
            );
            quote::quote! {
                #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_element_token_stream
                #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
            }
        };
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
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
                let mut value = vec![];
                for element in &common_route_with_row_and_rollback_syn_variants {
                    value.push(*element);
                }
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_payload_token_stream(
            &operation,
            &pub_field_ident_field_type_fields_named_excluding_primary_key_token_stream
        );
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream =
                generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                    &operation,
                    &primary_key_field_type_as_primary_key_upper_camel_case,
                    &type_variants_from_request_response_syn_variants,
                );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let mut column_names = fields.iter().fold(std::string::String::default(), |mut acc, element| {
                        acc.push_str(&format!("{}", &element.field_ident));
                        acc.push_str(",");
                        acc
                    });
                    let _: Option<char> = column_names.pop();
                    let mut column_increments = fields.iter().fold(std::string::String::default(), |mut acc, _| {
                        acc.push_str("{}");
                        acc.push_str(",");
                        acc
                    });
                    let _: Option<char> = column_increments.pop();
                    let format_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{insert_snake_case} {into_snake_case} {ident_snake_case_stringified} ({column_names}) {values_snake_case} ({column_increments}) {returning_primary_key_stringified}"));
                    let query_part_token_stream = fields.iter().map(|element| {
                        let element_field_ident = &element.field_ident;
                        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_token_stream(&element.syn_field.ty);
                        let checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                        quote::quote! {
                            match #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #create_query_part_snake_case(&#parameters_snake_case.#payload_snake_case.#element_field_ident, &mut #increment_snake_case) {
                                Ok(#value_snake_case) => #value_snake_case,
                                Err(#error_0_token_stream) => {
                                    #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                }
                            }
                        }
                    });
                    quote::quote! {
                        {
                            let mut increment: std::primitive::u64 = 0;
                            format!(
                                #format_handle_token_stream,
                                #(#query_part_token_stream),*
                            )
                        }
                    }
                };
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = fields.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_token_stream(&element.syn_field.ty);
                        quote::quote! {
                            query = #as_postgresql_crud_postgresql_type_postgresql_type_token_stream create_query_bind(
                                #parameters_snake_case.#payload_snake_case.#field_ident,
                                query
                            );
                        }
                    });
                    quote::quote! {
                        let mut query = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#binded_query_modifications_token_stream)*
                        query
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &generate_create_update_delete_one_fetch_token_stream(&CreateOneOrUpdateOneOrDeleteOne::CreateOne)
                );
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
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &primary_key_field_type_as_primary_key_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &value_snake_case,
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
            &generate_ident_operation_payload_upper_camel_case(&operation),
            &fields_initialiation_excluding_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_curly_braces_token_stream,
        );
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
            #operation_payload_example_route_logic_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &create_one_token_stream,
    // );
    let read_many_token_stream = {
        let operation = Operation::ReadMany;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(
            &{
                let mut value = vec![];
                for element in &common_route_syn_variants {
                    value.push(*element);
                }
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value.push(not_unique_column_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_payload_token_stream(
            &operation,
            &quote::quote! {
                #pub_fields_idents_std_option_option_std_vec_vec_where_inner_type_token_stream,
                #pub_handle_select_snake_case_std_vec_vec_ident_column_upper_camel_case_token_stream,
                pub #order_by_snake_case: #postgresql_crud_order_by_token_stream<#ident_select_upper_camel_case>,
                pub pagination: postgresql_crud::PaginationStartsWithZero,
            }
        );
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream =
                generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                    &operation,
                    &std_vec_vec_struct_options_ident_token_stream,
                    &type_variants_from_request_response_syn_variants,
                );
            let try_operation_route_logic_token_stream = {
                //todo maybe but checks into constructor function and use it inside deserilizaton serde impl
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let additional_parameters_modification_token_stream = fields.iter().enumerate().map(|(index, element)| {
                        let field_ident = &element.field_ident;
                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                        let maybe_is_first_push_to_additional_parameters_already_happend_true_token_stream = generate_maybe_is_first_push_to_additional_parameters_already_happend_true_token_stream(index);
                        let checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                        quote::quote! {
                            if let Some(#value_snake_case) = &#parameters_snake_case.#payload_snake_case.#field_ident {
                                match #postgresql_crud_postgresql_type_where_filter_query_part_token_stream(
                                    #value_snake_case,
                                    &mut increment,
                                    &#field_ident_double_quotes_token_stream,
                                    is_first_push_to_additional_parameters_already_happend,//todo generate is in proc macro (first element ignore)
                                ) {
                                    Ok(value) => {
                                        additional_parameters.push_str(&value);
                                        #maybe_is_first_push_to_additional_parameters_already_happend_true_token_stream
                                    }
                                    Err(#error_0_token_stream) => {
                                        #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                }
                            }
                        }
                    });
                    let handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{select_snake_case} {{}} {from_snake_case} {ident_snake_case_stringified} {{}}"));
                    let by_snake_case = naming::BySnakeCase;
                    let additional_parameters_order_by_handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{{}}{order_snake_case} {by_snake_case} {{}} {{}}"));
                    let prefix_snake_case = naming::PrefixSnakeCase;
                    let prefix_to_additional_parameters_token_stream = quote::quote! {
                        let #prefix_snake_case = match additional_parameters.is_empty() {
                            true => "",
                            false => " ",
                        };
                    };
                    let checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                    quote::quote! {
                        {
                            format!(
                                #handle_token_stream,
                                #select_query_part_vec_column_token_stream,
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
                                            #value_snake_case.#column_snake_case.pick_select(),//todo refactor pick_select
                                            #order_snake_case,
                                        ));
                                    }
                                    {
                                        #prefix_to_additional_parameters_token_stream
                                        let #value_snake_case = match #postgresql_crud_postgresql_type_where_filter_query_part_token_stream(
                                            &#parameters_snake_case.#payload_snake_case.pagination,
                                            &mut #increment_snake_case,
                                            &"",
                                            std::primitive::bool::default()
                                        ) {
                                            Ok(#value_snake_case) => #value_snake_case,
                                            Err(#error_0_token_stream) => {
                                                #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
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
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = fields.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        quote::quote! {
                            if let Some(#value_snake_case) = #parameters_snake_case.#payload_snake_case.#field_ident {
                                query = #postgresql_crud_postgresql_type_where_filter_query_bind_token_stream(
                                    value,
                                    query
                                );
                            }
                        }
                    });
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#binded_query_modifications_token_stream)*
                        #query_snake_case = #postgresql_crud_postgresql_type_where_filter_query_bind_token_stream(
                            #parameters_snake_case.#payload_snake_case.pagination,
                            #query_snake_case,
                        );
                        #query_snake_case
                    }
                };
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
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &{
                let mut value = common_http_request_syn_variants.clone();
                value.push(not_unique_column_syn_variant_wrapper.get_syn_variant().clone());
                value
            });
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_struct_options_ident_token_stream,
                &{
                    //todo where logic was commented. untill rewriting better where logic there is not point to add it again
                    quote::quote! {}
                },
                &quote::quote! {
                    #value_snake_case
                    .into_iter()
                    .fold(std::vec::Vec::new(), |mut #acc_snake_case, #element_snake_case| {
                        #acc_snake_case.push(#element_snake_case);
                        #acc_snake_case
                    })
                },
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
                &generate_ident_operation_payload_upper_camel_case(&operation),
                &{
                    let fields_token_stream = fields.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        quote::quote! {
                            #field_ident: Some(
                                #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                            ),
                        }
                    });
                    quote::quote! {
                        Self {
                            #(#fields_token_stream)*
                            #select_snake_case: #postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                            #order_by_snake_case: postgresql_crud::OrderBy {
                                #column_snake_case: #ident_select_upper_camel_case::#primary_key_field_ident_upper_camel_case_token_stream(
                                    #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                ),
                                #order_snake_case: Some(
                                    #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                                ),
                            },
                            pagination: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                        }
                    }
                }
            );
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
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
                let mut value = vec![];
                for element in &common_route_syn_variants {
                    value.push(*element);
                }
                value.push(not_unique_column_syn_variant_wrapper.get_syn_variant());
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_payload_token_stream(
            &operation,
            &{
                let pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream = generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream(
                    &naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(&primary_key_field.syn_field.ty)
                );
                quote::quote! {
                    #pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream,
                    #pub_handle_select_snake_case_std_vec_vec_ident_column_upper_camel_case_token_stream,
                }
            }
        );
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream =
                generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                    &operation,
                    &ident_read_upper_camel_case,
                    &type_variants_from_request_response_syn_variants,
                );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &{
                    //todo where logic was commented. untill rewriting better where logic there is not point to add it again
                    quote::quote! {}
                });
                let query_string_token_stream = {
                    let query_token_stream = generate_quotes::double_quotes_token_stream(
                        &format!("{select_snake_case} {{}} {from_snake_case} {ident_snake_case_stringified} {where_snake_case} {{}}"), //{primary_key_field_ident} = $1
                    );
                    let checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                    quote::quote! {
                        {
                            let mut increment: std::primitive::u64 = 0;
                            format!(
                                #query_token_stream,
                                #select_query_part_vec_column_token_stream,
                                match #postgresql_crud_postgresql_type_where_filter_query_part_token_stream(
                                    &parameters.payload.#primary_key_field_ident,
                                    &mut increment,
                                    &#primary_key_field_ident_double_quotes_token_stream,
                                    false
                                ) {
                                    Ok(value) => value,
                                    Err(#error_0_token_stream) => {
                                        #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                }
                            )
                        }
                    }
                };
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = quote::quote! {
                        let #query_snake_case = #postgresql_crud_postgresql_type_where_filter_query_bind_token_stream(#parameters_snake_case.#payload_snake_case.#primary_key_field_ident, #query_snake_case);
                    };
                    quote::quote! {
                        let #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #binded_query_modifications_token_stream
                        #query_snake_case
                    }
                };
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
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &{
                let mut value = common_http_request_syn_variants.clone();
                value.push(not_unique_column_syn_variant_wrapper.get_syn_variant().clone());
                value
            });
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &ident_read_upper_camel_case,
                &{
                    //todo where logic was commented. untill rewriting better where logic there is not point to add it again
                    quote::quote! {}
                },
                &quote::quote! {#value_snake_case},
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
            &generate_ident_operation_payload_upper_camel_case(&operation),
            &quote::quote! {
                Self {
                    #primary_key_field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream,
                    #select_snake_case: #postgresql_crud_all_enum_variants_array_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                }
            },
        );
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
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
                let mut value = vec![];
                for element in &common_route_syn_variants {
                    value.push(*element);
                }
                value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(non_existing_primary_keys_update_syn_variant_wrapper.get_syn_variant());
                value.push(non_existing_primary_keys_update_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(not_unique_primary_key_update_syn_variant_wrapper.get_syn_variant());
                value.push(no_payload_fields_primary_key_syn_variant_wrapper.get_syn_variant());
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_payload_and_payload_element_token_stream(
            &operation,
            &update_fields_token_stream
        );
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream = generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                &operation,
                &std_vec_vec_primary_key_field_type_as_postgresql_type_update_token_stream,
                &type_variants_from_request_response_syn_variants,
            );
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
                        .collect::<#std_vec_vec_primary_key_field_type_as_postgresql_type_update_token_stream>();
                };
                let query_string_token_stream = {
                    let query_start_token_stream = generate_quotes::double_quotes_token_stream(&format!("{update_snake_case} {ident_snake_case_stringified} {set_snake_case} "));
                    let query_snake_case = naming::QuerySnakeCase;
                    let fields_named_excluding_primary_key_update_assignment_token_stream = fields_without_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                        let is_field_ident_update_exists_snake_case = naming::parameter::IsSelfUpdateExistSnakeCase::from_tokens(&field_ident);
                        let case_snake_case = naming::CaseSnakeCase;
                        let field_ident_equals_case_token_stream = generate_quotes::double_quotes_token_stream(&format!("{field_ident} = {case_snake_case} "));
                        let else_snake_case = naming::ElseSnakeCase;
                        let end_snake_case = naming::EndSnakeCase;
                        let else_field_ident_end_token_stream = generate_quotes::double_quotes_token_stream(&format!("{else_snake_case} {field_ident} {end_snake_case},"));
                        let when_primary_key_field_ident_equals_then_token_stream = generate_quotes::double_quotes_token_stream(&format!("{} {primary_key_field_ident} = {{}} {} {{}} ", naming::WhenSnakeCase, naming::ThenSnakeCase));
                        let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
                        let field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_token_stream(&element.syn_field.ty);
                        let checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
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
                                                match #primary_key_field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_part_snake_case(
                                                    &#element_snake_case.#primary_key_field_ident,
                                                    &"",
                                                    &#primary_key_field_ident_double_quotes_token_stream,
                                                    &"",
                                                    &mut #increment_snake_case,
                                                ) {
                                                    Ok(#value_snake_case) => #value_snake_case,
                                                    Err(#error_0_token_stream) => {
                                                        #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                    }
                                                },
                                                match #field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_part_snake_case(
                                                    &#value_snake_case.#value_snake_case,
                                                    &"",
                                                    &#field_ident_double_quotes_token_stream,
                                                    &"",
                                                    &mut #increment_snake_case,
                                                ) {
                                                    Ok(#value_snake_case) => #value_snake_case,
                                                    Err(#error_0_token_stream) => {
                                                        #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                                    }
                                                }
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
                        let where_primary_key_field_ident_in_primary_keys_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&format!(" {where_snake_case} {primary_key_field_ident} {} ({{}}) {returning_snake_case} {primary_key_field_ident};", naming::InSnakeCase));
                        let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
                        let checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                        quote::quote! {
                            #query_snake_case.push_str(&format!(
                                #where_primary_key_field_ident_in_primary_keys_double_quotes_token_stream,
                                {
                                    let mut #acc_snake_case = #std_string_string::default();
                                    for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                        match #primary_key_field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_part_snake_case(
                                            &#element_snake_case.#primary_key_field_ident,
                                            &"",
                                            &#primary_key_field_ident_double_quotes_token_stream,
                                            &"",
                                            &mut #increment_snake_case,
                                        ) {
                                            Ok(#value_snake_case) => {
                                                #acc_snake_case.push_str(&format!("{value},"));
                                            },
                                            Err(#error_0_token_stream) => {
                                                #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                            }
                                        }
                                    }
                                    let _: Option<char> = #acc_snake_case.pop();
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
                            let _: Option<char> = #query_snake_case.pop();
                            #where_primary_key_field_ident_in_primary_keys_returning_primary_key_field_ident_token_stream
                            #query_snake_case
                        }
                    }
                };
                let binded_query_token_stream = {
                    let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
                    let fields_named_excluding_primary_key_update_assignment_token_stream = fields_without_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let is_field_ident_update_exists_snake_case = naming::parameter::IsSelfUpdateExistSnakeCase::from_tokens(&field_ident);
                        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_token_stream(&element.syn_field.ty);
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
                                            #query_snake_case = #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_bind_snake_case(
                                                #value_snake_case.#value_snake_case.clone(),
                                                #query_snake_case,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    });
                    let primary_key_update_assignment_token_stream = quote::quote! {
                        {
                            for #element_snake_case in &#parameters_snake_case.#payload_snake_case.0 {
                                #query_snake_case = #primary_key_field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_bind_snake_case(
                                    #element_snake_case.#primary_key_field_ident.clone(),
                                    #query_snake_case,
                                );
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
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation, 
                    &{
                        let fetch_token_stream = generate_create_update_delete_many_fetch_token_stream(&CreateManyOrUpdateManyOrDeleteMany::UpdateMany);
                        let non_existing_primary_keys_check_token_stream = generate_non_existing_primary_keys_check_token_stream(
                            &UpdateManyOrDeleteMany::UpdateMany,
                            &expected_primary_keys_snake_case
                        );
                        quote::quote! {
                            #fetch_token_stream
                            {
                                #non_existing_primary_keys_check_token_stream
                            }
                        }
                    }
                );
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
            quote::quote! {
                #try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream
                #try_operation_route_logic_token_stream
            }
        };
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &{
                let mut value = common_http_request_syn_variants.clone();
                value.push(not_unique_primary_key_update_syn_variant_wrapper.get_syn_variant().clone());
                value
            });
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_primary_key_field_type_as_postgresql_type_update_token_stream,
                &{
                    let filter_not_unique_primary_key_token_stream = generate_filter_not_unique_token_stream(
                        &quote::quote! {&#parameters_snake_case.#payload_snake_case.0},
                        &quote::quote! {&#element_snake_case.#primary_key_field_ident},
                        &quote::quote! {#element_snake_case.#primary_key_field_ident},
                        &quote::quote! {#element_snake_case.#primary_key_field_ident.clone()},
                        &{
                            let ident_try_operation_error_named_upper_camel_case = generate_ident_try_operation_error_named_upper_camel_case(&operation);
                            let not_unique_primary_key_syn_variant_initialization_token_stream = generate_initialization_token_stream(&not_unique_primary_key_update_syn_variant_wrapper, file!(), line!(), column!());
                            quote::quote! {
                                return Err(#ident_try_operation_error_named_upper_camel_case::#not_unique_primary_key_syn_variant_initialization_token_stream);
                            }
                        },
                    );
                    quote::quote! {
                        #filter_not_unique_primary_key_token_stream
                    }
                },
                &value_snake_case,
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream = {
            let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_element_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
                &generate_ident_operation_payload_element_upper_camel_case(&operation),
                &{
                    let primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                        quote::quote! {
                            #primary_key_field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                        }
                    };
                    let fields_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = fields_without_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        quote::quote! {
                            #field_ident: Some(postgresql_crud::Value{
                                #value_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                            })
                        }
                    });
                    quote::quote! {
                        Self{
                            #primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream,
                            #(#fields_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream),*
                        }
                    }
                }
            );
            let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
                &generate_ident_operation_payload_upper_camel_case(&operation),
                &quote::quote! {
                    Self(vec![#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream])
                },
            );
            quote::quote! {
                #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_element_token_stream
                #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
            }
        };
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
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
                let mut value = vec![];
                for element in &common_route_syn_variants {
                    value.push(*element);
                }
                value.push(no_payload_fields_primary_key_syn_variant_wrapper.get_syn_variant());
                value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_payload_token_stream(
            &operation,
            &update_fields_token_stream
        );
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream =
                generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                    &operation,
                    &primary_key_field_type_as_primary_key_upper_camel_case,
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
                    let update_query_part_snake_case = naming::UpdateQueryPartSnakeCase;
                    let additional_parameters_modification_token_stream = fields_without_primary_key
                        .iter()
                        .map(|element| {
                            let field_ident = &element.field_ident;
                            let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                            let field_ident_equals_dollar_increment_token_stream = generate_quotes::double_quotes_token_stream(&format!("{field_ident} = {{{value_snake_case}}},"));
                            let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_token_stream(&element.syn_field.ty);
                            let checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                            quote::quote! {
                                if let Some(#value_snake_case) = &#parameters_snake_case.#payload_snake_case.#field_ident {
                                    match #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_part_snake_case(
                                        &#value_snake_case.#value_snake_case,
                                        &"",
                                        &#field_ident_double_quotes_token_stream,
                                        &"",
                                        &mut #increment_snake_case,
                                    ) {
                                        Ok(#value_snake_case) => {
                                            //todo fix it. its incorrect
                                            #query_snake_case.push_str(&format!(#field_ident_equals_dollar_increment_token_stream));
                                        }
                                        Err(#error_0_token_stream) => {
                                            #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                        }
                                    }
                                }
                            }
                        })
                        .collect::<std::vec::Vec<proc_macro2::TokenStream>>();
                    let additional_parameters_primary_key_modification_token_stream = {
                        let query_part_token_stream = generate_quotes::double_quotes_token_stream(&format!(" {where_snake_case} {primary_key_field_ident} = {{{value_snake_case}}}"));
                        let checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                        quote::quote! {
                            match #primary_key_field_type_as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_part_snake_case(
                                &#parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                                &"",
                                &#primary_key_field_ident_double_quotes_token_stream,
                                &"",
                                &mut #increment_snake_case,
                            ) {
                                Ok(#value_snake_case) => {
                                    //todo fix it. its incorrect
                                    #query_snake_case.push_str(&format!(#query_part_token_stream));
                                },
                                Err(#error_0_token_stream) => {
                                    #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                },
                            }
                        }
                    };
                    quote::quote! {
                        {
                            #increment_initialization_token_stream
                            let mut #query_snake_case = #std_string_string::#from_snake_case(#query_start_token_stream);
                            #(#additional_parameters_modification_token_stream)*
                            let _: Option<char> = #query_snake_case.pop();
                            #additional_parameters_primary_key_modification_token_stream
                            #query_snake_case.push_str(&format!(#returning_primary_key_double_quotes_token_stream));
                            #query_snake_case
                        }
                    }
                };
                let binded_query_token_stream = {
                    let update_query_bind_snake_case = naming::UpdateQueryBindSnakeCase;
                    let binded_query_modifications_token_stream = fields_without_primary_key.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        let as_postgresql_crud_postgresql_type_postgresql_type_token_stream = generate_as_postgresql_crud_postgresql_type_postgresql_type_token_stream(&element.syn_field.ty);
                        quote::quote! {
                            if let Some(#value_snake_case) = #parameters_snake_case.#payload_snake_case.#field_ident {
                                #query_snake_case = #as_postgresql_crud_postgresql_type_postgresql_type_token_stream #update_query_bind_snake_case(
                                    #value_snake_case.#value_snake_case,
                                    #query_snake_case,
                                );
                            }
                        }
                    });
                    let binded_query_primary_key_modification_token_stream = quote::quote! {
                        #query_snake_case = <#primary_key_field_type as postgresql_crud::PostgresqlType>::#update_query_bind_snake_case(
                            #parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                            #query_snake_case,
                        );
                    };
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#binded_query_modifications_token_stream)*
                        #binded_query_primary_key_modification_token_stream
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &generate_create_update_delete_one_fetch_token_stream(&CreateOneOrUpdateOneOrDeleteOne::UpdateOne)
                );
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
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &primary_key_field_type_as_primary_key_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &value_snake_case,
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
            &generate_ident_operation_payload_upper_camel_case(&operation),
            &{
                let primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                    quote::quote! {
                        #primary_key_field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                    }
                };
                let fields_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = fields_without_primary_key.iter().map(|element| {
                    let field_ident = &element.field_ident;
                    quote::quote! {
                        #field_ident: Some(postgresql_crud::Value{
                            #value_snake_case: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                        })
                    }
                });
                quote::quote! {
                    Self {
                        #primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream,
                        #(#fields_without_primary_key_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream),*
                    }
                }
            }
        );
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
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
                let mut value = vec![];
                for element in &common_route_syn_variants {
                    value.push(*element);
                }
                value.push(no_payload_fields_syn_variant_wrapper.get_syn_variant());
                value.push(no_primary_keys_syn_variant_wrapper.get_syn_variant());
                value.push(row_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(non_existing_primary_keys_delete_syn_variant_wrapper.get_syn_variant());
                value.push(non_existing_primary_keys_delete_and_rollback_syn_variant_wrapper.get_syn_variant());
                value.push(not_unique_primary_key_update_syn_variant_wrapper.get_syn_variant());
                value.push(query_part_syn_variant_wrapper.get_syn_variant());
                value
            },
            &operation,
        );
        let parameters_token_stream = generate_parameters_pattern_payload_token_stream(
            &operation,
            &pub_fields_idents_std_option_option_std_vec_vec_where_inner_type_token_stream
        );
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream =
                generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                    &operation,
                    &std_vec_vec_primary_key_field_type_read_token_stream,
                    &type_variants_from_request_response_syn_variants,
                );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &{
                    //todo where logic was commented. untill rewriting better where logic there is not point to add it again
                    quote::quote! {}
                });
                let query_string_token_stream = {
                    let additional_parameters_modification_token_stream = fields.iter().enumerate().map(|(index, element)| {
                        let field_ident = &element.field_ident;
                        let field_ident_double_quotes_token_stream = generate_quotes::double_quotes_token_stream(&field_ident);
                        let maybe_is_first_push_to_additional_parameters_already_happend_true_token_stream = generate_maybe_is_first_push_to_additional_parameters_already_happend_true_token_stream(index);
                        let checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream = generate_operation_error_initialization_eprintln_response_creation_token_stream(&operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
                        quote::quote! {
                            if let Some(#value_snake_case) = &#parameters_snake_case.#payload_snake_case.#field_ident {
                                match #postgresql_crud_postgresql_type_where_filter_query_part_token_stream(
                                    #value_snake_case,
                                    &mut increment,
                                    &#field_ident_double_quotes_token_stream,
                                    is_first_push_to_additional_parameters_already_happend,//todo generate is in proc macro (first element ignore)
                                ) {
                                    Ok(value) => {
                                        additional_parameters.push_str(&value);
                                        #maybe_is_first_push_to_additional_parameters_already_happend_true_token_stream
                                    }
                                    Err(#error_0_token_stream) => {
                                        #checked_add_syn_variant_error_initialization_eprintln_response_creation_token_stream
                                    }
                                }
                            }
                        }
                    });
                    let handle_token_stream = generate_quotes::double_quotes_token_stream(&format!("{delete_snake_case} {from_snake_case} {ident_snake_case_stringified} {{}} returning {primary_key_field_ident}"));
                    quote::quote! {
                        {
                            format!(
                                #handle_token_stream,
                                {
                                    #increment_initialization_token_stream
                                    //todo is all None then do not start with "where"
                                    let mut additional_parameters = #std_string_string::from("where");
                                    let mut is_first_push_to_additional_parameters_already_happend = false;
                                    #(#additional_parameters_modification_token_stream)*
                                    additional_parameters
                                }
                            )
                        }
                    }
                };
                let binded_query_token_stream = {
                    let binded_query_modifications_token_stream = fields.iter().map(|element| {
                        let field_ident = &element.field_ident;
                        quote::quote! {
                            if let Some(#value_snake_case) = #parameters_snake_case.#payload_snake_case.#field_ident {
                                query = #postgresql_crud_postgresql_type_where_filter_query_bind_token_stream(
                                    value,
                                    query
                                );
                            }
                        }
                    });
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #(#binded_query_modifications_token_stream)*
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &generate_create_update_delete_many_fetch_token_stream(&CreateManyOrUpdateManyOrDeleteMany::DeleteMany)
                );
                // let non_existing_primary_keys_check_token_stream = generate_non_existing_primary_keys_check_token_stream(
                //     &UpdateManyOrDeleteMany::UpdateMany,
                //     &expected_primary_keys_snake_case
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
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &std_vec_vec_primary_key_field_type_read_token_stream,
                &proc_macro2::TokenStream::new(), //todo maybe add filter on not unique primary key like in read_many ?
                &value_snake_case,
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
            &generate_ident_operation_payload_upper_camel_case(&operation),
            &{
                let primary_key_token_stream = {
                    quote::quote! {
                        #primary_key_field_ident: Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                    }
                };
                let fields_without_primary_key_token_stream = fields_without_primary_key.iter().map(|element| {
                    let field_ident = &element.field_ident;
                    quote::quote! {
                        #field_ident: Some(#postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream)
                    }
                });
                quote::quote! {
                    Self {
                        #primary_key_token_stream,
                        #(#fields_without_primary_key_token_stream),*
                    }
                }
            }
        );
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
            #operation_payload_example_route_logic_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"delete_many",
    //     &delete_many_token_stream,
    // );
    let delete_one_token_stream = {
        let operation = Operation::DeleteOne;
        let type_variants_from_request_response_syn_variants = generate_type_variants_from_request_response_syn_variants(&common_route_with_row_and_rollback_syn_variants, &operation);
        let parameters_token_stream = generate_parameters_pattern_payload_token_stream(
            &operation,
            &generate_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_token_stream(&naming::parameter::SelfReadUpperCamelCase::from_type_last_segment(&primary_key_field.syn_field.ty))
        );
        let try_operation_route_logic_token_stream = {
            let try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream =
                generate_try_operation_route_logic_response_variants_impl_std_convert_from_try_operation_route_logic_error_named_for_try_operation_route_logic_response_variants_try_operation_route_logic_error_named_token_stream(
                    &operation,
                    &primary_key_field_type_as_primary_key_upper_camel_case,
                    &type_variants_from_request_response_syn_variants,
                );
            let try_operation_route_logic_token_stream = {
                let parameters_logic_token_stream = generate_parameters_logic_token_stream(&operation, &proc_macro2::TokenStream::new());
                let query_string_token_stream = {
                    let query_token_stream = generate_quotes::double_quotes_token_stream(&format!("{delete_snake_case} {from_snake_case} {ident_snake_case_stringified} {where_snake_case} {primary_key_field_ident} = $1 {returning_primary_key_stringified}"));
                    quote::quote! {format!(#query_token_stream) }
                };
                let binded_query_token_stream = {
                    quote::quote! {
                        let mut #query_snake_case = #sqlx_query_sqlx_postgres_token_stream(&#query_string_snake_case);
                        #query_snake_case = postgresql_crud::PostgresqlTypeWhereFilter::query_bind(
                            #parameters_snake_case.#payload_snake_case.#primary_key_field_ident,
                            #query_snake_case
                        );
                        #query_snake_case
                    }
                };
                let postgresql_logic_token_stream = wrap_content_into_postgresql_transaction_begin_commit_value_token_stream(
                    &operation,
                    &generate_create_update_delete_one_fetch_token_stream(&CreateOneOrUpdateOneOrDeleteOne::DeleteOne)
                );
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
        let try_operation_token_stream = {
            let try_operation_error_named_token_stream = generate_ident_try_operation_error_named_token_stream(&operation, &common_http_request_syn_variants);
            let try_operation_token_stream = generate_try_operation_token_stream(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &primary_key_field_type_as_primary_key_upper_camel_case,
                &proc_macro2::TokenStream::new(),
                &value_snake_case,
            );
            quote::quote! {
                #try_operation_error_named_token_stream
                #try_operation_token_stream
            }
        };
        let impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream = generate_impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_tokens_no_lifetime_token_stream(
            &generate_ident_operation_payload_upper_camel_case(&operation),
            &{
                let primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream = {
                    quote::quote! {
                        #primary_key_field_ident: #postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_call_token_stream
                    }
                };
                quote::quote! {
                    Self {
                        #primary_key_field_with_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_token_stream,
                    }
                }
            }
        );
        let operation_payload_example_route_logic_token_stream = generate_operation_payload_example_route_logic_token_stream(&operation);
        quote::quote! {
            #parameters_token_stream
            #try_operation_route_logic_token_stream
            #try_operation_token_stream
            #impl_postgresql_crud_default_but_option_is_always_some_and_vec_always_contains_one_element_for_operation_payload_token_stream
            #operation_payload_example_route_logic_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &"DeleteOne",
    //     &delete_one_token_stream,
    // );
    let common_token_stream = {
        quote::quote! {
            #impl_ident_token_stream
            #ident_read_token_stream
            #select_token_stream
            #ident_column_read_permission_token_stream
        }
    };
    // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
    //     &proc_macro_name_upper_camel_case,
    //     &common_token_stream,
    // );
    //todo pub and private impl quote group
    let generated = quote::quote! {
        #common_token_stream

        #create_many_token_stream
        #create_one_token_stream
        #read_many_token_stream
        #read_one_token_stream
        //todo fix trait calls in update many comparing with update_one
        // #update_many_token_stream
        #update_one_token_stream
        // #delete_many_token_stream
        #delete_one_token_stream
    };
    // if ident == "" {
        // macros_helpers::write_token_stream_into_file::write_token_stream_into_file(
        //     "GeneratePostgresqlCrud",
        //     &generated,
        // );
    // }
    generated.into()
}
