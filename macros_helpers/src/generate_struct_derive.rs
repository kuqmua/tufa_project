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
pub enum DeriveDefault {
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
pub enum DeriveEq {
    True,
    False
}
#[derive(Debug, Clone, Copy)]
pub enum DerivePartialOrd {
    True,
    False
}
#[derive(Debug, Clone, Copy)]
pub enum DeriveOrd {
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
    derive_default: DeriveDefault,
    derive_clone: DeriveClone,
    derive_copy: DeriveCopy,
    derive_partial_eq: DerivePartialEq,
    derive_eq: DeriveEq,
    derive_partial_ord: DerivePartialOrd,
    derive_ord: DeriveOrd,
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
    let maybe_default_token_stream = match derive_default {
        DeriveDefault::True => quote::quote!{Default,},
        DeriveDefault::False => proc_macro2::TokenStream::new(),
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
    let maybe_eq_token_stream = match derive_eq {
        DeriveEq::True => quote::quote!{Eq,},
        DeriveEq::False => proc_macro2::TokenStream::new(),
    };
    let maybe_partial_ord_token_stream = match derive_partial_ord {
        DerivePartialOrd::True => quote::quote!{PartialOrd,},
        DerivePartialOrd::False => proc_macro2::TokenStream::new(),
    };
    let maybe_ord_token_stream = match derive_ord {
        DeriveOrd::True => quote::quote!{Ord,},
        DeriveOrd::False => proc_macro2::TokenStream::new(),
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
            #maybe_default_token_stream
            #maybe_clone_token_stream
            #maybe_copy_token_stream
            #maybe_partial_eq_token_stream
            #maybe_eq_token_stream
            #maybe_partial_ord_token_stream
            #maybe_ord_token_stream
            #maybe_serde_serialize_token_stream
            #maybe_serde_deserialize_token_stream
            #maybe_utoipa_to_schema_token_stream
            #maybe_schemars_json_schema_token_stream
        )]
        #maybe_pub_token_stream struct #current_ident #content_token_stream
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct StructDeriveTokenStreamBuilder {
    make_pub: bool,
    derive_debug: bool,
    derive_default: bool,
    derive_clone: bool,
    derive_copy: bool,
    derive_partial_eq: bool,
    derive_eq: bool,
    derive_partial_ord: bool,
    derive_ord: bool,
    derive_serde_serialize: bool,
    derive_serde_deserialize: bool,
    derive_utoipa_to_schema: bool,
    derive_schemars_json_schema: bool,
}
impl StructDeriveTokenStreamBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn make_pub(mut self) -> Self {
        self.make_pub = true;
        self
    }
    pub fn derive_debug(mut self) -> Self {
        self.derive_debug = true;
        self
    }
    pub fn derive_default(mut self) -> Self {
        self.derive_default = true;
        self
    }
    pub fn derive_clone(mut self) -> Self {
        self.derive_clone = true;
        self
    }
    pub fn derive_copy(mut self) -> Self {
        self.derive_copy = true;
        self
    }
    pub fn derive_partial_eq(mut self) -> Self {
        self.derive_partial_eq = true;
        self
    }
    pub fn derive_eq(mut self) -> Self {
        self.derive_eq = true;
        self
    }
    pub fn derive_partial_ord(mut self) -> Self {
        self.derive_partial_ord = true;
        self
    }
    pub fn derive_ord(mut self) -> Self {
        self.derive_ord = true;
        self
    }
    pub fn derive_serde_serialize(mut self) -> Self {
        self.derive_serde_serialize = true;
        self
    }
    pub fn derive_serde_deserialize(mut self) -> Self {
        self.derive_serde_deserialize = true;
        self
    }
    pub fn derive_utoipa_to_schema(mut self) -> Self {
        self.derive_utoipa_to_schema = true;
        self
    }
    pub fn derive_schemars_json_schema(mut self) -> Self {
        self.derive_schemars_json_schema = true;
        self
    }
    pub fn build(
        self,
        current_ident: &dyn quote::ToTokens,
        content_token_stream: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        let maybe_pub_token_stream = self.make_pub.then(|| quote::quote!(pub));
        let maybe_derive_debug_token_stream = self.derive_debug.then(|| quote::quote!(Debug,));
        let maybe_derive_default_token_stream = self.derive_default.then(|| quote::quote!(Default,));
        let maybe_derive_clone_token_stream = self.derive_clone.then(|| quote::quote!(Clone,));
        let maybe_derive_copy_token_stream = self.derive_copy.then(|| quote::quote!(Copy,));
        let maybe_derive_partial_eq_token_stream = self.derive_partial_eq.then(|| quote::quote!(PartialEq,));
        let maybe_derive_eq_token_stream = self.derive_eq.then(|| quote::quote!(Eq,));
        let maybe_derive_partial_ord_token_stream = self.derive_partial_ord.then(|| quote::quote!(PartialOrd,));
        let maybe_derive_ord_token_stream = self.derive_ord.then(|| quote::quote!(Ord,));
        let maybe_derive_serde_serialize_token_stream = self.derive_serde_serialize.then(|| {
            let serde_serialize_token_stream = token_patterns::SerdeSerialize;
            quote::quote!(#serde_serialize_token_stream,)
        });
        let maybe_derive_serde_deserialize_token_stream = self.derive_serde_deserialize.then(|| {
            let serde_deserialize_token_stream = token_patterns::SerdeDeserialize;
            quote::quote!(#serde_deserialize_token_stream,)
        });
        let maybe_derive_utoipa_to_schema_token_stream = self.derive_utoipa_to_schema.then(|| {
            let utoipa_to_schema_token_stream = token_patterns::UtoipaToSchema;
            quote::quote!(#utoipa_to_schema_token_stream,)
        });
        let maybe_derive_schemars_json_schema_token_stream = self.derive_schemars_json_schema.then(|| {
            let schemars_json_schema_token_stream = token_patterns::SchemarsJsonSchema;
            quote::quote!(#schemars_json_schema_token_stream,)
        });
        quote::quote! {
            #[derive(
                #maybe_derive_debug_token_stream
                #maybe_derive_default_token_stream
                #maybe_derive_clone_token_stream
                #maybe_derive_copy_token_stream
                #maybe_derive_partial_eq_token_stream
                #maybe_derive_eq_token_stream
                #maybe_derive_partial_ord_token_stream
                #maybe_derive_ord_token_stream
                #maybe_derive_serde_serialize_token_stream
                #maybe_derive_serde_deserialize_token_stream
                #maybe_derive_utoipa_to_schema_token_stream
                #maybe_derive_schemars_json_schema_token_stream
            )]
            #maybe_pub_token_stream struct #current_ident #content_token_stream
        }
    }
}
