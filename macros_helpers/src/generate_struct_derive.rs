#[derive(Debug, Clone, Copy)]
pub enum IsPub {
    True,
    False
}
#[derive(Debug, Clone, Copy)]
pub enum DeriveDebug {
    True,
    False
}
#[derive(Debug, Clone, Copy)]
pub enum DeriveClone {
    True,
    False
}
#[derive(Debug, Clone, Copy)]
pub enum DeriveCopy {
    True,
    False
}
#[derive(Debug, Clone, Copy)]
pub enum DerivePartialEq {
    True,
    False
}
#[derive(Debug, Clone, Copy)]
pub enum DerivePartialOrd {
    True,
    False
}
#[derive(Debug, Clone, Copy)]
pub enum DeriveSerdeSerialize {
    True,
    False
}
#[derive(Debug, Clone, Copy)]
pub enum DeriveSerdeDeserialize {
    True,
    False
}
#[derive(Debug, Clone, Copy)]
pub enum DeriveUtoipaToSchema {
    True,
    False
}
#[derive(Debug, Clone, Copy)]
pub enum DeriveSchemarsJsonSchema {
    True,
    False
}
pub fn generate_struct_derive(
    is_pub: IsPub,
    current_ident: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
    derive_debug: DeriveDebug,
    derive_clone: DeriveClone,
    derive_copy: DeriveCopy,
    derive_partial_eq: DerivePartialEq,
    derive_partial_ord: DerivePartialOrd,
    derive_serde_serialize: DeriveSerdeSerialize,
    derive_serde_deserialize: DeriveSerdeDeserialize,
    derive_utoipa_to_schema: DeriveUtoipaToSchema,
    derive_schemars_json_schema: DeriveSchemarsJsonSchema,
) -> proc_macro2::TokenStream {
    let maybe_pub_token_stream = match is_pub {
        IsPub::True => quote::quote!{pub},
        IsPub::False => proc_macro2::TokenStream::new(),
    };
    let maybe_debug_token_stream = match derive_debug {
        DeriveDebug::True => quote::quote!{Debug,},
        DeriveDebug::False => proc_macro2::TokenStream::new(),
    };
    let maybe_clone_token_stream = match derive_clone {
        DeriveClone::True => quote::quote!{Clone,},
        DeriveClone::False => proc_macro2::TokenStream::new(),
    };
    let maybe_copy_token_stream = match derive_copy {
        DeriveCopy::True => quote::quote!{Copy,},
        DeriveCopy::False => proc_macro2::TokenStream::new(),
    };
    let maybe_partial_eq_token_stream = match derive_partial_eq {
        DerivePartialEq::True => quote::quote!{PartialEq,},
        DerivePartialEq::False => proc_macro2::TokenStream::new(),
    };
    let maybe_partial_ord_token_stream = match derive_partial_ord {
        DerivePartialOrd::True => quote::quote!{PartialOrd,},
        DerivePartialOrd::False => proc_macro2::TokenStream::new(),
    };
    let maybe_serde_serialize_token_stream = match derive_serde_serialize {
        DeriveSerdeSerialize::True => {
            let serde_serialize_token_stream = token_patterns::SerdeSerialize;
            quote::quote!{#serde_serialize_token_stream,}
        },
        DeriveSerdeSerialize::False => proc_macro2::TokenStream::new(),
    };
    let maybe_serde_deserialize_token_stream = match derive_serde_deserialize {
        DeriveSerdeDeserialize::True => {
            let serde_deserialize_token_stream = token_patterns::SerdeDeserialize;
            quote::quote!{#serde_deserialize_token_stream,}
        },
        DeriveSerdeDeserialize::False => proc_macro2::TokenStream::new(),
    };
    let maybe_utoipa_to_schema_token_stream = match derive_utoipa_to_schema {
        DeriveUtoipaToSchema::True => {
            let utoipa_to_schema_token_stream = token_patterns::UtoipaToSchema;
            quote::quote!{#utoipa_to_schema_token_stream,}
        },
        DeriveUtoipaToSchema::False => proc_macro2::TokenStream::new(),
    };
    let maybe_schemars_json_schema_token_stream = match derive_schemars_json_schema {
        DeriveSchemarsJsonSchema::True => {
            let schemars_json_schema_token_stream = token_patterns::SchemarsJsonSchema;
            quote::quote!{#schemars_json_schema_token_stream,}
        },
        DeriveSchemarsJsonSchema::False => proc_macro2::TokenStream::new(),
    };
    quote::quote! {
        #[derive(
            #maybe_debug_token_stream
            #maybe_clone_token_stream
            #maybe_copy_token_stream
            #maybe_partial_eq_token_stream
            #maybe_partial_ord_token_stream
            #maybe_serde_serialize_token_stream
            #maybe_serde_deserialize_token_stream
            #maybe_utoipa_to_schema_token_stream
            #maybe_schemars_json_schema_token_stream
        )]
        #maybe_pub_token_stream struct #current_ident #content_token_stream
    }
}