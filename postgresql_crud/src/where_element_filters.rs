generate_where_element_filters::generate_where_element_filters!();

//todo ExactSizeIterator now is not a solution. error[E0658]: use of unstable library feature `exact_size_is_empty`. maybe rewrite it later
pub trait IsEmpty {
    fn is_empty(&self) -> std::primitive::bool;
}




// pub struct Equal;
// impl Equal {
//     // pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens, postgresql_type_not_null_or_nullable: &crate::PostgresqlTypeNotNullOrNullable, where_operator_type: &
//     // pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
// }

// pub struct GreaterThan;
// impl GreaterThan {
//     // pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens, where_operator_type: &crate::WhereOperatorType) -> proc_macro2::TokenStream {
//     // pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
// }

// pub struct Between;
// impl Between {
//     // pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens, where_operator_type: &crate::WhereOperatorType, between_try_new_error_type: &BetweenTryNewErrorType, 
//     // pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, between_try_new_error_type: &BetweenTryNewErrorType, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant) -> 
// }

// pub struct In;
// impl In {
//     // pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens, where_operator_type: &crate::WhereOperatorType) -> proc_macro2::TokenStream {
//     // pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
// }

// pub struct CaseSensitiveRegularExpression;
// // impl CaseSensitiveRegularExpression {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// //     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }
// pub struct CaseInsensitiveRegularExpression;
// // impl CaseInsensitiveRegularExpression {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// //     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct Before;
// // impl Before {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct CurrentDate;
// // impl CurrentDate {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct GreaterThanCurrentDate;
// // impl GreaterThanCurrentDate {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct CurrentTimestamp;
// // impl CurrentTimestamp {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct GreaterThanCurrentTimestamp;
// // impl GreaterThanCurrentTimestamp {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct CurrentTime;
// // impl CurrentTime {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct GreaterThanCurrentTime;
// // impl GreaterThanCurrentTime {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct LengthEqual;
// impl LengthEqual {
//     // pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
//     // pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
// }

// pub struct LengthMoreThan;
// impl LengthMoreThan {
//     // pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
//     // pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
// }

// pub struct EqualToEncodedStringRepresentation;
// // impl EqualToEncodedStringRepresentation {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct ValueIsContainedWithinRange;
// // impl ValueIsContainedWithinRange {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
// // }

// pub struct ContainsAnotherRange;
// // impl ContainsAnotherRange {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct StrictlyToLeftOfRange;
// // impl StrictlyToLeftOfRange {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct StrictlyToRightOfRange;
// // impl StrictlyToRightOfRange {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct IncludedLowerBound;
// // impl IncludedLowerBound {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
// // }

// pub struct ExcludedUpperBound;
// // impl ExcludedUpperBound {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
// // }

// pub struct GreaterThanLowerBound;
// // impl GreaterThanLowerBound {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct OverlapWithRange;
// // impl OverlapWithRange {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct AdjacentWithRange;
// // impl AdjacentWithRange {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct RangeLength;
// // impl RangeLength {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// // pub struct BitVecPositionEqual;
// // impl BitVecPositionEqual {
// //     pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(&self, ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
// // }

// pub struct PositionEqual;
// impl PositionEqual {
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant, postgresql_json_type_variant_array_element: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
//         let postgresql_json_type_ident_wrapper = postgresql_json_type_variant.postgresql_json_type_ident_wrapper();
//         let postgresql_json_type_ident_wrapper_array_element = postgresql_json_type_variant_array_element.postgresql_json_type_ident_wrapper();

//         let self_upper_camel_case = WhereOperatorName::upper_camel_case(self);
//         let additional_type_declaration_token_stream = generate_additional_type_declaration_token_stream_34095bbb_d306_4a44_92e9_4df1a7354bc1(&postgresql_json_type_ident_wrapper_array_element);
//         generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
//             &generate_ident_where_element_filter_upper_camel_case(&postgresql_json_type_ident_wrapper, self_upper_camel_case),
//             ShouldWhereElementFieldsBePublic::False {
//                 ident: &postgresql_json_type_ident_wrapper,
//                 postfix: &self_upper_camel_case,
//                 try_new_error_named_variants_token_stream: &generate_try_new_error_named_variants_token_stream_06af1515_1384_4d10_a4cf_aaf07284fd08(),
//                 try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
//                 try_new_content_token_stream: &generate_try_new_content_token_stream_9a677220_67b3_4d4d_a7b7_92314cce8e40(&postgresql_json_type_ident_wrapper, &self_upper_camel_case),
//                 impl_deserialize_token_stream: &generate_impl_deserialize_token_stream_4b33e130_e350_4911_a82e_0b77a3c433da(&postgresql_json_type_ident_wrapper, &postgresql_json_type_ident_wrapper_array_element, &self_upper_camel_case),
//             },
//             &crate::ShouldDeriveSchemarsJsonSchema::True,
//             &additional_type_declaration_token_stream,
//             &generate_additional_default_initialization_token_stream_cd86231b_5ff2_4e91_9906_29b822838309(&token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall),
//             &Self::generate_try_generate_bind_increments_token_stream(),
//             &generate_bind_value_to_query_token_stream_1026d141_062b_43c0_bbbf_a45d6dfe68a2(),
//         )
//     }
// }

// pub struct PositionGreaterThan;
// impl PositionGreaterThan {
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant, postgresql_json_type_variant_array_element: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
//         let postgresql_json_type_ident_wrapper = postgresql_json_type_variant.postgresql_json_type_ident_wrapper();
//         let postgresql_json_type_ident_wrapper_array_element = postgresql_json_type_variant_array_element.postgresql_json_type_ident_wrapper();
//         let self_upper_camel_case = WhereOperatorName::upper_camel_case(self);
//         let additional_type_declaration_token_stream = generate_additional_type_declaration_token_stream_34095bbb_d306_4a44_92e9_4df1a7354bc1(&postgresql_json_type_ident_wrapper_array_element);
//         generate_postgresql_type_or_json_type_tokens_where_element_variant_token_stream(
//             &generate_ident_where_element_filter_upper_camel_case(&postgresql_json_type_ident_wrapper, self_upper_camel_case),
//             ShouldWhereElementFieldsBePublic::False {
//                 ident: &postgresql_json_type_ident_wrapper,
//                 postfix: &self_upper_camel_case,
//                 try_new_error_named_variants_token_stream: &generate_try_new_error_named_variants_token_stream_06af1515_1384_4d10_a4cf_aaf07284fd08(),
//                 try_new_additional_input_parameters_token_stream: &additional_type_declaration_token_stream,
//                 try_new_content_token_stream: &generate_try_new_content_token_stream_9a677220_67b3_4d4d_a7b7_92314cce8e40(&postgresql_json_type_ident_wrapper, &self_upper_camel_case),
//                 impl_deserialize_token_stream: &generate_impl_deserialize_token_stream_4b33e130_e350_4911_a82e_0b77a3c433da(&postgresql_json_type_ident_wrapper, &postgresql_json_type_ident_wrapper_array_element, &self_upper_camel_case),
//             },
//             &crate::ShouldDeriveSchemarsJsonSchema::True,
//             &additional_type_declaration_token_stream,
//             &generate_additional_default_initialization_token_stream_cd86231b_5ff2_4e91_9906_29b822838309(&token_patterns::CrateDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementCall),
//             &Self::generate_try_generate_bind_increments_token_stream(),
//             &generate_bind_value_to_query_token_stream_1026d141_062b_43c0_bbbf_a45d6dfe68a2(),
//         )
//     }
// }

// pub struct PositionCaseSensitiveRegularExpression;
// impl PositionCaseSensitiveRegularExpression {
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
// }

// pub struct PositionCaseInsensitiveRegularExpression;
// impl PositionCaseInsensitiveRegularExpression {
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
// }

// pub struct ContainsAllElementsOfArray;
// impl ContainsAllElementsOfArray {
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant, postgresql_json_type_variant_array_element: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
// }

// pub struct ContainedInArray;
// // impl ContainedInArray {
// //     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(
// // }

// pub struct OverlapsWithArray;
// impl OverlapsWithArray {
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant, postgresql_json_type_variant_array_element: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
// }

// pub struct AllElementsEqual;
// impl AllElementsEqual {
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_ident_wrapper: &proc_macro2::TokenStream, postgresql_json_type_variant_array_element: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
// }

// pub struct ContainsElementGreaterThan;
// impl ContainsElementGreaterThan {
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant, postgresql_json_type_variant_array_element: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
// }

// pub struct AllElementsGreaterThan;
// impl AllElementsGreaterThan {
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant, postgresql_json_type_variant_array_element: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
// }

// pub struct ContainsElementCaseSensitiveRegularExpression;
// impl ContainsElementCaseSensitiveRegularExpression {
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
//         generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_5d8e8b34_33ec_476c_a50b_19e0b8d5da69(WhereOperatorName::upper_camel_case(self), postgresql_json_type_variant, &Self::generate_try_generate_bind_increments_token_stream())
//     }
// }

// pub struct ContainsElementCaseInsensitiveRegularExpression;
// impl ContainsElementCaseInsensitiveRegularExpression {
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
//         generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_5d8e8b34_33ec_476c_a50b_19e0b8d5da69(WhereOperatorName::upper_camel_case(self), postgresql_json_type_variant, &Self::generate_try_generate_bind_increments_token_stream())
//     }
// }

// pub struct AllElementsCaseSensitiveRegularExpression;
// impl AllElementsCaseSensitiveRegularExpression {
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
//         generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_5d8e8b34_33ec_476c_a50b_19e0b8d5da69(WhereOperatorName::upper_camel_case(self), postgresql_json_type_variant, &Self::generate_try_generate_bind_increments_token_stream())
//     }
// }

// pub struct AllElementsCaseInsensitiveRegularExpression;
// impl AllElementsCaseInsensitiveRegularExpression {
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
//         generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_5d8e8b34_33ec_476c_a50b_19e0b8d5da69(WhereOperatorName::upper_camel_case(self), postgresql_json_type_variant, &Self::generate_try_generate_bind_increments_token_stream())
//     }
// }

// ////////////////////second dimension
// pub struct EqualSecondDimension;
// impl EqualSecondDimension {
//     // pub fn generate_postgresql_type_tokens_where_element_variant_handle_token_stream(
//     //     &self,
//     //     ident: &dyn quote::ToTokens,
//     //     where_operator_type: &crate::WhereOperatorType,
//     // ) -> proc_macro2::TokenStream {
//     //     generate_postgresql_type_tokens_where_element_variant_handle_token_stream_817a2973_b62c_4100_9a40_b3ee40f01e04(
//     //         WhereOperatorName::upper_camel_case(self),
//     //         ident,
//     //         where_operator_type,
//     //         &Self::dimension(),
//     //     )
//     // }
//     pub fn generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream(&self, postgresql_json_type_variant: &crate::PostgresqlJsonTypeVariant) -> proc_macro2::TokenStream {
//         generate_postgresql_json_type_tokens_where_element_variant_handle_token_stream_94ce15d6_0735_4407_af5b_4a82e434f91a(WhereOperatorName::upper_camel_case(self), postgresql_json_type_variant, &Self::dimension())
//     }
// }
