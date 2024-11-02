//todo maybe some derive impl does not need?
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    // postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonType,
)]
pub struct JsonStdPrimitiveI8(#[validate(range(min = -128i8, max = 127i8))] pub std::primitive::i8);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonType,
)]
pub struct JsonStdPrimitiveI16(#[validate(range(min = -32_768i16, max = 32_767i16))] pub std::primitive::i16);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonType,
// )]
// pub struct JsonStdPrimitiveI32(#[validate(range(min = -2_147_483_648i32, max = 2_147_483_647i32))] pub std::primitive::i32);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonType,
// )]
// pub struct JsonStdPrimitiveI64(#[validate(range(min = -9_223_372_036_854_775_808i64, max = 9_223_372_036_854_775_807i64))] pub std::primitive::i64);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonType,
// )]
// pub struct JsonStdPrimitiveU8(#[validate(range(min = 0u8, max = 255u8))] pub std::primitive::u8);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonType,
// )]
// pub struct JsonStdPrimitiveU16(#[validate(range(min = 0u16, max = 65_535u16))] pub std::primitive::u16);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonType,
// )]
// pub struct JsonStdPrimitiveU32(#[validate(range(min = 0u32, max = 4_294_967_295u32))] pub std::primitive::u32);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonType,
// )]
// pub struct JsonStdPrimitiveU64(#[validate(range(min = 0u64, max = 18_446_744_073_709_551_615u64))] pub std::primitive::u64);
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonType,
// )]
// pub struct JsonStdPrimitiveF32(
//     #[validate(range(min = -3.40282347E+38f32, max = 3.40282347E+38f32))] //todo maybe its not correct. https://doc.rust-lang.org/std/primitive.f32.html
//     pub std::primitive::f32,
// );
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonType,
// )]
// pub struct JsonStdPrimitiveF64(
//     #[validate(range(min = -1.7976931348623157E+308f64, max = 1.7976931348623157E+308f64))] //todo maybe its not correct. https://doc.rust-lang.org/core/primitive.f64.html
//     pub std::primitive::f64,
// );
// #[derive(
//     Debug,
//     Clone,
//     Copy,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonType,
// )]
// pub struct JsonStdPrimitiveBool(pub std::primitive::bool);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud_types_macro_logic_reuse::GenerateImplPostgresqlJsonType,
// )]
// pub struct JsonStdStringString(pub std::string::String);



// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde :: Serialize,
//     serde ::
// Deserialize,
//     utoipa :: ToSchema,
//     schemars :: JsonSchema,
// )]
// pub struct JsonStdPrimitiveI8ToCreate(pub std::primitive::i8);
// impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for JsonStdPrimitiveI8ToCreate {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(::core::default::Default::default())
//     }
// }
pub type JsonStdPrimitiveI8ToCreate = JsonStdPrimitiveI8;
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for JsonStdPrimitiveI8 {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}

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
pub struct JsonStdPrimitiveI8FieldReader {}
impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for JsonStdPrimitiveI8FieldReader {
    #[inline]
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde :: Serialize,
//     serde ::
// Deserialize,
//     utoipa :: ToSchema,
//     schemars :: JsonSchema,
// )]
// pub struct JsonStdPrimitiveI8OptionsToRead(pub std::primitive::i8);
// impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for JsonStdPrimitiveI8OptionsToRead {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(::core::default::Default::default())
//     }
// }
pub type JsonStdPrimitiveI8OptionsToRead = JsonStdPrimitiveI8;


// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde :: Serialize,
//     serde ::
// Deserialize,
//     utoipa :: ToSchema,
//     schemars :: JsonSchema,
// )]
// pub struct JsonStdPrimitiveI8OptionToUpdate(pub std::primitive::i8);
// impl crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for JsonStdPrimitiveI8OptionToUpdate {
//     #[inline]
//     fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(::core::default::Default::default())
//     }
// }

pub type JsonStdPrimitiveI8OptionToUpdate = JsonStdPrimitiveI8;


#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum JsonStdPrimitiveI8OptionToUpdateTryGenerateBindIncrementsErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl crate::generate_postgresql_query_part::PostgresqlJsonType for JsonStdPrimitiveI8 {
    type ToCreate<'a> = JsonStdPrimitiveI8ToCreate;
    fn json_create_try_generate_bind_increments(self_to_create: &Self::ToCreate<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::generate_postgresql_query_part::JsonCreateTryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                Ok(format!("${increment}"))
            }
            None => Err(crate::generate_postgresql_query_part::JsonCreateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn json_create_bind_value_to_query<'a>(self_to_create: Self::ToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self_to_create.0));
        query
    }
    type FieldReader<'a> = JsonStdPrimitiveI8FieldReader;
    type OptionsToRead<'a> = JsonStdPrimitiveI8OptionsToRead;
    fn generate_postgresql_query_part_field_to_read(options_to_read: &Self::OptionsToRead<'_>, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
        format!("jsonb_build_object('{field_ident}', jsonb_build_object('value', {column_name_and_maybe_field_getter}->'{field_ident}'))")
    }
    type OptionToUpdate<'a> = JsonStdPrimitiveI8OptionToUpdate;
    type OptionToUpdateTryGenerateBindIncrementsErrorNamed = JsonStdPrimitiveI8OptionToUpdateTryGenerateBindIncrementsErrorNamed;
    fn try_generate_bind_increments(
        option_to_update: &Self::OptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::OptionToUpdateTryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(Self::OptionToUpdateTryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn bind_value_to_query<'a>(option_to_update: Self::OptionToUpdate<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(option_to_update.0));
        query
    }
}
