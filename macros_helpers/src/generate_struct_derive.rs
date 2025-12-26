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
#[derive(Debug, Clone, Copy)]
enum StructOrEnum {
    Struct,
    Enum
}
#[derive(Debug, Default, Clone, Copy)]
pub struct StructOrEnumDeriveTokenStreamBuilder {
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
impl StructOrEnumDeriveTokenStreamBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub const fn make_pub(mut self) -> Self {
        self.make_pub = true;
        self
    }
    pub const fn make_pub_if(mut self, condition: IsPub) -> Self {
        if let IsPub::True = condition {
            self.make_pub = true;
        }
        self
    }
    pub const fn derive_debug(mut self) -> Self {
        self.derive_debug = true;
        self
    }
    pub const fn derive_debug_if(mut self, condition: DeriveDebug) -> Self {
        if let DeriveDebug::True = condition {
            self.derive_debug = true;
        }
        self
    }
    pub const fn derive_default(mut self) -> Self {
        self.derive_default = true;
        self
    }
    pub const fn derive_default_if(mut self, condition: DeriveDefault) -> Self {
        if let DeriveDefault::True = condition {
            self.derive_default = true;
        }
        self
    }
    pub const fn derive_clone(mut self) -> Self {
        self.derive_clone = true;
        self
    }
    pub const fn derive_clone_if(mut self, condition: DeriveClone) -> Self {
        if let DeriveClone::True = condition {
            self.derive_clone = true;
        }
        self
    }
    pub const fn derive_copy(mut self) -> Self {
        self.derive_copy = true;
        self
    }
    pub const fn derive_copy_if(mut self, condition: DeriveCopy) -> Self {
        if let DeriveCopy::True = condition {
            self.derive_copy = true;
        }
        self
    }
    pub const fn derive_partial_eq(mut self) -> Self {
        self.derive_partial_eq = true;
        self
    }
    pub const fn derive_partial_eq_if(mut self, condition: DerivePartialEq) -> Self {
        if let DerivePartialEq::True = condition {
            self.derive_partial_eq = true;
        }
        self
    }
    pub const fn derive_eq(mut self) -> Self {
        self.derive_eq = true;
        self
    }
    pub const fn derive_eq_if(mut self, condition: DeriveEq) -> Self {
        if let DeriveEq::True = condition {
            self.derive_eq = true;
        }
        self
    }
    pub const fn derive_partial_ord(mut self) -> Self {
        self.derive_partial_ord = true;
        self
    }
    pub const fn derive_partial_ord_if(mut self, condition: DerivePartialOrd) -> Self {
        if let DerivePartialOrd::True = condition {
            self.derive_partial_ord = true;
        }
        self
    }
    pub const fn derive_ord(mut self) -> Self {
        self.derive_ord = true;
        self
    }
    pub const fn derive_ord_if(mut self, condition: DeriveOrd) -> Self {
        if let DeriveOrd::True = condition {
            self.derive_ord = true;
        }
        self
    }
    pub const fn derive_serde_serialize(mut self) -> Self {
        self.derive_serde_serialize = true;
        self
    }
    pub const fn derive_serde_serialize_if(mut self, condition: DeriveSerdeSerialize) -> Self {
        if let DeriveSerdeSerialize::True = condition {
            self.derive_serde_serialize = true;
        }
        self
    }
    pub const fn derive_serde_deserialize(mut self) -> Self {
        self.derive_serde_deserialize = true;
        self
    }
    pub const fn derive_serde_deserialize_if(mut self, condition: DeriveSerdeDeserialize) -> Self {
        if let DeriveSerdeDeserialize::True = condition {
            self.derive_serde_deserialize = true;
        }
        self
    }
    pub const fn derive_utoipa_to_schema(mut self) -> Self {
        self.derive_utoipa_to_schema = true;
        self
    }
    pub const fn derive_utoipa_to_schema_if(mut self, condition: DeriveUtoipaToSchema) -> Self {
        if let DeriveUtoipaToSchema::True = condition {
            self.derive_utoipa_to_schema = true;
        }
        self
    }
    pub const fn derive_schemars_json_schema(mut self) -> Self {
        self.derive_schemars_json_schema = true;
        self
    }
    pub const fn derive_schemars_json_schema_if(mut self, condition: DeriveSchemarsJsonSchema) -> Self {
        if let DeriveSchemarsJsonSchema::True = condition {
            self.derive_schemars_json_schema = true;
        }
        self
    }
    fn build_handle(
        self,
        struct_or_enum: StructOrEnum,
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
        let struct_or_enum_token_stream = match struct_or_enum {
            StructOrEnum::Struct => quote::quote!{struct},
            StructOrEnum::Enum => quote::quote!{enum},
        };
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
            #maybe_pub_token_stream #struct_or_enum_token_stream #current_ident #content_token_stream
        }
    }
    pub fn build_struct(
        self,
        current_ident: &dyn quote::ToTokens,
        content_token_stream: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        self.build_handle(StructOrEnum::Struct, current_ident, content_token_stream)
    }
    pub fn build_enum(
        self,
        current_ident: &dyn quote::ToTokens,
        content_token_stream: &dyn quote::ToTokens,
    ) -> proc_macro2::TokenStream {
        self.build_handle(StructOrEnum::Enum, current_ident, content_token_stream)
    }
}
