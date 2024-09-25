// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum SomethingFieldToRead {
//     #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
//     StdPrimitiveI8(postgresql_crud::JsonStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdPrimitiveI8(postgresql_crud::JsonStdOptionOptionStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
//     StdVecVecStdPrimitiveI8(postgresql_crud::JsonStdVecVecStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "generic", deserialize = "generic"))]
//     Generic(GenericCatFieldReader),
//     #[serde(rename(serialize = "std_option_option_generic", deserialize = "std_option_option_generic"))]
//     StdOptionOptionGeneric(StdOptionOptionGenericCatFieldReader),
//     #[serde(rename(serialize = "std_vec_vec_generic_with_id", deserialize = "std_vec_vec_generic_with_id"))]
//     StdVecVecGenericWithId(StdVecVecGenericWithIdDoggieFieldReader),
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_generic_with_id", deserialize = "std_option_option_std_vec_vec_generic_with_id"))]
//     StdOptionOptionStdVecVecGenericWithId(StdOptionOptionStdVecVecGenericWithIdDoggieFieldReader),
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum SomethingWithIdFieldToRead {
//     #[serde(rename(serialize = "id", deserialize = "id"))]
//     Id(postgresql_crud::JsonUuidFieldReader),
//     #[serde(rename(serialize = "std_primitive_i8", deserialize = "std_primitive_i8"))]
//     StdPrimitiveI8(postgresql_crud::JsonStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_option_option_std_primitive_i8", deserialize = "std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdPrimitiveI8(postgresql_crud::JsonStdOptionOptionStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_vec_vec_std_primitive_i8", deserialize = "std_vec_vec_std_primitive_i8"))]
//     StdVecVecStdPrimitiveI8(postgresql_crud::JsonStdVecVecStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdPrimitiveI8(postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8", deserialize = "std_option_option_std_vec_vec_std_option_option_std_primitive_i8"))]
//     StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8FieldReader),
//     #[serde(rename(serialize = "generic", deserialize = "generic"))]
//     Generic(GenericCatFieldReader),
//     #[serde(rename(serialize = "std_option_option_generic", deserialize = "std_option_option_generic"))]
//     StdOptionOptionGeneric(StdOptionOptionGenericCatFieldReader),
//     #[serde(rename(serialize = "std_vec_vec_generic_with_id", deserialize = "std_vec_vec_generic_with_id"))]
//     StdVecVecGenericWithId(StdVecVecGenericWithIdDoggieFieldReader),
//     #[serde(rename(serialize = "std_option_option_std_vec_vec_generic_with_id", deserialize = "std_option_option_std_vec_vec_generic_with_id"))]
//     StdOptionOptionStdVecVecGenericWithId(StdOptionOptionStdVecVecGenericWithIdDoggieFieldReader),
// }
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
// pub struct GenericWithIdSomething {
//     pub id: postgresql_crud::JsonUuid,
//     pub std_primitive_i8: postgresql_crud::JsonStdPrimitiveI8,
//     pub std_option_option_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdPrimitiveI8,
//     pub std_vec_vec_std_primitive_i8: postgresql_crud::JsonStdVecVecStdPrimitiveI8,
//     pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8,
//     pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8,
//     pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
//     pub generic: GenericCat,
//     pub std_option_option_generic: StdOptionOptionGenericCat,
//     pub std_vec_vec_generic_with_id: StdVecVecGenericWithIdDoggie,
//     pub std_option_option_std_vec_vec_generic_with_id: StdOptionOptionStdVecVecGenericWithIdDoggie,
// }
// impl std::fmt::Display for GenericWithIdSomething {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct GenericWithIdSomethingFieldReader(std::vec::Vec<SomethingWithIdFieldToRead>);
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
// pub struct GenericSomething {
//     pub std_primitive_i8: postgresql_crud::JsonStdPrimitiveI8,
//     pub std_option_option_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdPrimitiveI8,
//     pub std_vec_vec_std_primitive_i8: postgresql_crud::JsonStdVecVecStdPrimitiveI8,
//     pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdVecVecStdPrimitiveI8,
//     pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::JsonStdVecVecStdOptionOptionStdPrimitiveI8,
//     pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::JsonStdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
//     pub generic: GenericCat,
//     pub std_option_option_generic: StdOptionOptionGenericCat,
//     pub std_vec_vec_generic_with_id: StdVecVecGenericWithIdDoggie,
//     pub std_option_option_std_vec_vec_generic_with_id: StdOptionOptionStdVecVecGenericWithIdDoggie,
// }
// impl std::fmt::Display for GenericSomething {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct GenericSomethingFieldReader(std::vec::Vec<SomethingFieldToRead>);
// impl postgresql_crud::GeneratePostgresqlQueryPartFieldToRead for GenericSomethingFieldReader {
//     fn generate_postgresql_query_part_field_to_read(&self, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
//         let mut acc = std::string::String::default();
//         for element in &self.0 {
//             match element {
//                 SomethingFieldToRead::StdPrimitiveI8(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(value, "std_primitive_i8", "{column_name_and_maybe_field_getter}->'std_primitive_i8'", "{column_name_and_maybe_field_getter_for_error_message}.std_primitive_i8")
//                     ));
//                 }
//                 SomethingFieldToRead::StdOptionOptionStdPrimitiveI8(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_option_option_std_primitive_i8",
//                             "{column_name_and_maybe_field_getter}->'std_option_option_std_primitive_i8'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_primitive_i8"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::StdVecVecStdPrimitiveI8(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_vec_vec_std_primitive_i8",
//                             "{column_name_and_maybe_field_getter}->'std_vec_vec_std_primitive_i8'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_vec_vec_std_primitive_i8"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::StdOptionOptionStdVecVecStdPrimitiveI8(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_option_option_std_vec_vec_std_primitive_i8",
//                             "{column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_primitive_i8'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_vec_vec_std_primitive_i8"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::StdVecVecStdOptionOptionStdPrimitiveI8(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_vec_vec_std_option_option_std_primitive_i8",
//                             "{column_name_and_maybe_field_getter}->'std_vec_vec_std_option_option_std_primitive_i8'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_vec_vec_std_option_option_std_primitive_i8"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
//                             "{column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_option_option_std_primitive_i8'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_vec_vec_std_option_option_std_primitive_i8"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::Generic(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(value, "generic", "{column_name_and_maybe_field_getter}->'generic'", "{column_name_and_maybe_field_getter_for_error_message}.generic")
//                     ));
//                 }
//                 SomethingFieldToRead::StdOptionOptionGeneric(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_option_option_generic",
//                             "{column_name_and_maybe_field_getter}->'std_option_option_generic'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_option_option_generic"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::StdVecVecGenericWithId(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_vec_vec_generic_with_id",
//                             "{column_name_and_maybe_field_getter}->'std_vec_vec_generic_with_id'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_vec_vec_generic_with_id"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::StdOptionOptionStdVecVecGenericWithId(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_option_option_std_vec_vec_generic_with_id",
//                             "{column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic_with_id'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_vec_vec_generic_with_id"
//                         )
//                     ));
//                 }
//             }
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         format!("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object')) end")
//     }
// }
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
// pub struct StdOptionOptionGenericSomething(pub std::option::Option<GenericSomething>);
// impl std::fmt::Display for StdOptionOptionGenericSomething {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct StdOptionOptionGenericSomethingFieldReader(std::vec::Vec<SomethingFieldToRead>);
// impl postgresql_crud::GeneratePostgresqlQueryPartFieldToRead for StdOptionOptionGenericSomethingFieldReader {
//     fn generate_postgresql_query_part_field_to_read(&self, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
//         let mut acc = std::string::String::default();
//         for element in &self.0 {
//             match element {
//                 SomethingFieldToRead::StdPrimitiveI8(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(value, "std_primitive_i8", "{column_name_and_maybe_field_getter}->'std_primitive_i8'", "{column_name_and_maybe_field_getter_for_error_message}.std_primitive_i8")
//                     ));
//                 }
//                 SomethingFieldToRead::StdOptionOptionStdPrimitiveI8(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_option_option_std_primitive_i8",
//                             "{column_name_and_maybe_field_getter}->'std_option_option_std_primitive_i8'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_primitive_i8"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::StdVecVecStdPrimitiveI8(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_vec_vec_std_primitive_i8",
//                             "{column_name_and_maybe_field_getter}->'std_vec_vec_std_primitive_i8'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_vec_vec_std_primitive_i8"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::StdOptionOptionStdVecVecStdPrimitiveI8(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_option_option_std_vec_vec_std_primitive_i8",
//                             "{column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_primitive_i8'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_vec_vec_std_primitive_i8"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::StdVecVecStdOptionOptionStdPrimitiveI8(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_vec_vec_std_option_option_std_primitive_i8",
//                             "{column_name_and_maybe_field_getter}->'std_vec_vec_std_option_option_std_primitive_i8'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_vec_vec_std_option_option_std_primitive_i8"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_option_option_std_vec_vec_std_option_option_std_primitive_i8",
//                             "{column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_std_option_option_std_primitive_i8'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_vec_vec_std_option_option_std_primitive_i8"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::Generic(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(value, "generic", "{column_name_and_maybe_field_getter}->'generic'", "{column_name_and_maybe_field_getter_for_error_message}.generic")
//                     ));
//                 }
//                 SomethingFieldToRead::StdOptionOptionGeneric(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_option_option_generic",
//                             "{column_name_and_maybe_field_getter}->'std_option_option_generic'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_option_option_generic"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::StdVecVecGenericWithId(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_vec_vec_generic_with_id",
//                             "{column_name_and_maybe_field_getter}->'std_vec_vec_generic_with_id'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_vec_vec_generic_with_id"
//                         )
//                     ));
//                 }
//                 SomethingFieldToRead::StdOptionOptionStdVecVecGenericWithId(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(
//                             value,
//                             "std_option_option_std_vec_vec_generic_with_id",
//                             "{column_name_and_maybe_field_getter}->'std_option_option_std_vec_vec_generic_with_id'",
//                             "{column_name_and_maybe_field_getter_for_error_message}.std_option_option_std_vec_vec_generic_with_id"
//                         )
//                     ));
//                 }
//             }
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         format!
//         ("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}) when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object and not null')) end")
//     }
// }
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
// pub struct StdVecVecGenericWithIdSomething(pub std::vec::Vec<GenericWithIdSomething>);
// impl std::fmt::Display for StdVecVecGenericWithIdSomething {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct StdVecVecGenericWithIdSomethingFieldReader {
//     field_vec: std::vec::Vec<SomethingWithIdFieldToRead>,
//     pagination: postgresql_crud::Pagination,
// }
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
// pub struct StdOptionOptionStdVecVecGenericWithIdSomething(std::option::Option<std::vec::Vec<GenericWithIdSomething>>);
// impl std::fmt::Display for StdOptionOptionStdVecVecGenericWithIdSomething {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct StdOptionOptionStdVecVecGenericWithIdSomethingFieldReader {
//     field_vec: std::vec::Vec<SomethingWithIdFieldToRead>,
//     pagination: postgresql_crud::Pagination,
// }
// ///////////////////////
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum DoggieFieldToRead {
//     #[serde(rename(serialize = "std_primitive_i16", deserialize = "std_primitive_i16"))]
//     StdPrimitiveI16(postgresql_crud::JsonStdPrimitiveI16FieldReader),
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum DoggieWithIdFieldToRead {
//     #[serde(rename(serialize = "id", deserialize = "id"))]
//     Id(postgresql_crud::JsonUuidFieldReader),
//     #[serde(rename(serialize = "std_primitive_i16", deserialize = "std_primitive_i16"))]
//     StdPrimitiveI16(postgresql_crud::JsonStdPrimitiveI16FieldReader),
// }
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
// pub struct GenericWithIdDoggie {
//     pub id: postgresql_crud::JsonUuid,
//     pub std_primitive_i16: postgresql_crud::JsonStdPrimitiveI16,
// }
// impl std::fmt::Display for GenericWithIdDoggie {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct GenericWithIdDoggieFieldReader(std::vec::Vec<DoggieWithIdFieldToRead>);
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
// pub struct GenericDoggie {
//     pub std_primitive_i16: postgresql_crud::JsonStdPrimitiveI16,
// }
// impl std::fmt::Display for GenericDoggie {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct GenericDoggieFieldReader(std::vec::Vec<DoggieFieldToRead>);
// impl postgresql_crud::GeneratePostgresqlQueryPartFieldToRead for GenericDoggieFieldReader {
//     fn generate_postgresql_query_part_field_to_read(&self, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
//         let mut acc = std::string::String::default();
//         for element in &self.0 {
//             match element {
//                 DoggieFieldToRead::StdPrimitiveI16(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(value, "std_primitive_i16", "{column_name_and_maybe_field_getter}->'std_primitive_i16'", "{column_name_and_maybe_field_getter_for_error_message}.std_primitive_i16")
//                     ));
//                 }
//             }
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         format!("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object')) end")
//     }
// }
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
// pub struct StdOptionOptionGenericDoggie(pub std::option::Option<GenericDoggie>);
// impl std::fmt::Display for StdOptionOptionGenericDoggie {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct StdOptionOptionGenericDoggieFieldReader(std::vec::Vec<DoggieFieldToRead>);
// impl postgresql_crud::GeneratePostgresqlQueryPartFieldToRead for StdOptionOptionGenericDoggieFieldReader {
//     fn generate_postgresql_query_part_field_to_read(&self, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
//         let mut acc = std::string::String::default();
//         for element in &self.0 {
//             match element {
//                 DoggieFieldToRead::StdPrimitiveI16(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(value, "std_primitive_i16", "{column_name_and_maybe_field_getter}->'std_primitive_i16'", "{column_name_and_maybe_field_getter_for_error_message}.std_primitive_i16")
//                     ));
//                 }
//             }
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         format!
//         ("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}) when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object and not null')) end")
//     }
// }
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
// pub struct StdVecVecGenericWithIdDoggie(pub std::vec::Vec<GenericWithIdDoggie>);
// impl std::fmt::Display for StdVecVecGenericWithIdDoggie {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct StdVecVecGenericWithIdDoggieFieldReader {
//     field_vec: std::vec::Vec<DoggieWithIdFieldToRead>,
//     pagination: postgresql_crud::Pagination,
// }
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
// pub struct StdOptionOptionStdVecVecGenericWithIdDoggie(std::option::Option<std::vec::Vec<GenericWithIdDoggie>>);
// impl std::fmt::Display for StdOptionOptionStdVecVecGenericWithIdDoggie {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct StdOptionOptionStdVecVecGenericWithIdDoggieFieldReader {
//     field_vec: std::vec::Vec<DoggieWithIdFieldToRead>,
//     pagination: postgresql_crud::Pagination,
// }
// /////////////////////
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum CatFieldToRead {
//     #[serde(rename(serialize = "std_primitive_i32", deserialize = "std_primitive_i32"))]
//     StdPrimitiveI32(postgresql_crud::JsonStdPrimitiveI32FieldReader),
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub enum CatWithIdFieldToRead {
//     #[serde(rename(serialize = "id", deserialize = "id"))]
//     Id(postgresql_crud::JsonUuidFieldReader),
//     #[serde(rename(serialize = "std_primitive_i32", deserialize = "std_primitive_i32"))]
//     StdPrimitiveI32(postgresql_crud::JsonStdPrimitiveI32FieldReader),
// }
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
// pub struct GenericWithIdCat {
//     pub id: postgresql_crud::JsonUuid,
//     pub std_primitive_i32: postgresql_crud::JsonStdPrimitiveI32,
// }
// impl std::fmt::Display for GenericWithIdCat {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct GenericWithIdCatFieldReader(std::vec::Vec<CatWithIdFieldToRead>);
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
// pub struct GenericCat {
//     pub std_primitive_i32: postgresql_crud::JsonStdPrimitiveI32,
// }
// impl std::fmt::Display for GenericCat {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct GenericCatFieldReader(std::vec::Vec<CatFieldToRead>);
// impl postgresql_crud::GeneratePostgresqlQueryPartFieldToRead for GenericCatFieldReader {
//     fn generate_postgresql_query_part_field_to_read(&self, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
//         let mut acc = std::string::String::default();
//         for element in &self.0 {
//             match element {
//                 CatFieldToRead::StdPrimitiveI32(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(value, "std_primitive_i32", "{column_name_and_maybe_field_getter}->'std_primitive_i32'", "{column_name_and_maybe_field_getter_for_error_message}.std_primitive_i32")
//                     ));
//                 }
//             }
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         format!("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object')) end")
//     }
// }
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
// pub struct StdOptionOptionGenericCat(pub std::option::Option<GenericCat>);
// impl std::fmt::Display for StdOptionOptionGenericCat {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct StdOptionOptionGenericCatFieldReader(std::vec::Vec<CatFieldToRead>);
// impl postgresql_crud::GeneratePostgresqlQueryPartFieldToRead for StdOptionOptionGenericCatFieldReader {
//     fn generate_postgresql_query_part_field_to_read(&self, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str) -> std::string::String {
//         let mut acc = std::string::String::default();
//         for element in &self.0 {
//             match element {
//                 CatFieldToRead::StdPrimitiveI32(value) => {
//                     acc.push_str(&format!(
//                         "jsonb_build_object('{field_ident}',{})||",
//                         postgresql_crud::GeneratePostgresqlQueryPartFieldToRead::generate_postgresql_query_part_field_to_read(value, "std_primitive_i32", "{column_name_and_maybe_field_getter}->'std_primitive_i32'", "{column_name_and_maybe_field_getter_for_error_message}.std_primitive_i32")
//                     ));
//                 }
//             }
//         }
//         let _ = acc.pop();
//         let _ = acc.pop();
//         format!
//         ("case when jsonb_typeof({column_name_and_maybe_field_getter}) = 'object' then jsonb_build_object('Ok',{acc}) when jsonb_typeof({column_name_and_maybe_field_getter}) = 'null' then jsonb_build_object('Ok',null) else jsonb_build_object(jsonb_build_object('Err','type of {column_name_and_maybe_field_getter_for_error_message} is not object and not null')) end")
//     }
// }
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
// pub struct StdVecVecGenericWithIdCat(pub std::vec::Vec<GenericWithIdCat>);
// impl std::fmt::Display for StdVecVecGenericWithIdCat {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct StdVecVecGenericWithIdCatFieldReader {
//     field_vec: std::vec::Vec<CatWithIdFieldToRead>,
//     pagination: postgresql_crud::Pagination,
// }
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
// pub struct StdOptionOptionStdVecVecGenericWithIdCat(std::option::Option<std::vec::Vec<GenericWithIdCat>>);
// impl std::fmt::Display for StdOptionOptionStdVecVecGenericWithIdCat {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", &self)
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
// pub struct StdOptionOptionStdVecVecGenericWithIdCatFieldReader {
//     field_vec: std::vec::Vec<CatWithIdFieldToRead>,
//     pagination: postgresql_crud::Pagination,
// }
