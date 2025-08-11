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
            check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
    }
}]
#[postgresql_crud::create_many_additional_logic{
    println!("GeneratePostgresqlTypesExample create_many log");
}]
#[postgresql_crud::create_one_additional_logic{}]
#[postgresql_crud::read_many_additional_logic{}]
#[postgresql_crud::read_one_additional_logic{}]
#[postgresql_crud::update_many_additional_logic{}]
#[postgresql_crud::update_one_additional_logic{}]
#[postgresql_crud::delete_many_additional_logic{}]
#[postgresql_crud::delete_one_additional_logic{}]
#[postgresql_crud::common_additional_logic{}]
pub struct Example {
    // #[generate_postgresql_crud_primary_key]
    // pub primary_key: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql,
    #[generate_postgresql_crud_primary_key]
    pub primary_key_column: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql,

    pub column_0: postgresql_crud::postgresql_type::StdPrimitiveI16AsNotNullInt2,
    // pub column_1: postgresql_crud::postgresql_type::OptionStdPrimitiveI16AsNullableInt2,
    // pub column_2: postgresql_crud::postgresql_type::VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2,
    // pub column_3: postgresql_crud::postgresql_type::OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2,
    // pub column_4: postgresql_crud::postgresql_type::VecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableInt2,
    // pub column_5: postgresql_crud::postgresql_type::OptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableInt2,
    // pub column_6: postgresql_crud::postgresql_type::StdPrimitiveI32AsNotNullInt4,
    // pub column_7: postgresql_crud::postgresql_type::OptionStdPrimitiveI32AsNullableInt4,
    // pub column_8: postgresql_crud::postgresql_type::VecOfStdPrimitiveI32AsNotNullArrayOfNotNullInt4,
    // pub column_9: postgresql_crud::postgresql_type::OptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullInt4,
    // pub column_10: postgresql_crud::postgresql_type::VecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableInt4,
    // pub column_11: postgresql_crud::postgresql_type::OptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableInt4,
    // pub column_12: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullInt8,
    // pub column_13: postgresql_crud::postgresql_type::OptionStdPrimitiveI64AsNullableInt8,
    // pub column_14: postgresql_crud::postgresql_type::VecOfStdPrimitiveI64AsNotNullArrayOfNotNullInt8,
    // pub column_15: postgresql_crud::postgresql_type::OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullInt8,
    // pub column_16: postgresql_crud::postgresql_type::VecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableInt8,
    // pub column_17: postgresql_crud::postgresql_type::OptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableInt8,
    // pub column_18: postgresql_crud::postgresql_type::StdPrimitiveF32AsNotNullFloat4,
    // pub column_19: postgresql_crud::postgresql_type::OptionStdPrimitiveF32AsNullableFloat4,
    // pub column_20: postgresql_crud::postgresql_type::VecOfStdPrimitiveF32AsNotNullArrayOfNotNullFloat4,
    // pub column_21: postgresql_crud::postgresql_type::OptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullFloat4,
    // pub column_22: postgresql_crud::postgresql_type::VecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableFloat4,
    // pub column_23: postgresql_crud::postgresql_type::OptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableFloat4,
    // pub column_24: postgresql_crud::postgresql_type::StdPrimitiveF64AsNotNullFloat8,
    // pub column_25: postgresql_crud::postgresql_type::OptionStdPrimitiveF64AsNullableFloat8,
    // pub column_26: postgresql_crud::postgresql_type::VecOfStdPrimitiveF64AsNotNullArrayOfNotNullFloat8,
    // pub column_27: postgresql_crud::postgresql_type::OptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullFloat8,
    // pub column_28: postgresql_crud::postgresql_type::VecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableFloat8,
    // pub column_29: postgresql_crud::postgresql_type::OptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableFloat8,
    // pub column_30: postgresql_crud::postgresql_type::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql,
    // pub column_31: postgresql_crud::postgresql_type::StdPrimitiveI32AsNotNullSerialInitializedByPostgresql,
    // pub column_32: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql,
    // pub column_33: postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsNotNullMoney,
    // pub column_34: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgMoneyAsNullableMoney,
    // pub column_35: postgresql_crud::postgresql_type::VecOfSqlxPostgresTypesPgMoneyAsNotNullArrayOfNotNullMoney,
    // pub column_36: postgresql_crud::postgresql_type::OptionVecOfSqlxPostgresTypesPgMoneyAsNullableArrayOfNotNullMoney,
    // pub column_37: postgresql_crud::postgresql_type::VecOfOptionSqlxPostgresTypesPgMoneyAsNotNullArrayOfNullableMoney,
    // pub column_38: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxPostgresTypesPgMoneyAsNullableArrayOfNullableMoney,
    // pub column_39: postgresql_crud::postgresql_type::StdPrimitiveBoolAsNotNullBool,
    // pub column_40: postgresql_crud::postgresql_type::OptionStdPrimitiveBoolAsNullableBool,
    // pub column_41: postgresql_crud::postgresql_type::VecOfStdPrimitiveBoolAsNotNullArrayOfNotNullBool,
    // pub column_42: postgresql_crud::postgresql_type::OptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullBool,
    // pub column_43: postgresql_crud::postgresql_type::VecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableBool,
    // pub column_44: postgresql_crud::postgresql_type::OptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableBool,
    // pub column_45: postgresql_crud::postgresql_type::StdStringStringAsNotNullText,
    // pub column_46: postgresql_crud::postgresql_type::OptionStdStringStringAsNullableText,
    // pub column_47: postgresql_crud::postgresql_type::VecOfStdStringStringAsNotNullArrayOfNotNullText,
    // pub column_48: postgresql_crud::postgresql_type::OptionVecOfStdStringStringAsNullableArrayOfNotNullText,
    // pub column_49: postgresql_crud::postgresql_type::VecOfOptionStdStringStringAsNotNullArrayOfNullableText,
    // pub column_50: postgresql_crud::postgresql_type::OptionVecOfOptionStdStringStringAsNullableArrayOfNullableText,
    // pub column_51: postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsNotNullBytea,
    // pub column_52: postgresql_crud::postgresql_type::OptionStdVecVecStdPrimitiveU8AsNullableBytea,
    // pub column_53: postgresql_crud::postgresql_type::VecOfStdVecVecStdPrimitiveU8AsNotNullArrayOfNotNullBytea,
    // pub column_54: postgresql_crud::postgresql_type::OptionVecOfStdVecVecStdPrimitiveU8AsNullableArrayOfNotNullBytea,
    // pub column_55: postgresql_crud::postgresql_type::VecOfOptionStdVecVecStdPrimitiveU8AsNotNullArrayOfNullableBytea,
    // pub column_56: postgresql_crud::postgresql_type::OptionVecOfOptionStdVecVecStdPrimitiveU8AsNullableArrayOfNullableBytea,
    // pub column_57: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsNotNullTime,
    // pub column_58: postgresql_crud::postgresql_type::OptionSqlxTypesChronoNaiveTimeAsNullableTime,
    // pub column_59: postgresql_crud::postgresql_type::VecOfSqlxTypesChronoNaiveTimeAsNotNullArrayOfNotNullTime,
    // pub column_60: postgresql_crud::postgresql_type::OptionVecOfSqlxTypesChronoNaiveTimeAsNullableArrayOfNotNullTime,
    // pub column_61: postgresql_crud::postgresql_type::VecOfOptionSqlxTypesChronoNaiveTimeAsNotNullArrayOfNullableTime,
    // pub column_62: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxTypesChronoNaiveTimeAsNullableArrayOfNullableTime,
    // pub column_63: postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsNotNullTime,
    // pub column_64: postgresql_crud::postgresql_type::OptionSqlxTypesTimeTimeAsNullableTime,
    // pub column_65: postgresql_crud::postgresql_type::VecOfSqlxTypesTimeTimeAsNotNullArrayOfNotNullTime,
    // pub column_66: postgresql_crud::postgresql_type::OptionVecOfSqlxTypesTimeTimeAsNullableArrayOfNotNullTime,
    // pub column_67: postgresql_crud::postgresql_type::VecOfOptionSqlxTypesTimeTimeAsNotNullArrayOfNullableTime,
    // pub column_68: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxTypesTimeTimeAsNullableArrayOfNullableTime,
    // pub column_69: postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsNotNullInterval,
    // pub column_70: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgIntervalAsNullableInterval,
    // pub column_71: postgresql_crud::postgresql_type::VecOfSqlxPostgresTypesPgIntervalAsNotNullArrayOfNotNullInterval,
    // pub column_72: postgresql_crud::postgresql_type::OptionVecOfSqlxPostgresTypesPgIntervalAsNullableArrayOfNotNullInterval,
    // pub column_73: postgresql_crud::postgresql_type::VecOfOptionSqlxPostgresTypesPgIntervalAsNotNullArrayOfNullableInterval,
    // pub column_74: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxPostgresTypesPgIntervalAsNullableArrayOfNullableInterval,
    // pub column_75: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsNotNullDate,
    // pub column_76: postgresql_crud::postgresql_type::OptionSqlxTypesChronoNaiveDateAsNullableDate,
    // pub column_77: postgresql_crud::postgresql_type::VecOfSqlxTypesChronoNaiveDateAsNotNullArrayOfNotNullDate,
    // pub column_78: postgresql_crud::postgresql_type::OptionVecOfSqlxTypesChronoNaiveDateAsNullableArrayOfNotNullDate,
    // pub column_79: postgresql_crud::postgresql_type::VecOfOptionSqlxTypesChronoNaiveDateAsNotNullArrayOfNullableDate,
    // pub column_80: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxTypesChronoNaiveDateAsNullableArrayOfNullableDate,
    // pub column_81: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp,
    // pub column_82: postgresql_crud::postgresql_type::OptionSqlxTypesChronoNaiveDateTimeAsNullableTimestamp,
    // pub column_83: postgresql_crud::postgresql_type::VecOfSqlxTypesChronoNaiveDateTimeAsNotNullArrayOfNotNullTimestamp,
    // pub column_84: postgresql_crud::postgresql_type::OptionVecOfSqlxTypesChronoNaiveDateTimeAsNullableArrayOfNotNullTimestamp,
    // pub column_85: postgresql_crud::postgresql_type::VecOfOptionSqlxTypesChronoNaiveDateTimeAsNotNullArrayOfNullableTimestamp,
    // pub column_86: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxTypesChronoNaiveDateTimeAsNullableArrayOfNullableTimestamp,
    // pub column_87: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTz,
    // pub column_88: postgresql_crud::postgresql_type::OptionSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableTimestampTz,
    // pub column_89: postgresql_crud::postgresql_type::VecOfSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullArrayOfNotNullTimestampTz,
    // pub column_90: postgresql_crud::postgresql_type::OptionVecOfSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableArrayOfNotNullTimestampTz,
    // pub column_91: postgresql_crud::postgresql_type::VecOfOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullArrayOfNullableTimestampTz,
    // pub column_92: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableArrayOfNullableTimestampTz,
    // pub column_93: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql,
    // pub column_94: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidInitializedByClient,
    // pub column_95: postgresql_crud::postgresql_type::OptionSqlxTypesUuidUuidAsNullableUuidInitializedByClient,
    // pub column_96: postgresql_crud::postgresql_type::VecOfSqlxTypesUuidUuidAsNotNullArrayOfNotNullUuidInitializedByClient,
    // pub column_97: postgresql_crud::postgresql_type::OptionVecOfSqlxTypesUuidUuidAsNullableArrayOfNotNullUuidInitializedByClient,
    // pub column_98: postgresql_crud::postgresql_type::VecOfOptionSqlxTypesUuidUuidAsNotNullArrayOfNullableUuidInitializedByClient,
    // pub column_99: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxTypesUuidUuidAsNullableArrayOfNullableUuidInitializedByClient,
    // pub column_100: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsNotNullInet,
    // pub column_101: postgresql_crud::postgresql_type::OptionSqlxTypesIpnetworkIpNetworkAsNullableInet,
    // pub column_102: postgresql_crud::postgresql_type::VecOfSqlxTypesIpnetworkIpNetworkAsNotNullArrayOfNotNullInet,
    // pub column_103: postgresql_crud::postgresql_type::OptionVecOfSqlxTypesIpnetworkIpNetworkAsNullableArrayOfNotNullInet,
    // pub column_104: postgresql_crud::postgresql_type::VecOfOptionSqlxTypesIpnetworkIpNetworkAsNotNullArrayOfNullableInet,
    // pub column_105: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxTypesIpnetworkIpNetworkAsNullableArrayOfNullableInet,
    // pub column_106: postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsNotNullMacAddr,
    // pub column_107: postgresql_crud::postgresql_type::OptionSqlxTypesMacAddressMacAddressAsNullableMacAddr,
    // pub column_108: postgresql_crud::postgresql_type::VecOfSqlxTypesMacAddressMacAddressAsNotNullArrayOfNotNullMacAddr,
    // pub column_109: postgresql_crud::postgresql_type::OptionVecOfSqlxTypesMacAddressMacAddressAsNullableArrayOfNotNullMacAddr,
    // pub column_110: postgresql_crud::postgresql_type::VecOfOptionSqlxTypesMacAddressMacAddressAsNotNullArrayOfNullableMacAddr,
    // pub column_111: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxTypesMacAddressMacAddressAsNullableArrayOfNullableMacAddr,

    // pub column_112: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4Range,
    // pub column_113: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeStdPrimitiveI32AsNullableInt4Range,
    // pub column_114: postgresql_crud::postgresql_type::VecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNotNullInt4Range,
    // pub column_115: postgresql_crud::postgresql_type::OptionVecOfSqlxPostgresTypesPgRangeStdPrimitiveI32AsNullableArrayOfNotNullInt4Range,
    // pub column_116: postgresql_crud::postgresql_type::VecOfOptionSqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullArrayOfNullableInt4Range,
    // pub column_117: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxPostgresTypesPgRangeStdPrimitiveI32AsNullableArrayOfNullableInt4Range,
    // pub column_118: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsNotNullInt8Range,
    // pub column_119: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeStdPrimitiveI64AsNullableInt8Range,
    // pub column_120: postgresql_crud::postgresql_type::VecOfSqlxPostgresTypesPgRangeStdPrimitiveI64AsNotNullArrayOfNotNullInt8Range,
    // pub column_121: postgresql_crud::postgresql_type::OptionVecOfSqlxPostgresTypesPgRangeStdPrimitiveI64AsNullableArrayOfNotNullInt8Range,
    // pub column_122: postgresql_crud::postgresql_type::VecOfOptionSqlxPostgresTypesPgRangeStdPrimitiveI64AsNotNullArrayOfNullableInt8Range,
    // pub column_123: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxPostgresTypesPgRangeStdPrimitiveI64AsNullableArrayOfNullableInt8Range,
    // pub column_124: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNotNullDateRange,
    // pub column_125: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNullableDateRange,
    // pub column_126: postgresql_crud::postgresql_type::VecOfSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNotNullArrayOfNotNullDateRange,
    // pub column_127: postgresql_crud::postgresql_type::OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNullableArrayOfNotNullDateRange,
    // pub column_128: postgresql_crud::postgresql_type::VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNotNullArrayOfNullableDateRange,
    // pub column_129: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNullableArrayOfNullableDateRange,
    // pub column_130: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRange,
    // pub column_131: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNullableTimestampRange,
    // pub column_132: postgresql_crud::postgresql_type::VecOfSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullArrayOfNotNullTimestampRange,
    // pub column_133: postgresql_crud::postgresql_type::OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNullableArrayOfNotNullTimestampRange,
    // pub column_134: postgresql_crud::postgresql_type::VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullArrayOfNullableTimestampRange,
    // pub column_135: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNullableArrayOfNullableTimestampRange,
    // pub column_136: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzRange,
    // pub column_137: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableTimestampTzRange,
    // pub column_138: postgresql_crud::postgresql_type::VecOfSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullArrayOfNotNullTimestampTzRange,
    // pub column_139: postgresql_crud::postgresql_type::OptionVecOfSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableArrayOfNotNullTimestampTzRange,
    // pub column_140: postgresql_crud::postgresql_type::VecOfOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullArrayOfNullableTimestampTzRange,
    // pub column_141: postgresql_crud::postgresql_type::OptionVecOfOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableArrayOfNullableTimestampTzRange,

    pub column_154: crate::repositories_types::server::routes::api::example::AnimalAsNotNullJsonbObject,
    // pub column_155: crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId,
    // pub column_156: crate::repositories_types::server::routes::api::example::OptionAnimalAsNullableJsonbObject,
    // pub column_157: crate::repositories_types::server::routes::api::example::OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId,
}

#[derive(Debug, postgresql_crud::GeneratePostgresqlJsonObjectType)]
#[postgresql_crud::postgresql_json_object_type_pattern{
    // "All"
    {
        "Concrete": 
        // [
            {
                "not_null_or_nullable": "NotNull",
                "postgresql_json_object_type_pattern": "Standart",
                "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
            }
            // ,
            // {
            //     "not_null_or_nullable": "NotNull",
            //     "postgresql_json_object_type_pattern": "Array",
            //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
            // },
            // {
            //     "not_null_or_nullable": "Nullable",
            //     "postgresql_json_object_type_pattern": "Standart",
            //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
            // },
            // {
            //     "not_null_or_nullable": "Nullable",
            //     "postgresql_json_object_type_pattern": "Array",
            //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
            // }
        // ]
    }
}]
pub struct Animal {
    pub field_0: postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber,
    pub field_1: postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber,
    // pub field_2: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_3: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber,
    // pub field_4: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber,
    // pub field_5: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber,
    // pub field_6: postgresql_crud::postgresql_json_type::VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_7: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_8: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_9: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_10: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_11: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_12: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_13: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_14: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_15: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_16: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_17: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_18: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_19: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_20: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_21: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_22: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_23: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_24: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_25: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_26: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_27: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_28: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_29: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,

    // pub field_30: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_31: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_32: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_33: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_34: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_35: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_36: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_37: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_38: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_39: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_40: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_41: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_42: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_43: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_44: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_45: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_46: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_47: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_48: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_49: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_50: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_51: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_52: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_53: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_54: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_55: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_56: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_57: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_58: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_59: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_60: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_61: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_62: postgresql_crud::postgresql_json_type::StdPrimitiveI16AsNotNullJsonbNumber,
    // pub field_63: postgresql_crud::postgresql_json_type::OptionStdPrimitiveI16AsNullableJsonbNumber,
    // pub field_64: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_65: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableJsonbNumber,
    // pub field_66: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullJsonbNumber,
    // pub field_67: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableJsonbNumber,
    // pub field_68: postgresql_crud::postgresql_json_type::VecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_69: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_70: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_71: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_72: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_73: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_74: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_75: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_76: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_77: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_78: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_79: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_80: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_81: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_82: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_83: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_84: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_85: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_86: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_87: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_88: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_89: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_90: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_91: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_92: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_93: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_94: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_95: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_96: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_97: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_98: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_99: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_100: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_101: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_102: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_103: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_104: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_105: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_106: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_107: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_108: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_109: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_110: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_111: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_112: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_113: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_114: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_115: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_116: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_117: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_118: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_119: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_120: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_121: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_122: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_123: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_124: postgresql_crud::postgresql_json_type::StdPrimitiveI32AsNotNullJsonbNumber,
    // pub field_125: postgresql_crud::postgresql_json_type::OptionStdPrimitiveI32AsNullableJsonbNumber,
    // pub field_126: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveI32AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_127: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableJsonbNumber,
    // pub field_128: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullJsonbNumber,
    // pub field_129: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableJsonbNumber,
    // pub field_130: postgresql_crud::postgresql_json_type::VecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_131: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_132: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_133: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_134: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_135: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_136: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_137: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_138: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_139: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_140: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_141: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_142: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_143: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_144: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_145: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_146: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_147: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_148: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_149: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_150: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_151: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_152: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_153: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_154: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_155: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_156: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_157: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_158: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_159: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_160: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_161: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_162: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_163: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_164: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_165: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_166: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_167: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_168: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_169: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_170: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_171: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_172: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_173: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_174: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_175: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_176: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_177: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_178: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_179: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_180: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_181: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_182: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_183: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_184: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_185: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_186: postgresql_crud::postgresql_json_type::StdPrimitiveI64AsNotNullJsonbNumber,
    // pub field_187: postgresql_crud::postgresql_json_type::OptionStdPrimitiveI64AsNullableJsonbNumber,
    // pub field_188: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveI64AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_189: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableJsonbNumber,
    // pub field_190: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullJsonbNumber,
    // pub field_191: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableJsonbNumber,
    // pub field_192: postgresql_crud::postgresql_json_type::VecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_193: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_194: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_195: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_196: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_197: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_198: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_199: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_200: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_201: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_202: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_203: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_204: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_205: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_206: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_207: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_208: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_209: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_210: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_211: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_212: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_213: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_214: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_215: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_216: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_217: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_218: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_219: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_220: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_221: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_222: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_223: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_224: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_225: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_226: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_227: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_228: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_229: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_230: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_231: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_232: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_233: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_234: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_235: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_236: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_237: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_238: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_239: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_240: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_241: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_242: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_243: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_244: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_245: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_246: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_247: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_248: postgresql_crud::postgresql_json_type::StdPrimitiveU8AsNotNullJsonbNumber,
    // pub field_249: postgresql_crud::postgresql_json_type::OptionStdPrimitiveU8AsNullableJsonbNumber,
    // pub field_250: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveU8AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_251: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableJsonbNumber,
    // pub field_252: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullJsonbNumber,
    // pub field_253: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableJsonbNumber,
    // pub field_254: postgresql_crud::postgresql_json_type::VecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_255: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_256: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_257: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_258: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_259: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_260: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_261: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_262: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_263: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_264: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_265: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_266: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_267: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_268: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_269: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_270: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_271: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_272: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_273: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_274: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_275: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_276: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_277: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_278: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_279: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_280: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_281: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_282: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_283: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_284: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_285: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_286: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_287: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_288: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_289: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_290: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_291: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_292: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_293: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_294: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_295: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_296: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_297: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_298: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_299: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_300: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_301: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_302: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_303: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_304: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_305: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_306: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_307: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_308: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_309: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_310: postgresql_crud::postgresql_json_type::StdPrimitiveU16AsNotNullJsonbNumber,
    // pub field_311: postgresql_crud::postgresql_json_type::OptionStdPrimitiveU16AsNullableJsonbNumber,
    // pub field_312: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveU16AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_313: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableJsonbNumber,
    // pub field_314: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullJsonbNumber,
    // pub field_315: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableJsonbNumber,
    // pub field_316: postgresql_crud::postgresql_json_type::VecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_317: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_318: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_319: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_320: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_321: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_322: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_323: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_324: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_325: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_326: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_327: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_328: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_329: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_330: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_331: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_332: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_333: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_334: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_335: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_336: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_337: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_338: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_339: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_340: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_341: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_342: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_343: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_344: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_345: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_346: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_347: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_348: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_349: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_350: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_351: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_352: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_353: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_354: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_355: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_356: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_357: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_358: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_359: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_360: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_361: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_362: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_363: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_364: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_365: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_366: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_367: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_368: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_369: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_370: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_371: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_372: postgresql_crud::postgresql_json_type::StdPrimitiveU32AsNotNullJsonbNumber,
    // pub field_373: postgresql_crud::postgresql_json_type::OptionStdPrimitiveU32AsNullableJsonbNumber,
    // pub field_374: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveU32AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_375: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableJsonbNumber,
    // pub field_376: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullJsonbNumber,
    // pub field_377: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableJsonbNumber,
    // pub field_378: postgresql_crud::postgresql_json_type::VecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_379: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_380: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_381: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_382: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_383: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_384: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_385: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_386: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_387: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_388: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_389: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_390: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_391: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_392: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_393: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_394: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_395: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_396: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_397: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_398: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_399: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_400: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_401: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_402: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_403: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_404: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_405: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_406: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_407: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_408: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_409: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_410: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_411: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_412: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_413: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_414: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_415: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_416: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_417: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_418: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_419: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_420: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_421: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_422: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_423: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_424: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_425: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_426: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_427: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_428: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_429: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_430: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_431: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_432: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_433: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_434: postgresql_crud::postgresql_json_type::StdPrimitiveU64AsNotNullJsonbNumber,
    // pub field_435: postgresql_crud::postgresql_json_type::OptionStdPrimitiveU64AsNullableJsonbNumber,
    // pub field_436: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveU64AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_437: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableJsonbNumber,
    // pub field_438: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullJsonbNumber,
    // pub field_439: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableJsonbNumber,
    // pub field_440: postgresql_crud::postgresql_json_type::VecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_441: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_442: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_443: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_444: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_445: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_446: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_447: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_448: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_449: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_450: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_451: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_452: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_453: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_454: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_455: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_456: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_457: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_458: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_459: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_460: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_461: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_462: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_463: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_464: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_465: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_466: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_467: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_468: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_469: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_470: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_471: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_472: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_473: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_474: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_475: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_476: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_477: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_478: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_479: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_480: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_481: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_482: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_483: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_484: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_485: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_486: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_487: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_488: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_489: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_490: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_491: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_492: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_493: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_494: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_495: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_496: postgresql_crud::postgresql_json_type::StdPrimitiveF32AsNotNullJsonbNumber,
    // pub field_497: postgresql_crud::postgresql_json_type::OptionStdPrimitiveF32AsNullableJsonbNumber,
    // pub field_498: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveF32AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_499: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableJsonbNumber,
    // pub field_500: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullJsonbNumber,
    // pub field_501: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableJsonbNumber,
    // pub field_502: postgresql_crud::postgresql_json_type::VecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_503: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_504: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_505: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_506: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_507: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_508: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_509: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_510: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_511: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_512: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_513: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_514: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_515: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_516: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_517: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_518: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_519: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_520: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_521: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_522: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_523: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_524: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_525: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_526: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_527: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_528: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_529: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_530: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_531: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_532: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_533: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_534: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_535: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_536: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_537: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_538: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_539: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_540: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_541: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_542: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_543: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_544: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_545: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_546: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_547: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_548: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_549: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_550: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_551: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_552: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_553: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_554: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_555: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_556: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_557: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_558: postgresql_crud::postgresql_json_type::StdPrimitiveF64AsNotNullJsonbNumber,
    // pub field_559: postgresql_crud::postgresql_json_type::OptionStdPrimitiveF64AsNullableJsonbNumber,
    // pub field_560: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveF64AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_561: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableJsonbNumber,
    // pub field_562: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullJsonbNumber,
    // pub field_563: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableJsonbNumber,
    // pub field_564: postgresql_crud::postgresql_json_type::VecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_565: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_566: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_567: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_568: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_569: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_570: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_571: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_572: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_573: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_574: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_575: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_576: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_577: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_578: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_579: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_580: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_581: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_582: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_583: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_584: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_585: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_586: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_587: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_588: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_589: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_590: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_591: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_592: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_593: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_594: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_595: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_596: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_597: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_598: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_599: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_600: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_601: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_602: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_603: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_604: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_605: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_606: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_607: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_608: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_609: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_610: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_611: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_612: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_613: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_614: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_615: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_616: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_617: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_618: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_619: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_620: postgresql_crud::postgresql_json_type::StdPrimitiveBoolAsNotNullJsonbBoolean,
    // pub field_621: postgresql_crud::postgresql_json_type::OptionStdPrimitiveBoolAsNullableJsonbBoolean,
    // pub field_622: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveBoolAsNotNullArrayOfNotNullJsonbBoolean,
    // pub field_623: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableJsonbBoolean,
    // pub field_624: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullJsonbBoolean,
    // pub field_625: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableJsonbBoolean,
    // pub field_626: postgresql_crud::postgresql_json_type::VecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_627: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_628: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_629: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_630: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_631: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_632: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_633: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_634: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_635: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_636: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_637: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_638: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_639: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_640: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_641: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_642: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_643: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_644: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_645: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_646: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_647: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_648: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_649: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_650: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_651: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_652: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_653: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_654: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_655: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_656: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_657: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_658: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_659: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_660: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_661: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_662: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_663: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_664: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_665: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_666: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_667: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_668: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_669: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_670: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_671: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_672: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_673: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_674: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_675: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_676: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_677: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_678: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_679: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_680: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_681: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_682: postgresql_crud::postgresql_json_type::StdStringStringAsNotNullJsonbString,
    // pub field_683: postgresql_crud::postgresql_json_type::OptionStdStringStringAsNullableJsonbString,
    // pub field_684: postgresql_crud::postgresql_json_type::VecOfStdStringStringAsNotNullArrayOfNotNullJsonbString,
    // pub field_685: postgresql_crud::postgresql_json_type::VecOfOptionStdStringStringAsNotNullArrayOfNullableJsonbString,
    // pub field_686: postgresql_crud::postgresql_json_type::OptionVecOfStdStringStringAsNullableArrayOfNotNullJsonbString,
    // pub field_687: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdStringStringAsNullableArrayOfNullableJsonbString,
    // pub field_688: postgresql_crud::postgresql_json_type::VecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_689: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_690: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_691: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_692: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_693: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_694: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_695: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_696: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_697: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_698: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_699: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_700: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_701: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_702: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_703: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_704: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_705: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_706: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_707: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_708: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_709: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_710: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_711: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_712: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_713: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_714: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_715: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_716: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_717: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_718: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_719: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_720: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_721: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_722: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_723: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_724: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_725: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_726: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_727: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_728: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_729: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_730: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_731: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_732: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_733: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_734: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_735: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_736: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_737: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_738: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_739: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_740: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_741: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_742: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_743: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_744: postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString,
    // pub field_745: postgresql_crud::postgresql_json_type::OptionUuidUuidAsNullableJsonbString,
    // pub field_746: postgresql_crud::postgresql_json_type::VecOfUuidUuidAsNotNullArrayOfNotNullJsonbString,
    // pub field_747: postgresql_crud::postgresql_json_type::VecOfOptionUuidUuidAsNotNullArrayOfNullableJsonbString,
    // pub field_748: postgresql_crud::postgresql_json_type::OptionVecOfUuidUuidAsNullableArrayOfNotNullJsonbString,
    // pub field_749: postgresql_crud::postgresql_json_type::OptionVecOfOptionUuidUuidAsNullableArrayOfNullableJsonbString,
    // pub field_750: postgresql_crud::postgresql_json_type::VecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_751: postgresql_crud::postgresql_json_type::VecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_752: postgresql_crud::postgresql_json_type::VecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_753: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_754: postgresql_crud::postgresql_json_type::OptionVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_755: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_756: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_757: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_758: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_759: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_760: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_761: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_762: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_763: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_764: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_765: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_766: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_767: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_768: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_769: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_770: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_771: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_772: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_773: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_774: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_775: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_776: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_777: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_778: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_779: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_780: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_781: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_782: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_783: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_784: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_785: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_786: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_787: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_788: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_789: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_790: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_791: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_792: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_793: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_794: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_795: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_796: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_797: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_798: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_799: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_800: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_801: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_802: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_803: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_804: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_805: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,

    // pub field_806: DoggieAsNotNullJsonbObject,
    // pub field_807: VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithId,
    // pub field_808: OptionDoggieAsNullableJsonbObject,
    // pub field_809: OptionVecOfDoggieWithIdAsNullableArrayOfNotNullJsonbObjectWithId,
}

// #[derive(Debug
//     , postgresql_crud::GeneratePostgresqlJsonObjectType
// )]
// // #[postgresql_crud::postgresql_json_object_type_pattern{"All"}]
// #[postgresql_crud::postgresql_json_object_type_pattern{
//     // "All"
//     {
//         "Concrete":
//         // [
//             {
//                 "not_null_or_nullable": "NotNull",
//                 "postgresql_json_object_type_pattern": "Standart",
//                 "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             }
//             // ,
//             // {
//             //     "not_null_or_nullable": "NotNull",
//             //     "postgresql_json_object_type_pattern": "Array",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // },
//             // {
//             //     "not_null_or_nullable": "Nullable",
//             //     "postgresql_json_object_type_pattern": "Standart",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // },
//             // {
//             //     "not_null_or_nullable": "Nullable",
//             //     "postgresql_json_object_type_pattern": "Array",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // }
//         // ]
//     }
// }]
// pub struct Doggie {
//     pub field_0: postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber,
//     // pub field_1: postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber,
// }

//variant with multiple parameters
// #[cfg(test)]
// mod example_tests {
//     #[test]
//     fn test_size_of() {
//         assert_eq!(std::mem::size_of::<super::Example>(), 0);
//     }
//     #[test]
//     fn test_crud() {
//         std::thread::Builder::new()
//             .stack_size(16 * 1024 * 1024)
//             .spawn(|| {
//                 tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build().unwrap().block_on(async {
//                     static CONFIG: std::sync::OnceLock<crate::repositories_types::server::config::Config> = std::sync::OnceLock::new();
//                     let config = CONFIG.get_or_init(|| crate::repositories_types::server::config::Config::try_from_env().unwrap());
//                     let postgres_pool = sqlx::postgres::PgPoolOptions::new().connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&config))).await.unwrap();
//                     let url = format!("http://{}", app_state::GetServiceSocketAddress::get_service_socket_address(&config));
//                     async fn drop_table_if_exists(postgres_pool: &sqlx::Pool<sqlx::Postgres>) {
//                         let query = "drop table if exists example";
//                         println!("{query}");
//                         let _unused = sqlx::query(query).execute(postgres_pool).await.unwrap();
//                     }
//                     drop_table_if_exists(&postgres_pool).await;
//                     let postgres_pool_for_tokio_spawn_sync_move = postgres_pool.clone();
//                     let _unused = tokio::spawn(async move {
//                         super::Example::prepare_postgresql(&postgres_pool_for_tokio_spawn_sync_move).await.unwrap();
//                         let app_state = std::sync::Arc::new(crate::repositories_types::server::routes::app_state::AppState {
//                             postgres_pool: postgres_pool_for_tokio_spawn_sync_move.clone(),
//                             config: &config,
//                             project_git_info: &git_info::PROJECT_GIT_INFO,
//                         });
//                         axum::serve(
//                             tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(&config)).await.unwrap(),
//                             axum::Router::new().merge(super::Example::routes(std::sync::Arc::<crate::repositories_types::server::routes::app_state::AppState<'_>>::clone(&app_state))).into_make_service(),
//                         )
//                         .await
//                         .unwrap_or_else(|error| panic!("axum builder serve await failed {error:#?}"));
//                     });
//                     tokio::time::sleep(std::time::Duration::from_millis(100)).await;
//                     let
//                 ident_create_default = super :: ExampleCreate
//                 {
//                     column_81 : < < postgresql_crud :: postgresql_type ::
//                     SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp as
//                     postgresql_crud :: PostgresqlType > :: Create as
//                     postgresql_crud ::
//                     DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement >
//                     ::
//                     default_but_option_is_always_some_and_vec_always_contains_one_element()
//                 };
//                     let vec_of_primary_keys_returned_from_create_many = super::Example::try_create_many(
//                         &url,
//                         super::ExampleCreateManyParameters {
//                             payload: super::ExampleCreateManyPayload(vec![ident_create_default.clone(), ident_create_default.clone()]),
//                         },
//                     )
//                     .await
//                     .unwrap();
//                     assert_eq!(2, vec_of_primary_keys_returned_from_create_many.len(), "try_create_many result different");
//                     let (primary_key_read_returned_from_create_many1, primary_key_read_returned_from_create_many2) = {
//                         let mut iter = vec_of_primary_keys_returned_from_create_many.into_iter();
//                         (iter.next().unwrap(), iter.next().unwrap())
//                     };
//                     let select_primary_key = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![super::ExampleSelect::PrimaryKeyColumn(
//                         <postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default(),
//                     )])
//                     .unwrap();
//                     let where_many_1_and_2_primary_keys = super::StdOptionOptionExampleWhereMany(Some(super::ExampleWhereMany {
//                         primary_key_column: Some(
//                             postgresql_crud::PostgresqlTypeWhere::try_new(
//                                 postgresql_crud::LogicalOperator::Or,
//                                 vec![
//                                     <postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement::Equal(postgresql_crud::where_element_filters::PostgresqlTypeWhereElementEqual {
//                                         logical_operator: postgresql_crud::LogicalOperator::Or,
//                                         value: <postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::TableTypeDeclaration::from(primary_key_read_returned_from_create_many1.clone()),
//                                     }),
//                                     <postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement::Equal(postgresql_crud::where_element_filters::PostgresqlTypeWhereElementEqual {
//                                         logical_operator: postgresql_crud::LogicalOperator::Or,
//                                         value: <postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::TableTypeDeclaration::from(primary_key_read_returned_from_create_many2.clone()),
//                                     }),
//                                 ],
//                             )
//                             .unwrap(),
//                         ),
//                         column_81: None,
//                     }));
//                     let vec_of_ident_read_with_primary_key_sort_by_primary_key = |mut vec: std::vec::Vec<super::ExampleRead>| -> std::vec::Vec<super::ExampleRead> {
//                         vec.sort_by_key(|element| element.primary_key_column.clone().unwrap().value);
//                         vec
//                     };
//                     let vec_of_ident_read_returned_from_read_many = vec_of_ident_read_with_primary_key_sort_by_primary_key(
//                         super::Example::try_read_many(
//                             &url,
//                             super::ExampleReadManyParameters {
//                                 payload: super::ExampleReadManyPayload {
//                                     where_many: where_many_1_and_2_primary_keys.clone(),
//                                     select: select_primary_key.clone(),
//                                     order_by: postgresql_crud::OrderBy {
//                                         column: super::ExampleSelect::PrimaryKeyColumn(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//                                         order: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//                                     },
//                                     pagination: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                                 },
//                             },
//                         )
//                         .await
//                         .unwrap(),
//                     );
//                     let some_value_primary_key_read_returned_from_create_many1 = Some(postgresql_crud::Value {
//                         value: primary_key_read_returned_from_create_many1.clone(),
//                     });
//                     let some_value_primary_key_read_returned_from_create_many2 = Some(postgresql_crud::Value {
//                         value: primary_key_read_returned_from_create_many2.clone(),
//                     });
//                     assert_eq!(
//                         vec_of_ident_read_with_primary_key_sort_by_primary_key(vec![
//                             super::ExampleRead {
//                                 primary_key_column: some_value_primary_key_read_returned_from_create_many1.clone(),
//                                 column_81: None
//                             },
//                             super::ExampleRead {
//                                 primary_key_column: some_value_primary_key_read_returned_from_create_many2.clone(),
//                                 column_81: None
//                             }
//                         ]),
//                         vec_of_ident_read_returned_from_read_many,
//                         "try_read_many result different after try_create_many"
//                     );
//                     let primary_key_returned_from_create_one = super::Example::try_create_one(&url, super::ExampleCreateOneParameters { payload: ident_create_default }).await.unwrap();
//                     let primary_key_read_returned_from_create_one = primary_key_returned_from_create_one;
//                     let ident_read_returned_from_read_one = super::Example::try_read_one(
//                         &url,
//                         super::ExampleReadOneParameters {
//                             payload: super::ExampleReadOnePayload {
//                                 primary_key_column: primary_key_read_returned_from_create_one.clone(),
//                                 select: select_primary_key.clone(),
//                             },
//                         },
//                     )
//                     .await
//                     .unwrap();
//                     let some_value_primary_key_read_returned_from_create_one = Some(postgresql_crud::Value {
//                         value: primary_key_read_returned_from_create_one.clone(),
//                     });
//                     assert_eq!(
//                         super::ExampleRead {
//                             primary_key_column: some_value_primary_key_read_returned_from_create_one.clone(),
//                             column_81: None
//                         },
//                         ident_read_returned_from_read_one,
//                         "try_read_one result different after try_create_one"
//                     );
//                     for element in <postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp as postgresql_crud::tests::PostgresqlTypeTestCasesTwo>::test_cases_two() {
//                        let some_value_update = Some(postgresql_crud::Value {
//                             value: <postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp as postgresql_crud::PostgresqlType>::Update::new_or_try_new_unwraped_for_test_two(element.clone()),
//                         });
//                         let vec_of_primary_keys_returned_from_update_many = {
//                             let mut value = super::Example::try_update_many(
//                                 &url,
//                                 super::ExampleUpdateManyParameters {
//                                     payload: super::ExampleUpdateManyPayload::try_new(vec![
//                                         super::ExampleUpdate::try_new(
//                                             <postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(primary_key_read_returned_from_create_many1.clone()),
//                                             some_value_update.clone(),
//                                         )
//                                         .unwrap(),
//                                         super::ExampleUpdate::try_new(
//                                             <postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(primary_key_read_returned_from_create_many2.clone()),
//                                             some_value_update.clone(),
//                                         )
//                                         .unwrap(),
//                                     ])
//                                     .unwrap(),
//                                 },
//                             )
//                             .await
//                             .unwrap();
//                             value.sort();
//                             value
//                         };
//                         assert_eq!(
//                             {
//                                 let mut value = vec![primary_key_read_returned_from_create_many1.clone(), primary_key_read_returned_from_create_many2.clone()];
//                                 value.sort();
//                                 value
//                             },
//                             vec_of_primary_keys_returned_from_update_many,
//                             "try_update_many result different for column_81: postgresql_crud :: postgresql_type ::SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp"
//                         );
//                         let select_primary_key_field_ident = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec!
//                         [super :: ExampleSelect ::
//                         PrimaryKeyColumn(< < postgresql_crud :: postgresql_type ::
//                         SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as
//                         postgresql_crud :: PostgresqlType > :: Select as
//                         postgresql_crud ::
//                         DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement >
//                         ::
//                         default_but_option_is_always_some_and_vec_always_contains_one_element()),
//                         super :: ExampleSelect ::
//                         Column81(< < postgresql_crud :: postgresql_type ::
//                         SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp as
//                         postgresql_crud :: PostgresqlType > :: Select as
//                         postgresql_crud ::
//                         DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement >
//                         ::
//                         default_but_option_is_always_some_and_vec_always_contains_one_element()),])
//                         .unwrap();
//                         let vec_of_ident_read_returned_from_read_many = vec_of_ident_read_with_primary_key_sort_by_primary_key(
//                             super::Example::try_read_many(
//                                 &url,
//                                 super::ExampleReadManyParameters {
//                                     payload: super::ExampleReadManyPayload {
//                                         where_many: where_many_1_and_2_primary_keys.clone(),
//                                         select: select_primary_key_field_ident.clone(),
//                                         order_by: postgresql_crud::OrderBy {
//                                             column: super::ExampleSelect::PrimaryKeyColumn(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//                                             order: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//                                         },
//                                         pagination: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                                     },
//                                 },
//                             )
//                             .await
//                             .unwrap(),
//                         );
//                         let some_value_field_ident_read = Some(postgresql_crud::Value {
//                             value: <postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp as postgresql_crud::PostgresqlType>::Read::new_or_try_new_unwraped_for_test_two(element),
//                         });
//                         assert_eq!(
//                             vec_of_ident_read_with_primary_key_sort_by_primary_key(vec![
//                                 super::ExampleRead {
//                                     primary_key_column: some_value_primary_key_read_returned_from_create_many1.clone(),
//                                     column_81: some_value_field_ident_read.clone(),
//                                 },
//                                 super::ExampleRead {
//                                     primary_key_column: some_value_primary_key_read_returned_from_create_many2.clone(),
//                                     column_81: some_value_field_ident_read.clone(),
//                                 }
//                             ]),
//                             vec_of_ident_read_returned_from_read_many,
//                             "try_read_many result different after try_update_many for column_81: postgresql_crud :: postgresql_type ::SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp"
//                         );
//                         let primary_key_returned_from_update_one = super::Example::try_update_one(
//                             &url,
//                             super::ExampleUpdateOneParameters {
//                                 payload: super::ExampleUpdate::try_new(
//                                     <postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(primary_key_read_returned_from_create_one.clone()),
//                                     some_value_update.clone(),
//                                 )
//                                 .unwrap(),
//                             },
//                         )
//                         .await
//                         .unwrap();
//                         assert_eq!(
//                             primary_key_read_returned_from_create_one.clone(),
//                             primary_key_returned_from_update_one,
//                             "try_update_one result different for column_81: postgresql_crud :: postgresql_type ::SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp"
//                         );
//                         let ident_read_returned_from_read_one = super::Example::try_read_one(
//                             &url,
//                             super::ExampleReadOneParameters {
//                                 payload: super::ExampleReadOnePayload {
//                                     primary_key_column: primary_key_read_returned_from_create_one.clone(),
//                                     select: select_primary_key_field_ident.clone(),
//                                 },
//                             },
//                         )
//                         .await
//                         .unwrap();
//                         assert_eq!(
//                             super::ExampleRead {
//                                 primary_key_column: some_value_primary_key_read_returned_from_create_one.clone(),
//                                 column_81: some_value_field_ident_read.clone(),
//                             },
//                             ident_read_returned_from_read_one,
//                             "try_read_one result different after try_update_one for column_81: postgresql_crud :: postgresql_type ::SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp"
//                         );
//                     }
//                     let vec_of_primary_keys_returned_from_delete_many = {
//                         let mut value = super::Example::try_delete_many(
//                             &url,
//                             super::ExampleDeleteManyParameters {
//                                 payload: super::ExampleDeleteManyPayload { where_many: where_many_1_and_2_primary_keys.clone() },
//                             },
//                         )
//                         .await
//                         .unwrap();
//                         value.sort();
//                         value
//                     };
//                     assert_eq!(
//                         {
//                             let mut value = vec![primary_key_read_returned_from_create_many1.clone(), primary_key_read_returned_from_create_many2.clone()];
//                             value.sort();
//                             value
//                         },
//                         vec_of_primary_keys_returned_from_delete_many,
//                         "try_delete_many result different"
//                     );
//                     let vec_of_ident_read_returned_from_read_many = super::Example::try_read_many(
//                         &url,
//                         super::ExampleReadManyParameters {
//                             payload: super::ExampleReadManyPayload {
//                                 where_many: where_many_1_and_2_primary_keys.clone(),
//                                 select: select_primary_key.clone(),
//                                 order_by: postgresql_crud::OrderBy {
//                                     column: super::ExampleSelect::PrimaryKeyColumn(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//                                     order: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//                                 },
//                                 pagination: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                             },
//                         },
//                     )
//                     .await
//                     .unwrap();
//                     assert_eq!(std::vec::Vec::<super::ExampleRead>::default(), vec_of_ident_read_returned_from_read_many, "try_read_many result different after try_delete_many");
//                     let primary_key_returned_from_delete_one = super::Example::try_delete_one(
//                         &url,
//                         super::ExampleDeleteOneParameters {
//                             payload: super::ExampleDeleteOnePayload {
//                                 primary_key_column: primary_key_read_returned_from_create_one.clone(),
//                             },
//                         },
//                     )
//                     .await
//                     .unwrap();
//                     assert_eq!(primary_key_read_returned_from_create_one.clone(), primary_key_returned_from_delete_one, "try_delete_one result different");
//                     if let Err(error) = super::Example::try_read_one(
//                         &url,
//                         super::ExampleReadOneParameters {
//                             payload: super::ExampleReadOnePayload {
//                                 primary_key_column: primary_key_read_returned_from_create_one.clone(),
//                                 select: select_primary_key.clone(),
//                             },
//                         },
//                     )
//                     .await
//                         && let super::ExampleTryReadOneErrorNamed::ExampleReadOneErrorNamedWithSerializeDeserialize {
//                             read_one_error_named_with_serialize_deserialize,
//                             code_occurence: _,
//                         } = &error
//                         && let super::ExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence: _ } = &read_one_error_named_with_serialize_deserialize
//                         && "no rows returned by a query that expected to return at least one row".to_string() != *postgresql
//                     {
//                         panic!("try_read_one result different after try_delete_one");
//                     }
//                     drop_table_if_exists(&postgres_pool).await;
//                 });
//             })
//             .unwrap()
//             .join()
//             .unwrap();
//     }
// }
