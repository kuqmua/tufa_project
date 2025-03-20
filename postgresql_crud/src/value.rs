//this needed coz serde std::option::Option<std::option::Option<T>> #[serde(skip_serializing_if = "Option::is_none")] - if both options: inner and parent is null then it skip - its not correct
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct Value<T> {
    pub value: T,
}
