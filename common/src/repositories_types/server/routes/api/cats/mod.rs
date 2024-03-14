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
    pub name: postgresql_crud::StdStringStringAsPostgresqlVarcharNotNull,
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
    // pub sqlx_types_time_time_as_postgresql_time_not_null: postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTimeNotNull,

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
//////////////

#[derive(Debug, utoipa :: ToSchema)]
pub struct CreateOnePayload {
    pub name: postgresql_crud::StdStringString,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateOnePayloadWithSerializeDeserialize {
    pub name: postgresql_crud::StdStringStringWithSerializeDeserialize,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserializeErrorNamed {
    NotUuid {
        #[eo_display_with_serialize_deserialize]
        not_uuid: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<CreateOnePayloadWithSerializeDeserialize> for CreateOnePayload {
    type Error = CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: CreateOnePayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let name = postgresql_crud::StdStringString::from(value.name);
        Ok(Self { name })
    }
}
impl std::convert::From<CreateOnePayload> for CreateOnePayloadWithSerializeDeserialize {
    fn from(value: CreateOnePayload) -> Self {
        let name = postgresql_crud::StdStringStringWithSerializeDeserialize::from(value.name);
        Self { name }
    }
}
#[derive(Debug)]
pub struct CreateOneParameters {
    pub payload: CreateOnePayload,
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryCreateOne {
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
    CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        create_one_payload_try_from_create_one_payload_with_serialize_deserialize:
            CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserializeErrorNamed,
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
pub enum TryCreateOneResponseVariants {
    Desirable(postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize),
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
    }, CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserialize
    {
        create_one_payload_try_from_create_one_payload_with_serialize_deserialize
        :
        CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
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
impl std::convert::From<TryCreateOne> for TryCreateOneResponseVariants {
    fn from(value: TryCreateOne) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryCreateOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence }, TryCreateOneWithSerializeDeserialize
            :: Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence }, TryCreateOneWithSerializeDeserialize
            :: RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryCreateOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryCreateOneWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryCreateOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryCreateOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryCreateOneWithSerializeDeserialize
            :: JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryCreateOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryCreateOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryCreateOneWithSerializeDeserialize ::
            CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserialize
            {
                create_one_payload_try_from_create_one_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserialize
            {
                create_one_payload_try_from_create_one_payload_with_serialize_deserialize,
                code_occurence
            }, TryCreateOneWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }, TryCreateOneWithSerializeDeserialize :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryCreateOneWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryCreateOneWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } => Self ::
            NoCommitExtractorHeader { no_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryCreateOneResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryCreateOneResponseVariants) -> Self {
        match value
        {
            TryCreateOneResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants ::
            Configuration { configuration : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            Database { database : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            CREATED, TryCreateOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            CREATED, TryCreateOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: CREATED, TryCreateOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants ::
            TypeNotFound { type_not_found : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateOneResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: CREATED,
            TryCreateOneResponseVariants :: Decode
            { decode : _, code_occurence : _ } => axum :: http :: StatusCode
            :: CREATED, TryCreateOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants ::
            WorkerCrashed { worker_crashed : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            Migrate { migrate : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateOneResponseVariants ::
            JsonDataError { json_data_error : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            JsonSyntaxError { json_syntax_error : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserialize
            {
                create_one_payload_try_from_create_one_payload_with_serialize_deserialize
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: CREATED,
            TryCreateOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: CREATED,
            TryCreateOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateOneResponseVariants ::
            NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateOneResponseVariantsTvfrr201Created {
    Desirable(postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize),
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr201Created>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr201Created) -> Self {
        match value {
            TryCreateOneResponseVariantsTvfrr201Created::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateOneResponseVariantsTvfrr500InternalServerError {
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
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr500InternalServerError>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryCreateOneResponseVariantsTvfrr500InternalServerError ::
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
pub enum TryCreateOneResponseVariantsTvfrr400BadRequest {
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
    }, CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserialize
    {
        create_one_payload_try_from_create_one_payload_with_serialize_deserialize
        :
        CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
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
impl std::convert::From<TryCreateOneResponseVariantsTvfrr400BadRequest>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryCreateOneResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateOneResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateOneResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryCreateOneResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateOneResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateOneResponseVariantsTvfrr400BadRequest ::
            CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserialize
            {
                create_one_payload_try_from_create_one_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserialize
            {
                create_one_payload_try_from_create_one_payload_with_serialize_deserialize,
                code_occurence
            }, TryCreateOneResponseVariantsTvfrr400BadRequest ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryCreateOneResponseVariantsTvfrr400BadRequest ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryCreateOneResponseVariantsTvfrr400BadRequest ::
            NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateOneResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr408RequestTimeout>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryCreateOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateOneResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneResponseVariantsTvfrr404NotFound>
    for TryCreateOneResponseVariants
{
    fn from(value: TryCreateOneResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryCreateOneResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
impl TryFrom<TryCreateOneResponseVariants>
    for postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize
{
    type Error = TryCreateOneWithSerializeDeserialize;
    fn try_from(value: TryCreateOneResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryCreateOneResponseVariants :: Desirable(i) => Ok(i),
            TryCreateOneResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryCreateOneResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Database
            { database, code_occurence }), TryCreateOneResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Io
            { io, code_occurence }), TryCreateOneResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryCreateOneResponseVariants :: Protocol
            { protocol, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryCreateOneResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryCreateOneResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryCreateOneResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryCreateOneResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryCreateOneResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryCreateOneResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryCreateOneWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryCreateOneResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryCreateOneResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryCreateOneResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryCreateOneResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryCreateOneResponseVariants ::
            JsonDataError { json_data_error, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }), TryCreateOneResponseVariants
            :: JsonSyntaxError { json_syntax_error, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryCreateOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryCreateOneResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }), TryCreateOneResponseVariants
            :: UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }), TryCreateOneResponseVariants
            :: CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserialize
            {
                create_one_payload_try_from_create_one_payload_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryCreateOneWithSerializeDeserialize ::
            CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserialize
            {
                create_one_payload_try_from_create_one_payload_with_serialize_deserialize,
                code_occurence
            }), TryCreateOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } =>
            Err(TryCreateOneWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }), TryCreateOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence }),
            TryCreateOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence }),
            TryCreateOneResponseVariants :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } =>
            Err(TryCreateOneWithSerializeDeserialize ::
            NoCommitExtractorHeader { no_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateOneRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryCreateOneWithSerializeDeserialize,
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
pub enum TryCreateOneStatusCodesChecker {
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
    CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserializeTvfrr400BadRequest,
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryCreateOneResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryCreateOneResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            } TryCreateOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants ::
            CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserialize
            {
                create_one_payload_try_from_create_one_payload_with_serialize_deserialize
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateOneResponseVariants :: NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateOneErrorNamed {
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient {
        #[eo_error_occurence]
        uuid_wrapper_try_from_possible_uuid_wrapper_in_client:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryCreateOneWithSerializeDeserialize,
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
pub async fn try_create_one<'a>(
    server_location: &str,
    parameters: CreateOneParameters,
) -> Result<postgresql_crud::SqlxTypesUuidUuid, TryCreateOneErrorNamed> {
    let payload = match serde_json::to_string(&CreateOnePayloadWithSerializeDeserialize::from(
        parameters.payload,
    )) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryCreateOneErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1662,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let url = format!("{}/dogs/create_one", server_location);
    let future = reqwest::Client::new()
        .post(&url)
        .header(postgresql_crud::COMMIT, git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            return Err(TryCreateOneErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2359,
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
            return Err(TryCreateOneErrorNamed::FailedToGetResponseText {
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
                        line: 2288,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let variants = if status_code == http::StatusCode::CREATED {
        match serde_json::from_str::<TryCreateOneResponseVariantsTvfrr201Created>(&response_text) {
            Ok(value) => TryCreateOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryCreateOneErrorNamed::DeserializeResponse {
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
                            line: 2325,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match serde_json::from_str::<TryCreateOneResponseVariantsTvfrr408RequestTimeout>(
            &response_text,
        ) {
            Ok(value) => TryCreateOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryCreateOneErrorNamed::DeserializeResponse {
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
                            line: 2325,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match serde_json::from_str::<TryCreateOneResponseVariantsTvfrr500InternalServerError>(
            &response_text,
        ) {
            Ok(value) => TryCreateOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryCreateOneErrorNamed::DeserializeResponse {
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
                            line: 2325,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match serde_json::from_str::<TryCreateOneResponseVariantsTvfrr404NotFound>(&response_text) {
            Ok(value) => TryCreateOneResponseVariants::from(value),
            Err(e) => {
                return Err(TryCreateOneErrorNamed::DeserializeResponse {
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
                            line: 2325,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else {
        return Err(TryCreateOneErrorNamed::UnexpectedStatusCode {
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
                    line: 2253,
                    column: 13,
                }),
            ),
        });
    };
    match postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize ::
    try_from(variants)
    {
        Ok(value) => match postgresql_crud::SqlxTypesUuidUuid ::
        try_from(value)
        {
            Ok(value) => Ok(value), Err(e) =>
            Err(TryCreateOneErrorNamed ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient
            {
                uuid_wrapper_try_from_possible_uuid_wrapper_in_client : e,
                code_occurence : error_occurence_lib :: code_occurence ::
                CodeOccurence ::
                new(file! ().to_string(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1993, column : 13,
                })),
            })
        }, Err(e) =>
        {
            return
            Err(TryCreateOneErrorNamed :: ExpectedType
            {
                expected_type : e, code_occurence : error_occurence_lib ::
                code_occurence :: CodeOccurence ::
                new(file! ().to_string(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 2215, column : 13,
                }))
            }) ;
        },
    }
}
#[utoipa ::
path(post, path = "/dogs/create_one", operation_id = "/dogs/create_one", tag =
"dogs",
request_body(content = CreateOnePayloadWithSerializeDeserialize, description =
"dogs create_one payload", content_type = "application/json"),
responses((status = 201, description = "created", body =
TryCreateOneResponseVariantsTvfrr201Created, content_type =
"application/json"),
(status = 500, description = "internal server error", body =
TryCreateOneResponseVariantsTvfrr500InternalServerError, content_type =
"application/json"),
(status = 404, description = "not found", body =
TryCreateOneResponseVariantsTvfrr404NotFound, content_type =
"application/json"),
(status = 400, description = "bad request", body =
TryCreateOneResponseVariantsTvfrr400BadRequest, content_type =
"application/json"),
(status = 408, description = "request timeout", body =
TryCreateOneResponseVariantsTvfrr408RequestTimeout, content_type =
"application/json")),)]
pub async fn create_one(
    app_state: axum::extract::State<
        postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
    >,
    payload_extraction_result: Result<
        axum::Json<CreateOnePayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = CreateOneParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                CreateOnePayloadWithSerializeDeserialize,
                TryCreateOneResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_state)
            {
                Ok(value) => match CreateOnePayload::try_from(value) {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryCreateOne ::
                    CreateOnePayloadTryFromCreateOnePayloadWithSerializeDeserialize
                    {
                        create_one_payload_try_from_create_one_payload_with_serialize_deserialize
                        : e, code_occurence : error_occurence_lib :: code_occurence
                        :: CodeOccurence ::
                        new(file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 3550, column : 17,
                        })),
                    } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryCreateOneResponseVariants::from(e);
                    }
                },
                Err(e) => {
                    return e;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        let query_string = { "insert into dogs(name) values ($1) returning id" };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.name, query);
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let e = TryCreateOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryCreateOneResponseVariants::from(e);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let e = TryCreateOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryCreateOneResponseVariants::from(e);
            }
        };
        match binded_query.fetch_one(pg_connection.as_mut()).await {
            Ok(value) => match {
                use sqlx::Row;
                value.try_get::<sqlx::types::uuid::Uuid, &str>("id")
            } {
                Ok(value) => TryCreateOneResponseVariants::Desirable(
                    postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize::from(
                        postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize(value),
                    ),
                ),
                Err(e) => {
                    let e = TryCreateOne ::
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
                            line : 1949, column : 13,
                        })),
                    } ;
                    error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                    return TryCreateOneResponseVariants::from(e);
                }
            },
            Err(e) => {
                let e = TryCreateOne::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryCreateOneResponseVariants::from(e);
            }
        }
    }
}
impl std::convert::From<crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed>
    for TryCreateOne
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
