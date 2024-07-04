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
    pub std_primitive_bool_as_postgresql_bool: postgresql_crud::StdPrimitiveBoolAsPostgresqlBool,
    // pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdPrimitiveBoolAsPostgresqlBoolNotNull,

    pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt,
    // pub std_primitive_i16_as_postgresql_small_int_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallIntNotNull,
    // pub std_primitive_i16_as_postgresql_small_serial: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerial,
    // pub std_primitive_i16_as_postgresql_small_serial_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerialNotNull,
    // pub std_primitive_i16_as_postgresql_small_int2: postgresql_crud::StdPrimitiveI16AsPostgresqlInt2,
    // pub std_primitive_i16_as_postgresql_small_int2_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlInt2NotNull,

    pub std_primitive_i32_as_postgresql_int: postgresql_crud::StdPrimitiveI32AsPostgresqlInt,
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
    pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,//todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough //fails too but primary key is a different logic. need refactor it as different task 

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
#[derive(Debug)]
pub struct DeleteManyPayload {
    pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key:
        std::option::Option<std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>>,
    pub std_primitive_bool_as_postgresql_bool:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveBool>>,
    pub std_primitive_i16_as_postgresql_small_int:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI16>>,
    pub std_primitive_i32_as_postgresql_int:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI32>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct DeleteManyPayloadWithSerializeDeserialize {
    sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key: std::option::Option<
        std::vec::Vec<postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize>,
    >,
    std_primitive_bool_as_postgresql_bool: std::option::Option<
        std::vec::Vec<
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
        >,
    >,
    std_primitive_i16_as_postgresql_small_int: std::option::Option<
        std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize>,
    >,
    std_primitive_i32_as_postgresql_int: std::option::Option<
        std::vec::Vec<postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize>,
    >,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserializeErrorNamed {
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey {
        #[eo_error_occurence]
        sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key:
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DeleteManyPayloadWithSerializeDeserialize> for DeleteManyPayload {
    type Error = DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: DeleteManyPayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key =
            match value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key {
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
                            SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey
                            {
                                sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key
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
        let std_primitive_bool_as_postgresql_bool =
            match value.std_primitive_bool_as_postgresql_bool {
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
        let std_primitive_i16_as_postgresql_small_int =
            match value.std_primitive_i16_as_postgresql_small_int {
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
        let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
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
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key,
        })
    }
}
impl std::convert::From<DeleteManyPayload> for DeleteManyPayloadWithSerializeDeserialize {
    fn from(value: DeleteManyPayload) -> Self {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key =
            match value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key {
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
        let std_primitive_bool_as_postgresql_bool = match
        value.std_primitive_bool_as_postgresql_bool
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
            :: from(element)).collect()), None => None
        };
        let std_primitive_i16_as_postgresql_small_int = match value
            .std_primitive_i16_as_postgresql_small_int
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
        let std_primitive_i32_as_postgresql_int = match value.std_primitive_i32_as_postgresql_int {
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
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key,
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
    }, NoPrimaryKeys
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
            NoPrimaryKeys { code_occurence } => Self :: NoPrimaryKeys
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
            },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollbackError { .. } => todo!()
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
    NoPrimaryKeys {
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
    RowAndRollbackError {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
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
                                line : 7527, column : 13,
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
                    &value.std_primitive_bool_as_postgresql_bool,
                    &value.std_primitive_i16_as_postgresql_small_int,
                    &value.std_primitive_i32_as_postgresql_int,
                    &value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key,
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
                if let Some(value) =
                    &value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key
                {
                    if value.is_empty() {
                        let error = TryDeleteManyRouteLogicErrorNamed::NoPrimaryKeys {
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 5522,
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
    //
    let expected_deleted_primary_keys = parameters.payload.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key.clone();
    //
    let query_string = format!
    ("delete from dogs where {} returning sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key",
    {
        let mut increment : u64 = 0; let mut additional_parameters = std ::
        string :: String :: default(); if let Some(value) = &
        parameters.payload.std_primitive_bool_as_postgresql_bool
        {
            for element in value
            {
                match postgresql_crud :: BindQuery ::
                try_increment(element, & mut increment)
                {
                    Ok(_) =>
                    {
                        let handle = format!
                        ("std_primitive_bool_as_postgresql_bool = ${increment}");
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
        parameters.payload.std_primitive_i16_as_postgresql_small_int
        {
            for element in value
            {
                match postgresql_crud :: BindQuery ::
                try_increment(element, & mut increment)
                {
                    Ok(_) =>
                    {
                        let handle = format!
                        ("std_primitive_i16_as_postgresql_small_int = ${increment}");
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
        parameters.payload.std_primitive_i32_as_postgresql_int
        {
            for element in value
            {
                match postgresql_crud :: BindQuery ::
                try_increment(element, & mut increment)
                {
                    Ok(_) =>
                    {
                        let handle = format!
                        ("std_primitive_i32_as_postgresql_int = ${increment}");
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
        Some(sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key) = &
        parameters.payload.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key
        {
            if let false = additional_parameters.is_empty()
            { additional_parameters.push_str(" and"); }
            additional_parameters.push_str(& format!
            (" sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key in ({})",
            {
                let mut additional_parameters = std :: string :: String ::
                default(); for element in
                sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key
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
        if let Some(value) = parameters.payload.std_primitive_bool_as_postgresql_bool {
            for element in value {
                query = postgresql_crud::BindQuery::bind_value_to_query(element, query);
            }
        }
        if let Some(value) = parameters.payload.std_primitive_i16_as_postgresql_small_int {
            for element in value {
                query = postgresql_crud::BindQuery::bind_value_to_query(element, query);
            }
        }
        if let Some(value) = parameters.payload.std_primitive_i32_as_postgresql_int {
            for element in value {
                query = postgresql_crud::BindQuery::bind_value_to_query(element, query);
            }
        }
        if let Some(sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key) = parameters
            .payload
            .sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key
        {
            for element in sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key {
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
    //
    let mut postgres_transaction = match {
        use sqlx::Acquire;
        pg_connection.begin()
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
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TryDeleteManyRouteLogicResponseVariants::from(error),
            ));
            *res.status_mut() = axum::http::StatusCode::CREATED;
            return res;
        }
    };
    //
    //
    //std::vec::Vec::with_capacity(#expected_updated_primary_keys_name_token_stream.len());
    // enum RowError {
    //     SqlxError(sqlx::Error),
    //     Something,
    // }
    let mut option_row_error: Option<sqlx::Error> = None;
    let results_vec = {
        let mut results_vec = std::vec::Vec::new();
        let mut rows = binded_query.fetch(postgres_transaction.as_mut());
        {
            while let (Some(Some(row)), None) = (
                    match {
                        use futures::TryStreamExt;
                        rows.try_next()
                    }
                    .await
                {
                    Ok(value) => match value {
                        Some(value) => match sqlx::Row::try_get::<sqlx::types::uuid::Uuid, &std::primitive::str>(
                            &value,
                            "sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key",
                        ) {
                            Ok(value) => Some(Some(
                                postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                                    postgresql_crud::SqlxTypesUuidUuid(value),
                                )
                            )),
                            Err(error) => {
                                option_row_error = Some(error);
                                None
                            }
                        },
                        None => None,
                    },
                    Err(error) => {
                        option_row_error = Some(error);
                        None
                    }
                },
                &option_row_error,
            ) {
                results_vec.push(row);
            }
        }
        results_vec
    };
    if let Some(value) = option_row_error {
            match postgres_transaction.rollback().await {
                Ok(_) => {
                    let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                        postgresql: value,
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
            }
        // match (value, &postgres_transaction.rollback().await) {
        //     (
        //         RowError::SqlxError(value),
        //         Ok(_)
        //     ) => {
        //         let error = TryDeleteManyRouteLogicErrorNamed::
        //         // RowAndRollbackError {
        //         //     row: sqlx::Error,
        //         //     rollback: rollback_error,
        //         //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        //         // }

        //         Postgresql {
        //             postgresql: value,
        //             code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
        //                 file!().to_owned(),
        //                 line!(),
        //                 column!(),
        //                 Some(error_occurence_lib::code_occurence::MacroOccurence {
        //                     file: std::string::String::from(
        //                         "postgresql_crud/generate_postgresql_crud/src/lib.rs",
        //                     ),
        //                     line: 1735,
        //                     column: 21,
        //                 }),
        //             ),
        //         };
        //         eprintln!("{error}");
        //         let mut res = axum::response::IntoResponse::into_response(axum::Json(
        //             TryDeleteManyRouteLogicResponseVariants::from(error),
        //         ));
        //         *res.status_mut() = axum::http::StatusCode::CREATED;
        //         return res;
        //     },
        //     (
        //         RowError::SqlxError(value),
        //         Err(error)
        //     ) => todo!(),
        //     (
        //         RowError::Something,
        //         Ok(_)
        //     ) => todo!(),
        //     (
        //         RowError::Something,
        //         Err(error)
        //     ) => todo!(),
        // }
        }

    // let mut vec = vec![];
    // for element in results_vec {
    //     match sqlx::Row::try_get::<sqlx::types::uuid::Uuid, &std::primitive::str>(
    //         &element,
    //         "sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key",
    //     ) {
    //         Ok(value) => {
    //             vec.push(
    //                 postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
    //                     postgresql_crud::SqlxTypesUuidUuid(value),
    //                 )
    //             );
    //         },
    //         Err(error) => {
    //             todo!()
    //         }
    //     }
    // }
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        TryDeleteManyRouteLogicResponseVariants::Desirable(results_vec),
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
        TryDeleteManyRouteLogicResponseVariants :: NoPrimaryKeys
        { code_occurence } =>
        TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NoPrimaryKeys { code_occurence },
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
                    line: 7029,
                    column: 13,
                }),
            ),
        },
    )
}