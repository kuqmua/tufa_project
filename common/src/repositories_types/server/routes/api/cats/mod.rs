#[derive(Debug, postgresql_crud::GeneratePostgresqlCrud)]
#[postgresql_crud::create_many_additional_error_variants{enum CreateManyAdditionalErrorVariants{}}]
#[postgresql_crud::create_one_additional_error_variants{enum CreateOneAdditionalErrorVariants{}}]
#[postgresql_crud::read_many_additional_error_variants{enum ReadManyAdditionalErrorVariants{}}]
#[postgresql_crud::read_one_additional_error_variants{enum ReadOneAdditionalErrorVariants{}}]
#[postgresql_crud::update_many_additional_error_variants{enum UpdateManyAdditionalErrorVariants{}}]
#[postgresql_crud::update_one_additional_error_variants{enum UpdateOneAdditionalErrorVariants{}}]
#[postgresql_crud::delete_many_additional_error_variants{enum DeleteManyAdditionalErrorVariants{}}]
#[postgresql_crud::delete_one_additional_error_variants{enum DeleteOneAdditionalErrorVariants{}}]
#[postgresql_crud::common_additional_error_variants{
    enum CommonAdditionalErrorVariants {
        CheckCommit {
            #[eo_error_occurence]
            check_commit: route_validators::check_commit::CheckCommitErrorNamed,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
    }
}]
#[postgresql_crud::create_many_additional_route_logic{
    println!("kekw");
}]
#[postgresql_crud::create_one_additional_route_logic{}]
#[postgresql_crud::read_many_additional_route_logic{}]
#[postgresql_crud::read_one_additional_route_logic{}]
#[postgresql_crud::update_many_additional_route_logic{}]
#[postgresql_crud::update_one_additional_route_logic{}]
#[postgresql_crud::delete_many_additional_route_logic{}]
#[postgresql_crud::delete_one_additional_route_logic{}]
#[postgresql_crud::common_additional_route_logic{
    if let Err(error) = route_validators::check_commit::check_commit(
        *app_state.get_enable_api_git_commit_check(),
        &headers,
    ) {
        let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
        //todo use reserved work instead of TryCreateManyRouteLogicErrorNamed
        let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
            check_commit: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        };
        eprintln!("{error}");
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TryCreateManyRouteLogicResponseVariants::from(error),
        ));
        *response.status_mut() = status_code;
        return response;
    }
}]
pub struct Dog {
    pub std_primitive_bool_as_postgresql_bool_kekw: postgresql_crud::StdPrimitiveBoolAsPostgresqlBool,
    // pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdPrimitiveBoolAsPostgresqlBoolNotNull,

    pub std_primitive_i16_as_postgresql_small_int_kekw: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt,
    // pub std_primitive_i16_as_postgresql_small_int_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallIntNotNull,
    // pub std_primitive_i16_as_postgresql_small_serial: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerial,
    // pub std_primitive_i16_as_postgresql_small_serial_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerialNotNull,
    // pub std_primitive_i16_as_postgresql_small_int2: postgresql_crud::StdPrimitiveI16AsPostgresqlInt2,
    // pub std_primitive_i16_as_postgresql_small_int2_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlInt2NotNull,

    pub std_primitive_i32_as_postgresql_int_kekw: postgresql_crud::StdPrimitiveI32AsPostgresqlInt,
    // pub std_primitive_i32_as_postgresql_int_not_null: postgresql_crud::StdPrimitiveI32AsPostgresqlIntNotNull,
    // pub std_primitive_i32_as_postgresql_serial: postgresql_crud::StdPrimitiveI32AsPostgresqlSerial,
    // pub std_primitive_i32_as_postgresql_serial_not_null: postgresql_crud::StdPrimitiveI32AsPostgresqlSerialNotNull,
    // pub std_primitive_i32_as_postgresql_int4: postgresql_crud::StdPrimitiveI32AsPostgresqlInt4,
    // pub std_primitive_i32_as_postgresql_int4_not_null: postgresql_crud::StdPrimitiveI32AsPostgresqlInt4NotNull,

    // pub std_primitive_i64_as_postgresql_big_int: postgresql_crud::StdPrimitiveI64AsPostgresqlBigInt,
    // pub std_primitive_i64_as_postgresql_big_int_not_null: postgresql_crud::StdPrimitiveI64AsPostgresqlBigIntNotNull,
    // pub std_primitive_i64_as_postgresql_big_serial: postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerial,
    // pub std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
    // pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
    // pub std_primitive_i64_as_postgresql_big_int8: postgresql_crud::StdPrimitiveI64AsPostgresqlInt8,
    // pub std_primitive_i64_as_postgresql_big_int8_not_null: postgresql_crud::StdPrimitiveI64AsPostgresqlInt8NotNull,

    // pub std_primitive_f32_as_postgresql_real: postgresql_crud::StdPrimitiveF32AsPostgresqlReal,
    // pub std_primitive_f32_as_postgresql_real_not_null: postgresql_crud::StdPrimitiveF32AsPostgresqlRealNotNull,
    // pub std_primitive_f32_as_postgresql_float4: postgresql_crud::StdPrimitiveF32AsPostgresqlFloat4,
    // pub std_primitive_f32_as_postgresql_float4_not_null: postgresql_crud::StdPrimitiveF32AsPostgresqlFloat4NotNull,

    // pub std_primitive_f64_as_postgresql_double_precision: postgresql_crud::StdPrimitiveF64AsPostgresqlDoublePrecision,
    // pub std_primitive_f64_as_postgresql_double_precision_not_null: postgresql_crud::StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull,
    // pub std_primitive_f64_as_postgresql_float8: postgresql_crud::StdPrimitiveF64AsPostgresqlFloat8,
    // pub std_primitive_f64_as_postgresql_float8_not_null: postgresql_crud::StdPrimitiveF64AsPostgresqlFloat8NotNull,

    // pub std_string_string_as_postgresql_varchar: postgresql_crud::StdStringStringAsPostgresqlVarchar,
    // pub std_string_string_as_postgresql_varchar_not_null: postgresql_crud::StdStringStringAsPostgresqlVarcharNotNull,
    // pub std_string_string_as_postgresql_char_n: postgresql_crud::StdStringStringAsPostgresqlCharN,
    // pub std_string_string_as_postgresql_char_n_not_null: postgresql_crud::StdStringStringAsPostgresqlCharNNotNull,
    // pub std_string_string_as_postgresql_text: postgresql_crud::StdStringStringAsPostgresqlText,
    // pub std_string_string_as_postgresql_text_not_null: postgresql_crud::StdStringStringAsPostgresqlTextNotNull,
    // pub std_string_string_as_postgresql_ci_text: postgresql_crud::StdStringStringAsPostgresqlCiText,
    // pub std_string_string_as_postgresql_ci_text_not_null: postgresql_crud::StdStringStringAsPostgresqlCiTextNotNull,

    // pub std_vec_vec_std_primitive_u8_as_postgresql_bytea: postgresql_crud::StdVecVecStdPrimitiveU8AsPostgresqlBytea,
    // pub std_vec_vec_std_primitive_u8_as_postgresql_bytea_not_null: postgresql_crud::StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull,

    // pub sqlx_postgres_types_pg_interval_as_postgresql_interval: postgresql_crud::SqlxPostgresTypesPgIntervalAsPostgresqlInterval,
    // pub sqlx_postgres_types_pg_interval_as_postgresql_interval_not_null: postgresql_crud::SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull,

    // pub sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range,
    // pub sqlx_postgres_types_pg_range_std_primitive_i64_as_postgresql_int8_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull,

    // pub sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range,
    // pub sqlx_postgres_types_pg_range_std_primitive_i32_as_postgresql_int4_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_ts_tz_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_ts_tz_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_offset_date_time_as_postgresql_ts_tz_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_time_as_postgresql_ts_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_primitive_date_time_as_postgresql_ts_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_chrono_naive_date_as_postgresql_date_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_time_date_as_postgresql_date_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_big_decimal_as_postgresql_num_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull,

    // pub sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange,
    // pub sqlx_postgres_types_pg_range_sqlx_types_decimal_as_postgresql_num_range_not_null: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull,

    // pub sqlx_postgres_types_pg_money_as_postgresql_money: postgresql_crud::SqlxPostgresTypesPgMoneyAsPostgresqlMoney,
    // pub sqlx_postgres_types_pg_money_as_postgresql_money_not_null: postgresql_crud::SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull,

    // pub sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text: postgresql_crud::SqlxPostgresTypesPgCiTextAsPostgresqlCiText,
    // pub sqlx_postgres_types_pg_ci_text_as_postgresql_ci_text_not_null: postgresql_crud::SqlxPostgresTypesPgCiTextAsPostgresqlCiTextNotNull,

    // pub sqlx_types_big_decimal_as_postgresql_numeric: postgresql_crud::SqlxTypesBigDecimalAsPostgresqlNumeric,
    // pub sqlx_types_big_decimal_as_postgresql_numeric_not_null: postgresql_crud::SqlxTypesBigDecimalAsPostgresqlNumericNotNull,

    // pub sqlx_types_decimal_as_postgresql_numeric: postgresql_crud::SqlxTypesDecimalAsPostgresqlNumeric,
    // pub sqlx_types_decimal_as_postgresql_numeric_not_null: postgresql_crud::SqlxTypesDecimalAsPostgresqlNumericNotNull,

    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz: postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz,
    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_postgresql_timestamp_tz_not_null: postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz: postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz,
    // pub sqlx_types_chrono_date_time_sqlx_types_chrono_local_as_postgresql_timestamp_tz_not_null: postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_chrono_naive_date_time_as_postgresql_timestamp: postgresql_crud::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp,
    // pub sqlx_types_chrono_naive_date_time_as_postgresql_timestamp_not_null: postgresql_crud::SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull,

    // pub sqlx_types_chrono_naive_date_as_postgresql_date: postgresql_crud::SqlxTypesChronoNaiveDateAsPostgresqlDate,
    // pub sqlx_types_chrono_naive_date_as_postgresql_date_not_null: postgresql_crud::SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull,

    // pub sqlx_types_chrono_naive_time_as_postgresql_time: postgresql_crud::SqlxTypesChronoNaiveTimeAsPostgresqlTime,
    // pub sqlx_types_chrono_naive_time_as_postgresql_time_not_null: postgresql_crud::SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull,

    // pub sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz: postgresql_crud::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz,
    // pub sqlx_postgres_types_pg_time_tz_as_postgresql_time_tz_not_null: postgresql_crud::SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull,

    // pub sqlx_types_time_primitive_date_time_as_postgresql_timestamp: postgresql_crud::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp,
    // pub sqlx_types_time_primitive_date_time_as_postgresql_timestamp_not_null: postgresql_crud::SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull,

    // pub sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz: postgresql_crud::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz,
    // pub sqlx_types_time_offset_date_time_as_postgresql_timestamp_tz_not_null: postgresql_crud::SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull,

    // pub sqlx_types_time_date_as_postgresql_date: postgresql_crud::SqlxTypesTimeDateAsPostgresqlDate,
    // pub sqlx_types_time_date_as_postgresql_date_not_null: postgresql_crud::SqlxTypesTimeDateAsPostgresqlDateNotNull,

    // pub sqlx_types_time_time_as_postgresql_time: postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTime,
    // pub sqlx_types_time_time_as_postgresql_time_not_null: postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTimeNotNull,

    // pub sqlx_types_uuid_uuid_as_postgresql_uuid: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuid,
    // pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNull,
    pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,//todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough //fails too but primary key is a different logic. need refactor it as different task 

    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_inet: postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlInet,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_inet_not_null: postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlInetNotNull,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_cidr: postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr,
    // pub sqlx_types_ipnetwork_ip_network_as_postgresql_cidr_not_null: postgresql_crud::SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull,

    // pub std_net_ip_addr_as_postgresql_inet: postgresql_crud::StdNetIpAddrAsPostgresqlInet,
    // pub std_net_ip_addr_as_postgresql_inet_not_null: postgresql_crud::StdNetIpAddrAsPostgresqlInetNotNull,
    // pub std_net_ip_addr_as_postgresql_cidr: postgresql_crud::StdNetIpAddrAsPostgresqlCidr,
    // pub std_net_ip_addr_as_postgresql_cidr_not_null: postgresql_crud::StdNetIpAddrAsPostgresqlCidrNotNull,

    // pub sqlx_types_mac_address_mac_address_as_postgresql_mac_addr: postgresql_crud::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr,
    // pub sqlx_types_mac_address_mac_address_as_postgresql_mac_addr_not_null: postgresql_crud::SqlxTypesMacAddressMacAddressAsPostgresqlMacAddrNotNull,

    // pub sqlx_types_bit_vec_as_postgresql_bit: postgresql_crud::SqlxTypesBitVecAsPostgresqlBit,
    // pub sqlx_types_bit_vec_as_postgresql_bit_not_null: postgresql_crud::SqlxTypesBitVecAsPostgresqlBitNotNull,
    // pub sqlx_types_bit_vec_as_postgresql_var_bit: postgresql_crud::SqlxTypesBitVecAsPostgresqlVarBit,
    // pub sqlx_types_bit_vec_as_postgresql_var_bit_not_null: postgresql_crud::SqlxTypesBitVecAsPostgresqlVarBitNotNull,

    //todo what to do with generic?
    // pub sqlx_types_json_t_as_postgresql_json: postgresql_crud::SqlxTypesJsonTAsPostgresqlJson::<Something>,//todo
    // pub sqlx_types_json_t_as_postgresql_json_not_null: postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonNotNull::<Something>,//todo
    // pub sqlx_types_json_t_as_postgresql_json_b: postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonB::<Something>,//todo
    // pub sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::SqlxTypesJsonTAsPostgresqlJsonBNotNull::<Something>,//todo

    // pub serde_json_value_as_postgresql_json: postgresql_crud::SerdeJsonValueAsPostgresqlJson,
    // pub serde_json_value_as_postgresql_json_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonNotNull,
    // pub serde_json_value_as_postgresql_json_b: postgresql_crud::SerdeJsonValueAsPostgresqlJsonB,
    // pub serde_json_value_as_postgresql_json_b_not_null: postgresql_crud::SerdeJsonValueAsPostgresqlJsonBNotNull,
}

// #[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
// pub struct Something {
//     something: std::string::String,
// }
///////////////////////
pub const TABLE_NAME: &std::primitive::str = "dogs";
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct DogOptions {
    pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw: std::option::Option<
        postgresql_crud::Value<postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize>,
    >,
    pub std_primitive_bool_as_postgresql_bool_kekw: std::option::Option<
        postgresql_crud::Value<
            postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
    >,
    pub std_primitive_i16_as_postgresql_small_int_kekw: std::option::Option<
        postgresql_crud::Value<
            postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
        >,
    >,
    pub std_primitive_i32_as_postgresql_int_kekw: std::option::Option<
        postgresql_crud::Value<
            postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
        >,
    >,
}
impl std::convert::From<Dog> for DogOptions {
    fn from(value: Dog) -> Self {
        Self {
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw: Some(
                postgresql_crud::Value {
                    value: postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                        value
                            .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
                            .0,
                    ),
                },
            ),
            std_primitive_bool_as_postgresql_bool_kekw: Some(postgresql_crud::Value {
                value:
                    postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(
                        value.std_primitive_bool_as_postgresql_bool_kekw.0,
                    ),
            }),
            std_primitive_i16_as_postgresql_small_int_kekw: Some(postgresql_crud::Value {
                value:
                    postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(
                        value.std_primitive_i16_as_postgresql_small_int_kekw.0,
                    ),
            }),
            std_primitive_i32_as_postgresql_int_kekw: Some(postgresql_crud::Value {
                value:
                    postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(
                        value.std_primitive_i32_as_postgresql_int_kekw.0,
                    ),
            }),
        }
    }
}
#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    enum_extension_lib
:: EnumExtension,
    postgresql_crud :: EnumIter,
    PartialEq,
    Eq,
    from_str ::
FromStr,
    Clone,
    Copy,
)]
pub enum DogColumn {
    #[serde(rename(
        serialize = "std_primitive_bool_as_postgresql_bool_kekw",
        deserialize = "std_primitive_bool_as_postgresql_bool_kekw"
    ))]
    StdPrimitiveBoolAsPostgresqlBoolKekw,
    #[serde(rename(
        serialize = "std_primitive_i16_as_postgresql_small_int_kekw",
        deserialize = "std_primitive_i16_as_postgresql_small_int_kekw"
    ))]
    StdPrimitiveI16AsPostgresqlSmallIntKekw,
    #[serde(rename(
        serialize = "std_primitive_i32_as_postgresql_int_kekw",
        deserialize = "std_primitive_i32_as_postgresql_int_kekw"
    ))]
    StdPrimitiveI32AsPostgresqlIntKekw,
    #[serde(rename(
        serialize = "sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw",
        deserialize = "sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw"
    ))]
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw,
}
impl std::fmt::Display for DogColumn {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StdPrimitiveBoolAsPostgresqlBoolKekw => {
                write!(formatter, "std_primitive_bool_as_postgresql_bool_kekw")
            }
            Self::StdPrimitiveI16AsPostgresqlSmallIntKekw => {
                write!(formatter, "std_primitive_i16_as_postgresql_small_int_kekw")
            }
            Self::StdPrimitiveI32AsPostgresqlIntKekw => {
                write!(formatter, "std_primitive_i32_as_postgresql_int_kekw")
            }
            Self::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw => write!(
                formatter,
                "sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw"
            ),
        }
    }
}
impl error_occurence_lib::ToStdStringString for DogColumn {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
fn generate_query_vec_column(value: &[DogColumn]) -> std::string::String {
    let mut value = value
        .iter()
        .fold(std::string::String::from(""), |mut acc, element| {
            acc += match element {
                DogColumn::StdPrimitiveBoolAsPostgresqlBoolKekw => {
                    "std_primitive_bool_as_postgresql_bool_kekw"
                }
                DogColumn::StdPrimitiveI16AsPostgresqlSmallIntKekw => {
                    "std_primitive_i16_as_postgresql_small_int_kekw"
                }
                DogColumn::StdPrimitiveI32AsPostgresqlIntKekw => {
                    "std_primitive_i32_as_postgresql_int_kekw"
                }
                DogColumn::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw => {
                    "sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw"
                }
            };
            acc += ",";
            acc
        });
    let _ = value.pop();
    value
}
#[derive(Debug)]
struct WrapperVecColumn(std::vec::Vec<DogColumn>);
impl WrapperVecColumn {
    fn options_try_from_sqlx_row<'a, R: sqlx::Row>(&self, row: &'a R) -> sqlx::Result<DogOptions>
    where
        &'a std::primitive::str: sqlx::ColumnIndex<R>,
        std::option::Option<sqlx::types::uuid::Uuid>: sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<sqlx::types::uuid::Uuid>: sqlx::types::Type<R::Database>,
        std::option::Option<std::option::Option<std::primitive::bool>>:
            sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<std::option::Option<std::primitive::bool>>:
            sqlx::types::Type<R::Database>,
        std::option::Option<std::option::Option<std::primitive::i16>>:
            sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<std::option::Option<std::primitive::i16>>:
            sqlx::types::Type<R::Database>,
        std::option::Option<std::option::Option<std::primitive::i32>>:
            sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<std::option::Option<std::primitive::i32>>:
            sqlx::types::Type<R::Database>,
    {
        let mut
        sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw :
        std :: option :: Option < postgresql_crud :: Value <
        postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize > > = None;
        let mut std_primitive_bool_as_postgresql_bool_kekw: std::option::Option<
            postgresql_crud::Value<
                postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
            >,
        > = None;
        let mut std_primitive_i16_as_postgresql_small_int_kekw: std::option::Option<
            postgresql_crud::Value<
                postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
            >,
        > = None;
        let mut std_primitive_i32_as_postgresql_int_kekw: std::option::Option<
            postgresql_crud::Value<
                postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
            >,
        > = None;
        for element in &self.0 {
            match element {
                DogColumn::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw => {
                    sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw = {
                        let value: std::option::Option<sqlx::types::uuid::Uuid> = row.try_get(
                            "sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw",
                        )?;
                        value.map(|value| postgresql_crud::Value {
                            value: postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                                postgresql_crud::SqlxTypesUuidUuid(value),
                            ),
                        })
                    };
                }
                DogColumn::StdPrimitiveBoolAsPostgresqlBoolKekw => {
                    std_primitive_bool_as_postgresql_bool_kekw = {
                        let value: std::option::Option<std::option::Option<std::primitive::bool>> =
                            row.try_get("std_primitive_bool_as_postgresql_bool_kekw")?;
                        value.map(|value| {
                            postgresql_crud :: Value
                        {
                            value :
                            postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
                            ::
                            from(postgresql_crud::StdOptionOptionStdPrimitiveBool(value))
                        }
                        })
                    };
                }
                DogColumn::StdPrimitiveI16AsPostgresqlSmallIntKekw => {
                    std_primitive_i16_as_postgresql_small_int_kekw = {
                        let value: std::option::Option<std::option::Option<std::primitive::i16>> =
                            row.try_get("std_primitive_i16_as_postgresql_small_int_kekw")?;
                        value.map(|value| {
                            postgresql_crud :: Value
                        {
                            value :
                            postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize
                            ::
                            from(postgresql_crud::StdOptionOptionStdPrimitiveI16(value))
                        }
                        })
                    };
                }
                DogColumn::StdPrimitiveI32AsPostgresqlIntKekw => {
                    std_primitive_i32_as_postgresql_int_kekw = {
                        let value: std::option::Option<std::option::Option<std::primitive::i32>> =
                            row.try_get("std_primitive_i32_as_postgresql_int_kekw")?;
                        value.map(|value| {
                            postgresql_crud :: Value
                        {
                            value :
                            postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize
                            ::
                            from(postgresql_crud::StdOptionOptionStdPrimitiveI32(value))
                        }
                        })
                    };
                }
            }
        }
        Ok(DogOptions {
            std_primitive_bool_as_postgresql_bool_kekw,
            std_primitive_i16_as_postgresql_small_int_kekw,
            std_primitive_i32_as_postgresql_int_kekw,
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
        })
    }
}
fn primary_key_try_from_sqlx_row<'a, R: sqlx::Row>(
    row: &'a R,
) -> sqlx::Result<postgresql_crud::SqlxTypesUuidUuid>
where
    &'a std::primitive::str: sqlx::ColumnIndex<R>,
    sqlx::types::uuid::Uuid: sqlx::decode::Decode<'a, R::Database>,
    sqlx::types::uuid::Uuid: sqlx::types::Type<R::Database>,
{
    let primary_key: sqlx::types::uuid::Uuid =
        row.try_get("sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw")?;
    Ok(postgresql_crud::SqlxTypesUuidUuid(primary_key))
}
pub const ALLOW_METHODS: [http::Method; 4] = [
    http::Method::GET,
    http::Method::POST,
    http::Method::PATCH,
    http::Method::DELETE,
];
#[derive(Debug, Clone, Copy)]
pub struct DogColumnReadPermission {
    std_primitive_bool_as_postgresql_bool_kekw: std::primitive::bool,
    std_primitive_i16_as_postgresql_small_int_kekw: std::primitive::bool,
    std_primitive_i32_as_postgresql_int_kekw: std::primitive::bool,
    sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw: std::primitive::bool,
}
pub use postgresql_crud::SqlxTypesUuidUuid;
pub use postgresql_crud::StdOptionOptionStdPrimitiveBool;
pub use postgresql_crud::StdOptionOptionStdPrimitiveI16;
pub use postgresql_crud::StdOptionOptionStdPrimitiveI32;
#[derive(Debug)]
pub struct Field<T> {
    pub value: T,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct FieldWithSerializeDeserialize<T> {
    pub value: T,
}
#[derive(Debug)]
pub struct CreateManyPayloadElement {
    pub std_primitive_bool_as_postgresql_bool_kekw:
        postgresql_crud::StdOptionOptionStdPrimitiveBool,
    pub std_primitive_i16_as_postgresql_small_int_kekw:
        postgresql_crud::StdOptionOptionStdPrimitiveI16,
    pub std_primitive_i32_as_postgresql_int_kekw: postgresql_crud::StdOptionOptionStdPrimitiveI32,
}
#[derive(Debug)]
pub struct CreateManyPayload(pub std::vec::Vec<CreateManyPayloadElement>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadElementWithSerializeDeserialize {
    std_primitive_bool_as_postgresql_bool_kekw:
        postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
    std_primitive_i16_as_postgresql_small_int_kekw:
        postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
    std_primitive_i32_as_postgresql_int_kekw:
        postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadWithSerializeDeserialize(
    pub std::vec::Vec<CreateManyPayloadElementWithSerializeDeserialize>,
);
impl std::convert::From<CreateManyPayloadElementWithSerializeDeserialize>
    for CreateManyPayloadElement
{
    fn from(value: CreateManyPayloadElementWithSerializeDeserialize) -> Self {
        let std_primitive_bool_as_postgresql_bool_kekw =
            postgresql_crud::StdOptionOptionStdPrimitiveBool::from(
                value.std_primitive_bool_as_postgresql_bool_kekw,
            );
        let std_primitive_i16_as_postgresql_small_int_kekw =
            postgresql_crud::StdOptionOptionStdPrimitiveI16::from(
                value.std_primitive_i16_as_postgresql_small_int_kekw,
            );
        let std_primitive_i32_as_postgresql_int_kekw =
            postgresql_crud::StdOptionOptionStdPrimitiveI32::from(
                value.std_primitive_i32_as_postgresql_int_kekw,
            );
        Self {
            std_primitive_bool_as_postgresql_bool_kekw,
            std_primitive_i16_as_postgresql_small_int_kekw,
            std_primitive_i32_as_postgresql_int_kekw,
        }
    }
}
impl std::convert::From<CreateManyPayloadWithSerializeDeserialize> for CreateManyPayload {
    fn from(value: CreateManyPayloadWithSerializeDeserialize) -> Self {
        let mut elements = std::vec::Vec::with_capacity(value.0.len());
        for element in value.0 {
            elements.push(CreateManyPayloadElement::from(element));
        }
        Self(elements)
    }
}
impl std::convert::From<CreateManyPayloadElement>
    for CreateManyPayloadElementWithSerializeDeserialize
{
    fn from(value: CreateManyPayloadElement) -> Self {
        let std_primitive_bool_as_postgresql_bool_kekw =
            postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(
                value.std_primitive_bool_as_postgresql_bool_kekw,
            );
        let std_primitive_i16_as_postgresql_small_int_kekw =
            postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(
                value.std_primitive_i16_as_postgresql_small_int_kekw,
            );
        let std_primitive_i32_as_postgresql_int_kekw =
            postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(
                value.std_primitive_i32_as_postgresql_int_kekw,
            );
        Self {
            std_primitive_bool_as_postgresql_bool_kekw,
            std_primitive_i16_as_postgresql_small_int_kekw,
            std_primitive_i32_as_postgresql_int_kekw,
        }
    }
}
impl std::convert::From<CreateManyPayload> for CreateManyPayloadWithSerializeDeserialize {
    fn from(value: CreateManyPayload) -> Self {
        Self(
            value
                .0
                .into_iter()
                .map(|element| CreateManyPayloadElementWithSerializeDeserialize::from(element))
                .collect(),
        )
    }
}
#[derive(Debug)]
pub struct CreateManyParameters {
    pub payload: CreateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize>),
    CheckBodySize {
        check_body_size:
            route_validators::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Json {
        json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: route_validators::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyRouteLogicErrorNamed>
    for TryCreateManyRouteLogicResponseVariants
{
    fn from(value: TryCreateManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
            { json, code_occurence } => Self :: Json { json, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Json {
        #[eo_to_std_string_string]
        json: axum::extract::rejection::JsonRejection,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: route_validators::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_many_route_logic(
    app_state : axum :: extract :: State < crate ::
repositories_types :: server :: routes :: app_state ::
DynArcCombinationOfAppStateLogicTraits, >,
    request: axum::extract::Request,
) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match route_validators::check_body_size::check_body_size(
        body,
        *app_state.get_maximum_size_of_http_body_in_bytes(),
    )
    .await
    {
        Ok(value) => value,
        Err(error) => {
            let status_code = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
            let error = TryCreateManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1687,
                        column: 13,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryCreateManyRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = status_code;
            return response;
        }
    };
    if let Err(error) = route_validators::check_commit::check_commit(
        *app_state.get_enable_api_git_commit_check(),
        &headers,
    ) {
        let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
        let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
            check_commit: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        };
        eprintln!("{error}");
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TryCreateManyRouteLogicResponseVariants::from(error),
        ));
        *response.status_mut() = status_code;
        return response;
    }
    println!("kekw");
    let parameters = CreateManyParameters {
        payload: match axum::Json::<CreateManyPayloadWithSerializeDeserialize>::from_bytes(
            &body_bytes,
        ) {
            Ok(axum::Json(value)) => {
                let value = CreateManyPayload::from(value);
                value
            }
            Err(error) => {
                let error = TryCreateManyRouteLogicErrorNamed::Json {
                    json: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1776,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryCreateManyRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string =
    "insert into dogs (std_primitive_bool_as_postgresql_bool_kekw, std_primitive_i16_as_postgresql_small_int_kekw, std_primitive_i32_as_postgresql_int_kekw) select std_primitive_bool_as_postgresql_bool_kekw, std_primitive_i16_as_postgresql_small_int_kekw, std_primitive_i32_as_postgresql_int_kekw from unnest($1, $2, $3) as a(std_primitive_bool_as_postgresql_bool_kekw, std_primitive_i16_as_postgresql_small_int_kekw, std_primitive_i32_as_postgresql_int_kekw) returning sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw";
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        let current_vec_len = parameters.payload.0.len();
        let (
            std_primitive_bool_as_postgresql_bool_kekw_vec,
            std_primitive_i16_as_postgresql_small_int_kekw_vec,
            std_primitive_i32_as_postgresql_int_kekw_vec,
        ) = parameters.payload.0.into_iter().fold(
            (
                std::vec::Vec::with_capacity(current_vec_len),
                std::vec::Vec::with_capacity(current_vec_len),
                std::vec::Vec::with_capacity(current_vec_len),
            ),
            |mut acc, element| {
                acc.0
                    .push(element.std_primitive_bool_as_postgresql_bool_kekw);
                acc.1
                    .push(element.std_primitive_i16_as_postgresql_small_int_kekw);
                acc.2.push(element.std_primitive_i32_as_postgresql_int_kekw);
                acc
            },
        );
        query = query.bind(
            postgresql_crud::StdOptionOptionStdPrimitiveBool::into_inner_type_vec(
                std_primitive_bool_as_postgresql_bool_kekw_vec,
            ),
        );
        query = query.bind(
            postgresql_crud::StdOptionOptionStdPrimitiveI16::into_inner_type_vec(
                std_primitive_i16_as_postgresql_small_int_kekw_vec,
            ),
        );
        query = query.bind(
            postgresql_crud::StdOptionOptionStdPrimitiveI32::into_inner_type_vec(
                std_primitive_i32_as_postgresql_int_kekw_vec,
            ),
        );
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error) => {
            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryCreateManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error) => {
            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryCreateManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let value = {
        let mut rows = binded_query.fetch(pg_connection.as_mut());
        let mut vec_values = std::vec::Vec::new();
        while let Some(value) = {
            match {
                use futures::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => value,
                Err(error) => {
                    let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                        postgresql: error,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1735,
                                column: 21,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryCreateManyRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        } {
            match sqlx::Row::try_get::<sqlx::types::uuid::Uuid, &std::primitive::str>(
                &value,
                "sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw",
            ) {
                Ok(value) => {
                    vec_values.push(
                        postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                            postgresql_crud::SqlxTypesUuidUuid(value),
                        ),
                    );
                }
                Err(error) => {
                    let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                        postgresql: error,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1735,
                                column: 21,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryCreateManyRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        }
        vec_values
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryCreateManyRouteLogicResponseVariants::Desirable(value),
    ));
    *response.status_mut() = axum::http::StatusCode::CREATED;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClient
    {
        #[eo_error_occurence]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_create_many_route_logic_error_named_with_serialize_deserialize:
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_many(
    server_location: &std::primitive::str,
    parameters: CreateManyParameters,
) -> Result<std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>, TryCreateManyErrorNamed> {
    let payload = {
        let value = CreateManyPayloadWithSerializeDeserialize::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryCreateManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1502,
                            column: 21,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/dogs/create_many", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(
            &postgresql_crud::CommitSnakeCase.to_string(),
            git_info::PROJECT_GIT_INFO.commit,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryCreateManyErrorNamed::Reqwest {
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1661,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryCreateManyErrorNamed::FailedToGetResponseText {
                status_code,
                headers,
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1558,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let expected_response =
        match serde_json::from_str::<TryCreateManyRouteLogicResponseVariants>(&response_text) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryCreateManyErrorNamed::DeserializeResponse {
                    status_code,
                    headers,
                    response_text,
                    serde: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1621,
                            column: 21,
                        }),
                    ),
                });
            }
        };
    let try_create_many_route_logic_error_named_with_serialize_deserialize
    = match expected_response
    {
        TryCreateManyRouteLogicResponseVariants :: Desirable(value) =>
        {
            let value =
            {
                let mut values = std :: vec :: Vec :: new(); for element in
                value
                {
                    match postgresql_crud::SqlxTypesUuidUuid ::
                    try_from(element)
                    {
                        Ok(value) => { values.push(value); }, Err(error) =>
                        {
                            return
                            Err(TryCreateManyErrorNamed ::
                            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClient
                            {
                                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client
                                : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1330, column : 29,
                                })),
                            });
                        }
                    }
                } values
            }; return Ok(value);
        }, TryCreateManyRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryCreateManyRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        Postgresql { postgresql, code_occurence },
        TryCreateManyRouteLogicResponseVariants :: Json
        { json, code_occurence } =>
        TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
        { json, code_occurence }, TryCreateManyRouteLogicResponseVariants ::
        CheckCommit { check_commit, code_occurence } =>
        TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckCommit { check_commit, code_occurence },
        TryCreateManyRouteLogicResponseVariants ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        }
    };
    Err(
        TryCreateManyErrorNamed::TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize {
            try_create_many_route_logic_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 6997,
                    column: 13,
                }),
            ),
        },
    )
}
#[derive(Debug)]
pub struct CreateOnePayload {
    pub std_primitive_bool_as_postgresql_bool_kekw:
        postgresql_crud::StdOptionOptionStdPrimitiveBool,
    pub std_primitive_i16_as_postgresql_small_int_kekw:
        postgresql_crud::StdOptionOptionStdPrimitiveI16,
    pub std_primitive_i32_as_postgresql_int_kekw: postgresql_crud::StdOptionOptionStdPrimitiveI32,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateOnePayloadWithSerializeDeserialize {
    std_primitive_bool_as_postgresql_bool_kekw:
        postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
    std_primitive_i16_as_postgresql_small_int_kekw:
        postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
    std_primitive_i32_as_postgresql_int_kekw:
        postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
}
impl std::convert::From<CreateOnePayloadWithSerializeDeserialize> for CreateOnePayload {
    fn from(value: CreateOnePayloadWithSerializeDeserialize) -> Self {
        let std_primitive_bool_as_postgresql_bool_kekw =
            postgresql_crud::StdOptionOptionStdPrimitiveBool::from(
                value.std_primitive_bool_as_postgresql_bool_kekw,
            );
        let std_primitive_i16_as_postgresql_small_int_kekw =
            postgresql_crud::StdOptionOptionStdPrimitiveI16::from(
                value.std_primitive_i16_as_postgresql_small_int_kekw,
            );
        let std_primitive_i32_as_postgresql_int_kekw =
            postgresql_crud::StdOptionOptionStdPrimitiveI32::from(
                value.std_primitive_i32_as_postgresql_int_kekw,
            );
        Self {
            std_primitive_bool_as_postgresql_bool_kekw,
            std_primitive_i16_as_postgresql_small_int_kekw,
            std_primitive_i32_as_postgresql_int_kekw,
        }
    }
}
impl std::convert::From<CreateOnePayload> for CreateOnePayloadWithSerializeDeserialize {
    fn from(value: CreateOnePayload) -> Self {
        let std_primitive_bool_as_postgresql_bool_kekw =
            postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(
                value.std_primitive_bool_as_postgresql_bool_kekw,
            );
        let std_primitive_i16_as_postgresql_small_int_kekw =
            postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(
                value.std_primitive_i16_as_postgresql_small_int_kekw,
            );
        let std_primitive_i32_as_postgresql_int_kekw =
            postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(
                value.std_primitive_i32_as_postgresql_int_kekw,
            );
        Self {
            std_primitive_bool_as_postgresql_bool_kekw,
            std_primitive_i16_as_postgresql_small_int_kekw,
            std_primitive_i32_as_postgresql_int_kekw,
        }
    }
}
#[derive(Debug)]
pub struct CreateOneParameters {
    pub payload: CreateOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateOneRouteLogicResponseVariants {
    Desirable(postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize),
    CheckBodySize {
        check_body_size:
            route_validators::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Json {
        json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: route_validators::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneRouteLogicErrorNamed>
    for TryCreateOneRouteLogicResponseVariants
{
    fn from(value: TryCreateOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize :: Json
            { json, code_occurence } => Self :: Json { json, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateOneRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Json {
        #[eo_to_std_string_string]
        json: axum::extract::rejection::JsonRejection,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: route_validators::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_one_route_logic(
    app_state : axum :: extract :: State < crate ::
repositories_types :: server :: routes :: app_state ::
DynArcCombinationOfAppStateLogicTraits, >,
    request: axum::extract::Request,
) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match route_validators::check_body_size::check_body_size(
        body,
        *app_state.get_maximum_size_of_http_body_in_bytes(),
    )
    .await
    {
        Ok(value) => value,
        Err(error) => {
            let status_code = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
            let error = TryCreateOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1687,
                        column: 13,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryCreateOneRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = status_code;
            return response;
        }
    };
    if let Err(error) = route_validators::check_commit::check_commit(
        *app_state.get_enable_api_git_commit_check(),
        &headers,
    ) {
        let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
        let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
            check_commit: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        };
        eprintln!("{error}");
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TryCreateManyRouteLogicResponseVariants::from(error),
        ));
        *response.status_mut() = status_code;
        return response;
    }
    let parameters = CreateOneParameters {
        payload: match axum::Json::<CreateOnePayloadWithSerializeDeserialize>::from_bytes(
            &body_bytes,
        ) {
            Ok(axum::Json(value)) => {
                let value = CreateOnePayload::from(value);
                value
            }
            Err(error) => {
                let error = TryCreateOneRouteLogicErrorNamed::Json {
                    json: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1776,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryCreateOneRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string =
    "insert into dogs(std_primitive_bool_as_postgresql_bool_kekw, std_primitive_i16_as_postgresql_small_int_kekw, std_primitive_i32_as_postgresql_int_kekw) values ($1, $2, $3) returning sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw";
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        query = postgresql_crud::BindQuery::bind_value_to_query(
            parameters
                .payload
                .std_primitive_bool_as_postgresql_bool_kekw,
            query,
        );
        query = postgresql_crud::BindQuery::bind_value_to_query(
            parameters
                .payload
                .std_primitive_i16_as_postgresql_small_int_kekw,
            query,
        );
        query = postgresql_crud::BindQuery::bind_value_to_query(
            parameters.payload.std_primitive_i32_as_postgresql_int_kekw,
            query,
        );
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error) => {
            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryCreateOneRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error) => {
            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryCreateOneRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let value = {
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(value) => match sqlx::Row::try_get::<sqlx::types::uuid::Uuid, &str>(
                &value,
                "sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw",
            ) {
                Ok(value) => postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                    postgresql_crud::SqlxTypesUuidUuid(value),
                ),
                Err(error) => {
                    let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                        postgresql: error,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1735,
                                column: 21,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryCreateOneRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            },
            Err(error) => {
                let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                    postgresql: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1735,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryCreateOneRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        }
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryCreateOneRouteLogicResponseVariants::Desirable(value),
    ));
    *response.status_mut() = axum::http::StatusCode::CREATED;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClient
    {
        #[eo_error_occurence]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_create_one_route_logic_error_named_with_serialize_deserialize:
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_one(
    server_location: &std::primitive::str,
    parameters: CreateOneParameters,
) -> Result<postgresql_crud::SqlxTypesUuidUuid, TryCreateOneErrorNamed> {
    let payload = {
        let value = CreateOnePayloadWithSerializeDeserialize::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryCreateOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1502,
                            column: 21,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/dogs/create_one", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(
            &postgresql_crud::CommitSnakeCase.to_string(),
            git_info::PROJECT_GIT_INFO.commit,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryCreateOneErrorNamed::Reqwest {
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1661,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryCreateOneErrorNamed::FailedToGetResponseText {
                status_code,
                headers,
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1558,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let expected_response =
        match serde_json::from_str::<TryCreateOneRouteLogicResponseVariants>(&response_text) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryCreateOneErrorNamed::DeserializeResponse {
                    status_code,
                    headers,
                    response_text,
                    serde: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1621,
                            column: 21,
                        }),
                    ),
                });
            }
        };
    let try_create_one_route_logic_error_named_with_serialize_deserialize =
    match expected_response
    {
        TryCreateOneRouteLogicResponseVariants :: Desirable(value) =>
        {
            let value = match postgresql_crud::SqlxTypesUuidUuid ::
            try_from(value)
            {
                Ok(value) => value, Err(error) =>
                {
                    return
                    Err(TryCreateOneErrorNamed ::
                    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClient
                    {
                        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client
                        : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1330, column : 29,
                        })),
                    });
                }
            }; return Ok(value);
        }, TryCreateOneRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryCreateOneRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize :: Postgresql
        { postgresql, code_occurence }, TryCreateOneRouteLogicResponseVariants
        :: Json { json, code_occurence } =>
        TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize :: Json
        { json, code_occurence }, TryCreateOneRouteLogicResponseVariants ::
        CheckCommit { check_commit, code_occurence } =>
        TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckCommit { check_commit, code_occurence },
        TryCreateOneRouteLogicResponseVariants ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        }
    };
    Err(
        TryCreateOneErrorNamed::TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize {
            try_create_one_route_logic_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 6997,
                    column: 13,
                }),
            ),
        },
    )
}
#[derive(Debug)]
pub struct ReadManyPayload {
    pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
        std::option::Option<std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>>,
    pub std_primitive_bool_as_postgresql_bool_kekw:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveBool>>,
    pub std_primitive_i16_as_postgresql_small_int_kekw:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI16>>,
    pub std_primitive_i32_as_postgresql_int_kekw:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI32>>,
    pub select: std::vec::Vec<DogColumn>,
    pub order_by: postgresql_crud::OrderBy<DogColumn>,
    pub limit: postgresql_crud::StdPrimitiveI64,
    pub offset: postgresql_crud::StdPrimitiveI64,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadManyPayloadWithSerializeDeserialize {
    sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw: std::option::Option<
        std::vec::Vec<postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize>,
    >,
    std_primitive_bool_as_postgresql_bool_kekw: std::option::Option<
        std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
    >,
    std_primitive_i16_as_postgresql_small_int_kekw: std::option::Option<
        std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize>,
    >,
    std_primitive_i32_as_postgresql_int_kekw: std::option::Option<
        std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize>,
    >,
    select: std::vec::Vec<DogColumn>,
    order_by: postgresql_crud::OrderBy<DogColumn>,
    limit: postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
    offset: postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserializeErrorNamed {
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw {
        #[eo_error_occurence]
        sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<ReadManyPayloadWithSerializeDeserialize> for ReadManyPayload {
    type Error = ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: ReadManyPayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw =
            match value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw {
                Some(value) => Some({
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        match postgresql_crud::SqlxTypesUuidUuid::try_from(element) {
                            Ok(value) => {
                                acc.push(value);
                            }
                            Err(error) => {
                                return
                            Err(Self :: Error ::
                            SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw
                            {
                                sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
                                : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 549, column : 13,
                                }))
                            });
                            }
                        }
                    }
                    acc
                }),
                None => None,
            };
        let std_primitive_bool_as_postgresql_bool_kekw =
            match value.std_primitive_bool_as_postgresql_bool_kekw {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| {
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveBool::from(element)
                        })
                        .collect(),
                ),
                None => None,
            };
        let std_primitive_i16_as_postgresql_small_int_kekw =
            match value.std_primitive_i16_as_postgresql_small_int_kekw {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| {
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16::from(element)
                        })
                        .collect(),
                ),
                None => None,
            };
        let std_primitive_i32_as_postgresql_int_kekw =
            match value.std_primitive_i32_as_postgresql_int_kekw {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| {
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveI32::from(element)
                        })
                        .collect(),
                ),
                None => None,
            };
        let select = value.select;
        let order_by = value.order_by;
        let limit = postgresql_crud::StdPrimitiveI64::from(value.limit);
        let offset = postgresql_crud::StdPrimitiveI64::from(value.offset);
        Ok(Self {
            std_primitive_bool_as_postgresql_bool_kekw,
            std_primitive_i16_as_postgresql_small_int_kekw,
            std_primitive_i32_as_postgresql_int_kekw,
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            select,
            order_by,
            limit,
            offset,
        })
    }
}
impl std::convert::From<ReadManyPayload> for ReadManyPayloadWithSerializeDeserialize {
    fn from(value: ReadManyPayload) -> Self {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw =
            match value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw {
                Some(value) => {
                    Some(
                        value
                            .into_iter()
                            .map(|element| {
                                postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                                    element,
                                )
                            })
                            .collect::<std::vec::Vec<
                                postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize,
                            >>(),
                    )
                }
                None => None,
            };
        let std_primitive_bool_as_postgresql_bool_kekw = match
        value.std_primitive_bool_as_postgresql_bool_kekw
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
            :: from(element)).collect()), None => None
        };
        let std_primitive_i16_as_postgresql_small_int_kekw = match value
            .std_primitive_i16_as_postgresql_small_int_kekw
        {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize
            :: from(element)
                    })
                    .collect(),
            ),
            None => None,
        };
        let std_primitive_i32_as_postgresql_int_kekw = match value
            .std_primitive_i32_as_postgresql_int_kekw
        {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize
            :: from(element)
                    })
                    .collect(),
            ),
            None => None,
        };
        let select = value.select;
        let order_by = value.order_by;
        let limit = postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(value.limit);
        let offset = postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(value.offset);
        Self {
            std_primitive_bool_as_postgresql_bool_kekw,
            std_primitive_i16_as_postgresql_small_int_kekw,
            std_primitive_i32_as_postgresql_int_kekw,
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            select,
            order_by,
            limit,
            offset,
        }
    }
}
#[derive(Debug)]
pub struct ReadManyParameters {
    pub payload: ReadManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadManyRouteLogicResponseVariants {
    Desirable(std :: vec :: Vec :: < DogOptions >), CheckBodySize
    {
        check_body_size : route_validators :: check_body_size ::
        CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Postgresql
    {
        postgresql : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Json
    {
        json : std :: string :: String, code_occurence : error_occurence_lib
        :: code_occurence :: CodeOccurence,
    }, CheckCommit
    {
        check_commit : route_validators :: check_commit ::
        CheckCommitErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, CheckedAdd
    {
        checked_add : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, BindQuery
    {
        bind_query : postgresql_crud ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, NotUniquePrimaryKey
    {
        not_unique_primary_key : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, NotUniqueStdPrimitiveBoolAsPostgresqlBoolKekw
    {
        not_unique_std_primitive_bool_as_postgresql_bool_kekw : std :: string
        :: String, code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, NotUniqueStdPrimitiveI16AsPostgresqlSmallIntKekw
    {
        not_unique_std_primitive_i16_as_postgresql_small_int_kekw : std ::
        string :: String, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    }, NotUniqueStdPrimitiveI32AsPostgresqlIntKekw
    {
        not_unique_std_primitive_i32_as_postgresql_int_kekw : std :: string ::
        String, code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, NotUniqueColumn
    {
        not_unique_column : DogColumn, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
        : std :: string :: String, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    }, ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize
    {
        read_many_payload_try_from_read_many_payload_with_serialize_deserialize
        :
        ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }
}
impl std::convert::From<TryReadManyRouteLogicErrorNamed> for TryReadManyRouteLogicResponseVariants {
    fn from(value: TryReadManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
            { json, code_occurence } => Self :: Json { json, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckedAdd { checked_add, code_occurence } => Self :: CheckedAdd
            { checked_add, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniquePrimaryKey { not_unique_primary_key, code_occurence } =>
            Self :: NotUniquePrimaryKey
            { not_unique_primary_key, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniqueStdPrimitiveBoolAsPostgresqlBoolKekw
            {
                not_unique_std_primitive_bool_as_postgresql_bool_kekw,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveBoolAsPostgresqlBoolKekw
            {
                not_unique_std_primitive_bool_as_postgresql_bool_kekw,
                code_occurence
            }, TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI16AsPostgresqlSmallIntKekw
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_kekw,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveI16AsPostgresqlSmallIntKekw
            {
                not_unique_std_primitive_i16_as_postgresql_small_int_kekw,
                code_occurence
            }, TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniqueStdPrimitiveI32AsPostgresqlIntKekw
            {
                not_unique_std_primitive_i32_as_postgresql_int_kekw,
                code_occurence
            } => Self :: NotUniqueStdPrimitiveI32AsPostgresqlIntKekw
            {
                not_unique_std_primitive_i32_as_postgresql_int_kekw,
                code_occurence
            }, TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniqueColumn { not_unique_column, code_occurence } => Self ::
            NotUniqueColumn { not_unique_column, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize
            {
                read_many_payload_try_from_read_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize
            {
                read_many_payload_try_from_read_many_payload_with_serialize_deserialize,
                code_occurence
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Json {
        #[eo_to_std_string_string]
        json: axum::extract::rejection::JsonRejection,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: route_validators::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        #[eo_to_std_string_string_serialize_deserialize]
        checked_add: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::SqlxTypesUuidUuid,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolKekw {
        #[eo_to_std_string_string]
        not_unique_std_primitive_bool_as_postgresql_bool_kekw: postgresql_crud::StdPrimitiveBool,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI16AsPostgresqlSmallIntKekw {
        #[eo_to_std_string_string]
        not_unique_std_primitive_i16_as_postgresql_small_int_kekw: postgresql_crud::StdPrimitiveI16,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI32AsPostgresqlIntKekw {
        #[eo_to_std_string_string]
        not_unique_std_primitive_i32_as_postgresql_int_kekw: postgresql_crud::StdPrimitiveI32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: DogColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        read_many_payload_try_from_read_many_payload_with_serialize_deserialize:
            ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_many_route_logic(
    app_state : axum :: extract :: State < crate ::
repositories_types :: server :: routes :: app_state ::
DynArcCombinationOfAppStateLogicTraits, >,
    request: axum::extract::Request,
) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match route_validators::check_body_size::check_body_size(
        body,
        *app_state.get_maximum_size_of_http_body_in_bytes(),
    )
    .await
    {
        Ok(value) => value,
        Err(error) => {
            let status_code = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
            let error = TryReadManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1687,
                        column: 13,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryReadManyRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = status_code;
            return response;
        }
    };
    if let Err(error) = route_validators::check_commit::check_commit(
        *app_state.get_enable_api_git_commit_check(),
        &headers,
    ) {
        let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
        let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
            check_commit: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        };
        eprintln!("{error}");
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TryCreateManyRouteLogicResponseVariants::from(error),
        ));
        *response.status_mut() = status_code;
        return response;
    }
    let parameters = ReadManyParameters {
        payload: match axum::Json::<ReadManyPayloadWithSerializeDeserialize>::from_bytes(
            &body_bytes,
        ) {
            Ok(axum::Json(value)) => {
                let value = match ReadManyPayload::try_from(value) {
                    Ok(value) => value,
                    Err(error) => {
                        let error = TryReadManyRouteLogicErrorNamed ::
                        ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize
                        {
                            read_many_payload_try_from_read_many_payload_with_serialize_deserialize
                            : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 7495, column : 13,
                            })),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryReadManyRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                };
                if let Some(sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw) =
                    &value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
                {
                    let mut acc = std::vec::Vec::new();
                    for element in sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
                    {
                        if !acc.contains(&element) {
                            acc.push(&element);
                        } else {
                            let error = TryReadManyRouteLogicErrorNamed ::
                            NotUniquePrimaryKey
                            {
                                not_unique_primary_key : element.clone(), code_occurence :
                                error_occurence_lib :: code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1868, column : 21,
                                })),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                if let Some(value) = &value.std_primitive_bool_as_postgresql_bool_kekw {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if let Some(value) = &element.value.0 {
                            if !acc.contains(&value) {
                                acc.push(&value);
                            } else {
                                let error = TryReadManyRouteLogicErrorNamed ::
                                NotUniqueStdPrimitiveBoolAsPostgresqlBoolKekw
                                {
                                    not_unique_std_primitive_bool_as_postgresql_bool_kekw :
                                    postgresql_crud::StdPrimitiveBool(value.clone()),
                                    code_occurence : error_occurence_lib :: code_occurence ::
                                    CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 3434, column : 41,
                                    })),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() =
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                }
                if let Some(value) = &value.std_primitive_i16_as_postgresql_small_int_kekw {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if let Some(value) = &element.value.0 {
                            if !acc.contains(&value) {
                                acc.push(&value);
                            } else {
                                let error = TryReadManyRouteLogicErrorNamed ::
                                NotUniqueStdPrimitiveI16AsPostgresqlSmallIntKekw
                                {
                                    not_unique_std_primitive_i16_as_postgresql_small_int_kekw :
                                    postgresql_crud::StdPrimitiveI16(value.clone()),
                                    code_occurence : error_occurence_lib :: code_occurence ::
                                    CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 3434, column : 41,
                                    })),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() =
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                }
                if let Some(value) = &value.std_primitive_i32_as_postgresql_int_kekw {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if let Some(value) = &element.value.0 {
                            if !acc.contains(&value) {
                                acc.push(&value);
                            } else {
                                let error = TryReadManyRouteLogicErrorNamed ::
                                NotUniqueStdPrimitiveI32AsPostgresqlIntKekw
                                {
                                    not_unique_std_primitive_i32_as_postgresql_int_kekw :
                                    postgresql_crud::StdPrimitiveI32(value.clone()),
                                    code_occurence : error_occurence_lib :: code_occurence ::
                                    CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 3434, column : 41,
                                    })),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() =
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                }
                let mut acc = std::vec::Vec::new();
                for element in &value.select {
                    if acc.contains(&element) {
                        let error = TryReadManyRouteLogicErrorNamed::NotUniqueColumn {
                            not_unique_column: *element,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1455,
                                    column: 21,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryReadManyRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    } else {
                        acc.push(element);
                    }
                }
                value
            }
            Err(error) => {
                let error = TryReadManyRouteLogicErrorNamed::Json {
                    json: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1776,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryReadManyRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = format!(
        "select {} from dogs {}",
        generate_query_vec_column(&parameters.payload.select),
        {
            let mut increment: u64 = 0;
            let mut additional_parameters = std::string::String::default();
            if let Some(_) = &parameters
                .payload
                .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "where",
                    false => " and",
                };
                match increment.checked_add(1) {
                    Some(value) => {
                        increment = value;
                    }
                    None => {
                        let error = TryReadManyRouteLogicErrorNamed::CheckedAdd {
                            checked_add: std::string::String::from("checked_add is None"),
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 834,
                                    column: 21,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryReadManyRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                additional_parameters.push_str(& format!
            ("{} sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw in (select unnest(${}))",
            prefix, increment));
            }
            if let Some(value) = &parameters
                .payload
                .std_primitive_bool_as_postgresql_bool_kekw
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "where",
                    false => " and",
                };
                let bind_increments = {
                    let mut bind_increments = std::string::String::default();
                    for (index, element) in value.iter().enumerate() {
                        match postgresql_crud::BindQuery::try_generate_bind_increments(
                            element,
                            &mut increment,
                        ) {
                            Ok(value) => {
                                let handle = format!(
                                    "std_primitive_bool_as_postgresql_bool_kekw ~ {value} "
                                );
                                match index == 0 {
                                    true => {
                                        bind_increments.push_str(&handle);
                                    }
                                    false => {
                                        bind_increments.push_str(&format!(
                                            "{} {handle}",
                                            element.conjuctive_operator
                                        ));
                                    }
                                }
                            }
                            Err(error) => {
                                let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                            {
                                bind_query : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1815, column : 21,
                                }))
                            };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() =
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                    if let false = bind_increments.is_empty() {
                        let _ = bind_increments.pop();
                    }
                    bind_increments
                };
                additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
            }
            if let Some(value) = &parameters
                .payload
                .std_primitive_i16_as_postgresql_small_int_kekw
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "where",
                    false => " and",
                };
                let bind_increments = {
                    let mut bind_increments = std::string::String::default();
                    for (index, element) in value.iter().enumerate() {
                        match postgresql_crud::BindQuery::try_generate_bind_increments(
                            element,
                            &mut increment,
                        ) {
                            Ok(value) => {
                                let handle = format!(
                                    "std_primitive_i16_as_postgresql_small_int_kekw ~ {value} "
                                );
                                match index == 0 {
                                    true => {
                                        bind_increments.push_str(&handle);
                                    }
                                    false => {
                                        bind_increments.push_str(&format!(
                                            "{} {handle}",
                                            element.conjuctive_operator
                                        ));
                                    }
                                }
                            }
                            Err(error) => {
                                let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                            {
                                bind_query : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1815, column : 21,
                                }))
                            };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() =
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                    if let false = bind_increments.is_empty() {
                        let _ = bind_increments.pop();
                    }
                    bind_increments
                };
                additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
            }
            if let Some(value) = &parameters.payload.std_primitive_i32_as_postgresql_int_kekw {
                let prefix = match additional_parameters.is_empty() {
                    true => "where",
                    false => " and",
                };
                let bind_increments = {
                    let mut bind_increments = std::string::String::default();
                    for (index, element) in value.iter().enumerate() {
                        match postgresql_crud::BindQuery::try_generate_bind_increments(
                            element,
                            &mut increment,
                        ) {
                            Ok(value) => {
                                let handle =
                                    format!("std_primitive_i32_as_postgresql_int_kekw ~ {value} ");
                                match index == 0 {
                                    true => {
                                        bind_increments.push_str(&handle);
                                    }
                                    false => {
                                        bind_increments.push_str(&format!(
                                            "{} {handle}",
                                            element.conjuctive_operator
                                        ));
                                    }
                                }
                            }
                            Err(error) => {
                                let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                            {
                                bind_query : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1815, column : 21,
                                }))
                            };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(
                                    axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                                );
                                *response.status_mut() =
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                    if let false = bind_increments.is_empty() {
                        let _ = bind_increments.pop();
                    }
                    bind_increments
                };
                additional_parameters.push_str(&format!("{prefix} {bind_increments}"));
            }
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "",
                    false => " ",
                };
                let value = &parameters.payload.order_by;
                let order_stringified = match &value.order {
                    Some(order) => order.to_string(),
                    None => postgresql_crud::Order::default().to_string(),
                };
                additional_parameters.push_str(&format!(
                    "{}order by {} {}",
                    prefix, value.column, order_stringified
                ));
            }
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "",
                    false => " ",
                };
                let value =
                    match postgresql_crud::BindQuery::try_generate_bind_increments(
                        &parameters.payload.limit,
                        &mut increment,
                    ) {
                        Ok(value) => value,
                        Err(error) => {
                            let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                    {
                        bind_query : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1815, column : 21,
                        }))
                    };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    };
                additional_parameters.push_str(&format!("{}limit {}", prefix, value));
            }
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "",
                    false => " ",
                };
                let value =
                    match postgresql_crud::BindQuery::try_generate_bind_increments(
                        &parameters.payload.offset,
                        &mut increment,
                    ) {
                        Ok(value) => value,
                        Err(error) => {
                            let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                    {
                        bind_query : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1815, column : 21,
                        }))
                    };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryReadManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    };
                additional_parameters.push_str(&format!("{}offset {}", prefix, value));
            }
            additional_parameters
        }
    );
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters
            .payload
            .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
        {
            query = query.bind(
                value
                    .into_iter()
                    .map(|element| element.into_inner().clone())
                    .collect::<std::vec::Vec<sqlx::types::uuid::Uuid>>(),
            );
        }
        if let Some(values) = parameters
            .payload
            .std_primitive_bool_as_postgresql_bool_kekw
        {
            for value in values {
                query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
            }
        }
        if let Some(values) = parameters
            .payload
            .std_primitive_i16_as_postgresql_small_int_kekw
        {
            for value in values {
                query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
            }
        }
        if let Some(values) = parameters.payload.std_primitive_i32_as_postgresql_int_kekw {
            for value in values {
                query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
            }
        }
        query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.limit, query);
        query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.offset, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error) => {
            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryReadManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error) => {
            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryReadManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let value = {
        let mut rows = binded_query.fetch(pg_connection.as_mut());
        let mut vec_values = std::vec::Vec::new();
        let wrapper_vec_column = WrapperVecColumn(parameters.payload.select);
        while let Some(row) = {
            match {
                use futures::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => value,
                Err(error) => {
                    let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                        postgresql: error,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1735,
                                column: 21,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryReadManyRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        } {
            match wrapper_vec_column.options_try_from_sqlx_row(&row) {
                Ok(value) => {
                    vec_values.push(value);
                }
                Err(error) => {
                    let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                        postgresql: error,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1735,
                                column: 21,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryReadManyRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        }
        vec_values
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryReadManyRouteLogicResponseVariants::Desirable(value),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::SqlxTypesUuidUuid,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolKekw {
        #[eo_to_std_string_string]
        not_unique_std_primitive_bool_as_postgresql_bool_kekw: postgresql_crud::StdPrimitiveBool,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI16AsPostgresqlSmallIntKekw {
        #[eo_to_std_string_string]
        not_unique_std_primitive_i16_as_postgresql_small_int_kekw: postgresql_crud::StdPrimitiveI16,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveI32AsPostgresqlIntKekw {
        #[eo_to_std_string_string]
        not_unique_std_primitive_i32_as_postgresql_int_kekw: postgresql_crud::StdPrimitiveI32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: DogColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryReadManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_read_many_route_logic_error_named_with_serialize_deserialize:
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_many(
    server_location: &std::primitive::str,
    parameters: ReadManyParameters,
) -> Result<std::vec::Vec<DogOptions>, TryReadManyErrorNamed> {
    let payload = {
        if let Some(value) = &parameters
            .payload
            .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
        {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if !acc.contains(&element) {
                    acc.push(&element);
                } else {
                    return Err(TryReadManyErrorNamed::NotUniquePrimaryKey {
                        not_unique_primary_key: element.clone(),
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1868,
                                column: 21,
                            }),
                        ),
                    });
                }
            }
        }
        if let Some(value) = &parameters
            .payload
            .std_primitive_bool_as_postgresql_bool_kekw
        {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if let Some(value) = &element.value.0 {
                    if !acc.contains(&value) {
                        acc.push(&value);
                    } else {
                        return
                        Err(TryReadManyErrorNamed ::
                        NotUniqueStdPrimitiveBoolAsPostgresqlBoolKekw
                        {
                            not_unique_std_primitive_bool_as_postgresql_bool_kekw :
                            postgresql_crud::StdPrimitiveBool(value.clone()),
                            code_occurence : error_occurence_lib :: code_occurence ::
                            CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 3850, column : 37,
                            })),
                        });
                    }
                }
            }
        }
        if let Some(value) = &parameters
            .payload
            .std_primitive_i16_as_postgresql_small_int_kekw
        {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if let Some(value) = &element.value.0 {
                    if !acc.contains(&value) {
                        acc.push(&value);
                    } else {
                        return
                        Err(TryReadManyErrorNamed ::
                        NotUniqueStdPrimitiveI16AsPostgresqlSmallIntKekw
                        {
                            not_unique_std_primitive_i16_as_postgresql_small_int_kekw :
                            postgresql_crud::StdPrimitiveI16(value.clone()),
                            code_occurence : error_occurence_lib :: code_occurence ::
                            CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 3850, column : 37,
                            })),
                        });
                    }
                }
            }
        }
        if let Some(value) = &parameters.payload.std_primitive_i32_as_postgresql_int_kekw {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if let Some(value) = &element.value.0 {
                    if !acc.contains(&value) {
                        acc.push(&value);
                    } else {
                        return
                        Err(TryReadManyErrorNamed ::
                        NotUniqueStdPrimitiveI32AsPostgresqlIntKekw
                        {
                            not_unique_std_primitive_i32_as_postgresql_int_kekw :
                            postgresql_crud::StdPrimitiveI32(value.clone()),
                            code_occurence : error_occurence_lib :: code_occurence ::
                            CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 3850, column : 37,
                            })),
                        });
                    }
                }
            }
        }
        let mut acc = std::vec::Vec::new();
        for element in &parameters.payload.select {
            if acc.contains(&element) {
                return Err(TryReadManyErrorNamed::NotUniqueColumn {
                    not_unique_column: *element,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1455,
                            column: 21,
                        }),
                    ),
                });
            } else {
                acc.push(element);
            }
        }
        let value = ReadManyPayloadWithSerializeDeserialize::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryReadManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1502,
                            column: 21,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/dogs/read_many", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(
            &postgresql_crud::CommitSnakeCase.to_string(),
            git_info::PROJECT_GIT_INFO.commit,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryReadManyErrorNamed::Reqwest {
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1661,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryReadManyErrorNamed::FailedToGetResponseText {
                status_code,
                headers,
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1558,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let expected_response =
        match serde_json::from_str::<TryReadManyRouteLogicResponseVariants>(&response_text) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryReadManyErrorNamed::DeserializeResponse {
                    status_code,
                    headers,
                    response_text,
                    serde: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1621,
                            column: 21,
                        }),
                    ),
                });
            }
        };
    let try_read_many_route_logic_error_named_with_serialize_deserialize =
    match expected_response
    {
        TryReadManyRouteLogicResponseVariants :: Desirable(value) =>
        {
            let value =
            value.into_iter().fold(std :: vec :: Vec :: new(), | mut acc,
            element | { acc.push(element); acc }); return Ok(value);
        }, TryReadManyRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryReadManyRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: Postgresql
        { postgresql, code_occurence }, TryReadManyRouteLogicResponseVariants
        :: Json { json, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
        { json, code_occurence }, TryReadManyRouteLogicResponseVariants ::
        CheckCommit { check_commit, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: CheckCommit
        { check_commit, code_occurence },
        TryReadManyRouteLogicResponseVariants :: CheckedAdd
        { checked_add, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: CheckedAdd
        { checked_add, code_occurence }, TryReadManyRouteLogicResponseVariants
        :: BindQuery { bind_query, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: BindQuery
        { bind_query, code_occurence }, TryReadManyRouteLogicResponseVariants
        :: NotUniquePrimaryKey { not_unique_primary_key, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
        TryReadManyRouteLogicResponseVariants ::
        NotUniqueStdPrimitiveBoolAsPostgresqlBoolKekw
        {
            not_unique_std_primitive_bool_as_postgresql_bool_kekw,
            code_occurence
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniqueStdPrimitiveBoolAsPostgresqlBoolKekw
        {
            not_unique_std_primitive_bool_as_postgresql_bool_kekw,
            code_occurence
        }, TryReadManyRouteLogicResponseVariants ::
        NotUniqueStdPrimitiveI16AsPostgresqlSmallIntKekw
        {
            not_unique_std_primitive_i16_as_postgresql_small_int_kekw,
            code_occurence
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniqueStdPrimitiveI16AsPostgresqlSmallIntKekw
        {
            not_unique_std_primitive_i16_as_postgresql_small_int_kekw,
            code_occurence
        }, TryReadManyRouteLogicResponseVariants ::
        NotUniqueStdPrimitiveI32AsPostgresqlIntKekw
        {
            not_unique_std_primitive_i32_as_postgresql_int_kekw,
            code_occurence
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniqueStdPrimitiveI32AsPostgresqlIntKekw
        {
            not_unique_std_primitive_i32_as_postgresql_int_kekw,
            code_occurence
        }, TryReadManyRouteLogicResponseVariants :: NotUniqueColumn
        { not_unique_column, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniqueColumn { not_unique_column, code_occurence },
        TryReadManyRouteLogicResponseVariants ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        }, TryReadManyRouteLogicResponseVariants ::
        ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize
        {
            read_many_payload_try_from_read_many_payload_with_serialize_deserialize,
            code_occurence
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize
        {
            read_many_payload_try_from_read_many_payload_with_serialize_deserialize,
            code_occurence
        }
    };
    Err(
        TryReadManyErrorNamed::TryReadManyRouteLogicErrorNamedWithSerializeDeserialize {
            try_read_many_route_logic_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 6997,
                    column: 13,
                }),
            ),
        },
    )
}
#[derive(Debug)]
pub struct ReadOnePayload {
    pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
        postgresql_crud::SqlxTypesUuidUuid,
    pub select: std::vec::Vec<DogColumn>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadOnePayloadWithSerializeDeserialize {
    sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
        postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize,
    select: std::vec::Vec<DogColumn>,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamed {
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw {
        #[eo_error_occurence]
        sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<ReadOnePayloadWithSerializeDeserialize> for ReadOnePayload {
    type Error = ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: ReadOnePayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw =
            match postgresql_crud::SqlxTypesUuidUuid::try_from(
                value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            ) {
                Ok(value) => value,
                Err(error) => {
                    return Err(
                        Self::Error::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw {
                            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
                                error,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 4076,
                                    column: 45,
                                }),
                            ),
                        },
                    );
                }
            };
        let select = value.select;
        Ok(Self {
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            select,
        })
    }
}
impl std::convert::From<ReadOnePayload> for ReadOnePayloadWithSerializeDeserialize {
    fn from(value: ReadOnePayload) -> Self {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw =
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            );
        let select = value.select;
        Self {
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            select,
        }
    }
}
#[derive(Debug)]
pub struct ReadOneParameters {
    pub payload: ReadOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadOneRouteLogicResponseVariants {
    Desirable(DogOptions), CheckBodySize
    {
        check_body_size : route_validators :: check_body_size ::
        CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Postgresql
    {
        postgresql : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Json
    {
        json : std :: string :: String, code_occurence : error_occurence_lib
        :: code_occurence :: CodeOccurence,
    }, CheckCommit
    {
        check_commit : route_validators :: check_commit ::
        CheckCommitErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, NotUniqueColumn
    {
        not_unique_column : DogColumn, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
        : std :: string :: String, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    }, ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
    {
        read_one_payload_try_from_read_one_payload_with_serialize_deserialize
        :
        ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }
}
impl std::convert::From<TryReadOneRouteLogicErrorNamed> for TryReadOneRouteLogicResponseVariants {
    fn from(value: TryReadOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize :: Json
            { json, code_occurence } => Self :: Json { json, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniqueColumn { not_unique_column, code_occurence } => Self ::
            NotUniqueColumn { not_unique_column, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryReadOneRouteLogicErrorNamedWithSerializeDeserialize ::
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
            {
                read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
                code_occurence
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadOneRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Json {
        #[eo_to_std_string_string]
        json: axum::extract::rejection::JsonRejection,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: route_validators::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: DogColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        read_one_payload_try_from_read_one_payload_with_serialize_deserialize:
            ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_one_route_logic(
    app_state : axum :: extract :: State < crate ::
repositories_types :: server :: routes :: app_state ::
DynArcCombinationOfAppStateLogicTraits, >,
    request: axum::extract::Request,
) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match route_validators::check_body_size::check_body_size(
        body,
        *app_state.get_maximum_size_of_http_body_in_bytes(),
    )
    .await
    {
        Ok(value) => value,
        Err(error) => {
            let status_code = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
            let error = TryReadOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1687,
                        column: 13,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryReadOneRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = status_code;
            return response;
        }
    };
    if let Err(error) = route_validators::check_commit::check_commit(
        *app_state.get_enable_api_git_commit_check(),
        &headers,
    ) {
        let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
        let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
            check_commit: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        };
        eprintln!("{error}");
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TryCreateManyRouteLogicResponseVariants::from(error),
        ));
        *response.status_mut() = status_code;
        return response;
    }
    let parameters = ReadOneParameters {
        payload: match axum::Json::<ReadOnePayloadWithSerializeDeserialize>::from_bytes(&body_bytes)
        {
            Ok(axum::Json(value)) => {
                let value = match ReadOnePayload::try_from(value) {
                    Ok(value) => value,
                    Err(error) => {
                        let error = TryReadOneRouteLogicErrorNamed ::
                        ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
                        {
                            read_one_payload_try_from_read_one_payload_with_serialize_deserialize
                            : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 7495, column : 13,
                            })),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryReadOneRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                };
                let mut acc = std::vec::Vec::new();
                for element in &value.select {
                    if acc.contains(&element) {
                        let error = TryReadOneRouteLogicErrorNamed::NotUniqueColumn {
                            not_unique_column: *element,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1455,
                                    column: 21,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryReadOneRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    } else {
                        acc.push(element);
                    }
                }
                value
            }
            Err(error) => {
                let error = TryReadOneRouteLogicErrorNamed::Json {
                    json: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1776,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryReadOneRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = format!
    ("select {} from dogs where sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw = $1",
    generate_query_vec_column(& parameters.payload.select),);
    println!("{}", query_string);
    let binded_query = {
        let query = sqlx::query::<sqlx::Postgres>(&query_string);
        let query = postgresql_crud::BindQuery::bind_value_to_query(
            parameters
                .payload
                .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            query,
        );
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error) => {
            let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryReadOneRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error) => {
            let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryReadOneRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let value = {
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(row) => {
                match WrapperVecColumn(parameters.payload.select).options_try_from_sqlx_row(&row) {
                    Ok(value) => value,
                    Err(error) => {
                        let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                            postgresql: error,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1735,
                                    column: 21,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryReadOneRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            Err(error) => {
                let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                    postgresql: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1735,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryReadOneRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        }
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryReadOneRouteLogicResponseVariants::Desirable(value),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: DogColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryReadOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_read_one_route_logic_error_named_with_serialize_deserialize:
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_one(
    server_location: &std::primitive::str,
    parameters: ReadOneParameters,
) -> Result<DogOptions, TryReadOneErrorNamed> {
    let payload = {
        let mut acc = std::vec::Vec::new();
        for element in &parameters.payload.select {
            if acc.contains(&element) {
                return Err(TryReadOneErrorNamed::NotUniqueColumn {
                    not_unique_column: *element,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1455,
                            column: 21,
                        }),
                    ),
                });
            } else {
                acc.push(element);
            }
        }
        let value = ReadOnePayloadWithSerializeDeserialize::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryReadOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1502,
                            column: 21,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/dogs/read_one", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(
            &postgresql_crud::CommitSnakeCase.to_string(),
            git_info::PROJECT_GIT_INFO.commit,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryReadOneErrorNamed::Reqwest {
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1661,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryReadOneErrorNamed::FailedToGetResponseText {
                status_code,
                headers,
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1558,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let expected_response =
        match serde_json::from_str::<TryReadOneRouteLogicResponseVariants>(&response_text) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryReadOneErrorNamed::DeserializeResponse {
                    status_code,
                    headers,
                    response_text,
                    serde: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1621,
                            column: 21,
                        }),
                    ),
                });
            }
        };
    let try_read_one_route_logic_error_named_with_serialize_deserialize =
    match expected_response
    {
        TryReadOneRouteLogicResponseVariants :: Desirable(value) =>
        { let value = value; return Ok(value); },
        TryReadOneRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryReadOneRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryReadOneRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryReadOneRouteLogicErrorNamedWithSerializeDeserialize :: Postgresql
        { postgresql, code_occurence }, TryReadOneRouteLogicResponseVariants
        :: Json { json, code_occurence } =>
        TryReadOneRouteLogicErrorNamedWithSerializeDeserialize :: Json
        { json, code_occurence }, TryReadOneRouteLogicResponseVariants ::
        CheckCommit { check_commit, code_occurence } =>
        TryReadOneRouteLogicErrorNamedWithSerializeDeserialize :: CheckCommit
        { check_commit, code_occurence }, TryReadOneRouteLogicResponseVariants
        :: NotUniqueColumn { not_unique_column, code_occurence } =>
        TryReadOneRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniqueColumn { not_unique_column, code_occurence },
        TryReadOneRouteLogicResponseVariants ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        }, TryReadOneRouteLogicResponseVariants ::
        ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
        {
            read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
            code_occurence
        } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize ::
        ReadOnePayloadTryFromReadOnePayloadWithSerializeDeserialize
        {
            read_one_payload_try_from_read_one_payload_with_serialize_deserialize,
            code_occurence
        }
    };
    Err(
        TryReadOneErrorNamed::TryReadOneRouteLogicErrorNamedWithSerializeDeserialize {
            try_read_one_route_logic_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 6997,
                    column: 13,
                }),
            ),
        },
    )
}
#[derive(Debug)]
pub struct UpdateManyPayloadElement {
    pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
        postgresql_crud::SqlxTypesUuidUuid,
    pub std_primitive_bool_as_postgresql_bool_kekw:
        std::option::Option<Field<postgresql_crud::StdOptionOptionStdPrimitiveBool>>,
    pub std_primitive_i16_as_postgresql_small_int_kekw:
        std::option::Option<Field<postgresql_crud::StdOptionOptionStdPrimitiveI16>>,
    pub std_primitive_i32_as_postgresql_int_kekw:
        std::option::Option<Field<postgresql_crud::StdOptionOptionStdPrimitiveI32>>,
}
#[derive(Debug)]
pub struct UpdateManyPayload(pub std::vec::Vec<UpdateManyPayloadElement>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayloadElementWithSerializeDeserialize {
    sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
        postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize,
    std_primitive_bool_as_postgresql_bool_kekw: std::option::Option<
        FieldWithSerializeDeserialize<
            postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
    >,
    std_primitive_i16_as_postgresql_small_int_kekw: std::option::Option<
        FieldWithSerializeDeserialize<
            postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
        >,
    >,
    std_primitive_i32_as_postgresql_int_kekw: std::option::Option<
        FieldWithSerializeDeserialize<
            postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
        >,
    >,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayloadWithSerializeDeserialize(
    pub std::vec::Vec<UpdateManyPayloadElementWithSerializeDeserialize>,
);
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed {
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw {
        #[eo_error_occurence]
        sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<UpdateManyPayloadElementWithSerializeDeserialize>
    for UpdateManyPayloadElement
{
    type Error =
        UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: UpdateManyPayloadElementWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw =
            match postgresql_crud::SqlxTypesUuidUuid::try_from(
                value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            ) {
                Ok(value) => value,
                Err(error) => {
                    return Err(
                        Self::Error::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw {
                            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
                                error,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 2219,
                                    column: 13,
                                }),
                            ),
                        },
                    );
                }
            };
        let std_primitive_bool_as_postgresql_bool_kekw =
            match value.std_primitive_bool_as_postgresql_bool_kekw {
                Some(value) => Some(Field {
                    value: postgresql_crud::StdOptionOptionStdPrimitiveBool::from(value.value),
                }),
                None => None,
            };
        let std_primitive_i16_as_postgresql_small_int_kekw =
            match value.std_primitive_i16_as_postgresql_small_int_kekw {
                Some(value) => Some(Field {
                    value: postgresql_crud::StdOptionOptionStdPrimitiveI16::from(value.value),
                }),
                None => None,
            };
        let std_primitive_i32_as_postgresql_int_kekw =
            match value.std_primitive_i32_as_postgresql_int_kekw {
                Some(value) => Some(Field {
                    value: postgresql_crud::StdOptionOptionStdPrimitiveI32::from(value.value),
                }),
                None => None,
            };
        Ok(Self {
            std_primitive_bool_as_postgresql_bool_kekw,
            std_primitive_i16_as_postgresql_small_int_kekw,
            std_primitive_i32_as_postgresql_int_kekw,
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
        })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamed {
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw {
        #[eo_error_occurence]
        sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl
    std::convert::From<
        UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed,
    > for UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamed
{
    fn from(
        value :
    UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed,
    ) -> Self {
        match value
        {
            UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed
            :: SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw
            {
                sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
                code_occurence,
            } => Self ::
            SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw
            {
                sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
                code_occurence,
            },
        }
    }
}
impl std::convert::TryFrom<UpdateManyPayloadWithSerializeDeserialize> for UpdateManyPayload {
    type Error = UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: UpdateManyPayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        match
        value.0.into_iter().map(| element | UpdateManyPayloadElement ::
        try_from(element)).collect :: < Result < std :: vec :: Vec <
        UpdateManyPayloadElement > ,
        UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed
        >> ()
        {
            Ok(value) => Ok(Self(value)), Err(error) =>
            Err(Self :: Error :: from(error)),
        }
    }
}
impl std::convert::From<UpdateManyPayloadElement>
    for UpdateManyPayloadElementWithSerializeDeserialize
{
    fn from(value: UpdateManyPayloadElement) -> Self {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw =
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            );
        let std_primitive_bool_as_postgresql_bool_kekw = match value
            .std_primitive_bool_as_postgresql_bool_kekw
        {
            Some(value) => Some(FieldWithSerializeDeserialize {
                value:
                    postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(
                        value.value,
                    ),
            }),
            None => None,
        };
        let std_primitive_i16_as_postgresql_small_int_kekw = match value
            .std_primitive_i16_as_postgresql_small_int_kekw
        {
            Some(value) => Some(FieldWithSerializeDeserialize {
                value:
                    postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(
                        value.value,
                    ),
            }),
            None => None,
        };
        let std_primitive_i32_as_postgresql_int_kekw = match value
            .std_primitive_i32_as_postgresql_int_kekw
        {
            Some(value) => Some(FieldWithSerializeDeserialize {
                value:
                    postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(
                        value.value,
                    ),
            }),
            None => None,
        };
        Self {
            std_primitive_bool_as_postgresql_bool_kekw,
            std_primitive_i16_as_postgresql_small_int_kekw,
            std_primitive_i32_as_postgresql_int_kekw,
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
        }
    }
}
impl std::convert::From<UpdateManyPayload> for UpdateManyPayloadWithSerializeDeserialize {
    fn from(value: UpdateManyPayload) -> Self {
        Self(
            value
                .0
                .into_iter()
                .map(|element| UpdateManyPayloadElementWithSerializeDeserialize::from(element))
                .collect(),
        )
    }
}
#[derive(Debug)]
pub struct UpdateManyParameters {
    pub payload: UpdateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateManyRouteLogicResponseVariants {
    Desirable(std :: vec :: Vec :: <
    postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize >),
    CheckBodySize
    {
        check_body_size : route_validators :: check_body_size ::
        CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Postgresql
    {
        postgresql : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Json
    {
        json : std :: string :: String, code_occurence : error_occurence_lib
        :: code_occurence :: CodeOccurence,
    }, CheckCommit
    {
        check_commit : route_validators :: check_commit ::
        CheckCommitErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, QueryAndRollbackFailed
    {
        query : std :: string :: String, rollback : std :: string :: String,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, PrimaryKeyFromRowAndFailedRollback
    {
        primary_key_from_row : std :: string :: String, rollback : std ::
        string :: String, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    }, NonExistingPrimaryKeys
    {
        non_existing_primary_keys : std :: vec :: Vec < std :: string ::
        String > , code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, NonExistingPrimaryKeysAndFailedRollback
    {
        non_existing_primary_keys : std :: vec :: Vec < std :: string ::
        String > , rollback : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, CommitFailed
    {
        commit_failed : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, NotUniquePrimaryKey
    {
        not_unique_primary_key : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, BindQuery
    {
        bind_query : postgresql_crud ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, NoPayloadFieldsPrimaryKey
    {
        no_payload_fields_primary_key : std :: string :: String,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
        : std :: string :: String, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    }, UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
    {
        update_many_payload_try_from_update_many_payload_with_serialize_deserialize
        :
        UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }
}
impl std::convert::From<TryUpdateManyRouteLogicErrorNamed>
    for TryUpdateManyRouteLogicResponseVariants
{
    fn from(value: TryUpdateManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
            { json, code_occurence } => Self :: Json { json, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            QueryAndRollbackFailed { query, rollback, code_occurence } => Self
            :: QueryAndRollbackFailed { query, rollback, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback, code_occurence } => Self ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback, code_occurence } => Self ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CommitFailed { commit_failed, code_occurence } => Self ::
            CommitFailed { commit_failed, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniquePrimaryKey { not_unique_primary_key, code_occurence } =>
            Self :: NotUniquePrimaryKey
            { not_unique_primary_key, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NoPayloadFieldsPrimaryKey
            { no_payload_fields_primary_key, code_occurence } => Self ::
            NoPayloadFieldsPrimaryKey
            { no_payload_fields_primary_key, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
            {
                update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
            {
                update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
                code_occurence
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateManyRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Json {
        #[eo_to_std_string_string]
        json: axum::extract::rejection::JsonRejection,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: route_validators::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryAndRollbackFailed {
        #[eo_to_std_string_string]
        query: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PrimaryKeyFromRowAndFailedRollback {
        #[eo_to_std_string_string]
        primary_key_from_row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        #[eo_vec_to_std_string_string]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndFailedRollback {
        #[eo_vec_to_std_string_string]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitFailed {
        #[eo_to_std_string_string]
        commit_failed: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::SqlxTypesUuidUuid,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFieldsPrimaryKey {
        #[eo_to_std_string_string]
        no_payload_fields_primary_key: postgresql_crud::SqlxTypesUuidUuid,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        update_many_payload_try_from_update_many_payload_with_serialize_deserialize:
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_many_route_logic(
    app_state : axum :: extract :: State < crate ::
repositories_types :: server :: routes :: app_state ::
DynArcCombinationOfAppStateLogicTraits, >,
    request: axum::extract::Request,
) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match route_validators::check_body_size::check_body_size(
        body,
        *app_state.get_maximum_size_of_http_body_in_bytes(),
    )
    .await
    {
        Ok(value) => value,
        Err(error) => {
            let status_code = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
            let error = TryUpdateManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1687,
                        column: 13,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryUpdateManyRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = status_code;
            return response;
        }
    };
    if let Err(error) = route_validators::check_commit::check_commit(
        *app_state.get_enable_api_git_commit_check(),
        &headers,
    ) {
        let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
        let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
            check_commit: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        };
        eprintln!("{error}");
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TryCreateManyRouteLogicResponseVariants::from(error),
        ));
        *response.status_mut() = status_code;
        return response;
    }
    let parameters = UpdateManyParameters {
        payload: match axum::Json::<UpdateManyPayloadWithSerializeDeserialize>::from_bytes(
            &body_bytes,
        ) {
            Ok(axum::Json(value)) => {
                let value = match UpdateManyPayload::try_from(value) {
                    Ok(value) => value,
                    Err(error) => {
                        let error = TryUpdateManyRouteLogicErrorNamed ::
                        UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
                        {
                            update_many_payload_try_from_update_many_payload_with_serialize_deserialize
                            : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 7495, column : 13,
                            })),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryUpdateManyRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                };
                {
                    let mut acc = std::vec::Vec::new();
                    for element in &value.0 {
                        if !acc.contains(
                            &element
                                .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
                        ) {
                            acc.push(element.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw.clone());
                        } else {
                            let error = TryUpdateManyRouteLogicErrorNamed ::
                            NotUniquePrimaryKey
                            {
                                not_unique_primary_key :
                                element.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw.clone(),
                                code_occurence : error_occurence_lib :: code_occurence ::
                                CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1868, column : 21,
                                })),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                for element in &value.0 {
                    if let (None, None, None) = (
                        &element.std_primitive_bool_as_postgresql_bool_kekw,
                        &element.std_primitive_i16_as_postgresql_small_int_kekw,
                        &element.std_primitive_i32_as_postgresql_int_kekw,
                    ) {
                        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw =
                            element
                                .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw;
                        let error = TryUpdateManyRouteLogicErrorNamed::NoPayloadFieldsPrimaryKey {
                            no_payload_fields_primary_key:
                                sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1925,
                                    column: 21,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryUpdateManyRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                value
            }
            Err(error) => {
                let error = TryUpdateManyRouteLogicErrorNamed::Json {
                    json: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1776,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryUpdateManyRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let expected_updated_primary_keys = parameters
        .payload
        .0
        .iter()
        .map(|element| {
            element
                .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
                .clone()
        })
        .collect::<std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>>();
    let query_string = {
        let mut query = std::string::String::from("update dogs set ");
        let mut increment: u64 = 0;
        {
            let mut is_std_primitive_bool_as_postgresql_bool_kekw_update_exist = false;
            for element in &parameters.payload.0 {
                if element.std_primitive_bool_as_postgresql_bool_kekw.is_some() {
                    is_std_primitive_bool_as_postgresql_bool_kekw_update_exist = true;
                    break;
                }
            }
            if is_std_primitive_bool_as_postgresql_bool_kekw_update_exist {
                let mut acc =
                    std::string::String::from("std_primitive_bool_as_postgresql_bool_kekw = case ");
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.std_primitive_bool_as_postgresql_bool_kekw {
                        acc.push_str(& format!
                        ("when sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw = {} then {} ",
                        match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(&
                        element.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
                        & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1815, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }, match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(& value.value, & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1815, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }));
                    }
                }
                query.push_str(&format!(
                    "{}{}",
                    acc, "else std_primitive_bool_as_postgresql_bool_kekw end,"
                ));
            }
        }
        {
            let mut is_std_primitive_i16_as_postgresql_small_int_kekw_update_exist = false;
            for element in &parameters.payload.0 {
                if element
                    .std_primitive_i16_as_postgresql_small_int_kekw
                    .is_some()
                {
                    is_std_primitive_i16_as_postgresql_small_int_kekw_update_exist = true;
                    break;
                }
            }
            if is_std_primitive_i16_as_postgresql_small_int_kekw_update_exist {
                let mut acc = std::string::String::from(
                    "std_primitive_i16_as_postgresql_small_int_kekw = case ",
                );
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.std_primitive_i16_as_postgresql_small_int_kekw {
                        acc.push_str(& format!
                        ("when sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw = {} then {} ",
                        match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(&
                        element.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
                        & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1815, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }, match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(& value.value, & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1815, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }));
                    }
                }
                query.push_str(&format!(
                    "{}{}",
                    acc, "else std_primitive_i16_as_postgresql_small_int_kekw end,"
                ));
            }
        }
        {
            let mut is_std_primitive_i32_as_postgresql_int_kekw_update_exist = false;
            for element in &parameters.payload.0 {
                if element.std_primitive_i32_as_postgresql_int_kekw.is_some() {
                    is_std_primitive_i32_as_postgresql_int_kekw_update_exist = true;
                    break;
                }
            }
            if is_std_primitive_i32_as_postgresql_int_kekw_update_exist {
                let mut acc =
                    std::string::String::from("std_primitive_i32_as_postgresql_int_kekw = case ");
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.std_primitive_i32_as_postgresql_int_kekw {
                        acc.push_str(& format!
                        ("when sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw = {} then {} ",
                        match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(&
                        element.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
                        & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1815, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }, match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(& value.value, & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1815, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }));
                    }
                }
                query.push_str(&format!(
                    "{}{}",
                    acc, "else std_primitive_i32_as_postgresql_int_kekw end,"
                ));
            }
        }
        let _ = query.pop();
        query.push_str(& format!
        (" where sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw in ({}) returning sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw;",
        {
            let mut acc = std :: string :: String :: default(); for element in
            & parameters.payload.0
            {
                match postgresql_crud :: BindQuery ::
                try_generate_bind_increments(&
                element.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
                & mut increment)
                {
                    Ok(value) => { acc.push_str(& format! ("{value},")); },
                    Err(error) =>
                    {
                        let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                        {
                            bind_query : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1815, column : 21,
                            }))
                        }; eprintln! ("{error}"); let mut response = axum ::
                        response :: IntoResponse ::
                        into_response(axum ::
                        Json(TryUpdateManyRouteLogicResponseVariants ::
                        from(error))); * response.status_mut() = axum :: http ::
                        StatusCode :: INTERNAL_SERVER_ERROR; return response;
                    }
                }
            } let _ = acc.pop(); acc
        }));
        query
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        {
            let mut is_std_primitive_bool_as_postgresql_bool_kekw_update_exist = false;
            for element in &parameters.payload.0 {
                if element.std_primitive_bool_as_postgresql_bool_kekw.is_some() {
                    is_std_primitive_bool_as_postgresql_bool_kekw_update_exist = true;
                    break;
                }
            }
            if is_std_primitive_bool_as_postgresql_bool_kekw_update_exist {
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.std_primitive_bool_as_postgresql_bool_kekw {
                        query = query.bind(
                            element
                                .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
                                .into_inner(),
                        );
                        query = query.bind(value.value.0);
                    }
                }
            }
        }
        {
            let mut is_std_primitive_i16_as_postgresql_small_int_kekw_update_exist = false;
            for element in &parameters.payload.0 {
                if element
                    .std_primitive_i16_as_postgresql_small_int_kekw
                    .is_some()
                {
                    is_std_primitive_i16_as_postgresql_small_int_kekw_update_exist = true;
                    break;
                }
            }
            if is_std_primitive_i16_as_postgresql_small_int_kekw_update_exist {
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.std_primitive_i16_as_postgresql_small_int_kekw {
                        query = query.bind(
                            element
                                .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
                                .into_inner(),
                        );
                        query = query.bind(value.value.0);
                    }
                }
            }
        }
        {
            let mut is_std_primitive_i32_as_postgresql_int_kekw_update_exist = false;
            for element in &parameters.payload.0 {
                if element.std_primitive_i32_as_postgresql_int_kekw.is_some() {
                    is_std_primitive_i32_as_postgresql_int_kekw_update_exist = true;
                    break;
                }
            }
            if is_std_primitive_i32_as_postgresql_int_kekw_update_exist {
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.std_primitive_i32_as_postgresql_int_kekw {
                        query = query.bind(
                            element
                                .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
                                .into_inner(),
                        );
                        query = query.bind(value.value.0);
                    }
                }
            }
        }
        {
            for element in &parameters.payload.0 {
                query = query.bind(
                    element
                        .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
                        .into_inner(),
                );
            }
        }
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error) => {
            let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryUpdateManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error) => {
            let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryUpdateManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let value = {
        let mut postgres_transaction = match {
            use sqlx::Acquire;
            pg_connection.begin()
        }
        .await
        {
            Ok(value) => value,
            Err(error) => {
                let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                    postgresql: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1735,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryUpdateManyRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let results_vec =
            {
                let mut results_vec =
                    std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
                let mut option_error: Option<sqlx::Error> = None;
                {
                    let mut rows = binded_query.fetch(postgres_transaction.as_mut());
                    while let (Some(Some(row)), None) = (
                        match {
                            use futures::TryStreamExt;
                            rows.try_next()
                        }
                        .await
                        {
                            Ok(value) => Some(value),
                            Err(error) => {
                                option_error = Some(error);
                                None
                            }
                        },
                        &option_error,
                    ) {
                        results_vec.push(row);
                    }
                }
                if let Some(error) = option_error {
                    match postgres_transaction.rollback().await {
                        Ok(_) => {
                            let error = TryUpdateManyRouteLogicErrorNamed :: Postgresql
                        {
                            postgresql : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1735, column : 21,
                            })),
                        };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(rollback_error) => {
                            let error = TryUpdateManyRouteLogicErrorNamed ::
                        QueryAndRollbackFailed
                        {
                            query : error, rollback : rollback_error, code_occurence :
                            error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 876, column : 21,
                            })),
                        };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
                results_vec
            };
        let primary_key_vec = {
            let mut primary_key_vec =
                std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
            for element in results_vec {
                match primary_key_try_from_sqlx_row(&element) {
                    Ok(primary_key) => {
                        primary_key_vec.push(primary_key);
                    }
                    Err(error) => match postgres_transaction.rollback().await {
                        Ok(_) => {
                            let error = TryUpdateManyRouteLogicErrorNamed :: Postgresql
                            {
                                postgresql : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1735, column : 21,
                                })),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(rollback_error) => {
                            let error = TryUpdateManyRouteLogicErrorNamed ::
                            PrimaryKeyFromRowAndFailedRollback
                            {
                                primary_key_from_row : error, rollback : rollback_error,
                                code_occurence : error_occurence_lib :: code_occurence ::
                                CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 922, column : 21,
                                })),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(
                                axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)),
                            );
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                }
            }
            primary_key_vec
        };
        {
            let non_existing_primary_keys = {
                let len = expected_updated_primary_keys.len();
                expected_updated_primary_keys.into_iter().fold(
                    std::vec::Vec::with_capacity(len),
                    |mut acc, element| {
                        if let false = primary_key_vec.contains(&element) {
                            acc.push(element);
                        }
                        acc
                    },
                )
            };
            if let false = non_existing_primary_keys.is_empty() {
                match postgres_transaction.rollback().await {
                    Ok(_) => {
                        let error = TryUpdateManyRouteLogicErrorNamed::NonExistingPrimaryKeys {
                            non_existing_primary_keys,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1130,
                                    column: 21,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryUpdateManyRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                    Err(error) => {
                        let error = TryUpdateManyRouteLogicErrorNamed ::
                        NonExistingPrimaryKeysAndFailedRollback
                        {
                            non_existing_primary_keys, rollback : error, code_occurence
                            : error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1173, column : 21,
                            })),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryUpdateManyRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
            }
        }
        match postgres_transaction.commit().await {
            Ok(_) => primary_key_vec
                .into_iter()
                .map(|element| {
                    postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(element)
                })
                .collect(),
            Err(error) => {
                let error = TryUpdateManyRouteLogicErrorNamed::CommitFailed {
                    commit_failed: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1210,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryUpdateManyRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        }
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryUpdateManyRouteLogicResponseVariants::Desirable(value),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClient
    {
        #[eo_error_occurence]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::SqlxTypesUuidUuid,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_update_many_route_logic_error_named_with_serialize_deserialize:
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_many(
    server_location: &std::primitive::str,
    parameters: UpdateManyParameters,
) -> Result<std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>, TryUpdateManyErrorNamed> {
    let payload = {
        let mut acc = std::vec::Vec::new();
        for element in &parameters.payload.0 {
            if !acc.contains(
                &&element.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            ) {
                acc.push(
                    &element.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
                );
            } else {
                return Err(TryUpdateManyErrorNamed::NotUniquePrimaryKey {
                    not_unique_primary_key: element
                        .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
                        .clone(),
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1868,
                            column: 21,
                        }),
                    ),
                });
            }
        }
        let value = UpdateManyPayloadWithSerializeDeserialize::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryUpdateManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1502,
                            column: 21,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/dogs/update_many", server_location,);
    let future = reqwest::Client::new()
        .patch(&url)
        .header(
            &postgresql_crud::CommitSnakeCase.to_string(),
            git_info::PROJECT_GIT_INFO.commit,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryUpdateManyErrorNamed::Reqwest {
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1661,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryUpdateManyErrorNamed::FailedToGetResponseText {
                status_code,
                headers,
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1558,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let expected_response =
        match serde_json::from_str::<TryUpdateManyRouteLogicResponseVariants>(&response_text) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryUpdateManyErrorNamed::DeserializeResponse {
                    status_code,
                    headers,
                    response_text,
                    serde: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1621,
                            column: 21,
                        }),
                    ),
                });
            }
        };
    let try_update_many_route_logic_error_named_with_serialize_deserialize
    = match expected_response
    {
        TryUpdateManyRouteLogicResponseVariants :: Desirable(value) =>
        {
            let value =
            {
                let mut values = std :: vec :: Vec :: new(); for element in
                value
                {
                    match postgresql_crud::SqlxTypesUuidUuid ::
                    try_from(element)
                    {
                        Ok(value) => { values.push(value); }, Err(error) =>
                        {
                            return
                            Err(TryUpdateManyErrorNamed ::
                            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClient
                            {
                                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client
                                : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1330, column : 29,
                                })),
                            });
                        }
                    }
                } values
            }; return Ok(value);
        }, TryUpdateManyRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        Postgresql { postgresql, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: Json
        { json, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
        { json, code_occurence }, TryUpdateManyRouteLogicResponseVariants ::
        CheckCommit { check_commit, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckCommit { check_commit, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: QueryAndRollbackFailed
        { query, rollback, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        QueryAndRollbackFailed { query, rollback, code_occurence },
        TryUpdateManyRouteLogicResponseVariants ::
        PrimaryKeyFromRowAndFailedRollback
        { primary_key_from_row, rollback, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        PrimaryKeyFromRowAndFailedRollback
        { primary_key_from_row, rollback, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: NonExistingPrimaryKeys
        { non_existing_primary_keys, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence },
        TryUpdateManyRouteLogicResponseVariants ::
        NonExistingPrimaryKeysAndFailedRollback
        { non_existing_primary_keys, rollback, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NonExistingPrimaryKeysAndFailedRollback
        { non_existing_primary_keys, rollback, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: CommitFailed
        { commit_failed, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CommitFailed { commit_failed, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: NotUniquePrimaryKey
        { not_unique_primary_key, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: BindQuery
        { bind_query, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize :: BindQuery
        { bind_query, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: NoPayloadFieldsPrimaryKey
        { no_payload_fields_primary_key, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NoPayloadFieldsPrimaryKey
        { no_payload_fields_primary_key, code_occurence },
        TryUpdateManyRouteLogicResponseVariants ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        }, TryUpdateManyRouteLogicResponseVariants ::
        UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
        {
            update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
            code_occurence
        } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
        {
            update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
            code_occurence
        }
    };
    Err(
        TryUpdateManyErrorNamed::TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize {
            try_update_many_route_logic_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 6997,
                    column: 13,
                }),
            ),
        },
    )
}
#[derive(Debug)]
pub struct UpdateOnePayload {
    pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
        postgresql_crud::SqlxTypesUuidUuid,
    pub std_primitive_bool_as_postgresql_bool_kekw:
        std::option::Option<Field<postgresql_crud::StdOptionOptionStdPrimitiveBool>>,
    pub std_primitive_i16_as_postgresql_small_int_kekw:
        std::option::Option<Field<postgresql_crud::StdOptionOptionStdPrimitiveI16>>,
    pub std_primitive_i32_as_postgresql_int_kekw:
        std::option::Option<Field<postgresql_crud::StdOptionOptionStdPrimitiveI32>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateOnePayloadWithSerializeDeserialize {
    sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
        postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize,
    std_primitive_bool_as_postgresql_bool_kekw: std::option::Option<
        FieldWithSerializeDeserialize<
            postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
    >,
    std_primitive_i16_as_postgresql_small_int_kekw: std::option::Option<
        FieldWithSerializeDeserialize<
            postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
        >,
    >,
    std_primitive_i32_as_postgresql_int_kekw: std::option::Option<
        FieldWithSerializeDeserialize<
            postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
        >,
    >,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum UpdateOnePayloadTryFromUpdateOnePayloadWithSerializeDeserializeErrorNamed {
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw {
        #[eo_error_occurence]
        sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<UpdateOnePayloadWithSerializeDeserialize> for UpdateOnePayload {
    type Error = UpdateOnePayloadTryFromUpdateOnePayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: UpdateOnePayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw =
            match postgresql_crud::SqlxTypesUuidUuid::try_from(
                value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            ) {
                Ok(value) => value,
                Err(error) => {
                    return Err(
                        Self::Error::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw {
                            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
                                error,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 2219,
                                    column: 13,
                                }),
                            ),
                        },
                    );
                }
            };
        let std_primitive_bool_as_postgresql_bool_kekw =
            match value.std_primitive_bool_as_postgresql_bool_kekw {
                Some(value) => Some(Field {
                    value: postgresql_crud::StdOptionOptionStdPrimitiveBool::from(value.value),
                }),
                None => None,
            };
        let std_primitive_i16_as_postgresql_small_int_kekw =
            match value.std_primitive_i16_as_postgresql_small_int_kekw {
                Some(value) => Some(Field {
                    value: postgresql_crud::StdOptionOptionStdPrimitiveI16::from(value.value),
                }),
                None => None,
            };
        let std_primitive_i32_as_postgresql_int_kekw =
            match value.std_primitive_i32_as_postgresql_int_kekw {
                Some(value) => Some(Field {
                    value: postgresql_crud::StdOptionOptionStdPrimitiveI32::from(value.value),
                }),
                None => None,
            };
        Ok(Self {
            std_primitive_bool_as_postgresql_bool_kekw,
            std_primitive_i16_as_postgresql_small_int_kekw,
            std_primitive_i32_as_postgresql_int_kekw,
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
        })
    }
}
impl std::convert::From<UpdateOnePayload> for UpdateOnePayloadWithSerializeDeserialize {
    fn from(value: UpdateOnePayload) -> Self {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw =
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            );
        let std_primitive_bool_as_postgresql_bool_kekw = match value
            .std_primitive_bool_as_postgresql_bool_kekw
        {
            Some(value) => Some(FieldWithSerializeDeserialize {
                value:
                    postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize::from(
                        value.value,
                    ),
            }),
            None => None,
        };
        let std_primitive_i16_as_postgresql_small_int_kekw = match value
            .std_primitive_i16_as_postgresql_small_int_kekw
        {
            Some(value) => Some(FieldWithSerializeDeserialize {
                value:
                    postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize::from(
                        value.value,
                    ),
            }),
            None => None,
        };
        let std_primitive_i32_as_postgresql_int_kekw = match value
            .std_primitive_i32_as_postgresql_int_kekw
        {
            Some(value) => Some(FieldWithSerializeDeserialize {
                value:
                    postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize::from(
                        value.value,
                    ),
            }),
            None => None,
        };
        Self {
            std_primitive_bool_as_postgresql_bool_kekw,
            std_primitive_i16_as_postgresql_small_int_kekw,
            std_primitive_i32_as_postgresql_int_kekw,
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
        }
    }
}
#[derive(Debug)]
pub struct UpdateOneParameters {
    pub payload: UpdateOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateOneRouteLogicResponseVariants {
    Desirable(postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize),
    CheckBodySize
    {
        check_body_size : route_validators :: check_body_size ::
        CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Postgresql
    {
        postgresql : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Json
    {
        json : std :: string :: String, code_occurence : error_occurence_lib
        :: code_occurence :: CodeOccurence,
    }, CheckCommit
    {
        check_commit : route_validators :: check_commit ::
        CheckCommitErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, BindQuery
    {
        bind_query : postgresql_crud ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, NoPayloadFieldsPrimaryKey
    {
        no_payload_fields_primary_key : std :: string :: String,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
        : std :: string :: String, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    }, UpdateOnePayloadTryFromUpdateOnePayloadWithSerializeDeserialize
    {
        update_one_payload_try_from_update_one_payload_with_serialize_deserialize
        :
        UpdateOnePayloadTryFromUpdateOnePayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }
}
impl std::convert::From<TryUpdateOneRouteLogicErrorNamed>
    for TryUpdateOneRouteLogicResponseVariants
{
    fn from(value: TryUpdateOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize :: Json
            { json, code_occurence } => Self :: Json { json, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            NoPayloadFieldsPrimaryKey
            { no_payload_fields_primary_key, code_occurence } => Self ::
            NoPayloadFieldsPrimaryKey
            { no_payload_fields_primary_key, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
            UpdateOnePayloadTryFromUpdateOnePayloadWithSerializeDeserialize
            {
                update_one_payload_try_from_update_one_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            UpdateOnePayloadTryFromUpdateOnePayloadWithSerializeDeserialize
            {
                update_one_payload_try_from_update_one_payload_with_serialize_deserialize,
                code_occurence
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateOneRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Json {
        #[eo_to_std_string_string]
        json: axum::extract::rejection::JsonRejection,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: route_validators::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFieldsPrimaryKey {
        #[eo_to_std_string_string]
        no_payload_fields_primary_key: postgresql_crud::SqlxTypesUuidUuid,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UpdateOnePayloadTryFromUpdateOnePayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        update_one_payload_try_from_update_one_payload_with_serialize_deserialize:
            UpdateOnePayloadTryFromUpdateOnePayloadWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_one_route_logic(
    app_state : axum :: extract :: State < crate ::
repositories_types :: server :: routes :: app_state ::
DynArcCombinationOfAppStateLogicTraits, >,
    request: axum::extract::Request,
) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match route_validators::check_body_size::check_body_size(
        body,
        *app_state.get_maximum_size_of_http_body_in_bytes(),
    )
    .await
    {
        Ok(value) => value,
        Err(error) => {
            let status_code = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
            let error = TryUpdateOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1687,
                        column: 13,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryUpdateOneRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = status_code;
            return response;
        }
    };
    if let Err(error) = route_validators::check_commit::check_commit(
        *app_state.get_enable_api_git_commit_check(),
        &headers,
    ) {
        let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
        let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
            check_commit: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        };
        eprintln!("{error}");
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TryCreateManyRouteLogicResponseVariants::from(error),
        ));
        *response.status_mut() = status_code;
        return response;
    }
    let parameters = UpdateOneParameters {
        payload: match axum::Json::<UpdateOnePayloadWithSerializeDeserialize>::from_bytes(
            &body_bytes,
        ) {
            Ok(axum::Json(value)) => {
                let value = match UpdateOnePayload::try_from(value) {
                    Ok(value) => value,
                    Err(error) => {
                        let error = TryUpdateOneRouteLogicErrorNamed ::
                        UpdateOnePayloadTryFromUpdateOnePayloadWithSerializeDeserialize
                        {
                            update_one_payload_try_from_update_one_payload_with_serialize_deserialize
                            : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 7495, column : 13,
                            })),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryUpdateOneRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                };
                if let (None, None, None) = (
                    &value.std_primitive_bool_as_postgresql_bool_kekw,
                    &value.std_primitive_i16_as_postgresql_small_int_kekw,
                    &value.std_primitive_i32_as_postgresql_int_kekw,
                ) {
                    let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw =
                        value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw;
                    let error = TryUpdateOneRouteLogicErrorNamed::NoPayloadFieldsPrimaryKey {
                        no_payload_fields_primary_key:
                            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1925,
                                column: 21,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryUpdateOneRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
                value
            }
            Err(error) => {
                let error = TryUpdateOneRouteLogicErrorNamed::Json {
                    json: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1776,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryUpdateOneRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = {
        let mut increment: u64 = 0;
        let mut query = std::string::String::from("update dogs set ");
        if let Some(value) = &parameters
            .payload
            .std_primitive_bool_as_postgresql_bool_kekw
        {
            match postgresql_crud::BindQuery::try_increment(&value.value, &mut increment) {
                Ok(_) => {
                    query.push_str(&format!(
                        "std_primitive_bool_as_postgresql_bool_kekw = ${}, ",
                        increment
                    ));
                }
                Err(error) => {
                    let error = TryUpdateOneRouteLogicErrorNamed::BindQuery {
                        bind_query: error,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1815,
                                column: 21,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryUpdateOneRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        }
        if let Some(value) = &parameters
            .payload
            .std_primitive_i16_as_postgresql_small_int_kekw
        {
            match postgresql_crud::BindQuery::try_increment(&value.value, &mut increment) {
                Ok(_) => {
                    query.push_str(&format!(
                        "std_primitive_i16_as_postgresql_small_int_kekw = ${}, ",
                        increment
                    ));
                }
                Err(error) => {
                    let error = TryUpdateOneRouteLogicErrorNamed::BindQuery {
                        bind_query: error,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1815,
                                column: 21,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryUpdateOneRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        }
        if let Some(value) = &parameters.payload.std_primitive_i32_as_postgresql_int_kekw {
            match postgresql_crud::BindQuery::try_increment(&value.value, &mut increment) {
                Ok(_) => {
                    query.push_str(&format!(
                        "std_primitive_i32_as_postgresql_int_kekw = ${}, ",
                        increment
                    ));
                }
                Err(error) => {
                    let error = TryUpdateOneRouteLogicErrorNamed::BindQuery {
                        bind_query: error,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1815,
                                column: 21,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryUpdateOneRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        }
        match postgresql_crud::BindQuery::try_increment(
            &parameters
                .payload
                .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            &mut increment,
        ) {
            Ok(_) => {
                query.push_str(& format!
                (" where sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw = ${increment}"));
            }
            Err(error) => {
                let error = TryUpdateOneRouteLogicErrorNamed::BindQuery {
                    bind_query: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1815,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryUpdateOneRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        }
        query.push_str(&format!(
            " returning sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw"
        ));
        query
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters
            .payload
            .std_primitive_bool_as_postgresql_bool_kekw
        {
            query = postgresql_crud::BindQuery::bind_value_to_query(value.value, query);
        }
        if let Some(value) = parameters
            .payload
            .std_primitive_i16_as_postgresql_small_int_kekw
        {
            query = postgresql_crud::BindQuery::bind_value_to_query(value.value, query);
        }
        if let Some(value) = parameters.payload.std_primitive_i32_as_postgresql_int_kekw {
            query = postgresql_crud::BindQuery::bind_value_to_query(value.value, query);
        }
        query = postgresql_crud::BindQuery::bind_value_to_query(
            parameters
                .payload
                .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            query,
        );
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error) => {
            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryUpdateOneRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error) => {
            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryUpdateOneRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let value = {
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(value) => match sqlx::Row::try_get::<sqlx::types::uuid::Uuid, &std::primitive::str>(
                &value,
                "sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw",
            ) {
                Ok(value) => postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                    postgresql_crud::SqlxTypesUuidUuid(value),
                ),
                Err(error) => {
                    let error = TryUpdateOneRouteLogicErrorNamed ::
                    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
                    {
                        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                        : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1247, column : 21,
                        })),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryUpdateOneRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            },
            Err(error) => {
                let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                    postgresql: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1735,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryUpdateOneRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        }
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryUpdateOneRouteLogicResponseVariants::Desirable(value),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClient
    {
        #[eo_error_occurence]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_update_one_route_logic_error_named_with_serialize_deserialize:
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_one(
    server_location: &std::primitive::str,
    parameters: UpdateOneParameters,
) -> Result<postgresql_crud::SqlxTypesUuidUuid, TryUpdateOneErrorNamed> {
    let payload = {
        let value = UpdateOnePayloadWithSerializeDeserialize::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryUpdateOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1502,
                            column: 21,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/dogs/update_one", server_location,);
    let future = reqwest::Client::new()
        .patch(&url)
        .header(
            &postgresql_crud::CommitSnakeCase.to_string(),
            git_info::PROJECT_GIT_INFO.commit,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryUpdateOneErrorNamed::Reqwest {
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1661,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryUpdateOneErrorNamed::FailedToGetResponseText {
                status_code,
                headers,
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1558,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let expected_response =
        match serde_json::from_str::<TryUpdateOneRouteLogicResponseVariants>(&response_text) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryUpdateOneErrorNamed::DeserializeResponse {
                    status_code,
                    headers,
                    response_text,
                    serde: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1621,
                            column: 21,
                        }),
                    ),
                });
            }
        };
    let try_update_one_route_logic_error_named_with_serialize_deserialize =
    match expected_response
    {
        TryUpdateOneRouteLogicResponseVariants :: Desirable(value) =>
        {
            let value = match postgresql_crud::SqlxTypesUuidUuid ::
            try_from(value)
            {
                Ok(value) => value, Err(error) =>
                {
                    return
                    Err(TryUpdateOneErrorNamed ::
                    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClient
                    {
                        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client
                        : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1330, column : 29,
                        })),
                    });
                }
            }; return Ok(value);
        }, TryUpdateOneRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryUpdateOneRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize :: Postgresql
        { postgresql, code_occurence }, TryUpdateOneRouteLogicResponseVariants
        :: Json { json, code_occurence } =>
        TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize :: Json
        { json, code_occurence }, TryUpdateOneRouteLogicResponseVariants ::
        CheckCommit { check_commit, code_occurence } =>
        TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckCommit { check_commit, code_occurence },
        TryUpdateOneRouteLogicResponseVariants :: BindQuery
        { bind_query, code_occurence } =>
        TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize :: BindQuery
        { bind_query, code_occurence }, TryUpdateOneRouteLogicResponseVariants
        :: NoPayloadFieldsPrimaryKey
        { no_payload_fields_primary_key, code_occurence } =>
        TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
        NoPayloadFieldsPrimaryKey
        { no_payload_fields_primary_key, code_occurence },
        TryUpdateOneRouteLogicResponseVariants ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        }, TryUpdateOneRouteLogicResponseVariants ::
        UpdateOnePayloadTryFromUpdateOnePayloadWithSerializeDeserialize
        {
            update_one_payload_try_from_update_one_payload_with_serialize_deserialize,
            code_occurence
        } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize ::
        UpdateOnePayloadTryFromUpdateOnePayloadWithSerializeDeserialize
        {
            update_one_payload_try_from_update_one_payload_with_serialize_deserialize,
            code_occurence
        }
    };
    Err(
        TryUpdateOneErrorNamed::TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize {
            try_update_one_route_logic_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 6997,
                    column: 13,
                }),
            ),
        },
    )
}
#[derive(Debug)]
pub struct DeleteManyPayload {
    pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
        std::option::Option<std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>>,
    pub std_primitive_bool_as_postgresql_bool_kekw:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveBool>>,
    pub std_primitive_i16_as_postgresql_small_int_kekw:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI16>>,
    pub std_primitive_i32_as_postgresql_int_kekw:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI32>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct DeleteManyPayloadWithSerializeDeserialize {
    sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw: std::option::Option<
        std::vec::Vec<postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize>,
    >,
    std_primitive_bool_as_postgresql_bool_kekw: std::option::Option<
        std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
    >,
    std_primitive_i16_as_postgresql_small_int_kekw: std::option::Option<
        std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize>,
    >,
    std_primitive_i32_as_postgresql_int_kekw: std::option::Option<
        std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize>,
    >,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserializeErrorNamed {
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw {
        #[eo_error_occurence]
        sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DeleteManyPayloadWithSerializeDeserialize> for DeleteManyPayload {
    type Error = DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: DeleteManyPayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw =
            match value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw {
                Some(value) => {
                    let mut values = std::vec::Vec::with_capacity(value.len());
                    for element in value {
                        match postgresql_crud::SqlxTypesUuidUuid::try_from(element) {
                            Ok(value) => {
                                values.push(value);
                            }
                            Err(error) => {
                                return
                            Err(Self :: Error ::
                            SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw
                            {
                                sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
                                : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 549, column : 13,
                                }))
                            });
                            }
                        }
                    }
                    Some(values)
                }
                None => None,
            };
        let std_primitive_bool_as_postgresql_bool_kekw =
            match value.std_primitive_bool_as_postgresql_bool_kekw {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| {
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveBool::from(element)
                        })
                        .collect(),
                ),
                None => None,
            };
        let std_primitive_i16_as_postgresql_small_int_kekw =
            match value.std_primitive_i16_as_postgresql_small_int_kekw {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| {
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16::from(element)
                        })
                        .collect(),
                ),
                None => None,
            };
        let std_primitive_i32_as_postgresql_int_kekw =
            match value.std_primitive_i32_as_postgresql_int_kekw {
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|element| {
                            postgresql_crud::WhereStdOptionOptionStdPrimitiveI32::from(element)
                        })
                        .collect(),
                ),
                None => None,
            };
        Ok(Self {
            std_primitive_bool_as_postgresql_bool_kekw,
            std_primitive_i16_as_postgresql_small_int_kekw,
            std_primitive_i32_as_postgresql_int_kekw,
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
        })
    }
}
impl std::convert::From<DeleteManyPayload> for DeleteManyPayloadWithSerializeDeserialize {
    fn from(value: DeleteManyPayload) -> Self {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw =
            match value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw {
                Some(value) => {
                    Some(
                        value
                            .into_iter()
                            .map(|element| {
                                postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                                    element,
                                )
                            })
                            .collect::<std::vec::Vec<
                                postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize,
                            >>(),
                    )
                }
                None => None,
            };
        let std_primitive_bool_as_postgresql_bool_kekw = match
        value.std_primitive_bool_as_postgresql_bool_kekw
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
            :: from(element)).collect()), None => None
        };
        let std_primitive_i16_as_postgresql_small_int_kekw = match value
            .std_primitive_i16_as_postgresql_small_int_kekw
        {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize
            :: from(element)
                    })
                    .collect(),
            ),
            None => None,
        };
        let std_primitive_i32_as_postgresql_int_kekw = match value
            .std_primitive_i32_as_postgresql_int_kekw
        {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize
            :: from(element)
                    })
                    .collect(),
            ),
            None => None,
        };
        Self {
            std_primitive_bool_as_postgresql_bool_kekw,
            std_primitive_i16_as_postgresql_small_int_kekw,
            std_primitive_i32_as_postgresql_int_kekw,
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
        }
    }
}
#[derive(Debug)]
pub struct DeleteManyParameters {
    pub payload: DeleteManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryDeleteManyRouteLogicResponseVariants {
    Desirable(std :: vec :: Vec :: <
    postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize >),
    CheckBodySize
    {
        check_body_size : route_validators :: check_body_size ::
        CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Postgresql
    {
        postgresql : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Json
    {
        json : std :: string :: String, code_occurence : error_occurence_lib
        :: code_occurence :: CodeOccurence,
    }, CheckCommit
    {
        check_commit : route_validators :: check_commit ::
        CheckCommitErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, BindQuery
    {
        bind_query : postgresql_crud ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, NotUniquePrimaryKey
    {
        not_unique_primary_key : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, NoPayloadFields
    {
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
        : std :: string :: String, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    }, DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
    {
        delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize
        :
        DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }
}
impl std::convert::From<TryDeleteManyRouteLogicErrorNamed>
    for TryDeleteManyRouteLogicResponseVariants
{
    fn from(value: TryDeleteManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
            { json, code_occurence } => Self :: Json { json, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniquePrimaryKey { not_unique_primary_key, code_occurence } =>
            Self :: NotUniquePrimaryKey
            { not_unique_primary_key, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NoPayloadFields { code_occurence } => Self :: NoPayloadFields
            { code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
            DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
            {
                delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
            {
                delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize,
                code_occurence
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteManyRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Json {
        #[eo_to_std_string_string]
        json: axum::extract::rejection::JsonRejection,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: route_validators::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::SqlxTypesUuidUuid,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize:
            DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_many_route_logic(
    app_state : axum :: extract :: State < crate ::
repositories_types :: server :: routes :: app_state ::
DynArcCombinationOfAppStateLogicTraits, >,
    request: axum::extract::Request,
) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match route_validators::check_body_size::check_body_size(
        body,
        *app_state.get_maximum_size_of_http_body_in_bytes(),
    )
    .await
    {
        Ok(value) => value,
        Err(error) => {
            let status_code = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
            let error = TryDeleteManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1687,
                        column: 13,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryDeleteManyRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = status_code;
            return response;
        }
    };
    if let Err(error) = route_validators::check_commit::check_commit(
        *app_state.get_enable_api_git_commit_check(),
        &headers,
    ) {
        let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
        let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
            check_commit: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        };
        eprintln!("{error}");
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TryCreateManyRouteLogicResponseVariants::from(error),
        ));
        *response.status_mut() = status_code;
        return response;
    }
    let parameters = DeleteManyParameters {
        payload: match axum::Json::<DeleteManyPayloadWithSerializeDeserialize>::from_bytes(
            &body_bytes,
        ) {
            Ok(axum::Json(value)) => {
                let value = match DeleteManyPayload::try_from(value) {
                    Ok(value) => value,
                    Err(error) => {
                        let error = TryDeleteManyRouteLogicErrorNamed ::
                        DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
                        {
                            delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize
                            : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 7495, column : 13,
                            })),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryDeleteManyRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                };
                if let (None, None, None, None) = (
                    &value.std_primitive_bool_as_postgresql_bool_kekw,
                    &value.std_primitive_i16_as_postgresql_small_int_kekw,
                    &value.std_primitive_i32_as_postgresql_int_kekw,
                    &value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
                ) {
                    let error = TryDeleteManyRouteLogicErrorNamed::NoPayloadFields {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 5493,
                                column: 25,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryDeleteManyRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
                value
            }
            Err(error) => {
                let error = TryDeleteManyRouteLogicErrorNamed::Json {
                    json: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1776,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryDeleteManyRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = format!
    ("delete from dogs where {} returning sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw",
    {
        let mut increment : u64 = 0; let mut additional_parameters = std ::
        string :: String :: default(); if let Some(value) = &
        parameters.payload.std_primitive_bool_as_postgresql_bool_kekw
        {
            for element in value
            {
                match postgresql_crud :: BindQuery ::
                try_increment(element, & mut increment)
                {
                    Ok(_) =>
                    {
                        let handle = format!
                        ("std_primitive_bool_as_postgresql_bool_kekw = ${increment}");
                        match additional_parameters.is_empty()
                        {
                            true => { additional_parameters.push_str(& handle); }, false
                            =>
                            {
                                additional_parameters.push_str(& format! (" AND {handle}"));
                            },
                        }
                    }, Err(error) =>
                    {
                        let error = TryDeleteManyRouteLogicErrorNamed :: BindQuery
                        {
                            bind_query : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1815, column : 21,
                            }))
                        }; eprintln! ("{error}"); let mut response = axum ::
                        response :: IntoResponse ::
                        into_response(axum ::
                        Json(TryDeleteManyRouteLogicResponseVariants ::
                        from(error))); * response.status_mut() = axum :: http ::
                        StatusCode :: INTERNAL_SERVER_ERROR; return response;
                    },
                }
            }
        } if let Some(value) = &
        parameters.payload.std_primitive_i16_as_postgresql_small_int_kekw
        {
            for element in value
            {
                match postgresql_crud :: BindQuery ::
                try_increment(element, & mut increment)
                {
                    Ok(_) =>
                    {
                        let handle = format!
                        ("std_primitive_i16_as_postgresql_small_int_kekw = ${increment}");
                        match additional_parameters.is_empty()
                        {
                            true => { additional_parameters.push_str(& handle); }, false
                            =>
                            {
                                additional_parameters.push_str(& format! (" AND {handle}"));
                            },
                        }
                    }, Err(error) =>
                    {
                        let error = TryDeleteManyRouteLogicErrorNamed :: BindQuery
                        {
                            bind_query : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1815, column : 21,
                            }))
                        }; eprintln! ("{error}"); let mut response = axum ::
                        response :: IntoResponse ::
                        into_response(axum ::
                        Json(TryDeleteManyRouteLogicResponseVariants ::
                        from(error))); * response.status_mut() = axum :: http ::
                        StatusCode :: INTERNAL_SERVER_ERROR; return response;
                    },
                }
            }
        } if let Some(value) = &
        parameters.payload.std_primitive_i32_as_postgresql_int_kekw
        {
            for element in value
            {
                match postgresql_crud :: BindQuery ::
                try_increment(element, & mut increment)
                {
                    Ok(_) =>
                    {
                        let handle = format!
                        ("std_primitive_i32_as_postgresql_int_kekw = ${increment}");
                        match additional_parameters.is_empty()
                        {
                            true => { additional_parameters.push_str(& handle); }, false
                            =>
                            {
                                additional_parameters.push_str(& format! (" AND {handle}"));
                            },
                        }
                    }, Err(error) =>
                    {
                        let error = TryDeleteManyRouteLogicErrorNamed :: BindQuery
                        {
                            bind_query : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1815, column : 21,
                            }))
                        }; eprintln! ("{error}"); let mut response = axum ::
                        response :: IntoResponse ::
                        into_response(axum ::
                        Json(TryDeleteManyRouteLogicResponseVariants ::
                        from(error))); * response.status_mut() = axum :: http ::
                        StatusCode :: INTERNAL_SERVER_ERROR; return response;
                    },
                }
            }
        } if let
        Some(sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw)
        = &
        parameters.payload.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
        {
            if let false = additional_parameters.is_empty()
            { additional_parameters.push_str(" and"); }
            additional_parameters.push_str(& format!
            (" sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw in ({})",
            {
                let mut additional_parameters = std :: string :: String ::
                default(); for element in
                sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
                {
                    match postgresql_crud :: BindQuery ::
                    try_increment(element, & mut increment,)
                    {
                        Ok(_) =>
                        {
                            additional_parameters.push_str(& format! ("${increment},"));
                        } Err(error) =>
                        {
                            let error = TryDeleteManyRouteLogicErrorNamed :: BindQuery
                            {
                                bind_query : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1815, column : 21,
                                }))
                            }; eprintln! ("{error}"); let mut response = axum ::
                            response :: IntoResponse ::
                            into_response(axum ::
                            Json(TryDeleteManyRouteLogicResponseVariants ::
                            from(error))); * response.status_mut() = axum :: http ::
                            StatusCode :: INTERNAL_SERVER_ERROR; return response;
                        }
                    }
                } let _ = additional_parameters.pop(); additional_parameters
            }));
        } additional_parameters
    });
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters
            .payload
            .std_primitive_bool_as_postgresql_bool_kekw
        {
            for element in value {
                query = postgresql_crud::BindQuery::bind_value_to_query(element, query);
            }
        }
        if let Some(value) = parameters
            .payload
            .std_primitive_i16_as_postgresql_small_int_kekw
        {
            for element in value {
                query = postgresql_crud::BindQuery::bind_value_to_query(element, query);
            }
        }
        if let Some(value) = parameters.payload.std_primitive_i32_as_postgresql_int_kekw {
            for element in value {
                query = postgresql_crud::BindQuery::bind_value_to_query(element, query);
            }
        }
        if let Some(sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw) = parameters
            .payload
            .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw
        {
            for element in sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw {
                query = postgresql_crud::BindQuery::bind_value_to_query(element, query);
            }
        }
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error) => {
            let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryDeleteManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error) => {
            let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryDeleteManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let value = {
        let mut rows = binded_query.fetch(pg_connection.as_mut());
        let mut vec_values = std::vec::Vec::new();
        while let Some(row) = {
            match {
                use futures::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => value,
                Err(error) => {
                    let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                        postgresql: error,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1735,
                                column: 21,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryDeleteManyRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        } {
            match sqlx::Row::try_get::<sqlx::types::uuid::Uuid, &std::primitive::str>(
                &row,
                "sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw",
            ) {
                Ok(value) => {
                    vec_values.push(
                        postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                            postgresql_crud::SqlxTypesUuidUuid(value),
                        ),
                    );
                }
                Err(error) => {
                    let error = TryDeleteManyRouteLogicErrorNamed ::
                    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
                    {
                        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                        : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1247, column : 21,
                        })),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryDeleteManyRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        }
        vec_values
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryDeleteManyRouteLogicResponseVariants::Desirable(value),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClient
    {
        #[eo_error_occurence]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_delete_many_route_logic_error_named_with_serialize_deserialize:
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_many(
    server_location: &std::primitive::str,
    parameters: DeleteManyParameters,
) -> Result<std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>, TryDeleteManyErrorNamed> {
    let payload = {
        let value = DeleteManyPayloadWithSerializeDeserialize::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryDeleteManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1502,
                            column: 21,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/dogs/delete_many", server_location,);
    let future = reqwest::Client::new()
        .delete(&url)
        .header(
            &postgresql_crud::CommitSnakeCase.to_string(),
            git_info::PROJECT_GIT_INFO.commit,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryDeleteManyErrorNamed::Reqwest {
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1661,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryDeleteManyErrorNamed::FailedToGetResponseText {
                status_code,
                headers,
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1558,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let expected_response =
        match serde_json::from_str::<TryDeleteManyRouteLogicResponseVariants>(&response_text) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryDeleteManyErrorNamed::DeserializeResponse {
                    status_code,
                    headers,
                    response_text,
                    serde: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1621,
                            column: 21,
                        }),
                    ),
                });
            }
        };
    let try_delete_many_route_logic_error_named_with_serialize_deserialize
    = match expected_response
    {
        TryDeleteManyRouteLogicResponseVariants :: Desirable(value) =>
        {
            let value =
            {
                let mut values = std :: vec :: Vec :: new(); for element in
                value
                {
                    match postgresql_crud::SqlxTypesUuidUuid ::
                    try_from(element)
                    {
                        Ok(value) => { values.push(value); }, Err(error) =>
                        {
                            return
                            Err(TryDeleteManyErrorNamed ::
                            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClient
                            {
                                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client
                                : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1330, column : 29,
                                })),
                            });
                        }
                    }
                } values
            }; return Ok(value);
        }, TryDeleteManyRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryDeleteManyRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
        Postgresql { postgresql, code_occurence },
        TryDeleteManyRouteLogicResponseVariants :: Json
        { json, code_occurence } =>
        TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
        { json, code_occurence }, TryDeleteManyRouteLogicResponseVariants ::
        CheckCommit { check_commit, code_occurence } =>
        TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckCommit { check_commit, code_occurence },
        TryDeleteManyRouteLogicResponseVariants :: BindQuery
        { bind_query, code_occurence } =>
        TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize :: BindQuery
        { bind_query, code_occurence },
        TryDeleteManyRouteLogicResponseVariants :: NotUniquePrimaryKey
        { not_unique_primary_key, code_occurence } =>
        TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
        TryDeleteManyRouteLogicResponseVariants :: NoPayloadFields
        { code_occurence } =>
        TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NoPayloadFields { code_occurence },
        TryDeleteManyRouteLogicResponseVariants ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        }, TryDeleteManyRouteLogicResponseVariants ::
        DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
        {
            delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize,
            code_occurence
        } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
        DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
        {
            delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize,
            code_occurence
        }
    };
    Err(
        TryDeleteManyErrorNamed::TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize {
            try_delete_many_route_logic_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 6997,
                    column: 13,
                }),
            ),
        },
    )
}
#[derive(Debug, Clone, Copy)]
pub struct DeleteOnePayload {
    pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
        postgresql_crud::SqlxTypesUuidUuid,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct DeleteOnePayloadWithSerializeDeserialize {
    sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
        postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserializeErrorNamed {
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw {
        #[eo_error_occurence]
        sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DeleteOnePayloadWithSerializeDeserialize> for DeleteOnePayload {
    type Error = DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: DeleteOnePayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        match postgresql_crud::SqlxTypesUuidUuid::try_from(
            value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
        ) {
            Ok(value) => Ok(Self {
                sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw: value,
            }),
            Err(error) => Err(
                Self::Error::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKeyKekw {
                    sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 549,
                            column: 13,
                        }),
                    ),
                },
            ),
        }
    }
}
impl std::convert::From<DeleteOnePayload> for DeleteOnePayloadWithSerializeDeserialize {
    fn from(value: DeleteOnePayload) -> Self {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw =
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            );
        Self {
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeleteOneParameters {
    pub payload: DeleteOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryDeleteOneRouteLogicResponseVariants {
    Desirable(postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize),
    CheckBodySize
    {
        check_body_size : route_validators :: check_body_size ::
        CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Postgresql
    {
        postgresql : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Json
    {
        json : std :: string :: String, code_occurence : error_occurence_lib
        :: code_occurence :: CodeOccurence,
    }, CheckCommit
    {
        check_commit : route_validators :: check_commit ::
        CheckCommitErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
        : std :: string :: String, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    }, DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
    {
        delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize
        :
        DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }
}
impl std::convert::From<TryDeleteOneRouteLogicErrorNamed>
    for TryDeleteOneRouteLogicResponseVariants
{
    fn from(value: TryDeleteOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize :: Json
            { json, code_occurence } => Self :: Json { json, code_occurence },
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize ::
            DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
            {
                delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
            {
                delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize,
                code_occurence
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteOneRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: route_validators::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Json {
        #[eo_to_std_string_string]
        json: axum::extract::rejection::JsonRejection,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: route_validators::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize:
            DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_one_route_logic(
    app_state : axum :: extract :: State < crate ::
repositories_types :: server :: routes :: app_state ::
DynArcCombinationOfAppStateLogicTraits, >,
    request: axum::extract::Request,
) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match route_validators::check_body_size::check_body_size(
        body,
        *app_state.get_maximum_size_of_http_body_in_bytes(),
    )
    .await
    {
        Ok(value) => value,
        Err(error) => {
            let status_code = http_logic::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
            let error = TryDeleteOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1687,
                        column: 13,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TryDeleteOneRouteLogicResponseVariants::from(error),
            ));
            *response.status_mut() = status_code;
            return response;
        }
    };
    if let Err(error) = route_validators::check_commit::check_commit(
        *app_state.get_enable_api_git_commit_check(),
        &headers,
    ) {
        let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
        let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
            check_commit: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        };
        eprintln!("{error}");
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TryCreateManyRouteLogicResponseVariants::from(error),
        ));
        *response.status_mut() = status_code;
        return response;
    }
    let parameters = DeleteOneParameters {
        payload: match axum::Json::<DeleteOnePayloadWithSerializeDeserialize>::from_bytes(
            &body_bytes,
        ) {
            Ok(axum::Json(value)) => {
                let value = match DeleteOnePayload::try_from(value) {
                    Ok(value) => value,
                    Err(error) => {
                        let error = TryDeleteOneRouteLogicErrorNamed ::
                        DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
                        {
                            delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize
                            : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 7495, column : 13,
                            })),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TryDeleteOneRouteLogicResponseVariants::from(error),
                        ));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                };
                value
            }
            Err(error) => {
                let error = TryDeleteOneRouteLogicErrorNamed::Json {
                    json: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1776,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryDeleteOneRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = {
        let mut query = format!("delete from dogs where");
        query.push_str(&format!(
            " sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw = $1"
        ));
        query.push_str(&format!(
            " returning sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw"
        ));
        query
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        query = postgresql_crud::BindQuery::bind_value_to_query(
            parameters
                .payload
                .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw,
            query,
        );
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error) => {
            let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryDeleteOneRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error) => {
            let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                postgresql: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1735,
                        column: 21,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryDeleteOneRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    let value = {
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(value) => match sqlx::Row::try_get::<sqlx::types::uuid::Uuid, &std::primitive::str>(
                &value,
                "sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key_kekw",
            ) {
                Ok(value) => postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                    postgresql_crud::SqlxTypesUuidUuid(value),
                ),
                Err(error) => {
                    let error = TryDeleteOneRouteLogicErrorNamed ::
                    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
                    {
                        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
                        : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1247, column : 21,
                        })),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TryDeleteOneRouteLogicResponseVariants::from(error),
                    ));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            },
            Err(error) => {
                let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                    postgresql: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1735,
                            column: 21,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TryDeleteOneRouteLogicResponseVariants::from(error),
                ));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        }
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryDeleteOneRouteLogicResponseVariants::Desirable(value),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClient
    {
        #[eo_error_occurence]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_delete_one_route_logic_error_named_with_serialize_deserialize:
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_one(
    server_location: &std::primitive::str,
    parameters: DeleteOneParameters,
) -> Result<postgresql_crud::SqlxTypesUuidUuid, TryDeleteOneErrorNamed> {
    let payload = {
        let value = DeleteOnePayloadWithSerializeDeserialize::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryDeleteOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1502,
                            column: 21,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/dogs/delete_one", server_location,);
    let future = reqwest::Client::new()
        .delete(&url)
        .header(
            &postgresql_crud::CommitSnakeCase.to_string(),
            git_info::PROJECT_GIT_INFO.commit,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryDeleteOneErrorNamed::Reqwest {
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1661,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(value) => value,
        Err(error) => {
            return Err(TryDeleteOneErrorNamed::FailedToGetResponseText {
                status_code,
                headers,
                reqwest: error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1558,
                        column: 21,
                    }),
                ),
            });
        }
    };
    let expected_response =
        match serde_json::from_str::<TryDeleteOneRouteLogicResponseVariants>(&response_text) {
            Ok(value) => value,
            Err(error) => {
                return Err(TryDeleteOneErrorNamed::DeserializeResponse {
                    status_code,
                    headers,
                    response_text,
                    serde: error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1621,
                            column: 21,
                        }),
                    ),
                });
            }
        };
    let try_delete_one_route_logic_error_named_with_serialize_deserialize =
    match expected_response
    {
        TryDeleteOneRouteLogicResponseVariants :: Desirable(value) =>
        {
            let value = match postgresql_crud::SqlxTypesUuidUuid ::
            try_from(value)
            {
                Ok(value) => value, Err(error) =>
                {
                    return
                    Err(TryDeleteOneErrorNamed ::
                    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClient
                    {
                        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client
                        : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1330, column : 29,
                        })),
                    });
                }
            }; return Ok(value);
        }, TryDeleteOneRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryDeleteOneRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize :: Postgresql
        { postgresql, code_occurence }, TryDeleteOneRouteLogicResponseVariants
        :: Json { json, code_occurence } =>
        TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize :: Json
        { json, code_occurence }, TryDeleteOneRouteLogicResponseVariants ::
        CheckCommit { check_commit, code_occurence } =>
        TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckCommit { check_commit, code_occurence },
        TryDeleteOneRouteLogicResponseVariants ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        } => TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        }, TryDeleteOneRouteLogicResponseVariants ::
        DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
        {
            delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize,
            code_occurence
        } => TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize ::
        DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
        {
            delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize,
            code_occurence
        }
    };
    Err(
        TryDeleteOneErrorNamed::TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize {
            try_delete_one_route_logic_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 6997,
                    column: 13,
                }),
            ),
        },
    )
}
