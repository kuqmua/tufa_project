// generate_postgresql_json_types::generate_postgresql_json_types!();

#[derive(Debug, Clone, PartialEq, PartialOrd, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdVecVecStdVecVecUuidUuid(pub std::vec::Vec<std::vec::Vec<UuidUuid>>);
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecStdVecVecUuidUuid {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![vec![
            crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        ]])
    }
}
impl error_occurence_lib::ToStdStringString for StdVecVecStdVecVecUuidUuid {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
pub type StdVecVecStdVecVecUuidUuidCreate = StdVecVecStdVecVecUuidUuid;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdVecVecStdVecVecUuidUuidSelect {
    //todo could not implement multi dimension pagination
    pagination: crate::pagination::Pagination,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecStdVecVecUuidUuidSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
pub type StdVecVecStdVecVecUuidUuidRead = StdVecVecStdVecVecUuidUuid;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct StdVecVecStdVecVecUuidUuidWhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: StdVecVecStdVecVecUuidUuid,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecStdVecVecUuidUuidWhereElementEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'_> for StdVecVecStdVecVecUuidUuidWhereElementEqual {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}

#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum StdVecVecStdVecVecUuidUuidWhereElement {
    Equal(StdVecVecStdVecVecUuidUuidWhereElementEqual),
}
impl crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'_> for StdVecVecStdVecVecUuidUuidWhereElement {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn where_query_bind<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for StdVecVecStdVecVecUuidUuidWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecStdVecVecUuidUuidWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::Equal(
            crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        )]
    }
}
pub type StdVecVecStdVecVecUuidUuidUpdate = StdVecVecStdVecVecUuidUuid;
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdVecVecStdVecVecUuidUuidOptionToUpdateTryGenerateErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl crate::postgresql_json_type_trait::PostgresqlJsonType for StdVecVecStdVecVecUuidUuid {
    type Create<'a> = StdVecVecStdVecVecUuidUuidCreate;
    fn create_query_part(_: &Self::Create<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::postgresql_json_type_trait::CreateQueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("${increment}"))
            }
            None => Err(crate::postgresql_json_type_trait::CreateQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn create_query_bind<'a>(value: Self::Create<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(value.0));
        query
    }
    type Select<'a> = StdVecVecStdVecVecUuidUuidSelect;
    type Read<'a> = StdVecVecStdVecVecUuidUuidRead;
    fn select_query_part(value: &Self::Select<'_>, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        // println!("fn select_query_part");
        //todo change
        let start = value.pagination.start();
        let end = value.pagination.end();
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {start} and {end})))")
    }
    type WhereElement<'a> = StdVecVecStdVecVecUuidUuidWhereElement;
    type Update<'a> = StdVecVecStdVecVecUuidUuidUpdate;
    type UpdateQueryPartErrorNamed = StdVecVecStdVecVecUuidUuidOptionToUpdateTryGenerateErrorNamed;
    fn update_query_part(_: &Self::Update<'_>, jsonb_set_accumulator: &std::primitive::str, _: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, Self::UpdateQueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(Self::UpdateQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn update_query_bind<'a>(value: Self::Update<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(value.0)); //todo remove .0 and impl Encode instead
        query
    }
}



//////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, PartialOrd, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UuidUuid(pub uuid::Uuid);
impl schemars::JsonSchema for UuidUuid {
    fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("UuidUuid")
    }
    fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
        schemars::_private::alloc::borrow::Cow::Borrowed("postgresql_crud_common::f::UuidUuid")
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        {
            let mut schema = generator.subschema_for::<std::string::String>();
            schemars::_private::insert_validation_property(&mut schema, "string", "minLength", 36);
            schemars::_private::insert_validation_property(&mut schema, "string", "maxLength", 36);
            schemars::_private::insert_validation_property(&mut schema, "array", "minItems", 36);
            schemars::_private::insert_validation_property(&mut schema, "array", "maxItems", 36);
            schema
        }
    }
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuid {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
impl error_occurence_lib::ToStdStringString for UuidUuid {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
pub type UuidUuidCreate = UuidUuid;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct UuidUuidSelect {}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
pub type UuidUuidRead = UuidUuid;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct UuidUuidWhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: UuidUuid,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidWhereElementEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for UuidUuidWhereElementEqual {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct UuidUuidWhereElementCaseSensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum UuidUuidWhereElementCaseSensitiveRegularExpressionTryNewErrorNamed {
    IsEmpty { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl UuidUuidWhereElementCaseSensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String) -> Result<Self, UuidUuidWhereElementCaseSensitiveRegularExpressionTryNewErrorNamed> {
        if !value.is_empty() {
            Ok(Self { logical_operator, value })
        } else {
            Err(UuidUuidWhereElementCaseSensitiveRegularExpressionTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for UuidUuidWhereElementCaseSensitiveRegularExpression {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<UuidUuidWhereElementCaseSensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = UuidUuidWhereElementCaseSensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct UuidUuidWhereElementCaseSensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct UuidUuidWhereElementCaseSensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct UuidUuidWhereElementCaseSensitiveRegularExpression with 2 elements"));
                        }
                    };
                    match UuidUuidWhereElementCaseSensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match UuidUuidWhereElementCaseSensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "UuidUuidWhereElementCaseSensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<UuidUuidWhereElementCaseSensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidWhereElementCaseSensitiveRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for UuidUuidWhereElementCaseSensitiveRegularExpression {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}(substring(({})::text from 2 for length(({})::text) - 2) ~ ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct UuidUuidWhereElementCaseInsensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum UuidUuidWhereElementCaseInsensitiveRegularExpressionTryNewErrorNamed {
    IsEmpty { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl UuidUuidWhereElementCaseInsensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String) -> Result<Self, UuidUuidWhereElementCaseInsensitiveRegularExpressionTryNewErrorNamed> {
        if !value.is_empty() {
            Ok(Self { logical_operator, value })
        } else {
            Err(UuidUuidWhereElementCaseInsensitiveRegularExpressionTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for UuidUuidWhereElementCaseInsensitiveRegularExpression {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<UuidUuidWhereElementCaseInsensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = UuidUuidWhereElementCaseInsensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct UuidUuidWhereElementCaseInsensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct UuidUuidWhereElementCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct UuidUuidWhereElementCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    match UuidUuidWhereElementCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match UuidUuidWhereElementCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "UuidUuidWhereElementCaseInsensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<UuidUuidWhereElementCaseInsensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidWhereElementCaseInsensitiveRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for UuidUuidWhereElementCaseInsensitiveRegularExpression {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}(substring(({})::text from 2 for length(({})::text) - 2) ~* ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum UuidUuidWhereElement {
    Equal(UuidUuidWhereElementEqual),
    CaseSensitiveRegularExpression(UuidUuidWhereElementCaseSensitiveRegularExpression),
    CaseInsensitiveRegularExpression(UuidUuidWhereElementCaseInsensitiveRegularExpression),
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for UuidUuidWhereElement {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CaseSensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CaseInsensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn where_query_bind(self, 
        // mut 
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::CaseSensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::CaseInsensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for UuidUuidWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UuidUuidWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CaseSensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CaseInsensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
pub type UuidUuidUpdate = UuidUuid;
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum UuidUuidUpdateErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl crate::postgresql_json_type_trait::PostgresqlJsonType for UuidUuid {
    type Create<'a> = UuidUuidCreate;
    fn create_query_part(value: &Self::Create<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::postgresql_json_type_trait::CreateQueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("${increment}"))
            }
            None => Err(crate::postgresql_json_type_trait::CreateQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn create_query_bind<'a>(value: Self::Create<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(value.0));
        query
    }
    type Select<'a> = UuidUuidSelect;
    fn select_query_part(value: &Self::Select<'_>, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value', {column_name_and_maybe_field_getter}->'{field_ident}'))")
    }
    type WhereElement<'a> = UuidUuidWhereElement;
    type Read<'a> = UuidUuidRead;
    type Update<'a> = UuidUuidUpdate;
    type UpdateQueryPartErrorNamed = UuidUuidUpdateErrorNamed;
    fn update_query_part(value: &Self::Update<'_>, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, Self::UpdateQueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(Self::UpdateQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn update_query_bind<'a>(value: Self::Update<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(value.0));
        query
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdOptionOptionUuidUuid(pub std::option::Option<UuidUuid>);
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdOptionOptionUuidUuid {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl error_occurence_lib::ToStdStringString for StdOptionOptionUuidUuid {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
pub type StdOptionOptionUuidUuidCreate = StdOptionOptionUuidUuid;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdOptionOptionUuidUuidSelect {}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdOptionOptionUuidUuidSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
pub type StdOptionOptionUuidUuidRead = StdOptionOptionUuidUuid;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct StdOptionOptionUuidUuidWhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: StdOptionOptionUuidUuid,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdOptionOptionUuidUuidWhereElementEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdOptionOptionUuidUuidWhereElementEqual {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpressionTryNewErrorNamed {
    IsEmpty { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String) -> Result<Self, StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpressionTryNewErrorNamed> {
        if !value.is_empty() {
            Ok(Self { logical_operator, value })
        } else {
            Err(StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpressionTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression with 2 elements"));
                        }
                    };
                    match StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}(substring(({})::text from 2 for length(({})::text) - 2) ~ ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpressionTryNewErrorNamed {
    IsEmpty { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String) -> Result<Self, StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpressionTryNewErrorNamed> {
        if !value.is_empty() {
            Ok(Self { logical_operator, value })
        } else {
            Err(StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpressionTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    match StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}(substring(({})::text from 2 for length(({})::text) - 2) ~* ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum StdOptionOptionUuidUuidWhereElement {
    Equal(StdOptionOptionUuidUuidWhereElementEqual),
    CaseSensitiveRegularExpression(StdOptionOptionUuidUuidWhereElementCaseSensitiveRegularExpression),
    CaseInsensitiveRegularExpression(StdOptionOptionUuidUuidWhereElementCaseInsensitiveRegularExpression),
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdOptionOptionUuidUuidWhereElement {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CaseSensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::CaseInsensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn where_query_bind(self, 
        // mut 
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::CaseSensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::CaseInsensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for StdOptionOptionUuidUuidWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdOptionOptionUuidUuidWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CaseSensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::CaseInsensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
pub type StdOptionOptionUuidUuidUpdate = StdOptionOptionUuidUuid;
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdOptionOptionUuidUuidUpdateErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl crate::postgresql_json_type_trait::PostgresqlJsonType for StdOptionOptionUuidUuid {
    type Create<'a> = StdOptionOptionUuidUuidCreate;
    fn create_query_part(value: &Self::Create<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::postgresql_json_type_trait::CreateQueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("${increment}"))
            }
            None => Err(crate::postgresql_json_type_trait::CreateQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn create_query_bind<'a>(value: Self::Create<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(value.0));
        query
    }
    type Select<'a> = StdOptionOptionUuidUuidSelect;
    fn select_query_part(value: &Self::Select<'_>, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value', {column_name_and_maybe_field_getter}->'{field_ident}'))")
    }
    type WhereElement<'a> = StdOptionOptionUuidUuidWhereElement;
    type Read<'a> = StdOptionOptionUuidUuidRead;
    type Update<'a> = StdOptionOptionUuidUuidUpdate;
    type UpdateQueryPartErrorNamed = StdOptionOptionUuidUuidUpdateErrorNamed;
    fn update_query_part(value: &Self::Update<'_>, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, Self::UpdateQueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(Self::UpdateQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn update_query_bind<'a>(value: Self::Update<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(value.0));
        query
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuid(pub std::vec::Vec<UuidUuid>);
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuid {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl error_occurence_lib::ToStdStringString for StdVecVecUuidUuid {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
pub type StdVecVecUuidUuidCreate = StdVecVecUuidUuid;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct StdVecVecUuidUuidSelect {
    pagination: crate::pagination::Pagination,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            pagination: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
pub type StdVecVecUuidUuidRead = StdVecVecUuidUuid;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: StdVecVecUuidUuid,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementEqual {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementLengthEqual {
    logical_operator: crate::LogicalOperator,
    value: std::primitive::i64,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum StdVecVecUuidUuidWhereElementLengthEqualTryNewErrorNamed {
    LengthIsNegative {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdVecVecUuidUuidWhereElementLengthEqual {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::primitive::i64) -> Result<Self, StdVecVecUuidUuidWhereElementLengthEqualTryNewErrorNamed> {
        if value >= 0 {
            Ok(Self { logical_operator, value })
        } else {
            Err(StdVecVecUuidUuidWhereElementLengthEqualTryNewErrorNamed::LengthIsNegative {
                value,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdVecVecUuidUuidWhereElementLengthEqual {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<StdVecVecUuidUuidWhereElementLengthEqual>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdVecVecUuidUuidWhereElementLengthEqual;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct StdVecVecUuidUuidWhereElementLengthEqual")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct StdVecVecUuidUuidWhereElementLengthEqual with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct StdVecVecUuidUuidWhereElementLengthEqual with 2 elements"));
                        }
                    };
                    match StdVecVecUuidUuidWhereElementLengthEqual::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::primitive::i64> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i64>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match StdVecVecUuidUuidWhereElementLengthEqual::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "StdVecVecUuidUuidWhereElementLengthEqual",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdVecVecUuidUuidWhereElementLengthEqual>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementLengthEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementLengthEqual {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}(jsonb_array_length({}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementLengthMoreThan {
    logical_operator: crate::LogicalOperator,
    value: std::primitive::i64,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum StdVecVecUuidUuidWhereElementLengthMoreThanTryNewErrorNamed {
    LengthIsNegative {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdVecVecUuidUuidWhereElementLengthMoreThan {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::primitive::i64) -> Result<Self, StdVecVecUuidUuidWhereElementLengthMoreThanTryNewErrorNamed> {
        if value >= 0 {
            Ok(Self { logical_operator, value })
        } else {
            Err(StdVecVecUuidUuidWhereElementLengthMoreThanTryNewErrorNamed::LengthIsNegative {
                value,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdVecVecUuidUuidWhereElementLengthMoreThan {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<StdVecVecUuidUuidWhereElementLengthMoreThan>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdVecVecUuidUuidWhereElementLengthMoreThan;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct StdVecVecUuidUuidWhereElementLengthMoreThan")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct StdVecVecUuidUuidWhereElementLengthMoreThan with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct StdVecVecUuidUuidWhereElementLengthMoreThan with 2 elements"));
                        }
                    };
                    match StdVecVecUuidUuidWhereElementLengthMoreThan::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::primitive::i64> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i64>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match StdVecVecUuidUuidWhereElementLengthMoreThan::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "StdVecVecUuidUuidWhereElementLengthMoreThan",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdVecVecUuidUuidWhereElementLengthMoreThan>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementLengthMoreThan {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementLengthMoreThan {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}(jsonb_array_length({}) > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementPositionEqual {
    logical_operator: crate::LogicalOperator,
    value: StdVecVecUuidUuid,
    position: std::primitive::i32,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum StdVecVecUuidUuidWhereElementPositionEqualTryNewErrorNamed {
    PositionIsLessThanZero {
        #[eo_to_std_string_string_serialize_deserialize]
        position: std::primitive::i32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdVecVecUuidUuidWhereElementPositionEqual {
    fn try_new(logical_operator: crate::LogicalOperator, value: StdVecVecUuidUuid, position: std::primitive::i32) -> Result<Self, StdVecVecUuidUuidWhereElementPositionEqualTryNewErrorNamed> {
        if position >= 0 {
            Ok(Self { logical_operator, value, position })
        } else {
            Err(StdVecVecUuidUuidWhereElementPositionEqualTryNewErrorNamed::PositionIsLessThanZero {
                position,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdVecVecUuidUuidWhereElementPositionEqual {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        "position" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        b"position" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<StdVecVecUuidUuidWhereElementPositionEqual>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdVecVecUuidUuidWhereElementPositionEqual;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct StdVecVecUuidUuidWhereElementPositionEqual")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct StdVecVecUuidUuidWhereElementPositionEqual with 3 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<StdVecVecUuidUuid>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct StdVecVecUuidUuidWhereElementPositionEqual with 3 elements"));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<std::primitive::i32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct StdVecVecUuidUuidWhereElementPositionEqual with 3 elements"));
                        }
                    };
                    match StdVecVecUuidUuidWhereElementPositionEqual::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<StdVecVecUuidUuid> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<std::primitive::i32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<StdVecVecUuidUuid>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("position"));
                                }
                                __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i32>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::de::missing_field("position")?,
                    };
                    match StdVecVecUuidUuidWhereElementPositionEqual::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value", "position"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "StdVecVecUuidUuidWhereElementPositionEqual",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdVecVecUuidUuidWhereElementPositionEqual>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementPositionEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            position: ::core::default::Default::default(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementPositionEqual {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(first_increment) => {
                *increment = first_increment;
                match increment.checked_add(1) {
                    Some(second_increment) => {
                        *increment = second_increment;
                        Ok(format!("{}({}->${} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, first_increment, second_increment,))
                    }
                    None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.position);
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementContainsAllElementsOfArray {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<StdVecVecUuidUuid>,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum StdVecVecUuidUuidWhereElementContainsAllElementsOfArrayTryNewErrorNamed {
    IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: StdVecVecUuidUuid,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdVecVecUuidUuidWhereElementContainsAllElementsOfArray {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<StdVecVecUuidUuid>) -> Result<Self, StdVecVecUuidUuidWhereElementContainsAllElementsOfArrayTryNewErrorNamed> {
        if value.is_empty() {
            return Err(StdVecVecUuidUuidWhereElementContainsAllElementsOfArrayTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            for element in &value {
                if !acc.contains(&element) {
                    acc.push(element);
                } else {
                    return Err(StdVecVecUuidUuidWhereElementContainsAllElementsOfArrayTryNewErrorNamed::NotUnique {
                        value: element.clone(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(Self { logical_operator, value })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdVecVecUuidUuidWhereElementContainsAllElementsOfArray {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<StdVecVecUuidUuidWhereElementContainsAllElementsOfArray>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdVecVecUuidUuidWhereElementContainsAllElementsOfArray;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct StdVecVecUuidUuidWhereElementContainsAllElementsOfArray")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct StdVecVecUuidUuidWhereElementContainsAllElementsOfArray with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<StdVecVecUuidUuid>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct StdVecVecUuidUuidWhereElementContainsAllElementsOfArray with 2 elements"));
                        }
                    };
                    match StdVecVecUuidUuidWhereElementContainsAllElementsOfArray::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::vec::Vec<StdVecVecUuidUuid>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<StdVecVecUuidUuid>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match StdVecVecUuidUuidWhereElementContainsAllElementsOfArray::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "StdVecVecUuidUuidWhereElementContainsAllElementsOfArray",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdVecVecUuidUuidWhereElementContainsAllElementsOfArray>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementContainsAllElementsOfArray {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()],
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementContainsAllElementsOfArray {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} @> ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementOverlapsWithArray {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<StdVecVecUuidUuid>,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum StdVecVecUuidUuidWhereElementOverlapsWithArrayTryNewErrorNamed {
    IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: StdVecVecUuidUuid,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdVecVecUuidUuidWhereElementOverlapsWithArray {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<StdVecVecUuidUuid>) -> Result<Self, StdVecVecUuidUuidWhereElementOverlapsWithArrayTryNewErrorNamed> {
        if value.is_empty() {
            return Err(StdVecVecUuidUuidWhereElementOverlapsWithArrayTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            for element in &value {
                if !acc.contains(&element) {
                    acc.push(element);
                } else {
                    return Err(StdVecVecUuidUuidWhereElementOverlapsWithArrayTryNewErrorNamed::NotUnique {
                        value: element.clone(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(Self { logical_operator, value })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdVecVecUuidUuidWhereElementOverlapsWithArray {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<StdVecVecUuidUuidWhereElementOverlapsWithArray>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdVecVecUuidUuidWhereElementOverlapsWithArray;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct StdVecVecUuidUuidWhereElementOverlapsWithArray")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct StdVecVecUuidUuidWhereElementOverlapsWithArray with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<StdVecVecUuidUuid>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct StdVecVecUuidUuidWhereElementOverlapsWithArray with 2 elements"));
                        }
                    };
                    match StdVecVecUuidUuidWhereElementOverlapsWithArray::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::vec::Vec<StdVecVecUuidUuid>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<StdVecVecUuidUuid>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match StdVecVecUuidUuidWhereElementOverlapsWithArray::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "StdVecVecUuidUuidWhereElementOverlapsWithArray",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdVecVecUuidUuidWhereElementOverlapsWithArray>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementOverlapsWithArray {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()],
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementOverlapsWithArray {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!(
                    "{}(exists (select 1 from jsonb_array_elements_text({}) as e1 join jsonb_array_elements_text(${}) as e2 on e1.value = e2.value))",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    column,
                    increment
                ))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementAllElementsEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: std::vec::Vec<UuidUuid>,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementAllElementsEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: vec![crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()],
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementAllElementsEqual {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}) as el where (el) <> ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
    position: std::primitive::i32,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpressionTryNewErrorNamed {
    PositionIsLessThanZero {
        #[eo_to_std_string_string_serialize_deserialize]
        position: std::primitive::i32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String, position: std::primitive::i32) -> Result<Self, StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpressionTryNewErrorNamed> {
        if position >= 0 {
            Ok(Self { logical_operator, value, position })
        } else {
            Err(StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpressionTryNewErrorNamed::PositionIsLessThanZero {
                position,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        "position" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        b"position" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression with 3 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression with 3 elements"));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<std::primitive::i32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression with 3 elements"));
                        }
                    };
                    match StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<std::primitive::i32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("position"));
                                }
                                __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i32>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::de::missing_field("position")?,
                    };
                    match StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value", "position"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
            position: ::core::default::Default::default(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(first_increment) => {
                *increment = first_increment;
                match increment.checked_add(1) {
                    Some(second_increment) => {
                        *increment = second_increment;
                        Ok(format!("{}({}->>${} ~ ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, first_increment, second_increment,))
                    }
                    None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.position);
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
    position: std::primitive::i32,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpressionTryNewErrorNamed {
    PositionIsLessThanZero {
        #[eo_to_std_string_string_serialize_deserialize]
        position: std::primitive::i32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String, position: std::primitive::i32) -> Result<Self, StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpressionTryNewErrorNamed> {
        if position >= 0 {
            Ok(Self { logical_operator, value, position })
        } else {
            Err(StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpressionTryNewErrorNamed::PositionIsLessThanZero {
                position,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        "position" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        b"position" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression with 3 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression with 3 elements"));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<std::primitive::i32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression with 3 elements"));
                        }
                    };
                    match StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<std::primitive::i32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("position"));
                                }
                                __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i32>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::de::missing_field("position")?,
                    };
                    match StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value", "position"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
            position: ::core::default::Default::default(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(first_increment) => {
                *increment = first_increment;
                match increment.checked_add(1) {
                    Some(second_increment) => {
                        *increment = second_increment;
                        Ok(format!("{}({}->>${} ~* ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, first_increment, second_increment,))
                    }
                    None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.position);
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpressionTryNewErrorNamed {
    IsEmpty { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String) -> Result<Self, StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpressionTryNewErrorNamed> {
        if !value.is_empty() {
            Ok(Self { logical_operator, value })
        } else {
            Err(StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpressionTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression with 2 elements"));
                        }
                    };
                    match StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!(
                    "{}(exists(select 1 from jsonb_array_elements({}) as el where substring(el::text from 2 for length(el::text) - 2) ~ ${}))",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    column,
                    increment
                ))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpressionTryNewErrorNamed {
    IsEmpty { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String) -> Result<Self, StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpressionTryNewErrorNamed> {
        if !value.is_empty() {
            Ok(Self { logical_operator, value })
        } else {
            Err(StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpressionTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    match StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!(
                    "{}(exists(select 1 from jsonb_array_elements({}) as el where substring(el::text from 2 for length(el::text) - 2) ~* ${}))",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    column,
                    increment
                ))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpressionTryNewErrorNamed {
    IsEmpty { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String) -> Result<Self, StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpressionTryNewErrorNamed> {
        if !value.is_empty() {
            Ok(Self { logical_operator, value })
        } else {
            Err(StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpressionTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression with 2 elements"));
                        }
                    };
                    match StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!(
                    "{}(not exists(select 1 from jsonb_array_elements({}) as el where substring(el::text from 2 for length(el::text) - 2) !~ ${}))",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    column,
                    increment
                ))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpressionTryNewErrorNamed {
    IsEmpty { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String) -> Result<Self, StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpressionTryNewErrorNamed> {
        if !value.is_empty() {
            Ok(Self { logical_operator, value })
        } else {
            Err(StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpressionTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression {
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
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
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
                marker: _serde::__private::PhantomData<StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    match StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!(
                    "{}(not exists(select 1 from jsonb_array_elements({}) as el where substring(el::text from 2 for length(el::text) - 2) !~* ${}))",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    column,
                    increment
                ))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum StdVecVecUuidUuidWhereElement {
    Equal(StdVecVecUuidUuidWhereElementEqual),
    LengthEqual(StdVecVecUuidUuidWhereElementLengthEqual),
    LengthMoreThan(StdVecVecUuidUuidWhereElementLengthMoreThan),
    PositionEqual(StdVecVecUuidUuidWhereElementPositionEqual),
    ContainsAllElementsOfArray(StdVecVecUuidUuidWhereElementContainsAllElementsOfArray),
    OverlapsWithArray(StdVecVecUuidUuidWhereElementOverlapsWithArray),
    AllElementsEqual(StdVecVecUuidUuidWhereElementAllElementsEqual),
    PositionCaseSensitiveRegularExpression(StdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression),
    PositionCaseInsensitiveRegularExpression(StdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression),
    ContainsElementCaseSensitiveRegularExpression(StdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression),
    ContainsElementCaseInsensitiveRegularExpression(StdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression),
    AllElementsCaseSensitiveRegularExpression(StdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression),
    AllElementsCaseInsensitiveRegularExpression(StdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression),
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElement {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::LengthEqual(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::LengthMoreThan(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::PositionEqual(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::ContainsAllElementsOfArray(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::OverlapsWithArray(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsEqual(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::PositionCaseSensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::PositionCaseInsensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::ContainsElementCaseSensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::ContainsElementCaseInsensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsCaseSensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsCaseInsensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn where_query_bind(self, 
        // mut 
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::LengthEqual(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::LengthMoreThan(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::PositionEqual(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::ContainsAllElementsOfArray(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::OverlapsWithArray(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::AllElementsEqual(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::PositionCaseSensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::PositionCaseInsensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::ContainsElementCaseSensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::ContainsElementCaseInsensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::AllElementsCaseSensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
            Self::AllElementsCaseInsensitiveRegularExpression(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for StdVecVecUuidUuidWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::LengthEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::LengthMoreThan(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::PositionEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::ContainsAllElementsOfArray(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::OverlapsWithArray(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::AllElementsEqual(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::PositionCaseSensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::PositionCaseInsensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::ContainsElementCaseSensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::ContainsElementCaseInsensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::AllElementsCaseSensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::AllElementsCaseInsensitiveRegularExpression(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct StdVecVecUuidUuidWhereElementEqualSecondDimension {
    pub logical_operator: crate::LogicalOperator,
    pub value: StdVecVecUuidUuid,
}
impl crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementEqualSecondDimension {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementEqualSecondDimension {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}(exists (select 1 from jsonb_array_elements({}) as el where el = ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum StdVecVecUuidUuidWhereElementSecondDimension {
    EqualSecondDimension(StdVecVecUuidUuidWhereElementEqualSecondDimension),
}
impl<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for StdVecVecUuidUuidWhereElementSecondDimension {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match &self {
            Self::EqualSecondDimension(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn where_query_bind(self, 
        // mut 
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::EqualSecondDimension(value) => crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for StdVecVecUuidUuidWhereElementSecondDimension {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecUuidUuidWhereElementSecondDimension {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::EqualSecondDimension(crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
pub type StdVecVecUuidUuidUpdate = StdVecVecUuidUuid;
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdVecVecUuidUuidUpdateErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl crate::postgresql_json_type_trait::PostgresqlJsonType for StdVecVecUuidUuid {
    type Create<'a> = StdVecVecUuidUuidCreate;
    fn create_query_part(value: &Self::Create<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::postgresql_json_type_trait::CreateQueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("${increment}"))
            }
            None => Err(crate::postgresql_json_type_trait::CreateQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn create_query_bind<'a>(value: Self::Create<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(value.0));
        query
    }
    type Select<'a> = StdVecVecUuidUuidSelect;
    fn select_query_part(value: &Self::Select<'_>, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let start = value.pagination.start();
        let end = value.pagination.end();
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {start} and {end})))")
    }
    type WhereElement<'a> = StdVecVecUuidUuidWhereElement;
    type Read<'a> = StdVecVecUuidUuidRead;
    type Update<'a> = StdVecVecUuidUuidUpdate;
    type UpdateQueryPartErrorNamed = StdVecVecUuidUuidUpdateErrorNamed;
    fn update_query_part(value: &Self::Update<'_>, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, Self::UpdateQueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(Self::UpdateQueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn update_query_bind<'a>(value: Self::Update<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(value.0));
        query
    }
}
