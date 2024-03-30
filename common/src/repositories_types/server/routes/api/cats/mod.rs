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
    // pub id: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey, //todo remove UuidWrapper todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    // pub name: postgresql_crud::StdStringStringAsPostgresqlVarcharNotNull,
    // pub color: postgresql_crud::StdStringStringAsPostgresqlVarcharNotNull,

    pub std_primitive_bool_as_postgresql_bool: postgresql_crud::StdPrimitiveBoolAsPostgresqlBool,
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
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
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

////////////////////////////////////////////////////////////////////////

#[derive(Debug, utoipa :: ToSchema)]
pub struct UpdateManyPayloadElement {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64,
    pub std_primitive_bool_as_postgresql_bool: postgresql_crud::StdPrimitiveBool,
}
#[derive(Debug, utoipa :: ToSchema)]
pub struct UpdateManyPayload(pub std::vec::Vec<UpdateManyPayloadElement>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayloadElementWithSerializeDeserialize {
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
    pub std_primitive_bool_as_postgresql_bool:
        postgresql_crud::StdPrimitiveBoolWithSerializeDeserialize,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayloadWithSerializeDeserialize(
    std::vec::Vec<UpdateManyPayloadElementWithSerializeDeserialize>,
);
impl std::convert::From<UpdateManyPayloadElementWithSerializeDeserialize>
    for UpdateManyPayloadElement
{
    fn from(value: UpdateManyPayloadElementWithSerializeDeserialize) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            postgresql_crud::StdPrimitiveI64::from(
                value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            );
        let std_primitive_bool_as_postgresql_bool =
            postgresql_crud::StdPrimitiveBool::from(value.std_primitive_bool_as_postgresql_bool);
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
        }
    }
}
impl std::convert::TryFrom<UpdateManyPayloadWithSerializeDeserialize> for UpdateManyPayload {
    type Error = UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: UpdateManyPayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        match
        value.0.into_iter().map(| element | UpdateManyPayloadElement ::
        try_from(element)).collect :: < Result < std :: vec :: Vec <
        UpdateManyPayloadElement >,
        UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed
        >> ()
        {
            Ok(value) => Ok(Self(value)), Err(e) =>
            Err(Self :: Error :: from(e)),
        }
    }
}
impl std::convert::From<UpdateManyPayloadElement>
    for UpdateManyPayloadElementWithSerializeDeserialize
{
    fn from(value: UpdateManyPayloadElement) -> Self {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(
                value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            );
        let std_primitive_bool_as_postgresql_bool =
            postgresql_crud::StdPrimitiveBoolWithSerializeDeserialize::from(
                value.std_primitive_bool_as_postgresql_bool,
            );
        Self {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
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
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryUpdateMany {
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
    NotUniquePrimaryKeys {
        #[eo_vec_display]
        not_unique_primary_keys: std::vec::Vec<postgresql_crud::StdPrimitiveI64>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        #[eo_display_with_serialize_deserialize]
        checked_add: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        #[eo_display_with_serialize_deserialize]
        no_payload_fields: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitFailed {
        #[eo_display]
        commit_failed: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        #[eo_vec_display]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::StdPrimitiveI64>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PrimaryKeyFromRowAndFailedRollback {
        #[eo_display]
        primary_key_from_row: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndFailedRollback {
        #[eo_vec_display]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::StdPrimitiveI64>,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryAndRollbackFailed {
        #[eo_display]
        query_error: sqlx::Error,
        #[eo_display]
        rollback_error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        update_many_payload_try_from_update_many_payload_with_serialize_deserialize:
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamed,
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
pub enum TryUpdateManyResponseVariants {
    Desirable(std :: vec :: Vec :: <
    postgresql_crud::StdPrimitiveI64WithSerializeDeserialize >), Configuration
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
    }, NotUniquePrimaryKeys
    {
        not_unique_primary_keys : std::vec::Vec<std::string::String>,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, BindQuery
    {
        bind_query :
        postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, CheckedAdd
    {
        checked_add : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NoPayloadFields
    {
        no_payload_fields : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CommitFailed
    {
        commit_failed : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NonExistingPrimaryKeys
    {
        non_existing_primary_keys : std::vec::Vec<std::string::String>,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, PrimaryKeyFromRowAndFailedRollback
    {
        primary_key_from_row : std :: string :: String, rollback_error : std
        :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NonExistingPrimaryKeysAndFailedRollback
    {
        non_existing_primary_keys : std::vec::Vec<std::string::String>,
        rollback_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, QueryAndRollbackFailed
    {
        query_error : std :: string :: String, rollback_error : std :: string
        :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
    {
        update_many_payload_try_from_update_many_payload_with_serialize_deserialize
        :
        UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
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
impl std::convert::From<TryUpdateMany> for TryUpdateManyResponseVariants {
    fn from(value: TryUpdateMany) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryUpdateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: Io { io, code_occurence }
            => Self :: Io { io, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryUpdateManyWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryUpdateManyWithSerializeDeserialize
            :: JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence } => Self ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: CheckedAdd
            { checked_add, code_occurence } => Self :: CheckedAdd
            { checked_add, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: CommitFailed
            { commit_failed, code_occurence } => Self :: CommitFailed
            { commit_failed, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryUpdateManyWithSerializeDeserialize ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } => Self
            :: PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence },
            TryUpdateManyWithSerializeDeserialize ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Self :: NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } => Self ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence },
            TryUpdateManyWithSerializeDeserialize ::
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
            {
                update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
            {
                update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
                code_occurence
            }, TryUpdateManyWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryUpdateManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryUpdateManyWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } => Self ::
            NoCommitExtractorHeader { no_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryUpdateManyResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryUpdateManyResponseVariants) -> Self {
        match value
        {
            TryUpdateManyResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: Database
            { database : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryUpdateManyResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryUpdateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryUpdateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryUpdateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateManyResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: OK, TryUpdateManyResponseVariants
            :: Decode { decode : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryUpdateManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateManyResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            BindQuery { bind_query : _, code_occurence : _ } => axum :: http
            :: StatusCode :: OK, TryUpdateManyResponseVariants :: CheckedAdd
            { checked_add : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: NoPayloadFields
            { no_payload_fields : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateManyResponseVariants :: CommitFailed
            { commit_failed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryUpdateManyResponseVariants ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            {
                primary_key_from_row : _, rollback_error : _, code_occurence :
                _
            } => axum :: http :: StatusCode :: OK,
            TryUpdateManyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            {
                non_existing_primary_keys : _, rollback_error : _,
                code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryUpdateManyResponseVariants :: QueryAndRollbackFailed
            { query_error : _, rollback_error : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
            {
                update_many_payload_try_from_update_many_payload_with_serialize_deserialize
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryUpdateManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryUpdateManyResponseVariants ::
            NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryUpdateManyResponseVariantsTvfrr200Ok {
    Desirable(std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>),
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr200Ok> for TryUpdateManyResponseVariants {
    fn from(value: TryUpdateManyResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryUpdateManyResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryUpdateManyResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr408RequestTimeout>
    for TryUpdateManyResponseVariants
{
    fn from(value: TryUpdateManyResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryUpdateManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
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
pub enum TryUpdateManyResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr404NotFound>
    for TryUpdateManyResponseVariants
{
    fn from(value: TryUpdateManyResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryUpdateManyResponseVariantsTvfrr404NotFound::RowNotFound {
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
pub enum TryUpdateManyResponseVariantsTvfrr500InternalServerError {
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
    CheckedAdd {
        checked_add: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitFailed {
        commit_failed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PrimaryKeyFromRowAndFailedRollback {
        primary_key_from_row: std::string::String,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryAndRollbackFailed {
        query_error: std::string::String,
        rollback_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr500InternalServerError>
    for TryUpdateManyResponseVariants
{
    fn from(value: TryUpdateManyResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            Migrate { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            CheckedAdd { checked_add, code_occurence } => Self :: CheckedAdd
            { checked_add, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            CommitFailed { commit_failed, code_occurence } => Self ::
            CommitFailed { commit_failed, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } => Self
            :: PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence },
            TryUpdateManyResponseVariantsTvfrr500InternalServerError ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } => Self ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryUpdateManyResponseVariantsTvfrr400BadRequest {
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
    }, NotUniquePrimaryKeys
    {
        not_unique_primary_keys : std::vec::Vec<std::string::String>,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, NoPayloadFields
    {
        no_payload_fields : std::string::String<>, code_occurence :
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
    }, UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
    {
        update_many_payload_try_from_update_many_payload_with_serialize_deserialize
        :
        UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
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
impl std::convert::From<TryUpdateManyResponseVariantsTvfrr400BadRequest>
    for TryUpdateManyResponseVariants
{
    fn from(value: TryUpdateManyResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            NotUniquePrimaryKeys { not_unique_primary_keys, code_occurence }
            => Self :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest :: NoPayloadFields
            { no_payload_fields, code_occurence } => Self :: NoPayloadFields
            { no_payload_fields, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Self :: NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
            {
                update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
            {
                update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
                code_occurence
            }, TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryUpdateManyResponseVariantsTvfrr400BadRequest ::
            NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
impl TryFrom<TryUpdateManyResponseVariants>
    for std::vec::Vec<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>
{
    type Error = TryUpdateManyWithSerializeDeserialize;
    fn try_from(value: TryUpdateManyResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryUpdateManyResponseVariants :: Desirable(i) => Ok(i),
            TryUpdateManyResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryUpdateManyResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: Database
            { database, code_occurence }), TryUpdateManyResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: Io
            { io, code_occurence }), TryUpdateManyResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryUpdateManyResponseVariants ::
            Protocol { protocol, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryUpdateManyResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryUpdateManyResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryUpdateManyResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryUpdateManyResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryUpdateManyResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryUpdateManyResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryUpdateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryUpdateManyResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryUpdateManyResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryUpdateManyResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryUpdateManyResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryUpdateManyResponseVariants ::
            JsonDataError { json_data_error, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }),
            TryUpdateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryUpdateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryUpdateManyResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }),
            TryUpdateManyResponseVariants :: UnexpectedCase
            { unexpected_case, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }),
            TryUpdateManyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: NotUniquePrimaryKeys
            { not_unique_primary_keys, code_occurence }),
            TryUpdateManyResponseVariants :: BindQuery
            { bind_query, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }), TryUpdateManyResponseVariants ::
            CheckedAdd { checked_add, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: CheckedAdd
            { checked_add, code_occurence }), TryUpdateManyResponseVariants ::
            NoPayloadFields { no_payload_fields, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: NoPayloadFields
            { no_payload_fields, code_occurence }),
            TryUpdateManyResponseVariants :: CommitFailed
            { commit_failed, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize :: CommitFailed
            { commit_failed, code_occurence }), TryUpdateManyResponseVariants
            :: NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence }),
            TryUpdateManyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback_error, code_occurence }),
            TryUpdateManyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback_error, code_occurence }),
            TryUpdateManyResponseVariants :: QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            QueryAndRollbackFailed
            { query_error, rollback_error, code_occurence }),
            TryUpdateManyResponseVariants ::
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
            {
                update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
            {
                update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
                code_occurence
            }), TryUpdateManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence }),
            TryUpdateManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence }),
            TryUpdateManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } =>
            Err(TryUpdateManyWithSerializeDeserialize ::
            NoCommitExtractorHeader { no_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateManyRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryUpdateManyWithSerializeDeserialize,
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
pub enum TryUpdateManyStatusCodesChecker {
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
    NotUniquePrimaryKeysTvfrr400BadRequest,
    BindQueryTvfrr500InternalServerError,
    CheckedAddTvfrr500InternalServerError,
    NoPayloadFieldsTvfrr400BadRequest,
    CommitFailedTvfrr500InternalServerError,
    NonExistingPrimaryKeysTvfrr400BadRequest,
    PrimaryKeyFromRowAndFailedRollbackTvfrr500InternalServerError,
    NonExistingPrimaryKeysAndFailedRollbackTvfrr400BadRequest,
    QueryAndRollbackFailedTvfrr500InternalServerError,
    UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeTvfrr400BadRequest,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryUpdateManyResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryUpdateManyResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            } TryUpdateManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: NotUniquePrimaryKeys
            { not_unique_primary_keys : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: CheckedAdd
            { checked_add : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: NoPayloadFields
            { no_payload_fields : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: CommitFailed
            { commit_failed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: NonExistingPrimaryKeys
            { non_existing_primary_keys : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants ::
            PrimaryKeyFromRowAndFailedRollback
            {
                primary_key_from_row : _, rollback_error : _, code_occurence :
                _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants ::
            NonExistingPrimaryKeysAndFailedRollback
            {
                non_existing_primary_keys : _, rollback_error : _,
                code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: QueryAndRollbackFailed
            { query_error : _, rollback_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants ::
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
            {
                update_many_payload_try_from_update_many_payload_with_serialize_deserialize
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryUpdateManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateManyErrorNamed {
    SerdeJsonToString
    {
        #[eo_display] serde_json_to_string : serde_json :: Error,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClientMany
    {
        #[eo_vec_error_occurence]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_client_many
        : std :: vec :: Vec <
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInClientErrorUnnamed
        >, code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, ExpectedType
    {
        #[eo_display_with_serialize_deserialize] expected_type :
        TryUpdateManyWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, UnexpectedStatusCode
    {
        #[eo_display] status_code : http :: StatusCode,
        #[eo_display_foreign_type] headers : reqwest :: header :: HeaderMap,
        #[eo_display_foreign_type] response_text_result : crate :: common ::
        api_request_unexpected_error :: ResponseTextResult, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, FailedToGetResponseText
    {
        #[eo_display_foreign_type] reqwest : reqwest :: Error, #[eo_display]
        status_code : http :: StatusCode, #[eo_display_foreign_type] headers :
        reqwest :: header :: HeaderMap, code_occurence : error_occurence_lib
        :: code_occurence :: CodeOccurence,
    }, DeserializeResponse
    {
        #[eo_display] serde : serde_json :: Error, #[eo_display] status_code :
        http :: StatusCode, #[eo_display_foreign_type] headers : reqwest ::
        header :: HeaderMap, #[eo_display_with_serialize_deserialize]
        response_text : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Reqwest
    {
        #[eo_display_foreign_type] reqwest : reqwest :: Error, code_occurence
        : error_occurence_lib :: code_occurence :: CodeOccurence,
    },
}
pub async fn try_update_many<'a>(
    server_location: &str,
    parameters: UpdateManyParameters,
) -> Result<std::vec::Vec<postgresql_crud::StdPrimitiveI64>, TryUpdateManyErrorNamed> {
    let payload = match serde_json::to_string(&UpdateManyPayloadWithSerializeDeserialize::from(
        parameters.payload,
    )) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryUpdateManyErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1588,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let url = format!("{}/dogs/update_many", server_location,);
    let future = reqwest::Client::new()
        .patch(&url)
        .header(postgresql_crud::COMMIT, git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            return Err(TryUpdateManyErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2432,
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
            return Err(TryUpdateManyErrorNamed::FailedToGetResponseText {
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
                        line: 2361,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let variants = if status_code == http::StatusCode::OK {
        match serde_json::from_str::<TryUpdateManyResponseVariantsTvfrr200Ok>(&response_text) {
            Ok(value) => TryUpdateManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryUpdateManyErrorNamed::DeserializeResponse {
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
                            line: 2398,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match serde_json::from_str::<TryUpdateManyResponseVariantsTvfrr404NotFound>(&response_text)
        {
            Ok(value) => TryUpdateManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryUpdateManyErrorNamed::DeserializeResponse {
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
                            line: 2398,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
        match serde_json::from_str::<TryUpdateManyResponseVariantsTvfrr408RequestTimeout>(
            &response_text,
        ) {
            Ok(value) => TryUpdateManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryUpdateManyErrorNamed::DeserializeResponse {
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
                            line: 2398,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match serde_json::from_str::<TryUpdateManyResponseVariantsTvfrr400BadRequest>(
            &response_text,
        ) {
            Ok(value) => TryUpdateManyResponseVariants::from(value),
            Err(e) => {
                return Err(TryUpdateManyErrorNamed::DeserializeResponse {
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
                            line: 2398,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else {
        return Err(TryUpdateManyErrorNamed::UnexpectedStatusCode {
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
                    line: 2326,
                    column: 13,
                }),
            ),
        });
    };
    match std::vec::Vec::<postgresql_crud::StdPrimitiveI64WithSerializeDeserialize>::try_from(
        variants,
    ) {
        Ok(value) => Ok(value
            .into_iter()
            .map(|element| postgresql_crud::StdPrimitiveI64::from(element))
            .collect()),
        Err(e) => {
            return Err(TryUpdateManyErrorNamed::ExpectedType {
                expected_type: e,
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
    }
}
#[utoipa ::
path(patch, path = "/dogs/update_many", operation_id = "/dogs/update_many",
tag = "dogs",
request_body(content = UpdateManyPayloadWithSerializeDeserialize, description
= "dogs update_many payload", content_type = "application/json"),
responses((status = 200, description = "ok", body =
TryUpdateManyResponseVariantsTvfrr200Ok, content_type = "application/json"),
(status = 500, description = "internal server error", body =
TryUpdateManyResponseVariantsTvfrr500InternalServerError, content_type =
"application/json"),
(status = 404, description = "not found", body =
TryUpdateManyResponseVariantsTvfrr404NotFound, content_type =
"application/json"),
(status = 400, description = "bad request", body =
TryUpdateManyResponseVariantsTvfrr400BadRequest, content_type =
"application/json"),
(status = 408, description = "request timeout", body =
TryUpdateManyResponseVariantsTvfrr408RequestTimeout, content_type =
"application/json")),)]
pub async fn update_many<'a>(
    app_state: axum::extract::State<
        postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
    >,
    payload_extraction_result: Result<
        axum::Json<UpdateManyPayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = UpdateManyParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                UpdateManyPayloadWithSerializeDeserialize,
                TryUpdateManyResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_state)
            {
                Ok(value) => match UpdateManyPayload::try_from(value) {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryUpdateMany ::
                    UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
                    {
                        update_many_payload_try_from_update_many_payload_with_serialize_deserialize
                        : e, code_occurence : error_occurence_lib :: code_occurence
                        :: CodeOccurence ::
                        new(file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 5715, column : 17,
                        })),
                    } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryUpdateManyResponseVariants::from(e);
                    }
                },
                Err(e) => {
                    return e;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        {
            let not_unique_primary_keys = {
                let mut vec = std::vec::Vec::with_capacity(parameters.payload.0.len());
                let mut not_unique_primary_keys =
                    std::vec::Vec::with_capacity(parameters.payload.0.len());
                for element in &parameters.payload.0 {
                    let handle =
                        &element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;
                    match vec.contains(&handle) {
                        true => {
                            not_unique_primary_keys.push(
                                element
                                    .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                                    .clone(),
                            );
                        }
                        false => {
                            vec.push(&
                            element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key)
                            ;
                        }
                    }
                }
                not_unique_primary_keys
            };
            if let false = not_unique_primary_keys.is_empty() {
                let e = TryUpdateMany::NotUniquePrimaryKeys {
                    not_unique_primary_keys,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1970,
                            column: 13,
                        }),
                    ),
                };
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryUpdateManyResponseVariants::from(e);
            }
        }
        let expected_updated_primary_keys = {
            parameters
                .payload
                .0
                .iter()
                .map(|element| {
                    element
                        .std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                        .clone()
                })
                .collect::<std::vec::Vec<postgresql_crud::StdPrimitiveI64>>()
        };
        let binded_query = {
            let query_string = {
                "update dogs as t set std_primitive_bool_as_postgresql_bool = data.std_primitive_bool_as_postgresql_bool from (select * from unnest($1, $2)) as data(std_primitive_bool_as_postgresql_bool, std_primitive_i64_as_postgresql_big_serial_not_null_primary_key) where t.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = data.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key returning data.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"
            };
            println!("{}", query_string);
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            let current_vec_len = parameters.payload.0.len();
            let (
                std_primitive_bool_as_postgresql_bool_vec,
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec,
            ) = parameters.payload.0.into_iter().fold(
                (
                    std::vec::Vec::with_capacity(current_vec_len),
                    std::vec::Vec::with_capacity(current_vec_len),
                ),
                |mut acc, element| {
                    acc.0.push(element.std_primitive_bool_as_postgresql_bool);
                    acc.1.push(
                        element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                    );
                    acc
                },
            );
            query = query.bind(
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key_vec
                    .into_iter()
                    .map(|element| element.into_inner())
                    .collect::<std::vec::Vec<std::primitive::i64>>(),
            );
            query = query.bind(
                std_primitive_bool_as_postgresql_bool_vec
                    .into_iter()
                    .map(|element| element.into_inner())
                    .collect::<std::vec::Vec<std::primitive::bool>>(),
            );
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let e = TryUpdateMany::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryUpdateManyResponseVariants::from(e);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let e = TryUpdateMany::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryUpdateManyResponseVariants::from(e);
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
                let e = TryUpdateMany::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryUpdateManyResponseVariants::from(e);
            }
        };
        let results_vec = {
            let mut results_vec = std::vec::Vec::with_capacity(expected_updated_primary_keys.len());
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
                        let e = TryUpdateMany::from(e);
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryUpdateManyResponseVariants::from(e);
                    }
                    Err(rollback_error) => {
                        let e = TryUpdateMany :: QueryAndRollbackFailed
                        {
                            query_error : e, rollback_error, code_occurence :
                            error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(file! ().to_string(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                line : 37, column : 13,
                            })),
                        } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryUpdateManyResponseVariants::from(e);
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
                    Err(e) => match postgres_transaction.rollback().await {
                        Ok(_) => {
                            let e = TryUpdateMany::from(e);
                            error_occurence_lib::error_log::ErrorLog::error_log(
                                &e,
                                app_state.as_ref(),
                            );
                            return TryUpdateManyResponseVariants::from(e);
                        }
                        Err(rollback_error) => {
                            let e = TryUpdateMany :: PrimaryKeyFromRowAndFailedRollback
                            {
                                primary_key_from_row : e, rollback_error, code_occurence :
                                error_occurence_lib :: code_occurence :: CodeOccurence ::
                                new(file! ().to_string(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                    line : 52, column : 13,
                                })),
                            } ;
                            error_occurence_lib::error_log::ErrorLog::error_log(
                                &e,
                                app_state.as_ref(),
                            );
                            return TryUpdateManyResponseVariants::from(e);
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
                        let e = TryUpdateMany :: NonExistingPrimaryKeys
                        {
                            non_existing_primary_keys, code_occurence :
                            error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(file! ().to_string(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                                line : 67, column : 13,
                            })),
                        } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryUpdateManyResponseVariants::from(e);
                    }
                    Err(e) => {
                        let e = TryUpdateMany ::
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
                                line : 81, column : 13,
                            })),
                        } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryUpdateManyResponseVariants::from(e);
                    }
                }
            }
        }
        match postgres_transaction.commit().await {
            Ok(_) => TryUpdateManyResponseVariants::Desirable(
                primary_key_vec
                    .into_iter()
                    .map(|element| {
                        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize::from(element)
                    })
                    .collect(),
            ),
            Err(e) => {
                let e = TryUpdateMany :: CommitFailed
                {
                    commit_failed : e, code_occurence : error_occurence_lib ::
                    code_occurence :: CodeOccurence ::
                    new(file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/generate_postgres_transaction.rs"),
                        line : 96, column : 13,
                    })),
                } ;
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                TryUpdateManyResponseVariants::from(e)
            }
        }
    }
}
impl std::convert::From<crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed>
    for TryUpdateMany
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
