pub trait DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement: Sized {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self;
}
pub trait AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement: Sized {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self>;
}
pub trait DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize: Sized {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size() -> Self;
}
pub trait AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize: Sized {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element_with_max_page_size() -> std::vec::Vec<Self>;
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Eq, PartialEq, schemars::JsonSchema)]
pub enum LogicalOperator {
    And,
    Or,
    AndNot,
    OrNot,
}
impl LogicalOperator {
    pub fn to_query_part(&self, is_need_to_add_logical_operator: std::primitive::bool) -> std::string::String {
        let not_space = format!("{} ", naming::NotSnakeCase);
        if is_need_to_add_logical_operator {
            let and_space = format!("{} ", naming::AndSnakeCase);
            let or_space = format!("{} ", naming::OrSnakeCase);
            match &self {
                Self::And => and_space,
                Self::Or => or_space,
                Self::AndNot => format!("{and_space}{not_space}"),
                Self::OrNot => format!("{or_space}{not_space}"),
            }
        } else {
            match &self {
                Self::And | Self::Or => std::string::String::default(),
                Self::AndNot | Self::OrNot => not_space,
            }
        }
    }
}
impl std::fmt::Display for LogicalOperator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl Default for LogicalOperator {
    fn default() -> Self {
        Self::Or
    }
}
impl DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for LogicalOperator {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub enum GreaterThanVariant {
    GreaterThan,
    NotGreaterThan,
    EqualNotGreaterThan,
}
impl GreaterThanVariant {
    pub fn logical_operator(&self) -> LogicalOperator {
        match &self {
            GreaterThanVariant::GreaterThan => LogicalOperator::Or,
            GreaterThanVariant::NotGreaterThan => LogicalOperator::OrNot,
            GreaterThanVariant::EqualNotGreaterThan => LogicalOperator::OrNot,
        }
    }
}
impl quote::ToTokens for GreaterThanVariant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            GreaterThanVariant::GreaterThan => quote::quote!{GreaterThan},
            GreaterThanVariant::NotGreaterThan => quote::quote!{NotGreaterThan},
            GreaterThanVariant::EqualNotGreaterThan => quote::quote!{EqualNotGreaterThan},
        }.to_tokens(tokens);
    }
}