naming_macros::generate_self_upper_camel_and_snake_case_stringified_and_token_stream!([
    ["self", "parameters"],
    ["self", "payload"],
    ["self", "payload", "with", "serialize", "deserialize"],
    ["self", "payload", "try", "from", "self", "payload", "with", "serialize", "deserialize"],
    ["self", "payload", "with", "serialize", "deserialize", "try", "from", "self", "payload"],
    ["self", "payload", "with", "serialize", "deserialize", "try", "from", "self", "payload", "error", "named"],
    ["try", "self"],
    ["try", "self", "response", "variants"],
    ["self", "payload", "element", "with", "serialize", "deserialize"],
    ["self", "payload", "element"],
    ["self", "payload", "element", "try", "from", "self", "payload", "with", "serialize", "deserialize"],
    ["self", "payload", "element", "try", "from", "self", "payload", "element", "with", "serialize", "deserialize", "error", "named"],
    ["try", "self", "error", "named"],
    ["try", "self", "request", "error"],
    ["self", "payload", "try", "from", "self", "payload", "with", "serialize", "deserialize", "error", "named"],
    ["try", "self", "with", "serialize", "deserialize"],
    ["tvfrr", "extraction", "logic", "try", "self"],
    ["try", "self", "generated", "route", "logic", "error", "named"],
    ["try", "self", "generated", "route", "logic", "desirable"],
    ["try", "self", "route", "logic"],
    ["try", "self", "route", "logic", "response", "variants"],
    ["try", "self", "route", "logic", "error", "named"],
    ["try", "self", "route", "logic", "error", "named", "with", "serialize", "deserialize"],
    ["try", "self", "generated", "route", "logic", "error", "named", "with", "serialize", "deserialize"],
    ["self", "payload", "example", "route", "logic"],
    ["self", "to", "create"],
    ["self", "to", "read"],
    ["self", "to", "update"],
    ["self", "to", "delete"],
    ["self", "options", "to", "read"],
    ["object", "self"],
    ["std", "option", "option", "object", "self"],
    ["object", "with", "id", "self"],
    ["std", "vec", "vec", "object", "with", "id", "self"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self"],
    ["self", "field", "reader"],
    ["self", "field", "to", "read", "without", "id"],
    ["self", "field", "to", "read", "with", "id"],
    ["object", "self", "field", "reader"],
    ["object", "with", "id", "self", "field", "reader"],
    ["std", "option", "option", "object", "self", "field", "reader"],
    ["std", "vec", "vec", "object", "with", "id", "self", "field", "reader"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "field", "reader"],
    ["self", "try", "new", "error", "named"],
    ["object", "self", "options", "to", "read"],
    ["std", "option", "option", "object", "self", "options", "to", "read"],
    ["std", "vec", "vec", "object", "with", "id", "self", "options", "to", "read"],
    ["std","option", "option","std", "vec", "vec", "object", "with", "id", "self", "options", "to", "read"],
    ["object", "self", "to", "create"],
    ["std", "option", "option", "object", "self", "to", "create"],
    ["std", "vec", "vec", "object", "with", "id", "self", "to", "create"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "to", "create"],
    ["object", "with", "id", "self", "options", "to", "read"],
    ["object", "with", "id", "self", "to", "create"],
    ["object", "self", "reader"],
    ["object", "with", "id", "self", "reader"],
    ["std", "option", "option", "object", "self", "reader"],
    ["std", "vec", "vec", "object", "with", "id", "self", "reader"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "reader"],
    ["self", "reader"],
    ["std", "option", "option", "object", "self", "to", "create", "origin"],
    ["std", "vec", "vec", "object", "with", "id", "self", "to", "create", "origin"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "to", "create", "origin"],
    ["std", "option", "option", "object", "self", "options", "to", "read", "origin"],
    ["std", "vec", "vec", "object", "with", "id", "self", "options", "to", "read", "origin"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "options", "to", "read", "origin"],
    ["self", "option", "to", "update"],
    ["object", "self", "option", "to", "update"],
    ["std", "option", "option", "object", "self", "option", "to", "update", "origin"],
    ["std", "option", "option", "object", "self", "option", "to", "update"],
    ["std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "origin"],
    ["std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "origin"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update"],
    ["object", "with", "id", "self", "option", "to", "update"],
    ["std", "vec", "vec", "object", "with", "id", "self", "options", "to", "update"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "options", "to", "update"],
    ["object", "self", "option", "to", "update", "origin"],
    ["std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change"],
    ["self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named", "with", "serialize", "deserialize"],
    ["self", "option", "to", "update", "origin"],
    ["self", "json", "array", "change"],
    ["self", "to", "create", "origin"],
    ["self", "options", "to", "update"],
    ["self", "to", "create", "with", "generated", "id"],
    ["self", "to", "create", "without", "generated", "id"],
    ["self", "json", "array", "change", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["self", "field", "to", "update"],
    ["self", "generate", "postgresql", "query", "part", "to", "read", "error", "named"],
    ["self", "try", "generate", "json", "array", "element", "update", "bind", "increments", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change", "try", "new", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "json", "array", "change", "try", "new", "error", "named"],
    ["not", "unique", "field", "self"],
    ["self", "options", "to", "update", "try", "new", "error", "named"],
    ["self", "options", "to", "read", "without", "id"],
    ["self", "options", "to", "read", "with", "id"],
    ["self", "option", "to", "update", "try", "new", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "options", "to", "read", "try", "new", "error", "named"],
    ["self", "generate", "postgresql", "query", "part", "to", "read", "from", "vec", "error", "named"],
    ["std", "option", "option", "object", "self", "option", "to", "update", "try", "new", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "options", "to", "read", "try", "new", "error", "named"],
    ["self", "field", "to", "read"],
    ["self", "options", "to", "read", "with", "or", "without", "id", "try", "from", "error", "named"],
    ["self", "field", "reader", "try", "new", "error", "named"],
    ["object", "self", "field", "reader", "try", "new", "error", "named"],
    ["std", "option", "option", "object", "self", "field", "reader", "try", "new", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "field", "reader", "try", "new", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "field", "reader", "try", "new", "error", "named"],
    ["object", "self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["object", "self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named", "with", "serialize", "deserialize"],
    ["std", "option", "option", "object", "self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["self", "with", "serialize", "deserialize"],
    ["self", "try", "from", "env", "error", "named"],
    ["get", "self"],
    ["try", "from", "std", "env", "var", "ok", "self", "error", "named"],
    ["self", "options"],
    ["error", "self"],
    ["not", "unique", "self"],
    ["is", "self", "update", "exist"],
    ["self", "column"],
    ["self", "column", "read", "permission"],
    ["self", "where"],
    ["std", "option", "option", "self"],
    ["where", "std", "option", "option", "self"],
    ["sqlx", "types", "json", "object", "self"],
    ["std", "option", "option", "sqlx", "types", "json", "object", "self"],
    ["sqlx", "types", "json", "std", "option", "option", "object", "self"],
    ["std", "option", "option", "sqlx", "types", "json", "std", "option", "option", "object", "self"],
    ["sqlx", "types", "json", "std", "vec", "vec", "object", "with", "id", "self"],
    ["std", "option", "option", "sqlx", "types", "json", "std", "vec", "vec", "object", "with", "id", "self"],
    ["sqlx", "types", "json", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self"],
    ["std", "option", "option", "sqlx", "types", "json", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self"],
    ["object", "self", "to", "read"],
    ["object", "self", "column"],
    ["object", "self", "where"],
    ["std", "option", "option", "object", "self", "column"],
    ["std", "vec", "vec", "object", "with", "id", "self", "column"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "column"],
    ["std", "option", "option", "object", "self", "where"],
    ["std", "vec", "vec", "object", "with", "id", "self", "where"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "where"],
    ["std", "option", "option", "object", "self", "to", "read"],
    ["std", "vec", "vec", "object", "with", "id", "self", "to", "read"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "to", "read"],
    ["postgresql", "json", "type", "self", "to", "create"],
    ["postgresql", "json", "type", "object", "self", "to", "create"],
    ["postgresql", "json", "type", "std", "option", "option", "object", "self", "to", "create"],
    ["postgresql", "json", "type", "std", "vec", "vec", "object", "with", "id", "self", "to", "create"],
    ["postgresql", "json", "type", "std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "to", "create"],
    ["postgresql", "json", "type", "self", "field", "reader"],
    ["postgresql", "json", "type", "object", "self", "field", "reader"],
    ["postgresql", "json", "type", "std", "option", "option", "object", "self", "field", "reader"],
    ["std", "vec", "vec", "object", "with", "id", "self", "postgresql", "json", "type", "field", "reader"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "postgresql", "json", "type", "field", "reader"],
    ["postgresql", "json", "type", "self", "field", "reader", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "object", "self", "postgresql", "json", "type", "field", "reader", "try", "new", "error", "named"],
    ["postgresql", "json", "type", "std", "option", "option", "object", "self", "field", "reader", "try", "new", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "postgresql", "json", "type", "field", "reader", "try", "new", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "postgresql", "json", "type", "field", "reader", "try", "new", "error", "named"],
    ["object", "self", "postgresql", "json", "type", "options", "to", "read"],
    ["std", "option", "option", "object", "self", "postgresql", "json", "type", "options", "to", "read"],
    ["std", "vec", "vec", "object", "with", "id", "self", "postgresql", "json", "type", "options", "to", "read"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "postgresql", "json", "type", "options", "to", "read"],
    ["postgresql", "json", "type", "self", "options", "to", "read"],
    ["postgresql", "json", "type", "self", "options", "to", "read", "without", "id"],
    ["postgresql", "json", "type", "self", "options", "to", "read", "with", "id"],
    ["postgresql", "json", "type", "self", "option", "to", "update", "origin"],
    ["postgresql", "json", "type", "self", "option", "to", "update"],
    ["postgresql", "json", "type", "self", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["postgresql", "json", "type", "self", "option", "to", "update", "try", "new", "error", "named"],
    ["object", "self", "postgresql", "json", "type", "option", "to", "update"],
    ["std", "option", "option", "object", "self", "postgresql", "json", "type", "option", "to", "update"],
    ["std", "vec", "vec", "object", "with", "id", "self", "postgresql", "json", "type", "option", "to", "update"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "postgresql", "json", "type", "option", "to","update"],
    ["object", "self", "postgresql", "json", "type", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["std", "option", "option", "object", "self", "postgresql", "json", "type", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["std", "vec", "vec", "object", "with", "id", "self", "postgresql", "json", "type", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["std", "option", "option", "std", "vec", "vec", "object", "with", "id", "self", "postgresql", "json", "type", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named"],
    ["object", "self", "postgresql", "json", "type", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named", "with", "serialize", "deserialize"],
    ["self", "postgresql", "json", "type", "option", "to", "update", "try", "generate", "postgresql", "query", "part", "error", "named", "with", "serialize", "deserialize"]
]);

////////////////////////////////////////////////////
// #[derive(Debug)]
// pub struct GenericSelfFieldReaderUpperCamelCase(std::string::String);
// impl GenericSelfFieldReaderUpperCamelCase {
//     fn wrap(value: &dyn std::fmt::Display) -> Self {
//         Self(format!("Generic{value}FieldReader"))
//     }
//     pub fn from_display(value: &dyn std::fmt::Display) -> Self {
//         Self::wrap(&generate_quotes::naming::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&value.to_string()))
//     }
//     pub fn from_tokens(value: &dyn quote::ToTokens) -> Self {
//         Self::wrap(&generate_quotes::naming::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&{
//             let mut tokens = proc_macro2::TokenStream::new();
//             quote::ToTokens::to_tokens(&value, &mut tokens);
//             tokens
//         }.to_string()))
//     }
// }
// impl std::fmt::Display for GenericSelfFieldReaderUpperCamelCase {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{}", self.0)
//     }
// }
// impl quote::ToTokens for GenericSelfFieldReaderUpperCamelCase {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         let value_stringified = self.to_string();
//         let value_token_stream = value_stringified.parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("failed to parse stringified GenericSelfFieldReaderUpperCamelCase into proc_macro2::TokenStream: {value_stringified}"));
//         value_token_stream.to_tokens(tokens)
//     }
// }


// #[derive(Debug)]
// pub struct GenericSelfFieldReaderSnakeCase(std::string::String);
// impl GenericSelfFieldReaderSnakeCase {
//     fn wrap(value: &dyn std::fmt::Display) -> Self {
//         Self(format!("generic_{value}_field_reader"))
//     }
//     pub fn from_display(value: &dyn std::fmt::Display) -> Self {
//         Self::wrap(&generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&value.to_string()))
//     }
//     pub fn from_tokens(value: &dyn quote::ToTokens) -> Self {
//         Self::wrap(&generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&{
//             let mut tokens = proc_macro2::TokenStream::new();
//             quote::ToTokens::to_tokens(&value, &mut tokens);
//             tokens
//         }.to_string()))
//     }
// }
// impl std::fmt::Display for GenericSelfFieldReaderSnakeCase {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{}", self.0)
//     }
// }
// impl quote::ToTokens for GenericSelfFieldReaderSnakeCase {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         let value_stringified = self.to_string();
//         let value_token_stream = value_stringified.parse::<proc_macro2::TokenStream>()
//         .unwrap_or_else(|_| panic!("failed to parse stringified GenericSelfFieldReaderSnakeCase into proc_macro2::TokenStream: {value_stringified}"));
//         value_token_stream.to_tokens(tokens)
//     }
// }





/////////
// pub struct StdOptionOptionGenericAccUpperCamelCase;
// impl std::fmt::Display for StdOptionOptionGenericAccUpperCamelCase {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(formatter, "StdOptionOptionGenericAcc")
//     }
// }
// impl quote::ToTokens for StdOptionOptionGenericAccUpperCamelCase {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         quote::quote! { StdOptionOptionGenericAcc }.to_tokens(tokens)
//     }
// }
// pub struct StdOptionOptionGenericAccSnakeCase;
// impl std::fmt::Display for StdOptionOptionGenericAccSnakeCase {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(formatter, "std_option_option_generic_acc")
//     }
// }
// impl quote::ToTokens for StdOptionOptionGenericAccSnakeCase {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         quote::quote! { std_option_option_generic_acc }.to_tokens(tokens)
//     }
// }
////////


// pub trait GenericSelfFieldReaderUpperCamelCaseStringified {
//     fn generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String;
// }
// impl<T> GenericSelfFieldReaderUpperCamelCaseStringified for T
// where
//     T: generate_quotes::naming::ToUpperCamelCaseStringified,
// {
//     fn generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String {
//         format!("Generic{}FieldReader", self.to_upper_camel_case_stringified(),)
//     }
// }
// pub trait GenericSelfFieldReaderSnakeCaseStringified {
//     fn generic_self_field_reader_snake_case_stringified(&self) -> std::string::String;
// }
// impl<T> GenericSelfFieldReaderSnakeCaseStringified for T
// where
//     T: generate_quotes::naming::ToSnakeCaseStringified,
// {
//     fn generic_self_field_reader_snake_case_stringified(&self) -> std::string::String {
//         format!("generic_{}_field_reader", self.to_snake_case_stringified(),)
//     }
// }
// pub trait GenericSelfFieldReaderUpperCamelCaseTokenStream {
//     fn generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> GenericSelfFieldReaderUpperCamelCaseTokenStream for T
// where
//     T: GenericSelfFieldReaderUpperCamelCaseStringified,
// {
//     fn generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.generic_self_field_reader_upper_camel_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait GenericSelfFieldReaderSnakeCaseTokenStream {
//     fn generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> GenericSelfFieldReaderSnakeCaseTokenStream for T
// where
//     T: GenericSelfFieldReaderSnakeCaseStringified,
// {
//     fn generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.generic_self_field_reader_snake_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseStringified {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseStringified for T
// where
//     T: quote::ToTokens,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String {
//         format!("Generic{}FieldReader", generate_quotes::naming::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&quote::quote! { #self }.to_string()),)
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseStringified {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_stringified(&self) -> std::string::String;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseStringified for T
// where
//     T: quote::ToTokens,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_stringified(&self) -> std::string::String {
//         format!("generic_{}_field_reader", generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&quote::quote! { #self }.to_string()),)
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseTokenStream {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseTokenStream for T
// where
//     T: ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseStringified,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseTokenStream {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseTokenStream for T
// where
//     T: ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseStringified,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.impl_quote_to_tokens_generic_self_field_reader_snake_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait GenericSelfFieldReaderUpperCamelCaseStringified {
//     fn generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String;
// }
// impl<T> GenericSelfFieldReaderUpperCamelCaseStringified for T
// where
//     T: generate_quotes::naming::ToUpperCamelCaseStringified,
// {
//     fn generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String {
//         format!("Generic{}FieldReader", self.to_upper_camel_case_stringified(),)
//     }
// }
// pub trait GenericSelfFieldReaderSnakeCaseStringified {
//     fn generic_self_field_reader_snake_case_stringified(&self) -> std::string::String;
// }
// impl<T> GenericSelfFieldReaderSnakeCaseStringified for T
// where
//     T: generate_quotes::naming::ToSnakeCaseStringified,
// {
//     fn generic_self_field_reader_snake_case_stringified(&self) -> std::string::String {
//         format!("generic_{}_field_reader", self.to_snake_case_stringified(),)
//     }
// }
// pub trait GenericSelfFieldReaderUpperCamelCaseTokenStream {
//     fn generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> GenericSelfFieldReaderUpperCamelCaseTokenStream for T
// where
//     T: GenericSelfFieldReaderUpperCamelCaseStringified,
// {
//     fn generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.generic_self_field_reader_upper_camel_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait GenericSelfFieldReaderSnakeCaseTokenStream {
//     fn generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> GenericSelfFieldReaderSnakeCaseTokenStream for T
// where
//     T: GenericSelfFieldReaderSnakeCaseStringified,
// {
//     fn generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.generic_self_field_reader_snake_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseStringified {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseStringified for T
// where
//     T: quote::ToTokens,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_stringified(&self) -> std::string::String {
//         format!("Generic{}FieldReader", generate_quotes::naming::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&quote::quote! { #self }.to_string()),)
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseStringified {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_stringified(&self) -> std::string::String;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseStringified for T
// where
//     T: quote::ToTokens,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_stringified(&self) -> std::string::String {
//         format!("generic_{}_field_reader", generate_quotes::naming::AsRefStrToSnakeCaseStringified::new(&quote::quote! { #self }.to_string()),)
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseTokenStream {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseTokenStream for T
// where
//     T: ImplQuoteToTokensGenericSelfFieldReaderUpperCamelCaseStringified,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.impl_quote_to_tokens_generic_self_field_reader_upper_camel_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }
// pub trait ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseTokenStream {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream;
// }
// impl<T> ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseTokenStream for T
// where
//     T: ImplQuoteToTokensGenericSelfFieldReaderSnakeCaseStringified,
// {
//     fn impl_quote_to_tokens_generic_self_field_reader_snake_case_token_stream(&self) -> proc_macro2::TokenStream {
//         let value = self.impl_quote_to_tokens_generic_self_field_reader_snake_case_stringified();
//         value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
//     }
// }

////////////////////////////////////

