#[derive(Debug, Clone, Copy
    , postgresql_crud::GeneratePostgresqlTable
)]
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
            check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
    }
}]
#[postgresql_crud::create_many_additional_logic{}]
#[postgresql_crud::create_one_additional_logic{}]
#[postgresql_crud::read_many_additional_logic{}]
#[postgresql_crud::read_one_additional_logic{}]
#[postgresql_crud::update_many_additional_logic{}]
#[postgresql_crud::update_one_additional_logic{}]
#[postgresql_crud::delete_many_additional_logic{}]
#[postgresql_crud::delete_one_additional_logic{}]
#[postgresql_crud::common_additional_logic{}]
pub struct TableExample {
    // #[generate_postgresql_crud_primary_key]
    // pub primary_key: postgresql_crud::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql,
    #[generate_postgresql_table_primary_key]
    pub primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql,

    pub column_0: postgresql_crud::StdPrimitiveI16AsNotNullInt2,
    // pub column_1: postgresql_crud::OptionStdPrimitiveI16AsNullableInt2,
    // pub column_2: postgresql_crud::VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2,
    // pub column_3: postgresql_crud::OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2,
    // pub column_4: postgresql_crud::VecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableInt2,
    // pub column_5: postgresql_crud::OptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableInt2,
    // pub column_6: postgresql_crud::StdPrimitiveI32AsNotNullInt4,
    // pub column_7: postgresql_crud::OptionStdPrimitiveI32AsNullableInt4,
    // pub column_8: postgresql_crud::VecOfStdPrimitiveI32AsNotNullArrayOfNotNullInt4,
    // pub column_9: postgresql_crud::OptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullInt4,
    // pub column_10: postgresql_crud::VecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableInt4,
    // pub column_11: postgresql_crud::OptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableInt4,
    // pub column_12: postgresql_crud::StdPrimitiveI64AsNotNullInt8,
    // pub column_13: postgresql_crud::OptionStdPrimitiveI64AsNullableInt8,
    // pub column_14: postgresql_crud::VecOfStdPrimitiveI64AsNotNullArrayOfNotNullInt8,
    // pub column_15: postgresql_crud::OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullInt8,
    // pub column_16: postgresql_crud::VecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableInt8,
    // pub column_17: postgresql_crud::OptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableInt8,
    // pub column_18: postgresql_crud::StdPrimitiveF32AsNotNullFloat4,
    // pub column_19: postgresql_crud::OptionStdPrimitiveF32AsNullableFloat4,
    // pub column_20: postgresql_crud::VecOfStdPrimitiveF32AsNotNullArrayOfNotNullFloat4,
    // pub column_21: postgresql_crud::OptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullFloat4,
    // pub column_22: postgresql_crud::VecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableFloat4,
    // pub column_23: postgresql_crud::OptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableFloat4,
    // pub column_24: postgresql_crud::StdPrimitiveF64AsNotNullFloat8,
    // pub column_25: postgresql_crud::OptionStdPrimitiveF64AsNullableFloat8,
    // pub column_26: postgresql_crud::VecOfStdPrimitiveF64AsNotNullArrayOfNotNullFloat8,
    // pub column_27: postgresql_crud::OptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullFloat8,
    // pub column_28: postgresql_crud::VecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableFloat8,
    // pub column_29: postgresql_crud::OptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableFloat8,
    // pub column_30: postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql,
    // pub column_31: postgresql_crud::StdPrimitiveI32AsNotNullSerialInitializedByPostgresql,
    // pub column_32: postgresql_crud::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql,
    // pub column_33: postgresql_crud::SqlxPostgresTypesPgMoneyAsNotNullMoney,
    // pub column_34: postgresql_crud::OptionSqlxPostgresTypesPgMoneyAsNullableMoney,
    // pub column_35: postgresql_crud::VecOfSqlxPostgresTypesPgMoneyAsNotNullArrayOfNotNullMoney,
    // pub column_36: postgresql_crud::OptionVecOfSqlxPostgresTypesPgMoneyAsNullableArrayOfNotNullMoney,
    // pub column_37: postgresql_crud::VecOfOptionSqlxPostgresTypesPgMoneyAsNotNullArrayOfNullableMoney,
    // pub column_38: postgresql_crud::OptionVecOfOptionSqlxPostgresTypesPgMoneyAsNullableArrayOfNullableMoney,
    // pub column_39: postgresql_crud::StdPrimitiveBoolAsNotNullBool,
    // pub column_40: postgresql_crud::OptionStdPrimitiveBoolAsNullableBool,
    // pub column_41: postgresql_crud::VecOfStdPrimitiveBoolAsNotNullArrayOfNotNullBool,
    // pub column_42: postgresql_crud::OptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullBool,
    // pub column_43: postgresql_crud::VecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableBool,
    // pub column_44: postgresql_crud::OptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableBool,
    // pub column_45: postgresql_crud::StdStringStringAsNotNullText,
    // pub column_46: postgresql_crud::OptionStdStringStringAsNullableText,
    // pub column_47: postgresql_crud::VecOfStdStringStringAsNotNullArrayOfNotNullText,
    // pub column_48: postgresql_crud::OptionVecOfStdStringStringAsNullableArrayOfNotNullText,
    // pub column_49: postgresql_crud::VecOfOptionStdStringStringAsNotNullArrayOfNullableText,
    // pub column_50: postgresql_crud::OptionVecOfOptionStdStringStringAsNullableArrayOfNullableText,
    // pub column_51: postgresql_crud::StdVecVecStdPrimitiveU8AsNotNullBytea,
    // pub column_52: postgresql_crud::OptionStdVecVecStdPrimitiveU8AsNullableBytea,
    // pub column_53: postgresql_crud::VecOfStdVecVecStdPrimitiveU8AsNotNullArrayOfNotNullBytea,
    // pub column_54: postgresql_crud::OptionVecOfStdVecVecStdPrimitiveU8AsNullableArrayOfNotNullBytea,
    // pub column_55: postgresql_crud::VecOfOptionStdVecVecStdPrimitiveU8AsNotNullArrayOfNullableBytea,
    // pub column_56: postgresql_crud::OptionVecOfOptionStdVecVecStdPrimitiveU8AsNullableArrayOfNullableBytea,
    // pub column_57: postgresql_crud::SqlxTypesChronoNaiveTimeAsNotNullTime,
    // pub column_58: postgresql_crud::OptionSqlxTypesChronoNaiveTimeAsNullableTime,
    // pub column_59: postgresql_crud::VecOfSqlxTypesChronoNaiveTimeAsNotNullArrayOfNotNullTime,
    // pub column_60: postgresql_crud::OptionVecOfSqlxTypesChronoNaiveTimeAsNullableArrayOfNotNullTime,
    // pub column_61: postgresql_crud::VecOfOptionSqlxTypesChronoNaiveTimeAsNotNullArrayOfNullableTime,
    // pub column_62: postgresql_crud::OptionVecOfOptionSqlxTypesChronoNaiveTimeAsNullableArrayOfNullableTime,
    // pub column_63: postgresql_crud::SqlxTypesTimeTimeAsNotNullTime,
    // pub column_64: postgresql_crud::OptionSqlxTypesTimeTimeAsNullableTime,
    // pub column_65: postgresql_crud::VecOfSqlxTypesTimeTimeAsNotNullArrayOfNotNullTime,
    // pub column_66: postgresql_crud::OptionVecOfSqlxTypesTimeTimeAsNullableArrayOfNotNullTime,
    // pub column_67: postgresql_crud::VecOfOptionSqlxTypesTimeTimeAsNotNullArrayOfNullableTime,
    // pub column_68: postgresql_crud::OptionVecOfOptionSqlxTypesTimeTimeAsNullableArrayOfNullableTime,
    // pub column_69: postgresql_crud::SqlxPostgresTypesPgIntervalAsNotNullInterval,
    // pub column_70: postgresql_crud::OptionSqlxPostgresTypesPgIntervalAsNullableInterval,
    // pub column_71: postgresql_crud::VecOfSqlxPostgresTypesPgIntervalAsNotNullArrayOfNotNullInterval,
    // pub column_72: postgresql_crud::OptionVecOfSqlxPostgresTypesPgIntervalAsNullableArrayOfNotNullInterval,
    // pub column_73: postgresql_crud::VecOfOptionSqlxPostgresTypesPgIntervalAsNotNullArrayOfNullableInterval,
    // pub column_74: postgresql_crud::OptionVecOfOptionSqlxPostgresTypesPgIntervalAsNullableArrayOfNullableInterval,
    // pub column_75: postgresql_crud::SqlxTypesChronoNaiveDateAsNotNullDate,
    // pub column_76: postgresql_crud::OptionSqlxTypesChronoNaiveDateAsNullableDate,
    // pub column_77: postgresql_crud::VecOfSqlxTypesChronoNaiveDateAsNotNullArrayOfNotNullDate,
    // pub column_78: postgresql_crud::OptionVecOfSqlxTypesChronoNaiveDateAsNullableArrayOfNotNullDate,
    // pub column_79: postgresql_crud::VecOfOptionSqlxTypesChronoNaiveDateAsNotNullArrayOfNullableDate,
    // pub column_80: postgresql_crud::OptionVecOfOptionSqlxTypesChronoNaiveDateAsNullableArrayOfNullableDate,
    // pub column_81: postgresql_crud::SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp,
    // pub column_82: postgresql_crud::OptionSqlxTypesChronoNaiveDateTimeAsNullableTimestamp,
    // pub column_83: postgresql_crud::VecOfSqlxTypesChronoNaiveDateTimeAsNotNullArrayOfNotNullTimestamp,
    // pub column_84: postgresql_crud::OptionVecOfSqlxTypesChronoNaiveDateTimeAsNullableArrayOfNotNullTimestamp,
    // pub column_85: postgresql_crud::VecOfOptionSqlxTypesChronoNaiveDateTimeAsNotNullArrayOfNullableTimestamp,
    // pub column_86: postgresql_crud::OptionVecOfOptionSqlxTypesChronoNaiveDateTimeAsNullableArrayOfNullableTimestamp,
    // pub column_87: postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTz,
    // pub column_88: postgresql_crud::OptionSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableTimestampTz,
    // pub column_89: postgresql_crud::VecOfSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullArrayOfNotNullTimestampTz,
    // pub column_90: postgresql_crud::OptionVecOfSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableArrayOfNotNullTimestampTz,
    // pub column_91: postgresql_crud::VecOfOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullArrayOfNullableTimestampTz,
    // pub column_92: postgresql_crud::OptionVecOfOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableArrayOfNullableTimestampTz,
    // pub column_93: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql,
    // pub column_94: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidInitializedByClient,
    // pub column_95: postgresql_crud::OptionSqlxTypesUuidUuidAsNullableUuidInitializedByClient,
    // pub column_96: postgresql_crud::VecOfSqlxTypesUuidUuidAsNotNullArrayOfNotNullUuidInitializedByClient,
    // pub column_97: postgresql_crud::OptionVecOfSqlxTypesUuidUuidAsNullableArrayOfNotNullUuidInitializedByClient,
    // pub column_98: postgresql_crud::VecOfOptionSqlxTypesUuidUuidAsNotNullArrayOfNullableUuidInitializedByClient,
    // pub column_99: postgresql_crud::OptionVecOfOptionSqlxTypesUuidUuidAsNullableArrayOfNullableUuidInitializedByClient,
    // pub column_100: postgresql_crud::SqlxTypesIpnetworkIpNetworkAsNotNullInet,
    // pub column_101: postgresql_crud::OptionSqlxTypesIpnetworkIpNetworkAsNullableInet,
    // pub column_102: postgresql_crud::VecOfSqlxTypesIpnetworkIpNetworkAsNotNullArrayOfNotNullInet,
    // pub column_103: postgresql_crud::OptionVecOfSqlxTypesIpnetworkIpNetworkAsNullableArrayOfNotNullInet,
    // pub column_104: postgresql_crud::VecOfOptionSqlxTypesIpnetworkIpNetworkAsNotNullArrayOfNullableInet,
    // pub column_105: postgresql_crud::OptionVecOfOptionSqlxTypesIpnetworkIpNetworkAsNullableArrayOfNullableInet,
    // pub column_106: postgresql_crud::SqlxTypesMacAddressMacAddressAsNotNullMacAddr,
    // pub column_107: postgresql_crud::OptionSqlxTypesMacAddressMacAddressAsNullableMacAddr,
    // pub column_108: postgresql_crud::VecOfSqlxTypesMacAddressMacAddressAsNotNullArrayOfNotNullMacAddr,
    // pub column_109: postgresql_crud::OptionVecOfSqlxTypesMacAddressMacAddressAsNullableArrayOfNotNullMacAddr,
    // pub column_110: postgresql_crud::VecOfOptionSqlxTypesMacAddressMacAddressAsNotNullArrayOfNullableMacAddr,
    // pub column_111: postgresql_crud::OptionVecOfOptionSqlxTypesMacAddressMacAddressAsNullableArrayOfNullableMacAddr,

    // pub column_112: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4Range,
    // pub column_113: postgresql_crud::OptionSqlxPostgresTypesPgRangeStdPrimitiveI32AsNullableInt4Range,
    // pub column_114: postgresql_crud::VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4Range,
    // pub column_115: postgresql_crud::OptionVecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNullableArrayOfNotNullInt4Range,
    // pub column_116: postgresql_crud::VecOfOptionSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNullableInt4Range,
    // pub column_117: postgresql_crud::OptionVecOfOptionSqlxPostgresTypesPgRangeStdPrimitiveI32AsNullableArrayOfNullableInt4Range,
    // pub column_118: postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI64AsNotNullInt8Range,
    // pub column_119: postgresql_crud::OptionSqlxPostgresTypesPgRangeStdPrimitiveI64AsNullableInt8Range,
    // pub column_120: postgresql_crud::VecOfSqlxPostgresTypesPgRangeStdPrimitiveI64AsNotNullArrayOfNotNullInt8Range,
    // pub column_121: postgresql_crud::OptionVecOfSqlxPostgresTypesPgRangeStdPrimitiveI64AsNullableArrayOfNotNullInt8Range,
    // pub column_122: postgresql_crud::VecOfOptionSqlxPostgresTypesPgRangeStdPrimitiveI64AsNotNullArrayOfNullableInt8Range,
    // pub column_123: postgresql_crud::OptionVecOfOptionSqlxPostgresTypesPgRangeStdPrimitiveI64AsNullableArrayOfNullableInt8Range,
    // pub column_124: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNotNullDateRange,
    // pub column_125: postgresql_crud::OptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNullableDateRange,
    // pub column_126: postgresql_crud::VecOfSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNotNullArrayOfNotNullDateRange,
    // pub column_127: postgresql_crud::OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNullableArrayOfNotNullDateRange,
    // pub column_128: postgresql_crud::VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNotNullArrayOfNullableDateRange,
    // pub column_129: postgresql_crud::OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNullableArrayOfNullableDateRange,
    // pub column_130: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRange,
    // pub column_131: postgresql_crud::OptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNullableTimestampRange,
    // pub column_132: postgresql_crud::VecOfSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullArrayOfNotNullTimestampRange,
    // pub column_133: postgresql_crud::OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNullableArrayOfNotNullTimestampRange,
    // pub column_134: postgresql_crud::VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullArrayOfNullableTimestampRange,
    // pub column_135: postgresql_crud::OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNullableArrayOfNullableTimestampRange,
    // pub column_136: postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzRange,
    // pub column_137: postgresql_crud::OptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableTimestampTzRange,
    // pub column_138: postgresql_crud::VecOfSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullArrayOfNotNullTimestampTzRange,
    // pub column_139: postgresql_crud::OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableArrayOfNotNullTimestampTzRange,
    // pub column_140: postgresql_crud::VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullArrayOfNullableTimestampTzRange,
    // pub column_141: postgresql_crud::OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableArrayOfNullableTimestampTzRange,
    // pub column_142: server_types::AnimalAsNotNullJsonbObject,
    // pub column_143: server_types::OptionAnimalAsNullableJsonbObject,
    // pub column_144: server_types::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId,
    // pub column_145: server_types::OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId,
}

#[cfg(test)]
mod table_example_tests {
    use super::*;
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<TableExample>(), 0);
    }
    #[test]
    fn crud() {
        std::thread::Builder::new()
            .stack_size(16 * 1024 * 1024)
            .spawn(|| {
                tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build().expect("error 38823c21-1879-449c-9b60-ce7293709959").block_on(async {
                    tracing_subscriber::fmt::init();
                    let no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row = "no rows returned by a query that expected to return at least one row";
                    static CONFIG: std::sync::OnceLock<server_config::Config> = std::sync::OnceLock::new();
                    let config = CONFIG.get_or_init(|| server_config::Config {
                        service_socket_address: <config_lib::ServiceSocketAddress as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("127.0.0.1:8080".to_string()).expect("error b5b3915a-0e18-4815-a614-6b0e9a00d73f").0,
                        database_url: <config_lib::DatabaseUrl as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("postgres://postgres:postgres@127.0.0.1:5432/postgres?connect_timeout=10".to_string()).expect("error f9c20f05-3cdf-46ae-b6d3-5943c627f0df").0,
                        timezone: <config_lib::Timezone as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("10800".to_string()).expect("error d00d8998-52f9-45c1-a4b0-c93bc95a313e").0,
                        tracing_level: <config_lib::TracingLevel as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("error".to_string()).expect("error 957178c9-4d92-4110-b524-9dc21d147a7c").0,
                        source_place_type: <config_lib::SourcePlaceType as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("source".to_string()).expect("error bec0950e-e9de-42f3-b3a2-67d9d98ae8a6").0,
                        enable_api_git_commit_check: <config_lib::EnableApiGitCommitCheck as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("true".to_string()).expect("error 31f02640-d62b-41ca-837d-d61b707d4baf").0,
                        maximum_size_of_http_body_in_bytes: <config_lib::MaximumSizeOfHttpBodyInBytes as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("1048576000".to_string()).expect("error 93b2f818-18be-4bb6-8a02-53c6e55ded2d").0,
                    });
                    let postgres_pool = sqlx::postgres::PgPoolOptions::new().max_connections(50).connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&config))).await.expect("error e3044bb9-7b76-4c0c-bc5f-eb34da05a103");
                    let url = format!("http://{}", app_state::GetServiceSocketAddress::get_service_socket_address(&config));
                    let table = "table_example";
                    let table_create_many = format!("{table}_create_many");
                    let table_create_one = format!("{table}_create_one");
                    let table_test_read_many_by_non_existent_primary_keys = format!("{table}_test_read_many_by_non_existent_primary_keys");
                    let table_test_read_many_by_equal_to_created_primary_keys = format!("{table}_test_read_many_by_equal_to_created_primary_keys");
                    let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0 = format!("{table}_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0");
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0 = format!("{table}_eb24448c_fa63_4259_bb05_3215802a78f6_column_0");
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0 = format!("{table}_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0");
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0 = format!("{table}_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0");
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0 = format!("{table}_5a52af33_a590_403b_808e_961df6d7e7aa_column_0");
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0 = format!("{table}_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0");
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0 = format!("{table}_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0");
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0 = format!("{table}_de556c26_9297_4adb_9483_22d474cf1e7d_column_0");
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0 = format!("{table}_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0");
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0 = format!("{table}_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0");
                    let table_read_one = format!("{table}_read_one");
                    let table_update_many = format!("{table}_update_many");
                    let table_update_one = format!("{table}_update_one");
                    let table_delete_many = format!("{table}_delete_many");
                    let table_delete_one = format!("{table}_delete_one");
                    let table_create_many_cloned = table_create_many.clone();
                    let table_create_one_cloned = table_create_one.clone();
                    let table_test_read_many_by_non_existent_primary_keys_cloned = table_test_read_many_by_non_existent_primary_keys.clone();
                    let table_test_read_many_by_equal_to_created_primary_keys_cloned = table_test_read_many_by_equal_to_created_primary_keys.clone();
                    let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0_cloned = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0.clone();
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0.clone();
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0.clone();
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0_cloned = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0.clone();
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0_cloned = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0.clone();
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0_cloned = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0.clone();
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0_cloned = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0.clone();
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0_cloned = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0.clone();
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0_cloned = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0.clone();
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0_cloned = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0.clone();
                    let table_read_one_cloned = table_read_one.clone();
                    let table_update_many_cloned = table_update_many.clone();
                    let table_update_one_cloned = table_update_one.clone();
                    let table_delete_many_cloned = table_delete_many.clone();
                    let table_delete_one_cloned = table_delete_one.clone();
                    let table_create_many_cloned2 = table_create_many.clone();
                    let table_create_one_cloned2 = table_create_one.clone();
                    let table_test_read_many_by_non_existent_primary_keys_cloned2 = table_test_read_many_by_non_existent_primary_keys.clone();
                    let table_test_read_many_by_equal_to_created_primary_keys_cloned2 = table_test_read_many_by_equal_to_created_primary_keys.clone();
                    let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0_cloned2 = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0.clone();
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned2 = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0.clone();
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned2 = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0.clone();
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0_cloned2 = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0.clone();
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0_cloned2 = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0.clone();
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0_cloned2 = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0.clone();
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0_cloned2 = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0.clone();
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0_cloned2 = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0.clone();
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0_cloned2 = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0.clone();
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0_cloned2 = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0.clone();
                    let table_read_one_cloned2 = table_read_one.clone();
                    let table_update_many_cloned2 = table_update_many.clone();
                    let table_update_one_cloned2 = table_update_one.clone();
                    let table_delete_one_cloned2 = table_delete_one.clone();
                    let drop_all_test_tables = async || {
                        let _unused = futures::future::try_join_all(
                            [
                                table,
                                &table_create_many,
                                &table_create_one,
                                &table_test_read_many_by_non_existent_primary_keys_cloned2,
                                &table_test_read_many_by_equal_to_created_primary_keys_cloned2,
                                &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0,
                                &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0,
                                &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0,
                                &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0,
                                &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0,
                                &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0,
                                &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0,
                                &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0,
                                &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0,
                                &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0,
                                &table_read_one,
                                &table_update_many,
                                &table_update_one,
                                &table_delete_many,
                                &table_delete_one,
                            ]
                            .iter()
                            .map(|table_name| {
                                let postgres_pool = &postgres_pool;
                                async move { sqlx::query(&format!("drop table if exists {table_name}")).execute(postgres_pool).await }
                            }),
                        )
                        .await
                        .expect("error b9c1eb2e-4ead-449b-abb8-0a160cf68efd");
                    };
                    drop_all_test_tables().await;
                    let postgres_pool_for_tokio_spawn_sync_move = postgres_pool.clone();
                    let (started_tx, started_rx) = tokio::sync::oneshot::channel();
                    let _unused = tokio::spawn(async move {
                        TableExample::prepare_extensions(&postgres_pool_for_tokio_spawn_sync_move).await.expect("error 0633ff48-ebc4-460f-a282-d750511f5d78");
                        let _unused = futures::future::try_join_all(
                            [
                                table,
                                &table_create_many_cloned,
                                &table_create_one_cloned,
                                &table_test_read_many_by_non_existent_primary_keys_cloned,
                                &table_test_read_many_by_equal_to_created_primary_keys_cloned,
                                &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0_cloned,
                                &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned,
                                &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned,
                                &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0_cloned,
                                &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0_cloned,
                                &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0_cloned,
                                &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0_cloned,
                                &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0_cloned,
                                &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0_cloned,
                                &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0_cloned,
                                &table_read_one_cloned,
                                &table_update_many_cloned,
                                &table_update_one_cloned,
                                &table_delete_many_cloned,
                                &table_delete_one_cloned,
                            ]
                            .iter()
                            .map(|table_name| TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, table_name)),
                        )
                        .await
                        .expect("error c7952247-dc94-441b-9aef-368b8fdc593c");
                        let app_state = std::sync::Arc::new(server_app_state::ServerAppState {
                            postgres_pool: postgres_pool_for_tokio_spawn_sync_move.clone(),
                            config,
                            project_git_info: &git_info::PROJECT_GIT_INFO,
                        });
                        let tcp_listener = tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(&config)).await.expect("error 663ae29e-bc00-4ea1-a7e9-4dddceb5b53a");
                        if let Err(error) = started_tx.send(()) {
                            panic!("error aa3b8154-1fe2-4d3f-a164-26f9d21245cd {error:#?}");
                        }
                        axum::serve(tcp_listener, {
                            let mut router = axum::Router::new().merge(TableExample::routes(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state)));
                            for table_name in [
                                &table_create_many_cloned,
                                &table_create_one_cloned,
                                &table_test_read_many_by_non_existent_primary_keys_cloned,
                                &table_test_read_many_by_equal_to_created_primary_keys_cloned,
                                &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0_cloned,
                                &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned,
                                &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned,
                                &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0_cloned,
                                &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0_cloned,
                                &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0_cloned,
                                &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0_cloned,
                                &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0_cloned,
                                &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0_cloned,
                                &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0_cloned,
                                &table_read_one_cloned,
                                &table_update_many_cloned,
                                &table_update_one_cloned,
                                &table_delete_many_cloned,
                                &table_delete_one_cloned,
                            ] {
                                router = router.merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), table_name));
                            }
                            router.into_make_service()
                        })
                        .await
                        .unwrap_or_else(|error| panic!("axum builder serve await failed {error:#?}"));
                    });
                    started_rx.await.expect("error 87003141-43a4-4975-8ddf-273148add50f");
                    let select_primary_key = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default())]).expect("error 0776170e-4dd6-4c14-a412-ce10b0c746f1");
                    let ident_create_default = TableExampleCreate {
                        column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                    };
                    let select_default_all_with_max_page_size = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![
                        TableExampleSelect::PrimaryKeyColumn(<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize>::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size()),
                        TableExampleSelect::Column0(<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize>::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size()),
                    ])
                    .expect("error 8f42ee4f-00d9-4b67-8ead-adddf5bcdf94");
                    let common_read_only_ids_returned_from_create_one = TableExample::try_create_one(&url, TableExampleCreateOneParameters { payload: ident_create_default.clone() }).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
                    let some_value_read_only_ids_returned_from_create_one = Some(postgresql_crud::Value {
                        value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(common_read_only_ids_returned_from_create_one.primary_key_column.clone()),
                    });
                    assert_eq!(
                        TableExampleRead {
                            primary_key_column: some_value_read_only_ids_returned_from_create_one.clone(),
                            column_0: None
                        },
                        TableExample::try_read_one(
                            &url,
                            TableExampleReadOneParameters {
                                payload: TableExampleReadOnePayload {
                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(common_read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                    select: select_primary_key.clone(),
                                },
                            },
                        )
                        .await
                        .expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
                        "error 3d9f2ec0-e374-48d2-a36b-486f5598b0b4"
                    );
                    assert_eq!(
                        TableExample::try_delete_one(
                            &url,
                            TableExampleDeleteOneParameters {
                                payload: TableExampleDeleteOnePayload {
                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(common_read_only_ids_returned_from_create_one.primary_key_column.clone())
                                }
                            }
                        )
                        .await
                        .expect("error 006b18e9-c965-45ee-afc0-a4f6b850ed06"),
                        <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(common_read_only_ids_returned_from_create_one.primary_key_column.clone()),
                        "error 26e2058b-4bc1-42da-8f35-0ab993904de5"
                    );
                    if let Err(error) = TableExample::try_read_one(
                        &url,
                        TableExampleReadOneParameters {
                            payload: TableExampleReadOnePayload {
                                primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(common_read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                select: select_default_all_with_max_page_size.clone(),
                            },
                        },
                    )
                    .await
                    {
                        if let TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize, .. } = error {
                            if let TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                                assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 58b9a6a4-cf9b-49f3-a20f-7007deea40fd");
                            } else {
                                panic!("error 0ad0117b-a2e0-4629-99d0-71935cd93d15");
                            }
                        } else {
                            panic!("error c6695392-4b5f-4482-86aa-b2f19c33a746")
                        }
                    } else {
                        panic!("error 67e43b7a-d3ec-4a3b-a3f1-8c11499fd090")
                    }
                    async fn generate_try_read_many_order_by_primary_key_with_big_pagination(endpoint_location: &str, current_ident_where_many: TableExampleWhereMany, select: postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>, table: &str) -> Result<Vec<TableExampleRead>, TableExampleTryReadManyErrorNamed> {
                        TableExample::try_read_many_handle(
                            &endpoint_location,
                            TableExampleReadManyParameters {
                                payload: TableExampleReadManyPayload {
                                    where_many: StdOptionOptionTableExampleWhereMany(Some(current_ident_where_many)),
                                    select,
                                    order_by: postgresql_crud::OrderBy {
                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                        order: Some(postgresql_crud::Order::Asc),
                                    },
                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error b0cdf0cb-1e31-4a7e-9e53-d2ff71efb983"),
                                },
                            },
                            &table,
                        )
                        .await
                    }
                    futures::StreamExt::for_each_concurrent(
                        futures::stream::iter({
                            let mut acc: Vec<futures::future::BoxFuture<'static, ()>> = vec![];
                            {
                                {
                                    let current_table = table_test_read_many_by_non_existent_primary_keys_cloned2.clone();
                                    async fn generate_test_read_many_by_non_existent_primary_keys(length: usize, url: &str, select_default_all_with_max_page_size: postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>, current_table: &str, ident_create_default: TableExampleCreate, no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row: &str) {
                                        let read_only_ids_from_try_create_one_default = TableExample::try_create_one_handle(url, TableExampleCreateOneParameters { payload: ident_create_default.clone() }, current_table).await.expect("error 71632985-ec25-4928-aa9e-1e224a7478c1");
                                        match generate_try_read_many_order_by_primary_key_with_big_pagination(
                                            url,
                                            TableExampleWhereMany::try_new(
                                                Some(
                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                        let mut acc = vec![];
                                                        for _ in 1..=length {
                                                            acc.push(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(uuid::Uuid::new_v4()),
                                                            }));
                                                        }
                                                        acc
                                                    })
                                                    .expect("error 6b0491b2-1555-4f1c-81f7-5b22d7d353fb"),
                                                ),
                                                None,
                                            )
                                            .expect("error 5fb2b219-8bd7-4edd-9722-b475826707f5"),
                                            select_default_all_with_max_page_size.clone(),
                                            current_table,
                                        )
                                        .await
                                        {
                                            Ok(value) => assert!(value.is_empty(), "error 06df4025-e2d1-4128-b819-c06613c6ae3f"),
                                            Err(error) => {
                                                panic!("error e661c49b-2288-4548-8783-35495e193976: {error:#?}");
                                            }
                                        }
                                        let _: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read = TableExample::try_delete_one_handle(
                                            url,
                                            TableExampleDeleteOneParameters {
                                                payload: TableExampleDeleteOnePayload {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one_default.primary_key_column.clone()),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        .expect("error cc3958f0-1a4a-4440-97c7-ca63611405c5");
                                        if let Err(error) = TableExample::try_read_one_handle(
                                            url,
                                            TableExampleReadOneParameters {
                                                payload: TableExampleReadOnePayload {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one_default.primary_key_column.clone()),
                                                    select: select_default_all_with_max_page_size.clone(),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        {
                                            if let TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize, .. } = error {
                                                if let TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                                                    assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 99bd4d82-4976-4e1e-8022-543b01221a91");
                                                } else {
                                                    panic!("error 5a86690f-80e2-4dbc-9853-1826f94748bd");
                                                }
                                            } else {
                                                panic!("error d90d6d02-33f7-4886-99df-dee76b83400f")
                                            }
                                        } else {
                                            panic!("error 62e65598-2c1f-4912-b3a2-dccd6e1714a1")
                                        }
                                    }
                                    let lengths = vec![1, 2];
                                    for element in lengths {
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        let current_table = current_table.clone();
                                        let ident_create_default_cloned = ident_create_default.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            generate_test_read_many_by_non_existent_primary_keys(element, &url_cloned, select_default_all_with_max_page_size_cloned, &current_table, ident_create_default_cloned, no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row).await;
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_test_read_many_by_equal_to_created_primary_keys_cloned2.clone();
                                    async fn generate_test_read_many_by_equal_to_created_primary_keys(length: usize, url: &str, select_default_all_with_max_page_size: postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>, current_table: &str, ident_create_default: TableExampleCreate, no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row: &str) {
                                        let read_only_ids_from_try_create_one_default = TableExample::try_create_one_handle(url, TableExampleCreateOneParameters { payload: ident_create_default.clone() }, current_table).await.expect("error 71632985-ec25-4928-aa9e-1e224a7478c1");
                                        let ident_vec_create = {
                                            let mut acc = vec![];
                                            for _ in 1..=length {
                                                acc.push(ident_create_default.clone());
                                            }
                                            acc
                                        };
                                        let read_only_ids_from_try_create_many = TableExample::try_create_many_handle(url, TableExampleCreateManyParameters { payload: TableExampleCreateManyPayload(ident_vec_create.clone()) }, current_table).await.expect("error d775179f-f7b1-41d3-9c83-4ca8bd1abeec");
                                        assert_eq!(
                                            {
                                                let mut acc = vec![];
                                                assert_eq!(read_only_ids_from_try_create_many.len(), ident_vec_create.len(), "error 52c9d1ea-1593-4b32-97d1-0ed4a529a74a");
                                                for (read_only_ids, create) in read_only_ids_from_try_create_many.clone().into_iter().zip(ident_vec_create.into_iter()) {
                                                    acc.push(TableExampleRead {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids.primary_key_column),
                                                        column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_0.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), create.column_0),
                                                    });
                                                }
                                                acc.sort_by(|first, second| {
                                                    if let (Some(first), Some(second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                        first.value.cmp(&second.value)
                                                    } else {
                                                        panic!("error 0faa6fb3-a7c0-44ca-9b51-13f6ca2fc543");
                                                    }
                                                });
                                                acc
                                            },
                                            generate_try_read_many_order_by_primary_key_with_big_pagination(
                                                url,
                                                TableExampleWhereMany::try_new(
                                                    Some(
                                                        postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                            let mut acc = vec![];
                                                            for element in &read_only_ids_from_try_create_many {
                                                                acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(
                                                                        <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(element.primary_key_column.clone()),
                                                                    )),
                                                                }));
                                                            }
                                                            acc
                                                        })
                                                        .expect("error 6b0491b2-1555-4f1c-81f7-5b22d7d353fb"),
                                                    ),
                                                    None
                                                )
                                                .expect("error 5fb2b219-8bd7-4edd-9722-b475826707f5"),
                                                select_default_all_with_max_page_size.clone(),
                                                &current_table
                                            )
                                            .await
                                            .expect("error 82cb984b-8312-4952-a649-389f7c5adcff"),
                                            "error 3b2cf1f5-2c4e-4908-ba66-f4af84fe0893"
                                        );
                                        let read_only_ids_from_try_delete_many = {
                                            let mut acc = TableExample::try_delete_many_handle(
                                                url,
                                                TableExampleDeleteManyParameters {
                                                    payload: TableExampleDeleteManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                            primary_key_column: Some(
                                                                postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                    let mut acc = vec![];
                                                                    for element in &read_only_ids_from_try_create_many {
                                                                        acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(
                                                                                <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(element.primary_key_column.clone()),
                                                                            )),
                                                                        }));
                                                                    }
                                                                    acc
                                                                })
                                                                .expect("error dbfe049c-4142-469f-907c-4ecc5dd132dc"),
                                                            ),
                                                            column_0: None,
                                                        })),
                                                    },
                                                },
                                                current_table,
                                            )
                                            .await
                                            .expect("error d5c23a9d-eb02-44e4-8654-e2a3d7752f51");
                                            acc.sort();
                                            acc
                                        };
                                        assert_eq!(
                                            read_only_ids_from_try_delete_many,
                                            {
                                                let mut acc = read_only_ids_from_try_create_many
                                                    .into_iter()
                                                    .map(|element| <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(element.primary_key_column))
                                                    .collect::<Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>();
                                                acc.sort();
                                                acc
                                            },
                                            "error ebbbea6e-c402-4637-9bab-02678c11926c"
                                        );
                                        match generate_try_read_many_order_by_primary_key_with_big_pagination(
                                            url,
                                            TableExampleWhereMany::try_new(
                                                Some(
                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                        let mut acc = vec![];
                                                        for element in &read_only_ids_from_try_delete_many {
                                                            acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.clone())),
                                                            }));
                                                        }
                                                        acc
                                                    })
                                                    .expect("error 6b0491b2-1555-4f1c-81f7-5b22d7d353fb"),
                                                ),
                                                None,
                                            )
                                            .expect("error 5fb2b219-8bd7-4edd-9722-b475826707f5"),
                                            select_default_all_with_max_page_size.clone(),
                                            current_table,
                                        )
                                        .await
                                        {
                                            Ok(value) => assert!(value == Vec::new(), "error d79c0af3-5e2e-4891-a7ff-d1007b573e77"),
                                            Err(error) => {
                                                panic!("error 1f079962-06af-4d21-a837-c88b0e7db265 {error:#?}");
                                            }
                                        }
                                        let _: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read = TableExample::try_delete_one_handle(
                                            url,
                                            TableExampleDeleteOneParameters {
                                                payload: TableExampleDeleteOnePayload {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one_default.primary_key_column.clone()),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        .expect("error cc3958f0-1a4a-4440-97c7-ca63611405c5");
                                        if let Err(error) = TableExample::try_read_one_handle(
                                            url,
                                            TableExampleReadOneParameters {
                                                payload: TableExampleReadOnePayload {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one_default.primary_key_column.clone()),
                                                    select: select_default_all_with_max_page_size.clone(),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        {
                                            if let TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize, .. } = error {
                                                if let TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                                                    assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 99bd4d82-4976-4e1e-8022-543b01221a91");
                                                } else {
                                                    panic!("error 5a86690f-80e2-4dbc-9853-1826f94748bd");
                                                }
                                            } else {
                                                panic!("error d90d6d02-33f7-4886-99df-dee76b83400f")
                                            }
                                        } else {
                                            panic!("error 62e65598-2c1f-4912-b3a2-dccd6e1714a1")
                                        }
                                    }
                                    let lengths = vec![1, 2];
                                    for element in lengths {
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        let current_table = current_table.clone();
                                        let ident_create_default_cloned = ident_create_default.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            generate_test_read_many_by_equal_to_created_primary_keys(element, &url_cloned, select_default_all_with_max_page_size_cloned, &current_table, ident_create_default_cloned, no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row).await;
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0_cloned2.clone();
                                    for element in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate { column_0: element };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            assert_eq!(
                                                vec![TableExampleRead {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                    column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone())
                                                }],
                                                generate_try_read_many_order_by_primary_key_with_big_pagination(
                                                    &url_cloned,
                                                    TableExampleWhereMany::try_new(
                                                        Some(
                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                postgresql_crud::LogicalOperator::And,
                                                                vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                    read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                    postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                )]
                                                            )
                                                            .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                        ),
                                                        Some(
                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                postgresql_crud::LogicalOperator::And,
                                                                vec![<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                    read_only_ids_returned_from_create_one.column_0.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"),
                                                                    ident_create.column_0.clone()
                                                                )]
                                                            )
                                                            .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                        )
                                                    )
                                                    .expect("error 5fb2b219-8bd7-4edd-9722-b475826707f5"),
                                                    select_default_all_with_max_page_size_cloned.clone(),
                                                    &current_table
                                                )
                                                .await
                                                .expect("error 82cb984b-8312-4952-a649-389f7c5adcff"),
                                                "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                            );
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(
                                                                                <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                                            )),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(
                                                read_only_ids_from_try_delete_many,
                                                vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())],
                                                "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9"
                                            );
                                            match generate_try_read_many_order_by_primary_key_with_big_pagination(
                                                &url_cloned,
                                                TableExampleWhereMany::try_new(
                                                    Some(
                                                        postgresql_crud::PostgresqlTypeWhere::try_new(
                                                            postgresql_crud::LogicalOperator::Or,
                                                            vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(
                                                                    <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                                )),
                                                            })],
                                                        )
                                                        .expect("error 6b0491b2-1555-4f1c-81f7-5b22d7d353fb"),
                                                    ),
                                                    None,
                                                )
                                                .expect("error 5fb2b219-8bd7-4edd-9722-b475826707f5"),
                                                select_default_all_with_max_page_size_cloned.clone(),
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned2.clone();
                                    for element in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate { column_0: element };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            assert_eq!(
                                                vec![TableExampleRead {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                    column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone())
                                                }],
                                                generate_try_read_many_order_by_primary_key_with_big_pagination(
                                                    &url_cloned,
                                                    TableExampleWhereMany::try_new(
                                                        Some(
                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                postgresql_crud::LogicalOperator::And,
                                                                vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                    read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                    postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                )]
                                                            )
                                                            .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                        ),
                                                        Some(
                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                postgresql_crud::LogicalOperator::And,
                                                                <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_equal_using_fields(read_only_ids_returned_from_create_one.column_0.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"), ident_create.column_0.clone())
                                                            )
                                                            .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                        )
                                                    )
                                                    .expect("error 5fb2b219-8bd7-4edd-9722-b475826707f5"),
                                                    select_default_all_with_max_page_size_cloned.clone(),
                                                    &current_table
                                                )
                                                .await
                                                .expect("error 82cb984b-8312-4952-a649-389f7c5adcff"),
                                                "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                            );
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(
                                                                                <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                                            )),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(
                                                read_only_ids_from_try_delete_many,
                                                vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())],
                                                "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9"
                                            );
                                            match generate_try_read_many_order_by_primary_key_with_big_pagination(
                                                &url_cloned,
                                                TableExampleWhereMany::try_new(
                                                    Some(
                                                        postgresql_crud::PostgresqlTypeWhere::try_new(
                                                            postgresql_crud::LogicalOperator::Or,
                                                            vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(
                                                                    <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                                )),
                                                            })],
                                                        )
                                                        .expect("error 6b0491b2-1555-4f1c-81f7-5b22d7d353fb"),
                                                    ),
                                                    None,
                                                )
                                                .expect("error 5fb2b219-8bd7-4edd-9722-b475826707f5"),
                                                select_default_all_with_max_page_size_cloned.clone(),
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned2.clone();
                                    for element in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate { column_0: element };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field(read_only_ids_returned_from_create_one.column_0.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"), ident_create.column_0.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone())
                                                        }],
                                                        generate_try_read_many_order_by_primary_key_with_big_pagination(
                                                            &url_cloned,
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::And,
                                                                        vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                            read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                        )]
                                                                    )
                                                                    .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                ),
                                                                Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),)
                                                            )
                                                            .expect("error 5fb2b219-8bd7-4edd-9722-b475826707f5"),
                                                            select_default_all_with_max_page_size_cloned.clone(),
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 82cb984b-8312-4952-a649-389f7c5adcff"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(
                                                                                <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                                            )),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(
                                                read_only_ids_from_try_delete_many,
                                                vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())],
                                                "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9"
                                            );
                                            match generate_try_read_many_order_by_primary_key_with_big_pagination(
                                                &url_cloned,
                                                TableExampleWhereMany::try_new(
                                                    Some(
                                                        postgresql_crud::PostgresqlTypeWhere::try_new(
                                                            postgresql_crud::LogicalOperator::Or,
                                                            vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(
                                                                    <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                                )),
                                                            })],
                                                        )
                                                        .expect("error 6b0491b2-1555-4f1c-81f7-5b22d7d353fb"),
                                                    ),
                                                    None,
                                                )
                                                .expect("error 5fb2b219-8bd7-4edd-9722-b475826707f5"),
                                                select_default_all_with_max_page_size_cloned.clone(),
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                            };
                            acc
                        }),
                        100,
                        async |fut| {
                            fut.await;
                        },
                    )
                    .await;
                    drop_all_test_tables().await;
                });
            })
            .expect("error 4d329978-f5af-424e-8757-e8a32dbeb5a1")
            .join()
            .unwrap_or_else(|error| {
                panic!("error b2f21a5f-d9ce-435c-809f-bd40741c8795 {error:#?}");
            });
    }
}