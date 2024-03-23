#[derive(Debug, postgresql_crud::GeneratePostgresqlCrud)]
#[postgresql_crud::create_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::create_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::read_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::read_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::update_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::update_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::delete_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::delete_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::additional_http_status_codes_error_variants{
    #[path(crate::server::extractors::commit_extractor::)]
    enum CommitExtractorCheckErrorNamed {
        #[tvfrr_400_bad_request]
        CommitExtractorNotEqual {
            #[eo_display_with_serialize_deserialize]
            commit_not_equal: std::string::String,
            #[eo_display_with_serialize_deserialize]
            commit_to_use: std::string::String,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
        #[tvfrr_400_bad_request]
        CommitExtractorToStrConversion {
            #[eo_display]
            commit_to_str_conversion: http::header::ToStrError,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
        #[tvfrr_400_bad_request]
        NoCommitExtractorHeader {
            #[eo_display_with_serialize_deserialize]
            no_commit_header: std::string::String,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
    }
    // ;
    // enum SomethingErrorNamed {
    //     #[tvfrr_400_bad_request]
    //     SomethingVariant {
    //         #[eo_display_with_serialize_deserialize]
    //         something_field: std::string::String,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //     },
    // }
}]
pub struct Dog {
    pub id: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey, //todo remove UuidWrapper todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    // pub name: postgresql_crud::StdStringStringAsPostgresqlVarcharNotNull,
    // pub color: postgresql_crud::StdStringStringAsPostgresqlVarcharNotNull,

    // pub std_primitive_bool_as_postgresql_bool: postgresql_crud::StdPrimitiveBoolAsPostgresqlBool,
    // pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdPrimitiveBoolAsPostgresqlBoolNotNull,

    // pub std_primitive_i16_as_postgresql_small_int: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallInt,
    // pub std_primitive_i16_as_postgresql_small_int_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallIntNotNull,
    // pub std_primitive_i16_as_postgresql_small_serial: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerial,
    // pub std_primitive_i16_as_postgresql_small_serial_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlSmallSerialNotNull,
    // pub std_primitive_i16_as_postgresql_small_int2: postgresql_crud::StdPrimitiveI16AsPostgresqlInt2,
    // pub std_primitive_i16_as_postgresql_small_int2_not_null: postgresql_crud::StdPrimitiveI16AsPostgresqlInt2NotNull,

    // pub std_primitive_i32_as_postgresql_int: postgresql_crud::StdPrimitiveI32AsPostgresqlInt,
    // pub std_primitive_i32_as_postgresql_int_not_null: postgresql_crud::StdPrimitiveI32AsPostgresqlIntNotNull,
    // pub std_primitive_i32_as_postgresql_serial: postgresql_crud::StdPrimitiveI32AsPostgresqlSerial,
    // pub std_primitive_i32_as_postgresql_serial_not_null: postgresql_crud::StdPrimitiveI32AsPostgresqlSerialNotNull,
    // pub std_primitive_i32_as_postgresql_int4: postgresql_crud::StdPrimitiveI32AsPostgresqlInt4,
    // pub std_primitive_i32_as_postgresql_int4_not_null: postgresql_crud::StdPrimitiveI32AsPostgresqlInt4NotNull,

    // pub std_primitive_i64_as_postgresql_big_int: postgresql_crud::StdPrimitiveI64AsPostgresqlBigInt,
    // pub std_primitive_i64_as_postgresql_big_int_not_null: postgresql_crud::StdPrimitiveI64AsPostgresqlBigIntNotNull,
    // pub std_primitive_i64_as_postgresql_big_serial: postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerial,
    // pub std_primitive_i64_as_postgresql_big_serial_not_null: postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerialNotNull,
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
    // pub std_string_string_as_postgresql_name: postgresql_crud::StdStringStringAsPostgresqlName,
    // pub std_string_string_as_postgresql_name_not_null: postgresql_crud::StdStringStringAsPostgresqlNameNotNull,
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
    pub sqlx_types_time_time_as_postgresql_time_not_null: postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTimeNotNull,

    // pub sqlx_types_uuid_uuid_as_postgresql_uuid: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuid,
    // pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNull,
    // pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,//todo Primary Key support only for Uuid - its simplification. maybe later support something else but now i think uuid v7 is enough //fails too but primary key is a different logic. need refactor it as different task 

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

    // //todo what to do with generic?
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

////////////////////////////////////////////////////////////////////////

#[derive(Debug, utoipa :: ToSchema)]
pub struct DeleteManyPayload {
    pub id: std::option::Option<std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>>,
    pub sqlx_types_time_time_as_postgresql_time_not_null:
        std::option::Option<std::vec::Vec<postgresql_crud::WhereSqlxTypesTimeTime>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct DeleteManyPayloadWithSerializeDeserialize {
    id: std::option::Option<
        std::vec::Vec<postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize>,
    >,
    sqlx_types_time_time_as_postgresql_time_not_null: std::option::Option<
        std::vec::Vec<postgresql_crud::WhereSqlxTypesTimeTimeWithSerializeDeserialize>,
    >,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserializeErrorNamed {
    Id {
        #[eo_error_occurence]
        sqlx_types_uuid_uuid: postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SqlxTypesTimeTimeAsPostgresqlTimeNotNull {
        #[eo_error_occurence]
        sqlx_types_time_time: postgresql_crud::SqlxTypesTimeTimeWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DeleteManyPayloadWithSerializeDeserialize> for DeleteManyPayload {
    type Error = DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: DeleteManyPayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let id = match value.id {
            Some(value) => {
                let mut values = std::vec::Vec::with_capacity(value.len());
                for element in value {
                    match postgresql_crud::SqlxTypesUuidUuid::try_from(element) {
                        Ok(value) => {
                            values.push(value);
                        }
                        Err(e) => {
                            return
                            Err(Self :: Error :: Id
                            {
                                sqlx_types_uuid_uuid : e, code_occurence :
                                error_occurence_lib :: code_occurence :: CodeOccurence ::
                                new(file! ().to_string(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 6343, column : 37,
                                })),
                            }) ;
                        }
                    }
                }
                Some(values)
            }
            None => None,
        };
        let sqlx_types_time_time_as_postgresql_time_not_null =
            match value.sqlx_types_time_time_as_postgresql_time_not_null {
                Some(value) => {
                    let mut values = std::vec::Vec::with_capacity(value.len());
                    for element in value {
                        match postgresql_crud::WhereSqlxTypesTimeTime::try_from(element) {
                            Ok(value) => {
                                values.push(value);
                            }
                            Err(e) => {
                                return
                            Err(Self :: Error ::
                            SqlxTypesTimeTimeAsPostgresqlTimeNotNull
                            {
                                sqlx_types_time_time : e, code_occurence :
                                error_occurence_lib :: code_occurence :: CodeOccurence ::
                                new(file! ().to_string(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 8935, column : 17,
                                })),
                            }) ;
                            }
                        }
                    }
                    Some(values)
                }
                None => None,
            };
        Ok(Self {
            id,
            sqlx_types_time_time_as_postgresql_time_not_null,
        })
    }
}
impl std::convert::From<DeleteManyPayload> for DeleteManyPayloadWithSerializeDeserialize {
    fn from(value: DeleteManyPayload) -> Self {
        let id =
            match value.id {
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
        let sqlx_types_time_time_as_postgresql_time_not_null =
            postgresql_crud::SqlxTypesTimeTimeWithSerializeDeserialize::from(
                value.sqlx_types_time_time_as_postgresql_time_not_null,
            );
        Self {
            id,
            sqlx_types_time_time_as_postgresql_time_not_null,
        }
    }
}
#[derive(Debug)]
pub struct DeleteManyParameters {
    pub payload: DeleteManyPayload,
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryDeleteMany {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueIdVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_id_vec:
            std::vec::Vec<postgresql_crud::WhereSqlxTypesUuidUuidWithSerializeDeserialize>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueSqlxTypesTimeTimeAsPostgresqlTimeNotNullVec {
        #[eo_vec_display_with_serialize_deserialize]
        not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec:
            std::vec::Vec<postgresql_crud::WhereSqlxTypesTimeTimeWithSerializeDeserialize>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKeys {
        #[eo_vec_display]
        not_unique_primary_keys: std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        #[eo_display_with_serialize_deserialize]
        no_payload_fields: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadParameters {
        #[eo_display_with_serialize_deserialize]
        no_payload_parameters: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        #[eo_vec_display]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndFailedRollback {
        #[eo_vec_display]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PrimaryKeyFromRowAndFailedRollback {
        #[eo_display]
        primary_key_from_row: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitFailed {
        #[eo_display]
        commit_failed: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryAndRollbackFailed {
        #[eo_display]
        query_error: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize:
            DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        #[eo_display]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        #[eo_display]
        commit_to_str_conversion: http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryDeleteManyResponseVariants {
    Desirable(std :: vec :: Vec :: <
    postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize >),
    Configuration
    {
        configuration : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Database
    {
        database : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Tls
    {
        tls : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Protocol
    {
        protocol : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, RowNotFound
    {
        row_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, TypeNotFound
    {
        type_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize<>, len : usize<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std::string::String<>, source_handle :
        std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Decode
    {
        decode : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, PoolTimedOut
    {
        pool_timed_out : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, PoolClosed
    {
        pool_closed : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NotUniqueIdVec
    {
        not_unique_id_vec :
        std::vec::Vec<postgresql_crud::WhereSqlxTypesUuidUuidWithSerializeDeserialize<>>,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, NotUniqueSqlxTypesTimeTimeAsPostgresqlTimeNotNullVec
    {
        not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec :
        std::vec::Vec<postgresql_crud::WhereSqlxTypesTimeTimeWithSerializeDeserialize<>>,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, NotUniquePrimaryKeys
    {
        not_unique_primary_keys : std::vec::Vec<std::string::String>,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, BindQuery
    {
        bind_query :
        postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, NoPayloadFields
    {
        no_payload_fields : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NoPayloadParameters
    {
        no_payload_parameters : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NonExistingPrimaryKeys
    {
        non_existing_primary_keys : std::vec::Vec<std::string::String>,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, NonExistingPrimaryKeysAndFailedRollback
    {
        non_existing_primary_keys : std::vec::Vec<std::string::String>,
        rollback_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, PrimaryKeyFromRowAndFailedRollback
    {
        primary_key_from_row : std :: string :: String, rollback_error : std
        :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CommitFailed
    {
        commit_failed : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, QueryAndRollbackFailed
    {
        query_error : std :: string :: String, rollback_error : std :: string
        :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
    {
        delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize
        :
        DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorNotEqual
    {
        commit_not_equal : std::string::String<>, commit_to_use :
        std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorToStrConversion
    {
        commit_to_str_conversion : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NoCommitExtractorHeader
    {
        no_commit_header : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryDeleteMany> for TryDeleteManyResponseVariants {
    fn from(value: TryDeleteMany) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryDeleteManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: Io { io, code_occurence }
            => Self :: Io { io, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryDeleteManyWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryDeleteManyWithSerializeDeserialize
            :: JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: NotUniqueIdVec
            { not_unique_id_vec, code_occurence } => Self :: NotUniqueIdVec
            { not_unique_id_vec, code_occurence },
            TryDeleteManyWithSerializeDeserialize ::
            NotUniqueSqlxTypesTimeTimeAsPostgresqlTimeNotNullVec
            {
                not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec,
                code_occurence
            } => Self :: NotUniqueSqlxTypesTimeTimeAsPostgresqlTimeNotNullVec
            {
                not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec,
                code_occurence
            }, TryDeleteManyWithSerializeDeserialize :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence } => Self ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: NoPayloadParameters
            { no_payload_parameters, code_occurence } => Self ::
            NoPayloadParameters { no_payload_parameters, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryDeleteManyWithSerializeDeserialize ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Self :: NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence },
            TryDeleteManyWithSerializeDeserialize ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } => Self
            :: PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: CommitFailed
            { commit_failed, code_occurence } => Self :: CommitFailed
            { commit_failed, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } => Self ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence },
            TryDeleteManyWithSerializeDeserialize ::
            DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
            {
                delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
            {
                delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize,
                code_occurence
            }, TryDeleteManyWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }, TryDeleteManyWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryDeleteManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryDeleteManyWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } => Self ::
            NoCommitExtractorHeader { no_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryDeleteManyResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryDeleteManyResponseVariants) -> Self {
        match value
        {
            TryDeleteManyResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: Database
            { database : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryDeleteManyResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryDeleteManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryDeleteManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryDeleteManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: OK, TryDeleteManyResponseVariants
            :: Decode { decode : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryDeleteManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            NotUniqueIdVec { not_unique_id_vec : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            NotUniqueSqlxTypesTimeTimeAsPostgresqlTimeNotNullVec
            {
                not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteManyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            BindQuery { bind_query : _, code_occurence : _ } => axum :: http
            :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            NoPayloadFields { no_payload_fields : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            NoPayloadParameters
            { no_payload_parameters : _, code_occurence : _ } => axum :: http
            :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            {
                non_existing_primary_keys : _, rollback_error : _,
                code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteManyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            {
                primary_key_from_row : _, rollback_error : _, code_occurence :
                _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteManyResponseVariants :: CommitFailed
            { commit_failed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteManyResponseVariants ::
            QueryAndRollbackFailed
            { query_error : _, rollback_error : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
            {
                delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryDeleteManyResponseVariants ::
            NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteManyResponseVariantsTvfrr200Ok {
    Desirable(std::vec::Vec<postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize>),
}
impl std::convert::From<TryDeleteManyResponseVariantsTvfrr200Ok> for TryDeleteManyResponseVariants {
    fn from(value: TryDeleteManyResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryDeleteManyResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteManyResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteManyResponseVariantsTvfrr404NotFound>
    for TryDeleteManyResponseVariants
{
    fn from(value: TryDeleteManyResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryDeleteManyResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteManyResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PrimaryKeyFromRowAndFailedRollback {
        primary_key_from_row: std::string::String,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitFailed {
        commit_failed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryAndRollbackFailed {
        query_error: std::string::String,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteManyResponseVariantsTvfrr500InternalServerError>
    for TryDeleteManyResponseVariants
{
    fn from(value: TryDeleteManyResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            Migrate { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } => Self
            :: PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            CommitFailed { commit_failed, code_occurence } => Self ::
            CommitFailed { commit_failed, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } => Self ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence },
            TryDeleteManyResponseVariantsTvfrr500InternalServerError ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteManyResponseVariantsTvfrr400BadRequest {
    TypeNotFound
    {
        type_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NotUniqueIdVec
    {
        not_unique_id_vec :
        std::vec::Vec<postgresql_crud::WhereSqlxTypesUuidUuidWithSerializeDeserialize<>>,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, NotUniqueSqlxTypesTimeTimeAsPostgresqlTimeNotNullVec
    {
        not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec :
        std::vec::Vec<postgresql_crud::WhereSqlxTypesTimeTimeWithSerializeDeserialize<>>,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, NotUniquePrimaryKeys
    {
        not_unique_primary_keys : std::vec::Vec<std::string::String>,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, NoPayloadFields
    {
        no_payload_fields : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NoPayloadParameters
    {
        no_payload_parameters : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NonExistingPrimaryKeys
    {
        non_existing_primary_keys : std::vec::Vec<std::string::String>,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, NonExistingPrimaryKeysAndFailedRollback
    {
        non_existing_primary_keys : std::vec::Vec<std::string::String>,
        rollback_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
    {
        delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize
        :
        DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorNotEqual
    {
        commit_not_equal : std::string::String<>, commit_to_use :
        std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorToStrConversion
    {
        commit_to_str_conversion : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NoCommitExtractorHeader
    {
        no_commit_header : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryDeleteManyResponseVariantsTvfrr400BadRequest>
    for TryDeleteManyResponseVariants
{
    fn from(value: TryDeleteManyResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryDeleteManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest :: NotUniqueIdVec
            { not_unique_id_vec, code_occurence } => Self :: NotUniqueIdVec
            { not_unique_id_vec, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NotUniqueSqlxTypesTimeTimeAsPostgresqlTimeNotNullVec
            {
                not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec,
                code_occurence
            } => Self :: NotUniqueSqlxTypesTimeTimeAsPostgresqlTimeNotNullVec
            {
                not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec,
                code_occurence
            }, TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence }
            => Self :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NoPayloadParameters { no_payload_parameters, code_occurence } =>
            Self :: NoPayloadParameters
            { no_payload_parameters, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Self :: NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
            {
                delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
            {
                delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize,
                code_occurence
            }, TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryDeleteManyResponseVariantsTvfrr400BadRequest ::
            NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteManyResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteManyResponseVariantsTvfrr408RequestTimeout>
    for TryDeleteManyResponseVariants
{
    fn from(value: TryDeleteManyResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryDeleteManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
impl TryFrom<TryDeleteManyResponseVariants>
    for std::vec::Vec<postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize>
{
    type Error = TryDeleteManyWithSerializeDeserialize;
    fn try_from(value: TryDeleteManyResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryDeleteManyResponseVariants :: Desirable(i) => Ok(i),
            TryDeleteManyResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryDeleteManyResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: Database
            { database, code_occurence }), TryDeleteManyResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: Io
            { io, code_occurence }), TryDeleteManyResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryDeleteManyResponseVariants ::
            Protocol { protocol, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryDeleteManyResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryDeleteManyResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryDeleteManyResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryDeleteManyResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryDeleteManyResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryDeleteManyResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryDeleteManyWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryDeleteManyResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryDeleteManyResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryDeleteManyResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryDeleteManyResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryDeleteManyResponseVariants ::
            JsonDataError { json_data_error, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }),
            TryDeleteManyResponseVariants :: JsonSyntaxError
            { json_syntax_error, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryDeleteManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryDeleteManyResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }),
            TryDeleteManyResponseVariants :: UnexpectedCase
            { unexpected_case, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }),
            TryDeleteManyResponseVariants :: NotUniqueIdVec
            { not_unique_id_vec, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: NotUniqueIdVec
            { not_unique_id_vec, code_occurence }),
            TryDeleteManyResponseVariants ::
            NotUniqueSqlxTypesTimeTimeAsPostgresqlTimeNotNullVec
            {
                not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec,
                code_occurence
            } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            NotUniqueSqlxTypesTimeTimeAsPostgresqlTimeNotNullVec
            {
                not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec,
                code_occurence
            }), TryDeleteManyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence }),
            TryDeleteManyResponseVariants :: BindQuery
            { bind_query, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }), TryDeleteManyResponseVariants ::
            NoPayloadFields { no_payload_fields, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence }),
            TryDeleteManyResponseVariants :: NoPayloadParameters
            { no_payload_parameters, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: NoPayloadParameters
            { no_payload_parameters, code_occurence }),
            TryDeleteManyResponseVariants :: NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence }),
            TryDeleteManyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence }),
            TryDeleteManyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence }),
            TryDeleteManyResponseVariants :: CommitFailed
            { commit_failed, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize :: CommitFailed
            { commit_failed, code_occurence }), TryDeleteManyResponseVariants
            :: QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence }),
            TryDeleteManyResponseVariants ::
            DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
            {
                delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
            {
                delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize,
                code_occurence
            }), TryDeleteManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }), TryDeleteManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence }),
            TryDeleteManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence }),
            TryDeleteManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } =>
            Err(TryDeleteManyWithSerializeDeserialize ::
            NoCommitExtractorHeader { no_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteManyRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryDeleteManyWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub enum TryDeleteManyStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    NotUniqueIdVecTvfrr400BadRequest,
    NotUniqueSqlxTypesTimeTimeAsPostgresqlTimeNotNullVecTvfrr400BadRequest,
    NotUniquePrimaryKeysTvfrr400BadRequest,
    BindQueryTvfrr500InternalServerError,
    NoPayloadFieldsTvfrr400BadRequest,
    NoPayloadParametersTvfrr400BadRequest,
    NonExistingPrimaryKeysTvfrr400BadRequest,
    NonExistingPrimaryKeysAndFailedRollbackTvfrr400BadRequest,
    PrimaryKeyFromRowAndFailedRollbackTvfrr500InternalServerError,
    CommitFailedTvfrr500InternalServerError,
    QueryAndRollbackFailedTvfrr500InternalServerError,
    DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserializeTvfrr400BadRequest,
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryDeleteManyResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryDeleteManyResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            } TryDeleteManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: NotUniqueIdVec
            { not_unique_id_vec : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants ::
            NotUniqueSqlxTypesTimeTimeAsPostgresqlTimeNotNullVec
            {
                not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: NoPayloadFields
            { no_payload_fields : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: NoPayloadParameters
            { no_payload_parameters : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: NonExistingPrimaryKeys
            { non_existing_primary_keys : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            {
                non_existing_primary_keys : _, rollback_error : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            {
                primary_key_from_row : _, rollback_error : _, code_occurence :
                _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: CommitFailed
            { commit_failed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: QueryAndRollbackFailed
            { query_error : _, rollback_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants ::
            DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
            {
                delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteManyErrorNamed {
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient {
        #[eo_vec_error_occurence]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client:
            std::vec::Vec<
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed,
            >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryDeleteManyWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_many<'a>(
    server_location: &str,
    parameters: DeleteManyParameters,
) -> Result<std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>, TryDeleteManyErrorNamed> {
    let payload = match serde_json::to_string(&DeleteManyPayloadWithSerializeDeserialize::from(
        parameters.payload,
    )) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryDeleteManyErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1674,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let url = format!("{}/dogs/delete_many", server_location,);
    let future = reqwest::Client::new()
        .delete(&url)
        .header(postgresql_crud::COMMIT, git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            return Err(TryDeleteManyErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2376,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(response_text) => response_text,
        Err(e) => {
            return Err(TryDeleteManyErrorNamed::FailedToGetResponseText {
                reqwest: e,
                status_code,
                headers,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2305,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let variants = if status_code == http::StatusCode::OK {
        match serde_json::from_str::<TryDeleteManyResponseVariantsTvfrr200Ok>(&response_text) {
            Ok(value) => TryDeleteManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryDeleteManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2342,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match serde_json::from_str::<TryDeleteManyResponseVariantsTvfrr408RequestTimeout>(
            &response_text,
        ) {
            Ok(value) => TryDeleteManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryDeleteManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2342,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match serde_json::from_str::<TryDeleteManyResponseVariantsTvfrr400BadRequest>(
            &response_text,
        ) {
            Ok(value) => TryDeleteManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryDeleteManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2342,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match serde_json::from_str::<TryDeleteManyResponseVariantsTvfrr500InternalServerError>(
            &response_text,
        ) {
            Ok(value) => TryDeleteManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryDeleteManyErrorNamed::DeserializeResponse {
                    serde: e,
                    status_code,
                    headers,
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 2342,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else {
        return Err(TryDeleteManyErrorNamed::UnexpectedStatusCode {
            status_code,
            headers,
            response_text_result:
                crate::common::api_request_unexpected_error::ResponseTextResult::ResponseText(
                    response_text,
                ),
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_string(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(
                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                    ),
                    line: 2270,
                    column: 13,
                }),
            ),
        });
    };
    match std::vec::Vec::<postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize>::try_from(
        variants,
    ) {
        Ok(value) => {
            let mut vec_values = std::vec::Vec::with_capacity(value.len());
            let mut vec_errors = std::vec::Vec::with_capacity(value.len());
            for element in value {
                match postgresql_crud::SqlxTypesUuidUuid::try_from(element) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        vec_errors.push(OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed
                        ::
                        OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient(e))
                        ;
                    }
                }
            }
            if let false = vec_errors.is_empty() {
                return
                Err(TryDeleteManyErrorNamed ::
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient
                {
                    operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client
                    : vec_errors, code_occurence : error_occurence_lib ::
                    code_occurence :: CodeOccurence ::
                    new(file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 1981, column : 13,
                    }))
                }) ;
            }
            Ok(vec_values)
        }
        Err(e) => {
            return Err(TryDeleteManyErrorNamed::ExpectedType {
                expected_type: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2232,
                        column: 13,
                    }),
                ),
            });
        }
    }
}
#[utoipa ::
path(delete, path = "/dogs/delete_many", operation_id = "/dogs/delete_many",
tag = "dogs",
request_body(content = DeleteManyPayloadWithSerializeDeserialize, description
= "dogs delete_many payload", content_type = "application/json"),
responses((status = 200, description = "ok", body =
TryDeleteManyResponseVariantsTvfrr200Ok, content_type = "application/json"),
(status = 500, description = "internal server error", body =
TryDeleteManyResponseVariantsTvfrr500InternalServerError, content_type =
"application/json"),
(status = 404, description = "not found", body =
TryDeleteManyResponseVariantsTvfrr404NotFound, content_type =
"application/json"),
(status = 400, description = "bad request", body =
TryDeleteManyResponseVariantsTvfrr400BadRequest, content_type =
"application/json"),
(status = 408, description = "request timeout", body =
TryDeleteManyResponseVariantsTvfrr408RequestTimeout, content_type =
"application/json")),)]
pub async fn delete_many<'a>(
    app_state: axum::extract::State<
        postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
    >,
    payload_extraction_result: Result<
        axum::Json<DeleteManyPayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = DeleteManyParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                DeleteManyPayloadWithSerializeDeserialize,
                TryDeleteManyResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_state)
            {
                Ok(value) => match DeleteManyPayload::try_from(value) {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryDeleteMany ::
                    DeleteManyPayloadTryFromDeleteManyPayloadWithSerializeDeserialize
                    {
                        delete_many_payload_try_from_delete_many_payload_with_serialize_deserialize
                        : e, code_occurence : error_occurence_lib :: code_occurence
                        :: CodeOccurence ::
                        new(file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 7024, column : 17,
                        })),
                    } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryDeleteManyResponseVariants::from(e);
                    }
                },
                Err(e) => {
                    return e;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        if let (None, None) = (
            &parameters.payload.id,
            &parameters
                .payload
                .sqlx_types_time_time_as_postgresql_time_not_null,
        ) {
            return TryDeleteManyResponseVariants::NoPayloadFields {
                no_payload_fields: std::string::String::from("no payload fields"),
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/check_for_none.rs",
                        ),
                        line: 49,
                        column: 13,
                    }),
                ),
            };
        }
        match (
            &parameters.payload.id,
            &parameters
                .payload
                .sqlx_types_time_time_as_postgresql_time_not_null,
        ) {
            (Some(id), None) => {
                let not_unique_primary_keys = {
                    let mut vec = std::vec::Vec::with_capacity(id.len());
                    let mut not_unique_primary_keys = std::vec::Vec::with_capacity(id.len());
                    for element in id {
                        let handle = element;
                        match vec.contains(&handle) {
                            true => {
                                not_unique_primary_keys.push(element.clone());
                            }
                            false => {
                                vec.push(element);
                            }
                        }
                    }
                    not_unique_primary_keys
                };
                if let false = not_unique_primary_keys.is_empty() {
                    let e = TryDeleteMany::NotUniquePrimaryKeys {
                        not_unique_primary_keys,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_string(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from(
                                    "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                ),
                                line: 1898,
                                column: 13,
                            }),
                        ),
                    };
                    error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                    return TryDeleteManyResponseVariants::from(e);
                }
                let expected_updated_primary_keys = {
                    id.iter()
                        .map(|element| element.clone())
                        .collect::<std::vec::Vec<postgresql_crud::SqlxTypesUuidUuid>>()
                };
                let binded_query = {
                    let query_string =
                        { "delete from dogs where id in (select unnest($1)) returning id" };
                    println!("{}", query_string);
                    let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                    query = query.bind(
                        id.into_iter()
                            .map(|element| element.clone().into_inner())
                            .collect::<std::vec::Vec<sqlx::types::Uuid>>(),
                    );
                    query
                };
                let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryDeleteMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryDeleteManyResponseVariants::from(e);
                    }
                };
                let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryDeleteMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryDeleteManyResponseVariants::from(e);
                    }
                };
                let mut postgres_transaction = match {
                    use sqlx::Acquire;
                    pg_connection.begin()
                }
                .await
                {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryDeleteMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryDeleteManyResponseVariants::from(e);
                    }
                };
                let results_vec = {
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
                                Err(e) => {
                                    option_error = Some(e);
                                    None
                                }
                            },
                            &option_error,
                        ) {
                            results_vec.push(row);
                        }
                    }
                    if let Some(e) = option_error {
                        match postgres_transaction.rollback().await {
                            Ok(_) => {
                                let e = TryDeleteMany::from(e);
                                error_occurence_lib::error_log::ErrorLog::error_log(
                                    &e,
                                    app_state.as_ref(),
                                );
                                return TryDeleteManyResponseVariants::from(e);
                            }
                            Err(rollback_error) => {
                                let e = TryDeleteMany :: QueryAndRollbackFailed
                                {
                                    query_error : e, rollback_error, code_occurence :
                                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                                    new(file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                        line : 38, column : 13,
                                    })),
                                } ;
                                error_occurence_lib::error_log::ErrorLog::error_log(
                                    &e,
                                    app_state.as_ref(),
                                );
                                return TryDeleteManyResponseVariants::from(e);
                            }
                        }
                    }
                    results_vec
                };
                let primary_key_vec = {
                    let mut primary_key_vec =
                        std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
                    for element in results_vec {
                        match primary_key_uuid_wrapper_try_from_sqlx_row(&element) {
                            Ok(primary_key) => {
                                primary_key_vec.push(primary_key);
                            }
                            Err(e) => match postgres_transaction.rollback().await {
                                Ok(_) => {
                                    let e = TryDeleteMany::from(e);
                                    error_occurence_lib::error_log::ErrorLog::error_log(
                                        &e,
                                        app_state.as_ref(),
                                    );
                                    return TryDeleteManyResponseVariants::from(e);
                                }
                                Err(rollback_error) => {
                                    let e = TryDeleteMany :: PrimaryKeyFromRowAndFailedRollback
                                    {
                                        primary_key_from_row : e, rollback_error, code_occurence :
                                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                                        new(file! ().to_string(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                            line : 53, column : 13,
                                        })),
                                    } ;
                                    error_occurence_lib::error_log::ErrorLog::error_log(
                                        &e,
                                        app_state.as_ref(),
                                    );
                                    return TryDeleteManyResponseVariants::from(e);
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
                                let e = TryDeleteMany :: NonExistingPrimaryKeys
                                {
                                    non_existing_primary_keys, code_occurence :
                                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                                    new(file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                        line : 68, column : 13,
                                    })),
                                } ;
                                error_occurence_lib::error_log::ErrorLog::error_log(
                                    &e,
                                    app_state.as_ref(),
                                );
                                return TryDeleteManyResponseVariants::from(e);
                            }
                            Err(e) => {
                                let e = TryDeleteMany ::
                                NonExistingPrimaryKeysAndFailedRollback
                                {
                                    non_existing_primary_keys, rollback_error : e,
                                    code_occurence : error_occurence_lib :: code_occurence ::
                                    CodeOccurence ::
                                    new(file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                        line : 82, column : 13,
                                    })),
                                } ;
                                error_occurence_lib::error_log::ErrorLog::error_log(
                                    &e,
                                    app_state.as_ref(),
                                );
                                return TryDeleteManyResponseVariants::from(e);
                            }
                        }
                    }
                }
                match postgres_transaction.commit().await {
                    Ok(_) => TryDeleteManyResponseVariants::Desirable(
                        primary_key_vec
                            .into_iter()
                            .map(|element| {
                                postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                                    element,
                                )
                            })
                            .collect(),
                    ),
                    Err(e) => {
                        let e = TryDeleteMany :: CommitFailed
                        {
                            commit_failed : e, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_string(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                line : 97, column : 13,
                            })),
                        } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        TryDeleteManyResponseVariants::from(e)
                    }
                }
            }
            _ => {
                if let Some(id) = &parameters.payload.id {
                    let not_unique_primary_keys = {
                        let mut vec = std::vec::Vec::with_capacity(id.len());
                        let mut not_unique_primary_keys = std::vec::Vec::with_capacity(id.len());
                        for element in id {
                            let handle = element;
                            match vec.contains(&handle) {
                                true => {
                                    not_unique_primary_keys.push(element.clone());
                                }
                                false => {
                                    vec.push(element);
                                }
                            }
                        }
                        not_unique_primary_keys
                    };
                    if let false = not_unique_primary_keys.is_empty() {
                        let e = TryDeleteMany::NotUniquePrimaryKeys {
                            not_unique_primary_keys,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_string(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from(
                                        "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                                    ),
                                    line: 1898,
                                    column: 13,
                                }),
                            ),
                        };
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryDeleteManyResponseVariants::from(e);
                    }
                }
                let sqlx_types_time_time_as_postgresql_time_not_null_handle = match parameters
                    .payload
                    .sqlx_types_time_time_as_postgresql_time_not_null
                {
                    Some(value) => {
                        let is_unique = {
                            let mut vec = std::vec::Vec::with_capacity(value.len());
                            let mut is_unique = true;
                            for element in &value {
                                match vec.contains(&element) {
                                    true => {
                                        is_unique = false;
                                        break;
                                    }
                                    false => {
                                        vec.push(element);
                                    }
                                }
                            }
                            is_unique
                        };
                        match is_unique {
                            true => Some(value),
                            false => {
                                let not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec = {
                                    let mut vec = std::vec::Vec::with_capacity(value.len());
                                    let mut
                                    not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec =
                                        std::vec::Vec::with_capacity(value.len());
                                    for element in value {
                                        match vec.contains(&element) {
                                            true => {
                                                not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec.push(element)
                                                ;
                                            }
                                            false => {
                                                vec.push(element);
                                            }
                                        }
                                    }
                                    not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec.into_iter().map(|
                                    element |
                                    postgresql_crud::WhereSqlxTypesTimeTimeWithSerializeDeserialize
                                    :: from(element)).collect()
                                };
                                let e = TryDeleteMany ::
                                NotUniqueSqlxTypesTimeTimeAsPostgresqlTimeNotNullVec
                                {
                                    not_unique_sqlx_types_time_time_as_postgresql_time_not_null_vec,
                                    code_occurence : error_occurence_lib :: code_occurence ::
                                    CodeOccurence ::
                                    new(file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 6757, column : 33,
                                    })),
                                } ;
                                error_occurence_lib::error_log::ErrorLog::error_log(
                                    &e,
                                    app_state.as_ref(),
                                );
                                return TryDeleteManyResponseVariants::from(e);
                            }
                        }
                    }
                    None => None,
                };
                let query_string = {
                    format!("delete from dogs where {} returning id", {
                        let mut increment: u64 = 0;
                        let mut additional_parameters = std::string::String::default();
                        if let Some(value) =
                            &sqlx_types_time_time_as_postgresql_time_not_null_handle
                        {
                            for element in value {
                                match postgresql_crud::BindQuery::try_increment(
                                    element,
                                    &mut increment,
                                ) {
                                    Ok(_) => {
                                        let handle = format!
                                        ("sqlx_types_time_time_as_postgresql_time_not_null = ${increment}")
                                        ;
                                        match additional_parameters.is_empty() {
                                            true => {
                                                additional_parameters.push_str(&handle);
                                            }
                                            false => {
                                                additional_parameters
                                                    .push_str(&format!(" AND {handle}"));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        return TryDeleteManyResponseVariants :: BindQuery
                                        {
                                            bind_query : e.into_serialize_deserialize_version(),
                                            code_occurence : error_occurence_lib :: code_occurence ::
                                            CodeOccurence ::
                                            new(file! ().to_string(), line! (), column! (),
                                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                            {
                                                file : std :: string :: String ::
                                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line : 1754, column : 13,
                                            }))
                                        } ;
                                    }
                                }
                            }
                        }
                        if let Some(id) = &parameters.payload.id {
                            if let false = additional_parameters.is_empty() {
                                additional_parameters.push_str(" and");
                            }
                            additional_parameters.push_str(& format!
                            (" id in ({})",
                            {
                                let mut additional_parameters = std :: string :: String ::
                                default() ; for element in id
                                {
                                    match postgresql_crud :: BindQuery ::
                                    try_increment(element, & mut increment,)
                                    {
                                        Ok(_) =>
                                        {
                                            additional_parameters.push_str(& format! ("${increment},"))
                                            ;
                                        } Err(e) =>
                                        {
                                            return TryDeleteManyResponseVariants :: BindQuery
                                            {
                                                bind_query : e.into_serialize_deserialize_version(),
                                                code_occurence : error_occurence_lib :: code_occurence ::
                                                CodeOccurence ::
                                                new(file! ().to_string(), line! (), column! (),
                                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                                {
                                                    file : std :: string :: String ::
                                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                    line : 1754, column : 13,
                                                }))
                                            } ;
                                        }
                                    }
                                } additional_parameters.pop() ; additional_parameters
                            })) ;
                        }
                        additional_parameters
                    })
                };
                println!("{}", query_string);
                let binded_query = {
                    let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                    if let Some(value) = sqlx_types_time_time_as_postgresql_time_not_null_handle {
                        for element in value {
                            query = postgresql_crud::BindQuery::bind_value_to_query(element, query);
                        }
                    }
                    if let Some(id) = parameters.payload.id {
                        for element in id {
                            query = postgresql_crud::BindQuery::bind_value_to_query(element, query);
                        }
                    }
                    query
                };
                let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryDeleteMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryDeleteManyResponseVariants::from(e);
                    }
                };
                let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryDeleteMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryDeleteManyResponseVariants::from(e);
                    }
                };
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
                        Err(e) => {
                            let e = TryDeleteMany::from(e);
                            error_occurence_lib::error_log::ErrorLog::error_log(
                                &e,
                                app_state.as_ref(),
                            );
                            return TryDeleteManyResponseVariants::from(e);
                        }
                    }
                } {
                    match {
                        use sqlx::Row;
                        row.try_get::<sqlx::types::uuid::Uuid, &str>("id")
                    } {
                        Ok(value) => {
                            vec_values.push(
                                postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                                    postgresql_crud::SqlxTypesUuidUuid(value),
                                ),
                            );
                        }
                        Err(e) => {
                            let e = TryDeleteMany ::
                            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
                            {
                                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                                : e, code_occurence : error_occurence_lib :: code_occurence
                                :: CodeOccurence ::
                                new(file! ().to_string(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1960, column : 13,
                                })),
                            } ;
                            error_occurence_lib::error_log::ErrorLog::error_log(
                                &e,
                                app_state.as_ref(),
                            );
                            return TryDeleteManyResponseVariants::from(e);
                        }
                    }
                }
                TryDeleteManyResponseVariants::Desirable(vec_values)
            }
        }
    }
}
impl std::convert::From<crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed>
    for TryDeleteMany
{
    fn from(
        value: crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
