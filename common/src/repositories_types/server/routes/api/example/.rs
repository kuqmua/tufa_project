#[derive(Debug)]
pub struct AnimalAsNotNullJsonbObject;
#[derive(Debug)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithId;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalAsNotNullJsonbObjectTableTypeDeclaration {
    field_0: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
    field_1: <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
}
impl AnimalAsNotNullJsonbObjectTableTypeDeclaration {
    pub fn new(
        field_0: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
        field_1: <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
    ) -> Self {
        Self { field_0, field_1 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_0: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            field_1: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}



//here
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIds {
    id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read,
    //todo additional
}
impl sqlx::Decode<'_, sqlx::Postgres> for AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIds {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}







#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
    field_0: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
    field_1: <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn new(
        id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
        field_0: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
        field_1: <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
    ) -> Self {
        Self { id, field_0, field_1 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            field_0: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            field_1: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl AnimalAsNotNullJsonbObjectTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(AnimalAsNotNullJsonbObjectTableTypeDeclaration)).unwrap())
    }
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!(
            "{column} jsonb not null check (jsonb_matches_schema('{}', {column}))",
            serde_json::to_string(&schemars::schema_for!(AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration)).unwrap()
        )
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalAsNotNullJsonbObjectCreate {
    field_0: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create,
    field_1: <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create,
}
impl AnimalAsNotNullJsonbObjectCreate {
    pub fn new(field_0: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create, field_1: <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create) -> Self {
        Self { field_0, field_1 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_0: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            field_1: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    field_0: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create,
    field_1: <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create,
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    pub fn new(field_0: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create, field_1: <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create) -> Self {
        Self { field_0, field_1 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_0: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            field_1: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl std::fmt::Display for AnimalAsNotNullJsonbObjectCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for AnimalAsNotNullJsonbObjectCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl AnimalAsNotNullJsonbObjectCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_0, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_0", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_1, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_1", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("{increments}"))
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_0, query);
        query = <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_1, query);
        query
    }
}
impl std::fmt::Display for AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_0, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_0", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_1, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_1", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("jsonb_build_object('id', to_jsonb(gen_random_uuid()))||{increments}"))
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_0, query);
        query = <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_1, query);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalAsNotNullJsonbObjectSelect(postgresql_crud::NotEmptyUniqueEnumVec<AnimalAsNotNullJsonbObjectSelectElement>);
impl AnimalAsNotNullJsonbObjectSelect {
    pub fn new(value: postgresql_crud::NotEmptyUniqueEnumVec<AnimalAsNotNullJsonbObjectSelectElement>) -> Self {
        Self(value)
    }
}
impl AnimalAsNotNullJsonbObjectSelect {
    fn select_query_part_postgresql_type(&self, column: &std::primitive::str) -> std::string::String {
        let field_ident = column;
        let column_name_and_maybe_field_getter = column;
        let column_name_and_maybe_field_getter_for_error_message = column;
        let is_postgresql_type = true;
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = column_name_and_maybe_field_getter.to_string();
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in self.0.to_vec() {
            acc.push_str(& format!
            ("{}||", match element
            {
                AnimalAsNotNullJsonbObjectSelectElement :: Field0(value) => <
                postgresql_crud :: postgresql_json_type ::
                StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud ::
                PostgresqlJsonType > ::
                select_query_part(& value, "field_0", &
                column_name_and_maybe_field_getter_field_ident, &
                column_name_and_maybe_field_getter_for_error_message_field_ident,
                false,), AnimalAsNotNullJsonbObjectSelectElement ::
                Field1(value) => < postgresql_crud :: postgresql_json_type ::
                OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud
                :: PostgresqlJsonType > ::
                select_query_part(& value, "field_1", &
                column_name_and_maybe_field_getter_field_ident, &
                column_name_and_maybe_field_getter_for_error_message_field_ident,
                false,)
            }));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        if is_postgresql_type { format!("{acc}") } else { format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))") }
    }
}
impl sqlx::Type<sqlx::Postgres> for AnimalAsNotNullJsonbObjectSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for AnimalAsNotNullJsonbObjectSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalAsNotNullJsonbObjectSelectElement {
    #[serde(rename(serialize = "field_0", deserialize = "field_0"))]
    Field0(<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Select),
    #[serde(rename(serialize = "field_1", deserialize = "field_1"))]
    Field1(<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Select),
}
impl error_occurence_lib::ToStdStringString for AnimalAsNotNullJsonbObjectSelectElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectSelectElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            AnimalAsNotNullJsonbObjectSelectElement::Field0(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            AnimalAsNotNullJsonbObjectSelectElement::Field1(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdSelect(postgresql_crud::NotEmptyUniqueEnumVec<AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement>);
impl AnimalWithIdAsNotNullJsonbObjectWithIdSelect {
    pub fn new(value: postgresql_crud::NotEmptyUniqueEnumVec<AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement>) -> Self {
        Self(value)
    }
}
impl sqlx::Type<sqlx::Postgres> for AnimalWithIdAsNotNullJsonbObjectWithIdSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for AnimalWithIdAsNotNullJsonbObjectWithIdSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id(<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Select),
    #[serde(rename(serialize = "field_0", deserialize = "field_0"))]
    Field0(<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Select),
    #[serde(rename(serialize = "field_1", deserialize = "field_1"))]
    Field1(<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Select),
}
impl error_occurence_lib::ToStdStringString for AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Id(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Field0(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Field1(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum AnimalAsNotNullJsonbObjectWhereElement {
    Field0(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    Field1(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for AnimalAsNotNullJsonbObjectWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::Field0(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_0'"), is_need_to_add_logical_operator),
            Self::Field1(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_1'"), is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Field0(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Field1(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for AnimalAsNotNullJsonbObjectWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Field0(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Field1(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum AnimalWithIdAsNotNullJsonbObjectWithIdWhereElement {
    Id(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    Field0(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    Field1(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for AnimalWithIdAsNotNullJsonbObjectWithIdWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::Id(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'id'"), is_need_to_add_logical_operator),
            Self::Field0(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_0'"), is_need_to_add_logical_operator),
            Self::Field1(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_1'"), is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Id(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Field0(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Field1(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for AnimalWithIdAsNotNullJsonbObjectWithIdWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Id(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Field0(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Field1(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalAsNotNullJsonbObjectRead {
    #[serde(skip_serializing_if = "Option::is_none")]
    field_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_1: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
}
impl AnimalAsNotNullJsonbObjectRead {
    fn into_inner(self) -> AnimalAsNotNullJsonbObjectReadInner {
        AnimalAsNotNullJsonbObjectReadInner {
            field_0: match self.field_0 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
            field_1: match self.field_1 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalAsNotNullJsonbObjectReadTryFromErrorNamed {
    AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl AnimalAsNotNullJsonbObjectRead {
    pub fn try_new(
        field_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
        field_1: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
    ) -> Result<Self, AnimalAsNotNullJsonbObjectReadTryFromErrorNamed> {
        if let (None, None) = (&field_0, &field_1) {
            return Err(AnimalAsNotNullJsonbObjectReadTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { field_0, field_1 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for AnimalAsNotNullJsonbObjectRead {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => serde::__private::Ok(__Field::__field0),
                        1u64 => serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "field_0" => serde::__private::Ok(__Field::__field0),
                        "field_1" => serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"field_0" => serde::__private::Ok(__Field::__field0),
                        b"field_1" => serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<AnimalAsNotNullJsonbObjectRead>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AnimalAsNotNullJsonbObjectRead;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct AnimalAsNotNullJsonbObjectRead")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalAsNotNullJsonbObjectRead with 2 elements"));
                        }
                    };
                    let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalAsNotNullJsonbObjectRead with 2 elements"));
                        }
                    };
                    match AnimalAsNotNullJsonbObjectRead::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_0"));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                    std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
                                >(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if serde::__private::Option::is_some(&__field1) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_1"));
                                }
                                __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                    std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
                                >(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        serde::__private::Some(__field0) => __field0,
                        serde::__private::None => serde::__private::de::missing_field("field_0")?,
                    };
                    let __field1 = match __field1 {
                        serde::__private::Some(__field1) => __field1,
                        serde::__private::None => serde::__private::de::missing_field("field_1")?,
                    };
                    match AnimalAsNotNullJsonbObjectRead::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["field_0", "field_1"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "AnimalAsNotNullJsonbObjectRead",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<AnimalAsNotNullJsonbObjectRead>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_0: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
            field_1: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for AnimalAsNotNullJsonbObjectRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for AnimalAsNotNullJsonbObjectRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_1: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    fn into_inner(self) -> AnimalWithIdAsNotNullJsonbObjectWithIdReadInner {
        AnimalWithIdAsNotNullJsonbObjectWithIdReadInner {
            id: match self.id {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
            field_0: match self.field_0 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
            field_1: match self.field_1 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed {
    AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    pub fn try_new(
        id: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>,
        field_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
        field_1: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
    ) -> Result<Self, AnimalWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed> {
        if let (None, None, None) = (&id, &field_0, &field_1) {
            return Err(AnimalWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { id, field_0, field_1 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for AnimalWithIdAsNotNullJsonbObjectWithIdRead {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => serde::__private::Ok(__Field::__field0),
                        1u64 => serde::__private::Ok(__Field::__field1),
                        2u64 => serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "id" => serde::__private::Ok(__Field::__field0),
                        "field_0" => serde::__private::Ok(__Field::__field1),
                        "field_1" => serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => serde::__private::Ok(__Field::__field0),
                        b"field_0" => serde::__private::Ok(__Field::__field1),
                        b"field_1" => serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<AnimalWithIdAsNotNullJsonbObjectWithIdRead>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AnimalWithIdAsNotNullJsonbObjectWithIdRead;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct AnimalWithIdAsNotNullJsonbObjectWithIdRead")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalWithIdAsNotNullJsonbObjectWithIdRead with 3 elements"));
                        }
                    };
                    let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalWithIdAsNotNullJsonbObjectWithIdRead with 3 elements"));
                        }
                    };
                    let __field2 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalWithIdAsNotNullJsonbObjectWithIdRead with 3 elements"));
                        }
                    };
                    match AnimalWithIdAsNotNullJsonbObjectWithIdRead::try_new(__field0, __field1, __field2) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    let mut __field2: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("id"));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                    std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>,
                                >(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if serde::__private::Option::is_some(&__field1) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_0"));
                                }
                                __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                    std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
                                >(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if serde::__private::Option::is_some(&__field2) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_1"));
                                }
                                __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<
                                    std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
                                >(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        serde::__private::Some(__field0) => __field0,
                        serde::__private::None => serde::__private::de::missing_field("id")?,
                    };
                    let __field1 = match __field1 {
                        serde::__private::Some(__field1) => __field1,
                        serde::__private::None => serde::__private::de::missing_field("field_0")?,
                    };
                    let __field2 = match __field2 {
                        serde::__private::Some(__field2) => __field2,
                        serde::__private::None => serde::__private::de::missing_field("field_1")?,
                    };
                    match AnimalWithIdAsNotNullJsonbObjectWithIdRead::try_new(__field0, __field1, __field2) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["id", "field_0", "field_1"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "AnimalWithIdAsNotNullJsonbObjectWithIdRead",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<AnimalWithIdAsNotNullJsonbObjectWithIdRead>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
            field_0: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
            field_1: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AnimalAsNotNullJsonbObjectReadInner {
    field_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
    field_1: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdReadInner {
    id: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
    field_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
    field_1: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalAsNotNullJsonbObjectUpdate(postgresql_crud::NotEmptyUniqueEnumVec<AnimalAsNotNullJsonbObjectUpdateElement>);
impl AnimalAsNotNullJsonbObjectUpdate {
    pub fn new(value: postgresql_crud::NotEmptyUniqueEnumVec<AnimalAsNotNullJsonbObjectUpdateElement>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl AnimalAsNotNullJsonbObjectUpdate {
    fn update_query_part_postgresql_type(&self, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut std_option_option_object_acc = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
        let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
        for element in self.0.to_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => {
                    match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_0"), "field_0", increment) {
                        Ok(value) => {
                            std_option_option_object_acc = value;
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => {
                    match <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_1"), "field_1", increment) {
                        Ok(value) => {
                            std_option_option_object_acc = value;
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
            }
        }
        println!("HERE CAN BE BUG");
        if jsonb_set_path.is_empty() {
            Ok(std_option_option_object_acc)
        } else {
            Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',{std_option_option_object_acc})"))
        }
    }
    fn update_query_part_postgresql_json_type(&self, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut std_option_option_object_acc = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
        let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
        for element in self.0.to_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => {
                    match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_0"), "field_0", increment) {
                        Ok(value) => {
                            std_option_option_object_acc = value;
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => {
                    match <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_1"), "field_1", increment) {
                        Ok(value) => {
                            std_option_option_object_acc = value;
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
            }
        }
        println!("HERE CAN BE BUG");
        if jsonb_set_path.is_empty() {
            Ok(std_option_option_object_acc)
        } else {
            Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',{std_option_option_object_acc})"))
        }
    }
    fn update_query_bind_postgresql_type(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.0.into_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => {
                    query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
                }
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => {
                    query = <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
                }
            }
        }
        query
    }
    fn update_query_bind_postgresql_json_type(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.0.into_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => {
                    query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
                }
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => {
                    query = <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
                }
            }
        }
        query
    }
}
pub type AnimalWithIdAsNotNullJsonbObjectWithIdUpdate = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalAsNotNullJsonbObjectUpdateElement {
    #[serde(rename(serialize = "field_0", deserialize = "field_0"))]
    Field0(postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Update>),
    #[serde(rename(serialize = "field_1", deserialize = "field_1"))]
    Field1(postgresql_crud::Value<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Update>),
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectUpdateElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            AnimalAsNotNullJsonbObjectUpdateElement::Field0(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
            AnimalAsNotNullJsonbObjectUpdateElement::Field1(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update,
    fields: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update,
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    pub fn new(id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update, fields: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update) -> Self {
        Self { id, fields }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            fields: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    fn update_query_part_postgresql_json_type(&self, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let id_increment = match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                increment.to_string()
            }
            None => {
                return Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        };
        match self.fields.update_query_part_postgresql_json_type(&jsonb_set_accumulator, &jsonb_set_target, &jsonb_set_path, increment) {
            Ok(value) => Ok(format!("when {jsonb_set_target}->>'id' = ${id_increment} then {value} ")),
            Err(error) => Err(error),
        }
    }
    fn update_query_bind_postgresql_json_type(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = self.id.query_bind_as_postgresql_text(query);
        query = self.fields.update_query_bind_postgresql_json_type(query);
        query
    }
}
impl postgresql_crud::PostgresqlJsonType for AnimalAsNotNullJsonbObject {
    type TableTypeDeclaration = AnimalAsNotNullJsonbObjectTableTypeDeclaration;
    type Create = AnimalAsNotNullJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = AnimalAsNotNullJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = if is_postgresql_type { column_name_and_maybe_field_getter.to_string() } else { format!("{column_name_and_maybe_field_getter}->'{field_ident}'") };
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in value.0.to_vec() {
            acc.push_str(& format!
            ("{}||", match element
            {
                AnimalAsNotNullJsonbObjectSelectElement :: Field0(value) => <
                postgresql_crud :: postgresql_json_type ::
                StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud ::
                PostgresqlJsonType > ::
                select_query_part(& value, "field_0", &
                column_name_and_maybe_field_getter_field_ident, &
                column_name_and_maybe_field_getter_for_error_message_field_ident,
                false,), AnimalAsNotNullJsonbObjectSelectElement ::
                Field1(value) => < postgresql_crud :: postgresql_json_type ::
                OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud
                :: PostgresqlJsonType > ::
                select_query_part(& value, "field_1", &
                column_name_and_maybe_field_getter_field_ident, &
                column_name_and_maybe_field_getter_for_error_message_field_ident,
                false,)
            }));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        if is_postgresql_type { format!("{acc}") } else { format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))") }
    }
    type WhereElement = AnimalAsNotNullJsonbObjectWhereElement;
    type Read = AnimalAsNotNullJsonbObjectRead;
    type ReadOnlyIds = ();//todo
    type ReadInner = AnimalAsNotNullJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = AnimalAsNotNullJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.update_query_part_postgresql_json_type(jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment)
    }
    fn update_query_bind(value: Self::Update, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.update_query_bind_postgresql_json_type(query)
    }
}
impl postgresql_crud::PostgresqlType for AnimalAsNotNullJsonbObject {
    type TableTypeDeclaration = AnimalAsNotNullJsonbObjectTableTypeDeclaration;
    type Create = AnimalAsNotNullJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = AnimalAsNotNullJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{} as {column}", value.select_query_part_postgresql_type(column))
    }
    type WhereElement = AnimalAsNotNullJsonbObjectWhereElement;
    type Read = AnimalAsNotNullJsonbObjectRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadInner = AnimalAsNotNullJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = AnimalAsNotNullJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.update_query_part_postgresql_type(jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment)
    }
    fn update_query_bind<'a>(value: Self::Update, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.update_query_bind_postgresql_type(query)
    }
}
impl postgresql_crud::PostgresqlJsonType for AnimalWithIdAsNotNullJsonbObjectWithId {
    type TableTypeDeclaration = AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = AnimalWithIdAsNotNullJsonbObjectWithIdCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = AnimalWithIdAsNotNullJsonbObjectWithIdSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = if is_postgresql_type { column_name_and_maybe_field_getter.to_string() } else { format!("{column_name_and_maybe_field_getter}->'{field_ident}'") };
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in value.0.to_vec() {
            acc.push_str(& format!
            ("{}||", match element
            {
                AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement ::
                Id(value) => < postgresql_crud :: postgresql_json_type ::
                UuidUuidAsNotNullJsonbString as postgresql_crud ::
                PostgresqlJsonType > ::
                select_query_part(& value, "id", &
                column_name_and_maybe_field_getter_field_ident, &
                column_name_and_maybe_field_getter_for_error_message_field_ident,
                false,), AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement
                :: Field0(value) => < postgresql_crud :: postgresql_json_type
                :: StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud ::
                PostgresqlJsonType > ::
                select_query_part(& value, "field_0", &
                column_name_and_maybe_field_getter_field_ident, &
                column_name_and_maybe_field_getter_for_error_message_field_ident,
                false,), AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement
                :: Field1(value) => < postgresql_crud :: postgresql_json_type
                :: OptionStdPrimitiveI8AsNullableJsonbNumber as
                postgresql_crud :: PostgresqlJsonType > ::
                select_query_part(& value, "field_1", &
                column_name_and_maybe_field_getter_field_ident, &
                column_name_and_maybe_field_getter_for_error_message_field_ident,
                false,)
            }));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        if is_postgresql_type { format!("{acc}") } else { format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))") }
    }
    type WhereElement = AnimalWithIdAsNotNullJsonbObjectWithIdWhereElement;
    type Read = AnimalWithIdAsNotNullJsonbObjectWithIdRead;
    type ReadOnlyIds = ();//todo
    type ReadInner = AnimalWithIdAsNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = AnimalWithIdAsNotNullJsonbObjectWithIdUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.update_query_part_postgresql_json_type(jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment)
    }
    fn update_query_bind(value: Self::Update, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.update_query_bind_postgresql_json_type(query)
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlTypeTestCases<AnimalAsNotNullJsonbObjectReadInner> for AnimalAsNotNullJsonbObject {
    type Element = Self;
    fn test_cases() -> std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner> {
        let mut acc = vec![];
        for field_0 in <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>::test_cases() {
            acc.push(AnimalAsNotNullJsonbObjectReadInner
            {
                field_0 : Some(postgresql_crud :: Value { value : field_0 }),
                field_1 :
                Some(postgresql_crud :: Value
                {
                    value : < postgresql_crud :: postgresql_json_type ::
                    OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud
                    :: PostgresqlJsonType > ::
                    into_inner(< < postgresql_crud :: postgresql_json_type ::
                    OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud
                    :: PostgresqlJsonType > :: Read as postgresql_crud ::
                    DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement >
                    ::
                    default_but_option_is_always_some_and_vec_always_contains_one_element())
                }),
            });
        }
        for field_1 in
            <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>::test_cases()
        {
            acc.push(AnimalAsNotNullJsonbObjectReadInner {
                field_1: Some(postgresql_crud::Value { value: field_1 }),
                field_0: Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(
                        <<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                    ),
                }),
            });
        }
        acc
    }
    fn read_new_or_try_new_unwraped_for_test(value: AnimalAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Read {
        < Self :: Element as postgresql_crud :: PostgresqlType > :: Read ::
        try_new(match value.field_0
        {
            Some(value) =>
            Some(postgresql_crud :: Value
            {
                value : < postgresql_crud :: postgresql_json_type ::
                StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: tests
                :: PostgresqlJsonTypeTestCases < < postgresql_crud ::
                postgresql_json_type :: StdPrimitiveI8AsNotNullJsonbNumber as
                postgresql_crud :: PostgresqlJsonType > :: ReadInner > > ::
                read_new_or_try_new_unwraped_for_test(value.value)
            }), None => None
        }, match value.field_1
        {
            Some(value) =>
            Some(postgresql_crud :: Value
            {
                value : < postgresql_crud :: postgresql_json_type ::
                OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud
                :: tests :: PostgresqlJsonTypeTestCases < < postgresql_crud ::
                postgresql_json_type ::
                OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud
                :: PostgresqlJsonType > :: ReadInner > > ::
                read_new_or_try_new_unwraped_for_test(value.value)
            }), None => None
        }).unwrap()
    }
    fn update_new_or_try_new_unwraped_for_test(value: AnimalAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(
            postgresql_crud::NotEmptyUniqueEnumVec::try_new({
                let mut acc = vec![];
                if let Some(value) = value.field_0 {
                    acc.push(AnimalAsNotNullJsonbObjectUpdateElement ::
                Field0(postgresql_crud :: Value
                {
                    value : < postgresql_crud :: postgresql_json_type ::
                    StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud ::
                    tests :: PostgresqlJsonTypeTestCases < < postgresql_crud ::
                    postgresql_json_type :: StdPrimitiveI8AsNotNullJsonbNumber
                    as postgresql_crud :: PostgresqlJsonType > :: ReadInner > >
                    :: update_new_or_try_new_unwraped_for_test(value.value),
                }));
                }
                if let Some(value) = value.field_1 {
                    acc.push(AnimalAsNotNullJsonbObjectUpdateElement::Field1(postgresql_crud::Value {
                        value: <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases<
                            <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner,
                        >>::update_new_or_try_new_unwraped_for_test(value.value),
                    }));
                }
                acc
            })
            .unwrap(),
        )
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlJsonTypeTestCases<AnimalAsNotNullJsonbObjectReadInner> for AnimalAsNotNullJsonbObject {
    type Element = Self;
    fn test_cases() -> std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner> {
        let mut acc = vec![];
        for field_0 in <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>::test_cases() {
            acc.push(AnimalAsNotNullJsonbObjectReadInner
            {
                field_0 : Some(postgresql_crud :: Value { value : field_0 }),
                field_1 :
                Some(postgresql_crud :: Value
                {
                    value : < postgresql_crud :: postgresql_json_type ::
                    OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud
                    :: PostgresqlJsonType > ::
                    into_inner(< < postgresql_crud :: postgresql_json_type ::
                    OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud
                    :: PostgresqlJsonType > :: Read as postgresql_crud ::
                    DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement >
                    ::
                    default_but_option_is_always_some_and_vec_always_contains_one_element())
                }),
            });
        }
        for field_1 in
            <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases<<postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>::test_cases()
        {
            acc.push(AnimalAsNotNullJsonbObjectReadInner {
                field_1: Some(postgresql_crud::Value { value: field_1 }),
                field_0: Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(
                        <<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                    ),
                }),
            });
        }
        acc
    }
    fn read_new_or_try_new_unwraped_for_test(value: AnimalAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        < Self :: Element as postgresql_crud :: PostgresqlType > :: Read ::
        try_new(match value.field_0
        {
            Some(value) =>
            Some(postgresql_crud :: Value
            {
                value : < postgresql_crud :: postgresql_json_type ::
                StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud :: tests
                :: PostgresqlJsonTypeTestCases < < postgresql_crud ::
                postgresql_json_type :: StdPrimitiveI8AsNotNullJsonbNumber as
                postgresql_crud :: PostgresqlJsonType > :: ReadInner > > ::
                read_new_or_try_new_unwraped_for_test(value.value)
            }), None => None
        }, match value.field_1
        {
            Some(value) =>
            Some(postgresql_crud :: Value
            {
                value : < postgresql_crud :: postgresql_json_type ::
                OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud
                :: tests :: PostgresqlJsonTypeTestCases < < postgresql_crud ::
                postgresql_json_type ::
                OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud
                :: PostgresqlJsonType > :: ReadInner > > ::
                read_new_or_try_new_unwraped_for_test(value.value)
            }), None => None
        }).unwrap()
    }
    fn update_new_or_try_new_unwraped_for_test(value: AnimalAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(
            postgresql_crud::NotEmptyUniqueEnumVec::try_new({
                let mut acc = vec![];
                if let Some(value) = value.field_0 {
                    acc.push(AnimalAsNotNullJsonbObjectUpdateElement ::
                Field0(postgresql_crud :: Value
                {
                    value : < postgresql_crud :: postgresql_json_type ::
                    StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud ::
                    tests :: PostgresqlJsonTypeTestCases < < postgresql_crud ::
                    postgresql_json_type :: StdPrimitiveI8AsNotNullJsonbNumber
                    as postgresql_crud :: PostgresqlJsonType > :: ReadInner > >
                    :: update_new_or_try_new_unwraped_for_test(value.value),
                }));
                }
                if let Some(value) = value.field_1 {
                    acc.push(AnimalAsNotNullJsonbObjectUpdateElement::Field1(postgresql_crud::Value {
                        value: <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases<
                            <postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner,
                        >>::update_new_or_try_new_unwraped_for_test(value.value),
                    }));
                }
                acc
            })
            .unwrap(),
        )
    }
}