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

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize, Eq, PartialEq, schemars::JsonSchema)]
pub enum LogicalOperator {
    And,
    #[default]
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
impl DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for LogicalOperator {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PostgresqlTypeGreaterThanVariant {
    GreaterThan,
    NotGreaterThan,
    EqualNotGreaterThan,
}
impl PostgresqlTypeGreaterThanVariant {
    pub const fn logical_operator(&self) -> LogicalOperator {
        match &self {
            Self::GreaterThan => LogicalOperator::Or,
            Self::NotGreaterThan |
            Self::EqualNotGreaterThan => LogicalOperator::OrNot,
        }
    }
}
impl quote::ToTokens for PostgresqlTypeGreaterThanVariant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::GreaterThan => quote::quote!{GreaterThan},
            Self::NotGreaterThan => quote::quote!{NotGreaterThan},
            Self::EqualNotGreaterThan => quote::quote!{EqualNotGreaterThan},
        }.to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PostgresqlJsonTypeLengthGreaterThanVariant {
    LengthGreaterThan,
    NotLengthGreaterThan,
    EqualNotLengthGreaterThan,
}
impl PostgresqlJsonTypeLengthGreaterThanVariant {
    pub const fn logical_operator(&self) -> LogicalOperator {
        match &self {
            Self::LengthGreaterThan => LogicalOperator::Or,
            Self::NotLengthGreaterThan |
            Self::EqualNotLengthGreaterThan => LogicalOperator::OrNot,
        }
    }
}
impl quote::ToTokens for PostgresqlJsonTypeLengthGreaterThanVariant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::LengthGreaterThan => quote::quote!{LengthGreaterThan},
            Self::NotLengthGreaterThan => quote::quote!{NotLengthGreaterThan},
            Self::EqualNotLengthGreaterThan => quote::quote!{EqualNotLengthGreaterThan},
        }.to_tokens(tokens);
    }
}