#[derive(Debug
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
    // pub primary_key: postgresql_crud::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql,
    #[generate_postgresql_table_primary_key]
    pub primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql,

    // pub column_0: postgresql_crud::StdPrimitiveI16AsNotNullInt2,
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

    pub column_154: crate::repositories_types::server::routes::api::example::AnimalAsNotNullJsonbObject,
    pub column_155: crate::repositories_types::server::routes::api::example::OptionAnimalAsNullableJsonbObject,
    pub column_156: crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId,
    pub column_157: crate::repositories_types::server::routes::api::example::OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId,
}

#[derive(Debug
    // , postgresql_crud::GeneratePostgresqlJsonObjectType
)] //
// #[postgresql_crud::postgresql_json_object_type_pattern{
//     // "All"
//     {
//         "Concrete":
//         // [
//             // {
//             //     "not_null_or_nullable": "NotNull",
//             //     "postgresql_json_object_type_pattern": "Standart",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // }
//             // ,
//             // {
//             //     "not_null_or_nullable": "Nullable",
//             //     "postgresql_json_object_type_pattern": "Standart",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // }
//             // ,
//             // {
//             //     "not_null_or_nullable": "NotNull",
//             //     "postgresql_json_object_type_pattern": "Array",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // }
//             // ,
//             {
//                 "not_null_or_nullable": "Nullable",
//                 "postgresql_json_object_type_pattern": "Array",
//                 "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             }
//         // ]
//     }
// }]
pub struct Animal {
    pub field_0: postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber,
    pub field_1: postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber,
    // pub field_2: postgresql_crud::VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_3: postgresql_crud::VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber,
    // pub field_4: postgresql_crud::OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber,
    // pub field_5: postgresql_crud::OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber,
    // pub field_6: postgresql_crud::VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_7: postgresql_crud::VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_8: postgresql_crud::VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_9: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_10: postgresql_crud::OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_11: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_12: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_13: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_14: postgresql_crud::VecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_15: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_16: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_17: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_18: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_19: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_20: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_21: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_22: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_23: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_24: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_25: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_26: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_27: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_28: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_29: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,

    // pub field_30: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_31: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_32: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_33: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_34: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_35: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_36: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_37: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_38: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_39: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_40: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_41: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_42: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_43: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_44: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_45: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_46: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_47: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_48: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_49: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_50: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_51: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_52: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_53: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_54: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_55: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_56: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_57: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_58: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_59: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_60: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_61: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_62: postgresql_crud::StdPrimitiveI16AsNotNullJsonbNumber,
    // pub field_63: postgresql_crud::OptionStdPrimitiveI16AsNullableJsonbNumber,
    // pub field_64: postgresql_crud::VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_65: postgresql_crud::VecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableJsonbNumber,
    // pub field_66: postgresql_crud::OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullJsonbNumber,
    // pub field_67: postgresql_crud::OptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableJsonbNumber,
    // pub field_68: postgresql_crud::VecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_69: postgresql_crud::VecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_70: postgresql_crud::VecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_71: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_72: postgresql_crud::OptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_73: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_74: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_75: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_76: postgresql_crud::VecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_77: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_78: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_79: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_80: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_81: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_82: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_83: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_84: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_85: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_86: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_87: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_88: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_89: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_90: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_91: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_92: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_93: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_94: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_95: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_96: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_97: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_98: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_99: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_100: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_101: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_102: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_103: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_104: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_105: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_106: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_107: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_108: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_109: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_110: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_111: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_112: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_113: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_114: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_115: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_116: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_117: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_118: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_119: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_120: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_121: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_122: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_123: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_124: postgresql_crud::StdPrimitiveI32AsNotNullJsonbNumber,
    // pub field_125: postgresql_crud::OptionStdPrimitiveI32AsNullableJsonbNumber,
    // pub field_126: postgresql_crud::VecOfStdPrimitiveI32AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_127: postgresql_crud::VecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableJsonbNumber,
    // pub field_128: postgresql_crud::OptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullJsonbNumber,
    // pub field_129: postgresql_crud::OptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableJsonbNumber,
    // pub field_130: postgresql_crud::VecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_131: postgresql_crud::VecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_132: postgresql_crud::VecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_133: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_134: postgresql_crud::OptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_135: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_136: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_137: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_138: postgresql_crud::VecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_139: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_140: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_141: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_142: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_143: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_144: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_145: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_146: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_147: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_148: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_149: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_150: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_151: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_152: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_153: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_154: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_155: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_156: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_157: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_158: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_159: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_160: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_161: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_162: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_163: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_164: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_165: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_166: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_167: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_168: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_169: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_170: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_171: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_172: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_173: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_174: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_175: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_176: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_177: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_178: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_179: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_180: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_181: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_182: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_183: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_184: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_185: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_186: postgresql_crud::StdPrimitiveI64AsNotNullJsonbNumber,
    // pub field_187: postgresql_crud::OptionStdPrimitiveI64AsNullableJsonbNumber,
    // pub field_188: postgresql_crud::VecOfStdPrimitiveI64AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_189: postgresql_crud::VecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableJsonbNumber,
    // pub field_190: postgresql_crud::OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullJsonbNumber,
    // pub field_191: postgresql_crud::OptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableJsonbNumber,
    // pub field_192: postgresql_crud::VecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_193: postgresql_crud::VecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_194: postgresql_crud::VecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_195: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_196: postgresql_crud::OptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_197: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_198: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_199: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_200: postgresql_crud::VecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_201: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_202: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_203: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_204: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_205: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_206: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_207: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_208: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_209: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_210: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_211: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_212: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_213: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_214: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_215: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_216: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_217: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_218: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_219: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_220: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_221: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_222: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_223: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_224: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_225: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_226: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_227: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_228: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_229: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_230: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_231: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_232: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_233: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_234: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_235: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_236: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_237: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_238: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_239: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_240: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_241: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_242: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_243: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_244: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_245: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_246: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_247: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_248: postgresql_crud::StdPrimitiveU8AsNotNullJsonbNumber,
    // pub field_249: postgresql_crud::OptionStdPrimitiveU8AsNullableJsonbNumber,
    // pub field_250: postgresql_crud::VecOfStdPrimitiveU8AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_251: postgresql_crud::VecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableJsonbNumber,
    // pub field_252: postgresql_crud::OptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullJsonbNumber,
    // pub field_253: postgresql_crud::OptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableJsonbNumber,
    // pub field_254: postgresql_crud::VecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_255: postgresql_crud::VecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_256: postgresql_crud::VecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_257: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_258: postgresql_crud::OptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_259: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_260: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_261: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_262: postgresql_crud::VecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_263: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_264: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_265: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_266: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_267: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_268: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_269: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_270: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_271: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_272: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_273: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_274: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_275: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_276: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_277: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_278: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_279: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_280: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_281: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_282: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_283: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_284: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_285: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_286: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_287: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_288: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_289: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_290: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_291: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_292: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_293: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_294: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_295: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_296: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_297: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_298: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_299: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_300: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_301: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_302: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_303: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_304: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_305: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_306: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_307: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_308: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_309: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_310: postgresql_crud::StdPrimitiveU16AsNotNullJsonbNumber,
    // pub field_311: postgresql_crud::OptionStdPrimitiveU16AsNullableJsonbNumber,
    // pub field_312: postgresql_crud::VecOfStdPrimitiveU16AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_313: postgresql_crud::VecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableJsonbNumber,
    // pub field_314: postgresql_crud::OptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullJsonbNumber,
    // pub field_315: postgresql_crud::OptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableJsonbNumber,
    // pub field_316: postgresql_crud::VecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_317: postgresql_crud::VecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_318: postgresql_crud::VecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_319: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_320: postgresql_crud::OptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_321: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_322: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_323: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_324: postgresql_crud::VecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_325: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_326: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_327: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_328: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_329: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_330: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_331: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_332: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_333: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_334: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_335: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_336: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_337: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_338: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_339: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_340: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_341: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_342: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_343: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_344: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_345: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_346: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_347: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_348: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_349: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_350: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_351: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_352: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_353: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_354: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_355: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_356: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_357: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_358: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_359: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_360: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_361: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_362: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_363: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_364: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_365: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_366: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_367: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_368: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_369: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_370: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_371: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_372: postgresql_crud::StdPrimitiveU32AsNotNullJsonbNumber,
    // pub field_373: postgresql_crud::OptionStdPrimitiveU32AsNullableJsonbNumber,
    // pub field_374: postgresql_crud::VecOfStdPrimitiveU32AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_375: postgresql_crud::VecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableJsonbNumber,
    // pub field_376: postgresql_crud::OptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullJsonbNumber,
    // pub field_377: postgresql_crud::OptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableJsonbNumber,
    // pub field_378: postgresql_crud::VecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_379: postgresql_crud::VecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_380: postgresql_crud::VecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_381: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_382: postgresql_crud::OptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_383: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_384: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_385: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_386: postgresql_crud::VecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_387: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_388: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_389: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_390: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_391: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_392: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_393: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_394: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_395: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_396: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_397: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_398: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_399: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_400: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_401: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_402: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_403: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_404: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_405: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_406: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_407: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_408: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_409: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_410: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_411: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_412: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_413: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_414: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_415: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_416: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_417: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_418: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_419: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_420: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_421: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_422: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_423: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_424: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_425: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_426: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_427: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_428: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_429: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_430: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_431: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_432: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_433: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_434: postgresql_crud::StdPrimitiveU64AsNotNullJsonbNumber,
    // pub field_435: postgresql_crud::OptionStdPrimitiveU64AsNullableJsonbNumber,
    // pub field_436: postgresql_crud::VecOfStdPrimitiveU64AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_437: postgresql_crud::VecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableJsonbNumber,
    // pub field_438: postgresql_crud::OptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullJsonbNumber,
    // pub field_439: postgresql_crud::OptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableJsonbNumber,
    // pub field_440: postgresql_crud::VecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_441: postgresql_crud::VecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_442: postgresql_crud::VecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_443: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_444: postgresql_crud::OptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_445: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_446: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_447: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_448: postgresql_crud::VecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_449: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_450: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_451: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_452: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_453: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_454: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_455: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_456: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_457: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_458: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_459: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_460: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_461: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_462: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_463: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_464: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_465: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_466: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_467: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_468: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_469: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_470: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_471: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_472: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_473: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_474: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_475: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_476: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_477: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_478: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_479: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_480: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_481: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_482: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_483: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_484: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_485: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_486: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_487: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_488: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_489: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_490: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_491: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_492: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_493: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_494: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_495: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_496: postgresql_crud::StdPrimitiveF32AsNotNullJsonbNumber,
    // pub field_497: postgresql_crud::OptionStdPrimitiveF32AsNullableJsonbNumber,
    // pub field_498: postgresql_crud::VecOfStdPrimitiveF32AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_499: postgresql_crud::VecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableJsonbNumber,
    // pub field_500: postgresql_crud::OptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullJsonbNumber,
    // pub field_501: postgresql_crud::OptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableJsonbNumber,
    // pub field_502: postgresql_crud::VecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_503: postgresql_crud::VecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_504: postgresql_crud::VecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_505: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_506: postgresql_crud::OptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_507: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_508: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_509: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_510: postgresql_crud::VecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_511: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_512: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_513: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_514: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_515: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_516: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_517: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_518: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_519: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_520: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_521: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_522: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_523: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_524: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_525: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_526: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_527: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_528: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_529: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_530: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_531: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_532: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_533: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_534: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_535: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_536: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_537: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_538: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_539: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_540: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_541: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_542: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_543: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_544: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_545: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_546: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_547: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_548: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_549: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_550: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_551: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_552: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_553: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_554: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_555: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_556: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_557: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_558: postgresql_crud::StdPrimitiveF64AsNotNullJsonbNumber,
    // pub field_559: postgresql_crud::OptionStdPrimitiveF64AsNullableJsonbNumber,
    // pub field_560: postgresql_crud::VecOfStdPrimitiveF64AsNotNullArrayOfNotNullJsonbNumber,
    // pub field_561: postgresql_crud::VecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableJsonbNumber,
    // pub field_562: postgresql_crud::OptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullJsonbNumber,
    // pub field_563: postgresql_crud::OptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableJsonbNumber,
    // pub field_564: postgresql_crud::VecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_565: postgresql_crud::VecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_566: postgresql_crud::VecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_567: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_568: postgresql_crud::OptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_569: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_570: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_571: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_572: postgresql_crud::VecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_573: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_574: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_575: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_576: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_577: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_578: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_579: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_580: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_581: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_582: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_583: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_584: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_585: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_586: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_587: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_588: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_589: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_590: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_591: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_592: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_593: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_594: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_595: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_596: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_597: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_598: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_599: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_600: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_601: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_602: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_603: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_604: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_605: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_606: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_607: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_608: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_609: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_610: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_611: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_612: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_613: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_614: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_615: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_616: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub field_617: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub field_618: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub field_619: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub field_620: postgresql_crud::StdPrimitiveBoolAsNotNullJsonbBoolean,
    // pub field_621: postgresql_crud::OptionStdPrimitiveBoolAsNullableJsonbBoolean,
    // pub field_622: postgresql_crud::VecOfStdPrimitiveBoolAsNotNullArrayOfNotNullJsonbBoolean,
    // pub field_623: postgresql_crud::VecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableJsonbBoolean,
    // pub field_624: postgresql_crud::OptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullJsonbBoolean,
    // pub field_625: postgresql_crud::OptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableJsonbBoolean,
    // pub field_626: postgresql_crud::VecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_627: postgresql_crud::VecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_628: postgresql_crud::VecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_629: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_630: postgresql_crud::OptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_631: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_632: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_633: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_634: postgresql_crud::VecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_635: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_636: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_637: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_638: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_639: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_640: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_641: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_642: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_643: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_644: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_645: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_646: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_647: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_648: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_649: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_650: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_651: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_652: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_653: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_654: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_655: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_656: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_657: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_658: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_659: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_660: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_661: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_662: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_663: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_664: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_665: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_666: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_667: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_668: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_669: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_670: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_671: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_672: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_673: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_674: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_675: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_676: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_677: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_678: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
    // pub field_679: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
    // pub field_680: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
    // pub field_681: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
    // pub field_682: postgresql_crud::StdStringStringAsNotNullJsonbString,
    // pub field_683: postgresql_crud::OptionStdStringStringAsNullableJsonbString,
    // pub field_684: postgresql_crud::VecOfStdStringStringAsNotNullArrayOfNotNullJsonbString,
    // pub field_685: postgresql_crud::VecOfOptionStdStringStringAsNotNullArrayOfNullableJsonbString,
    // pub field_686: postgresql_crud::OptionVecOfStdStringStringAsNullableArrayOfNotNullJsonbString,
    // pub field_687: postgresql_crud::OptionVecOfOptionStdStringStringAsNullableArrayOfNullableJsonbString,
    // pub field_688: postgresql_crud::VecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_689: postgresql_crud::VecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_690: postgresql_crud::VecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_691: postgresql_crud::VecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_692: postgresql_crud::OptionVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_693: postgresql_crud::OptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_694: postgresql_crud::OptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_695: postgresql_crud::OptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_696: postgresql_crud::VecOfVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_697: postgresql_crud::VecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_698: postgresql_crud::VecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_699: postgresql_crud::VecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_700: postgresql_crud::VecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_701: postgresql_crud::VecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_702: postgresql_crud::VecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_703: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_704: postgresql_crud::OptionVecOfVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_705: postgresql_crud::OptionVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_706: postgresql_crud::OptionVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_707: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_708: postgresql_crud::OptionVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_709: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_710: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_711: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_712: postgresql_crud::VecOfVecOfVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_713: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_714: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_715: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_716: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_717: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_718: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_719: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_720: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_721: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_722: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_723: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_724: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_725: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_726: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_727: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_728: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_729: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_730: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_731: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_732: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_733: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_734: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_735: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_736: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_737: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_738: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_739: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_740: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_741: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_742: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_743: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_744: postgresql_crud::UuidUuidAsNotNullJsonbString,
    // pub field_745: postgresql_crud::OptionUuidUuidAsNullableJsonbString,
    // pub field_746: postgresql_crud::VecOfUuidUuidAsNotNullArrayOfNotNullJsonbString,
    // pub field_747: postgresql_crud::VecOfOptionUuidUuidAsNotNullArrayOfNullableJsonbString,
    // pub field_748: postgresql_crud::OptionVecOfUuidUuidAsNullableArrayOfNotNullJsonbString,
    // pub field_749: postgresql_crud::OptionVecOfOptionUuidUuidAsNullableArrayOfNullableJsonbString,
    // pub field_750: postgresql_crud::VecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_751: postgresql_crud::VecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_752: postgresql_crud::VecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_753: postgresql_crud::VecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_754: postgresql_crud::OptionVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_755: postgresql_crud::OptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_756: postgresql_crud::OptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_757: postgresql_crud::OptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_758: postgresql_crud::VecOfVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_759: postgresql_crud::VecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_760: postgresql_crud::VecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_761: postgresql_crud::VecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_762: postgresql_crud::VecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_763: postgresql_crud::VecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_764: postgresql_crud::VecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_765: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_766: postgresql_crud::OptionVecOfVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_767: postgresql_crud::OptionVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_768: postgresql_crud::OptionVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_769: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_770: postgresql_crud::OptionVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_771: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_772: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_773: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_774: postgresql_crud::VecOfVecOfVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_775: postgresql_crud::VecOfVecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_776: postgresql_crud::VecOfVecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_777: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_778: postgresql_crud::VecOfVecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_779: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_780: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_781: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_782: postgresql_crud::VecOfOptionVecOfVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_783: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_784: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_785: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_786: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_787: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_788: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_789: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_790: postgresql_crud::OptionVecOfVecOfVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_791: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_792: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_793: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_794: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_795: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_796: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_797: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
    // pub field_798: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_799: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_800: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_801: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
    // pub field_802: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
    // pub field_803: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
    // pub field_804: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
    // pub field_805: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,

    // pub field_806: DoggieAsNotNullJsonbObject,
    // pub field_808: OptionDoggieAsNullableJsonbObject,
    // pub field_807: VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithId,
    // pub field_809: OptionVecOfDoggieWithIdAsNullableArrayOfNotNullJsonbObjectWithId,
}

// #[derive(Debug, postgresql_crud::GeneratePostgresqlJsonObjectType)]
// // #[postgresql_crud::postgresql_json_object_type_pattern{"All"}]
// #[postgresql_crud::postgresql_json_object_type_pattern{
//     // "All"
//     {
//         "Concrete":
//         // [
//             // {
//             //     "not_null_or_nullable": "NotNull",
//             //     "postgresql_json_object_type_pattern": "Standart",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // },
//             // {
//             //     "not_null_or_nullable": "Nullable",
//             //     "postgresql_json_object_type_pattern": "Standart",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // },
//             // {
//             //     "not_null_or_nullable": "NotNull",
//             //     "postgresql_json_object_type_pattern": "Array",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // },
//             {
//                 "not_null_or_nullable": "Nullable",
//                 "postgresql_json_object_type_pattern": "Array",
//                 "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             }
//         // ]
//     }
// }]
// pub struct Doggie {
//     // pub field_0: postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber,
//     // pub field_1: postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber,
//     // pub field_2: postgresql_crud::VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber,
//     // pub field_3: postgresql_crud::VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber,
//     // pub field_4: postgresql_crud::OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber,
//     // pub field_5: postgresql_crud::OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber,
//     // pub field_6: postgresql_crud::VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_7: postgresql_crud::VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_8: postgresql_crud::VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_9: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_10: postgresql_crud::OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_11: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_12: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_13: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_14: postgresql_crud::VecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_15: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_16: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_17: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_18: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_19: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_20: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_21: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_22: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_23: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_24: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_25: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_26: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_27: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_28: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_29: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_30: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_31: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_32: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_33: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_34: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_35: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_36: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_37: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_38: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_39: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_40: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_41: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_42: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_43: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_44: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_45: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_46: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_47: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_48: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_49: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_50: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_51: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_52: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_53: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_54: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_55: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_56: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_57: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_58: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_59: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_60: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_61: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_62: postgresql_crud::StdPrimitiveI16AsNotNullJsonbNumber,
//     // pub field_63: postgresql_crud::OptionStdPrimitiveI16AsNullableJsonbNumber,
//     // pub field_64: postgresql_crud::VecOfStdPrimitiveI16AsNotNullArrayOfNotNullJsonbNumber,
//     // pub field_65: postgresql_crud::VecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableJsonbNumber,
//     // pub field_66: postgresql_crud::OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullJsonbNumber,
//     // pub field_67: postgresql_crud::OptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableJsonbNumber,
//     // pub field_68: postgresql_crud::VecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_69: postgresql_crud::VecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_70: postgresql_crud::VecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_71: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_72: postgresql_crud::OptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_73: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_74: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_75: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_76: postgresql_crud::VecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_77: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_78: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_79: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_80: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_81: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_82: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_83: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_84: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_85: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_86: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_87: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_88: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_89: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_90: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_91: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_92: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_93: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_94: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_95: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_96: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_97: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_98: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_99: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_100: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_101: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_102: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_103: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_104: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_105: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_106: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_107: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_108: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_109: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_110: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_111: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_112: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_113: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_114: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_115: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_116: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_117: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_118: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_119: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_120: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_121: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_122: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_123: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,

//     pub field_806: CatAsNotNullJsonbObject,
//     pub field_807: OptionCatAsNullableJsonbObject,
//     pub field_808: VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId,
//     pub field_809: OptionVecOfCatWithIdAsNullableArrayOfNotNullJsonbObjectWithId,
// }

// #[derive(Debug, postgresql_crud::GeneratePostgresqlJsonObjectType)]
// // #[postgresql_crud::postgresql_json_object_type_pattern{"All"}]
// #[postgresql_crud::postgresql_json_object_type_pattern{
//     // "All"
//     {
//         "Concrete":
//         // [
//             // {
//             //     "not_null_or_nullable": "NotNull",
//             //     "postgresql_json_object_type_pattern": "Standart",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // },
//             // {
//             //     "not_null_or_nullable": "Nullable",
//             //     "postgresql_json_object_type_pattern": "Standart",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // },
//             // {
//             //     "not_null_or_nullable": "NotNull",
//             //     "postgresql_json_object_type_pattern": "Array",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // },
//             {
//                 "not_null_or_nullable": "Nullable",
//                 "postgresql_json_object_type_pattern": "Array",
//                 "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             }
//         // ]
//     }
// }]
// pub struct Cat {
//     pub field_0: postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber,
// }


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
//                 tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build().expect("error 38823c21-1879-449c-9b60-ce7293709959").block_on(async {
//                     tracing_subscriber::fmt::init();
//                     static CONFIG: std::sync::OnceLock<crate::repositories_types::server::config::Config> = std::sync::OnceLock::new();
//                     let config = CONFIG.get_or_init(|| crate::repositories_types::server::config::Config::try_from_env().expect("error d7a6ef78-c306-40e7-b560-297ce4e8a8d1"));
//                     let postgres_pool = sqlx::postgres::PgPoolOptions::new().max_connections(50).connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&config))).await.expect("error e3044bb9-7b76-4c0c-bc5f-eb34da05a103");
//                     let url = format!("http://{}", app_state::GetServiceSocketAddress::get_service_socket_address(&config));
//                     async fn drop_table_if_exists(postgres_pool: &sqlx::Pool<sqlx::Postgres>) {
//                         let query = "drop table if exists example";
//                         println!("{query}");
//                         let _unused = sqlx::query(query).execute(postgres_pool).await.expect("error 1b11bf1b-9180-419f-bae7-b1ab93cd9c57");
//                     }
//                     drop_table_if_exists(&postgres_pool).await;
//                     let postgres_pool_for_tokio_spawn_sync_move = postgres_pool.clone();
//                     let _unused = tokio::spawn(async move {
//                         super::Example::prepare_postgresql(&postgres_pool_for_tokio_spawn_sync_move).await.expect("error 0a7889da-c2b5-4205-adf1-75904ad80cc0");
//                         let app_state = std::sync::Arc::new(crate::repositories_types::server::routes::app_state::AppState { postgres_pool: postgres_pool_for_tokio_spawn_sync_move.clone(), config: &config, project_git_info: &git_info::PROJECT_GIT_INFO });
//                         axum::serve(tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(&config)).await.expect("error 663ae29e-bc00-4ea1-a7e9-4dddceb5b53a"), axum::Router::new().merge(super::Example::routes(std::sync::Arc::<crate::repositories_types::server::routes::app_state::AppState<'_>>::clone(&app_state))).into_make_service()).await.unwrap_or_else(|error| panic!("axum builder serve await failed {error:#?}"));
//                     });
//                     tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

//                     let select_primary_key = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default())]).expect("error 0776170e-4dd6-4c14-a412-ce10b0c746f1");
//                     let sort_vec_of_ident_read_with_primary_key_by_primary_key = |mut value: std::vec::Vec<super::ExampleRead>| -> std::vec::Vec<super::ExampleRead> {
//                         value.sort_by_key(|element| element.primary_key_column.clone().expect("error 4f25860e-5b1a-408f-a4db-d49b6969ad4a").value);
//                         value
//                     };


//                     let ident_create_default = super::ExampleCreate {
//                         column_154: <<crate::repositories_types::server::routes::api::example::AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                         column_155: <<crate::repositories_types::server::routes::api::example::OptionAnimalAsNullableJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                         column_156: <<crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                         column_157: <<crate::repositories_types::server::routes::api::example::OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                     };

//                     let common_read_only_ids_returned_from_create_one = super::Example::try_create_one(&url, super::ExampleCreateOneParameters { payload: ident_create_default.clone() }).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
//                     let some_value_read_only_ids_returned_from_create_one = Some(postgresql_crud::Value { value: common_read_only_ids_returned_from_create_one.primary_key_column.clone() });
//                     assert_eq!(super::ExampleRead { primary_key_column: some_value_read_only_ids_returned_from_create_one.clone(), column_154: None, column_155: None, column_156: None, column_157: None }, super::Example::try_read_one(&url, super::ExampleReadOneParameters { payload: super::ExampleReadOnePayload { primary_key_column: common_read_only_ids_returned_from_create_one.primary_key_column.clone(), select: select_primary_key.clone() } },).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b"), "try_read_one result different after try_create_one");

//                     // let read_only_ids_returned_from_create_many
//                     let read_only_ids_vec = {
//                         let acc = {
//                             let mut acc = vec![];
//                             if let Some(value) = &common_read_only_ids_returned_from_create_one.column_154 {
//                                 for element0 in <crate::repositories_types::server::routes::api::example::AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::test_cases(&value) {
//                                     for element1 in element0 {
//                                         acc.push(ident_create_default.clone());
//                                     }
//                                 }
//                             }
//                             if let Some(value) = &common_read_only_ids_returned_from_create_one.column_155 {
//                                 for element0 in <crate::repositories_types::server::routes::api::example::OptionAnimalAsNullableJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::test_cases(&value) {
//                                     for element1 in element0 {
//                                         acc.push(ident_create_default.clone());
//                                     }
//                                 }
//                             }
//                             if let Some(value) = &common_read_only_ids_returned_from_create_one.column_156 {
//                                 for element0 in <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::test_cases(&value) {
//                                     for element1 in element0 {
//                                         acc.push(ident_create_default.clone());
//                                     }
//                                 }
//                             }
//                             if let Some(value) = &common_read_only_ids_returned_from_create_one.column_157 {
//                                 for element0 in <crate::repositories_types::server::routes::api::example::OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::test_cases(&value) {
//                                     for element1 in element0 {
//                                         acc.push(ident_create_default.clone());
//                                     }
//                                 }
//                             }
//                             acc
//                         };
//                         let mut two_dimension_acc = vec![];
//                         let mut if_complete = false;
//                         let mut result = vec![];
//                         while !is_complite {//todo another check?
//                             match super::Example::try_create_many(
//                                 &url,
//                                 super::ExampleCreateManyParameters {
//                                     payload: super::ExampleCreateManyPayload(
//                                         acc
//                                     )
//                                 }
//                             ).await {
//                                 Ok(value) => {
//                                     result = value;
//                                     if_complete = true;
//                                 },
//                                 Err(_) => {

//                                 }
//                             }
//                         }
//                         result
//                         // .expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98")
//                     };


//                     let mut all_future_counter = 0;
//                     if let Some(value) = &common_read_only_ids_returned_from_create_one.column_154 {
//                         for element0 in <crate::repositories_types::server::routes::api::example::AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::test_cases(&value) {
//                             for element1 in element0 {
//                                 all_future_counter += 1;
//                             }
//                         }
//                     }
//                     if let Some(value) = &common_read_only_ids_returned_from_create_one.column_155 {
//                         for element0 in <crate::repositories_types::server::routes::api::example::OptionAnimalAsNullableJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::test_cases(&value) {
//                             for element1 in element0 {
//                                 all_future_counter += 1;
//                             }
//                         }
//                     }
//                     if let Some(value) = &common_read_only_ids_returned_from_create_one.column_156 {
//                         for element0 in <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::test_cases(&value) {
//                             for element1 in element0 {
//                                 all_future_counter += 1;
//                             }
//                         }
//                     }
//                     if let Some(value) = &common_read_only_ids_returned_from_create_one.column_157 {
//                         for element0 in <crate::repositories_types::server::routes::api::example::OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::test_cases(&value) {
//                             for element1 in element0 {
//                                 all_future_counter += 1;
//                             }
//                         }
//                     }
//                     let mut future_counter = 0;
//                     let mut acc: std::vec::Vec<futures::future::BoxFuture<'static, ()>> = vec![];
//                     if let Some(value) = &common_read_only_ids_returned_from_create_one.column_154 {
//                         for element0 in <crate::repositories_types::server::routes::api::example::AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::test_cases(&value) {
//                             for element1 in element0 {
//                                 let url_cloned = url.clone();
//                                 let ident_create_default_cloned = ident_create_default.clone();
//                                 let select_primary_key_cloned = select_primary_key.clone();

//                                 let read_only_ids_returned_from_create_one = read_only_ids_vec
//                                 .get(future_counter)
//                                 .expect("error 0125dabf-df60-4382-b152-ce222cc249ca").clone();

//                                 future_counter += 1;

//                                 acc.push(futures::FutureExt::boxed(async move {
//                                     let start = chrono::Local::now();
//                                     // let read_only_ids_returned_from_create_one = super::Example::try_create_one(
//                                     //     &url_cloned,
//                                     //     super::ExampleCreateOneParameters {
//                                     //         payload: ident_create_default_cloned
//                                     //     }
//                                     // ).await.expect("error 870927ab-3ba2-445f-96b5-0f7b8618fc63 column_154");
//                                     let middle = chrono::Local::now();
//                                     let update = <crate::repositories_types::server::routes::api::example::AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test(element1.clone());
//                                     assert_eq!(super::ExampleReadOnlyIds { primary_key_column: read_only_ids_returned_from_create_one.primary_key_column.clone(), column_154: Some(<crate::repositories_types::server::routes::api::example::AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update)), column_155: None, column_156: None, column_157: None }, super::Example::try_update_one(&url_cloned, super::ExampleUpdateOneParameters { payload: super::ExampleUpdate::try_new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(read_only_ids_returned_from_create_one.primary_key_column.clone()), Some(postgresql_crud::Value { value: update }), None, None, None).expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2 column_154") },).await.expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52 column_154"), "try_update_one result different column_154");
//                                     let end = chrono::Local::now();
//                                     let duration = end - start;
//                                     println!("start: {}, middle: {}, end: {}, diff: {} seconds, counter: {} of {}", start.format("%Y-%m-%d %H:%M:%S"), middle.format("%Y-%m-%d %H:%M:%S"), end.format("%Y-%m-%d %H:%M:%S"), duration.num_seconds(), future_counter, all_future_counter);
//                                 }));
//                             }
//                         }
//                     }
//                     if let Some(value) = &common_read_only_ids_returned_from_create_one.column_155 {
//                         for element0 in <crate::repositories_types::server::routes::api::example::OptionAnimalAsNullableJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::test_cases(&value) {
//                             for element1 in element0 {
//                                 let url_cloned = url.clone();
//                                 let ident_create_default_cloned = ident_create_default.clone();
//                                 let select_primary_key_cloned = select_primary_key.clone();

//                                 let read_only_ids_returned_from_create_one = read_only_ids_vec
//                                 .get(future_counter)
//                                 .expect("error 0125dabf-df60-4382-b152-ce222cc249ca").clone();

//                                 future_counter += 1;

//                                 acc.push(futures::FutureExt::boxed(async move {
//                                     let start = chrono::Local::now();
//                                     // let read_only_ids_returned_from_create_one = super::Example::try_create_one(
//                                     //     &url_cloned,
//                                     //     super::ExampleCreateOneParameters {
//                                     //         payload: ident_create_default_cloned
//                                     //     }
//                                     // ).await.expect("error 870927ab-3ba2-445f-96b5-0f7b8618fc63 column_155");
//                                     let middle = chrono::Local::now();
//                                     let update = <crate::repositories_types::server::routes::api::example::OptionAnimalAsNullableJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test(element1.clone());
//                                     assert_eq!(super::ExampleReadOnlyIds { primary_key_column: read_only_ids_returned_from_create_one.primary_key_column.clone(), column_154: None, column_155: Some(<crate::repositories_types::server::routes::api::example::OptionAnimalAsNullableJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update)), column_156: None, column_157: None }, super::Example::try_update_one(&url_cloned, super::ExampleUpdateOneParameters { payload: super::ExampleUpdate::try_new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(read_only_ids_returned_from_create_one.primary_key_column.clone()), None, Some(postgresql_crud::Value { value: update }), None, None).expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2 column_155") },).await.expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52 column_155"), "try_update_one result different column_155");
//                                     let end = chrono::Local::now();
//                                     let duration = end - start;
//                                     println!("start: {}, middle: {}, end: {}, diff: {} seconds, counter: {} of {}", start.format("%Y-%m-%d %H:%M:%S"), middle.format("%Y-%m-%d %H:%M:%S"), end.format("%Y-%m-%d %H:%M:%S"), duration.num_seconds(), future_counter, all_future_counter);
//                                 }));
//                             }
//                         }
//                     }
//                     if let Some(value) = &common_read_only_ids_returned_from_create_one.column_156 {
//                         for element0 in <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::test_cases(&value) {
//                             for element1 in element0 {
//                                 let url_cloned = url.clone();
//                                 let ident_create_default_cloned = ident_create_default.clone();
//                                 let select_primary_key_cloned = select_primary_key.clone();
                                
//                                 let read_only_ids_returned_from_create_one = read_only_ids_vec
//                                 .get(future_counter)
//                                 .expect("error 0125dabf-df60-4382-b152-ce222cc249ca").clone();
                                
//                                 future_counter += 1;

//                                 acc.push(futures::FutureExt::boxed(async move {
//                                     let start = chrono::Local::now();
//                                     // let read_only_ids_returned_from_create_one = super::Example::try_create_one(
//                                     //     &url_cloned,
//                                     //     super::ExampleCreateOneParameters {
//                                     //         payload: ident_create_default_cloned
//                                     //     }
//                                     // ).await.expect("error 870927ab-3ba2-445f-96b5-0f7b8618fc63 column_156");
//                                     let middle = chrono::Local::now();
//                                     let update = <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test(element1.clone());
//                                     assert_eq!(super::ExampleReadOnlyIds { primary_key_column: read_only_ids_returned_from_create_one.primary_key_column.clone(), column_154: None, column_155: None, column_156: Some(<crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update)), column_157: None }, super::Example::try_update_one(&url_cloned, super::ExampleUpdateOneParameters { payload: super::ExampleUpdate::try_new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(read_only_ids_returned_from_create_one.primary_key_column.clone()), None, None, Some(postgresql_crud::Value { value: update }), None).expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2 column_156") },).await.expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52 column_156"), "try_update_one result different column_156");
//                                     let end = chrono::Local::now();
//                                     let duration = end - start;
//                                     println!("start: {}, middle: {}, end: {}, diff: {} seconds, counter: {} of {}", start.format("%Y-%m-%d %H:%M:%S"), middle.format("%Y-%m-%d %H:%M:%S"), end.format("%Y-%m-%d %H:%M:%S"), duration.num_seconds(), future_counter, all_future_counter);
//                                 }));
//                             }
//                         }
//                     }
//                     if let Some(value) = &common_read_only_ids_returned_from_create_one.column_157 {
//                         for element0 in <crate::repositories_types::server::routes::api::example::OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::test_cases(&value) {
//                             for element1 in element0 {
//                                 let url_cloned = url.clone();
//                                 let ident_create_default_cloned = ident_create_default.clone();
//                                 let select_primary_key_cloned = select_primary_key.clone();

//                                 let read_only_ids_returned_from_create_one = read_only_ids_vec
//                                 .get(future_counter)
//                                 .expect("error 0125dabf-df60-4382-b152-ce222cc249ca").clone();

//                                 future_counter += 1;

//                                 acc.push(futures::FutureExt::boxed(async move {
//                                     let start = chrono::Local::now();
//                                     // let read_only_ids_returned_from_create_one = super::Example::try_create_one(
//                                     //     &url_cloned,
//                                     //     super::ExampleCreateOneParameters {
//                                     //         payload: ident_create_default_cloned
//                                     //     }
//                                     // ).await.expect("error 870927ab-3ba2-445f-96b5-0f7b8618fc63 column_157");
//                                     let middle = chrono::Local::now();
//                                     let update = <crate::repositories_types::server::routes::api::example::OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test(element1.clone());
//                                     assert_eq!(super::ExampleReadOnlyIds { primary_key_column: read_only_ids_returned_from_create_one.primary_key_column.clone(), column_154: None, column_155: None, column_156: None, column_157: Some(<crate::repositories_types::server::routes::api::example::OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update)) }, super::Example::try_update_one(&url_cloned, super::ExampleUpdateOneParameters { payload: super::ExampleUpdate::try_new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(read_only_ids_returned_from_create_one.primary_key_column.clone()), None, None, None, Some(postgresql_crud::Value { value: update })).expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2 column_157") },).await.expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52 column_157"), "try_update_one result different column_157");
//                                     let end = chrono::Local::now();
//                                     let duration = end - start;
//                                     println!("start: {}, middle: {}, end: {}, diff: {} seconds, counter: {} of {}", start.format("%Y-%m-%d %H:%M:%S"), middle.format("%Y-%m-%d %H:%M:%S"), end.format("%Y-%m-%d %H:%M:%S"), duration.num_seconds(), future_counter, all_future_counter);
//                                 }));
//                             }
//                         }
//                     }
//                     println!("UPDATES LEN {}", acc.len());
//                     futures::StreamExt::for_each_concurrent(futures::stream::iter(acc), 50, |fut| async move {
//                         fut.await;
//                     })
//                     .await;
//                 });
//             })
//             .expect("error 4d329978-f5af-424e-8757-e8a32dbeb5a1")
//             .join()
//             .unwrap_or_else(|error| {
//                 panic!("error b2f21a5f-d9ce-435c-809f-bd40741c8795 {error:#?}");
//             });
//     }
// }


///////////////
#[derive(Debug)]
pub struct AnimalAsNotNullJsonbObject;
#[derive(Debug)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithId;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalAsNotNullJsonbObjectTableTypeDeclaration {
    field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
    field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
}
impl AnimalAsNotNullJsonbObjectTableTypeDeclaration {
    pub fn new(field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration, field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration) -> Self {
        Self { field_0, field_1 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { field_0: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(), field_1: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    id: <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
    field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
    field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn new(id: <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration, field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration, field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration) -> Self {
        Self { id, field_0, field_1 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { id: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(), field_0: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(), field_1: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }
    }
}
impl AnimalAsNotNullJsonbObjectTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(AnimalAsNotNullJsonbObjectTableTypeDeclaration)).unwrap())
    }
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration)).unwrap())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalAsNotNullJsonbObjectCreate {
    field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create,
    field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create,
}
impl AnimalAsNotNullJsonbObjectCreate {
    pub fn new(field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create, field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create) -> Self {
        Self { field_0, field_1 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { field_0: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(), field_1: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create,
    field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create,
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    pub fn new(field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create, field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create) -> Self {
        Self { field_0, field_1 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { field_0: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(), field_1: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }
    }
}
impl std::fmt::Display for AnimalAsNotNullJsonbObjectCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for AnimalAsNotNullJsonbObjectCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl AnimalAsNotNullJsonbObjectCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        unreachable!()
    }
    //here
    // fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
    //     match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_0, query) {
    //         Ok(value) => {
    //             query = value;
    //         }
    //         Err(error) => {
    //             return Err(error);
    //         }
    //     }
    //     match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_1, query) {
    //         Ok(value) => {
    //             query = value;
    //         }
    //         Err(error) => {
    //             return Err(error);
    //         }
    //     }
    //     Ok(query)
    // }
}
impl std::fmt::Display for AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_0, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_0", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_1, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_1", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("jsonb_build_object('id', to_jsonb(gen_random_uuid()))||{increments}"))
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_0, query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_1, query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalAsNotNullJsonbObjectSelect(postgresql_crud::NotEmptyUniqueEnumVec<AnimalAsNotNullJsonbObjectSelectElement>);
impl AnimalAsNotNullJsonbObjectSelect {
    pub fn new(value: postgresql_crud::NotEmptyUniqueEnumVec<AnimalAsNotNullJsonbObjectSelectElement>) -> Self {
        Self(value)
    }
}
impl AnimalAsNotNullJsonbObjectSelect {
    fn select_query_part_postgresql_type(&self, column: &std::primitive::str) -> std::string::String {
        let field_ident = column;
        let column_name_and_maybe_field_getter = column;
        let column_name_and_maybe_field_getter_for_error_message = column;
        let is_postgresql_type = true;
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = column_name_and_maybe_field_getter.to_string();
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in self.0.to_vec() {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalAsNotNullJsonbObjectSelectElement::Field0(value) => <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_0", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                    AnimalAsNotNullJsonbObjectSelectElement::Field1(value) => <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_1", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        if is_postgresql_type { format!("{acc}") } else { format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))") }
    }
}
impl sqlx::Type<sqlx::Postgres> for AnimalAsNotNullJsonbObjectSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for AnimalAsNotNullJsonbObjectSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalAsNotNullJsonbObjectSelectElement {
    #[serde(rename(serialize = "field_0", deserialize = "field_0"))]
    Field0(<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Select),
    #[serde(rename(serialize = "field_1", deserialize = "field_1"))]
    Field1(<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Select),
}
impl error_occurence_lib::ToStdStringString for AnimalAsNotNullJsonbObjectSelectElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectSelectElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![AnimalAsNotNullJsonbObjectSelectElement::Field0(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()), AnimalAsNotNullJsonbObjectSelectElement::Field1(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdSelect(postgresql_crud::NotEmptyUniqueEnumVec<AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement>);
impl AnimalWithIdAsNotNullJsonbObjectWithIdSelect {
    pub fn new(value: postgresql_crud::NotEmptyUniqueEnumVec<AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement>) -> Self {
        Self(value)
    }
}
impl sqlx::Type<sqlx::Postgres> for AnimalWithIdAsNotNullJsonbObjectWithIdSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for AnimalWithIdAsNotNullJsonbObjectWithIdSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id(<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Select),
    #[serde(rename(serialize = "field_0", deserialize = "field_0"))]
    Field0(<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Select),
    #[serde(rename(serialize = "field_1", deserialize = "field_1"))]
    Field1(<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Select),
}
impl error_occurence_lib::ToStdStringString for AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Id(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()), AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Field0(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()), AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Field1(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum AnimalAsNotNullJsonbObjectWhereElement {
    Field0(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    Field1(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for AnimalAsNotNullJsonbObjectWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::Field0(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_0'"), is_need_to_add_logical_operator),
            Self::Field1(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_1'"), is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self {
            Self::Field0(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Field1(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for AnimalAsNotNullJsonbObjectWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::Field0(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()), Self::Field1(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum AnimalWithIdAsNotNullJsonbObjectWithIdWhereElement {
    Id(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    Field0(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    Field1(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for AnimalWithIdAsNotNullJsonbObjectWithIdWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::Id(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'id'"), is_need_to_add_logical_operator),
            Self::Field0(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_0'"), is_need_to_add_logical_operator),
            Self::Field1(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_1'"), is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self {
            Self::Id(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Field0(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Field1(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for AnimalWithIdAsNotNullJsonbObjectWithIdWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::Id(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()), Self::Field0(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()), Self::Field1(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalAsNotNullJsonbObjectRead {
    #[serde(skip_serializing_if = "Option::is_none")]
    field_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_1: std::option::Option<postgresql_crud::Value<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
}
impl AnimalAsNotNullJsonbObjectRead {
    fn into_inner(self) -> AnimalAsNotNullJsonbObjectReadInner {
        AnimalAsNotNullJsonbObjectReadInner {
            field_0: match self.field_0 {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(value.value) }),
                None => None,
            },
            field_1: match self.field_1 {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(value.value) }),
                None => None,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalAsNotNullJsonbObjectReadTryFromErrorNamed {
    AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl AnimalAsNotNullJsonbObjectRead {
    pub fn try_new(field_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>, field_1: std::option::Option<postgresql_crud::Value<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>) -> Result<Self, AnimalAsNotNullJsonbObjectReadTryFromErrorNamed> {
        if let (None, None) = (&field_0, &field_1) {
            return Err(AnimalAsNotNullJsonbObjectReadTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { field_0, field_1 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for AnimalAsNotNullJsonbObjectRead {
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
            #[automatically_derived]
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
                        0u64 => serde::__private::Ok(__Field::__field0),
                        1u64 => serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "field_0" => serde::__private::Ok(__Field::__field0),
                        "field_1" => serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"field_0" => serde::__private::Ok(__Field::__field0),
                        b"field_1" => serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
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
                marker: _serde::__private::PhantomData<AnimalAsNotNullJsonbObjectRead>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AnimalAsNotNullJsonbObjectRead;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct AnimalAsNotNullJsonbObjectRead")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalAsNotNullJsonbObjectRead with 2 elements"));
                        }
                    };
                    let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalAsNotNullJsonbObjectRead with 2 elements"));
                        }
                    };
                    match AnimalAsNotNullJsonbObjectRead::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_0"));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if serde::__private::Option::is_some(&__field1) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_1"));
                                }
                                __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        serde::__private::Some(__field0) => __field0,
                        serde::__private::None => serde::__private::de::missing_field("field_0")?,
                    };
                    let __field1 = match __field1 {
                        serde::__private::Some(__field1) => __field1,
                        serde::__private::None => serde::__private::de::missing_field("field_1")?,
                    };
                    match AnimalAsNotNullJsonbObjectRead::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["field_0", "field_1"];
            _serde::Deserializer::deserialize_struct(__deserializer, "AnimalAsNotNullJsonbObjectRead", FIELDS, __Visitor { marker: _serde::__private::PhantomData::<AnimalAsNotNullJsonbObjectRead>, lifetime: _serde::__private::PhantomData })
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { field_0: Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }), field_1: Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }) }
    }
}
impl sqlx::Type<sqlx::Postgres> for AnimalAsNotNullJsonbObjectRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for AnimalAsNotNullJsonbObjectRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: std::option::Option<postgresql_crud::Value<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_1: std::option::Option<postgresql_crud::Value<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    fn into_inner(self) -> AnimalWithIdAsNotNullJsonbObjectWithIdReadInner {
        AnimalWithIdAsNotNullJsonbObjectWithIdReadInner {
            id: match self.id {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::into_inner(value.value) }),
                None => None,
            },
            field_0: match self.field_0 {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(value.value) }),
                None => None,
            },
            field_1: match self.field_1 {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(value.value) }),
                None => None,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum AnimalWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed {
    AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    pub fn try_new(id: std::option::Option<postgresql_crud::Value<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>, field_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>, field_1: std::option::Option<postgresql_crud::Value<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>) -> Result<Self, AnimalWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed> {
        if let (None, None, None) = (&id, &field_0, &field_1) {
            return Err(AnimalWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { id, field_0, field_1 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for AnimalWithIdAsNotNullJsonbObjectWithIdRead {
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
            #[automatically_derived]
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
                        0u64 => serde::__private::Ok(__Field::__field0),
                        1u64 => serde::__private::Ok(__Field::__field1),
                        2u64 => serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "id" => serde::__private::Ok(__Field::__field0),
                        "field_0" => serde::__private::Ok(__Field::__field1),
                        "field_1" => serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => serde::__private::Ok(__Field::__field0),
                        b"field_0" => serde::__private::Ok(__Field::__field1),
                        b"field_1" => serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
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
                marker: _serde::__private::PhantomData<AnimalWithIdAsNotNullJsonbObjectWithIdRead>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AnimalWithIdAsNotNullJsonbObjectWithIdRead;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct AnimalWithIdAsNotNullJsonbObjectWithIdRead")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalWithIdAsNotNullJsonbObjectWithIdRead with 3 elements"));
                        }
                    };
                    let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalWithIdAsNotNullJsonbObjectWithIdRead with 3 elements"));
                        }
                    };
                    let __field2 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalWithIdAsNotNullJsonbObjectWithIdRead with 3 elements"));
                        }
                    };
                    match AnimalWithIdAsNotNullJsonbObjectWithIdRead::try_new(__field0, __field1, __field2) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    let mut __field2: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("id"));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if serde::__private::Option::is_some(&__field1) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_0"));
                                }
                                __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if serde::__private::Option::is_some(&__field2) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_1"));
                                }
                                __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        serde::__private::Some(__field0) => __field0,
                        serde::__private::None => serde::__private::de::missing_field("id")?,
                    };
                    let __field1 = match __field1 {
                        serde::__private::Some(__field1) => __field1,
                        serde::__private::None => serde::__private::de::missing_field("field_0")?,
                    };
                    let __field2 = match __field2 {
                        serde::__private::Some(__field2) => __field2,
                        serde::__private::None => serde::__private::de::missing_field("field_1")?,
                    };
                    match AnimalWithIdAsNotNullJsonbObjectWithIdRead::try_new(__field0, __field1, __field2) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["id", "field_0", "field_1"];
            _serde::Deserializer::deserialize_struct(__deserializer, "AnimalWithIdAsNotNullJsonbObjectWithIdRead", FIELDS, __Visitor { marker: _serde::__private::PhantomData::<AnimalWithIdAsNotNullJsonbObjectWithIdRead>, lifetime: _serde::__private::PhantomData })
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }),
            field_0: Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }),
            field_1: Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct AnimalAsNotNullJsonbObjectReadOnlyIdsHandle {
    field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct AnimalAsNotNullJsonbObjectReadOnlyIds(postgresql_crud::Value<AnimalAsNotNullJsonbObjectReadOnlyIdsHandle>);
impl sqlx::Decode<'_, sqlx::Postgres> for AnimalAsNotNullJsonbObjectReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for AnimalAsNotNullJsonbObjectReadOnlyIds {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIdsHandle {
    id: <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIds(pub postgresql_crud::Value<AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIdsHandle>);
impl sqlx::Decode<'_, sqlx::Postgres> for AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIds {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AnimalAsNotNullJsonbObjectReadInner {
    field_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
    field_1: std::option::Option<postgresql_crud::Value<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdReadInner {
    id: std::option::Option<postgresql_crud::Value<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
    field_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
    field_1: std::option::Option<postgresql_crud::Value<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalAsNotNullJsonbObjectUpdate(postgresql_crud::NotEmptyUniqueEnumVec<AnimalAsNotNullJsonbObjectUpdateElement>);
impl AnimalAsNotNullJsonbObjectUpdate {
    pub fn new(value: postgresql_crud::NotEmptyUniqueEnumVec<AnimalAsNotNullJsonbObjectUpdateElement>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum AnimalAsNotNullJsonbObjectUpdateElement {
    #[serde(rename(serialize = "field_0", deserialize = "field_0"))]
    Field0(postgresql_crud::Value<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Update>),
    #[serde(rename(serialize = "field_1", deserialize = "field_1"))]
    Field1(postgresql_crud::Value<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Update>),
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectUpdateElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![AnimalAsNotNullJsonbObjectUpdateElement::Field0(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }), AnimalAsNotNullJsonbObjectUpdateElement::Field1(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() })]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    id: <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update,
    fields: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update,
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    pub fn new(id: <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update, fields: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update) -> Self {
        Self { id, fields }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { id: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(), fields: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdUpdate(postgresql_crud::UniqueVec<AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement>);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdUpdate {
    pub fn new(value: postgresql_crud::UniqueVec<AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement>) -> Self {
        Self(value)
    }
}
impl std::default::Default for AnimalWithIdAsNotNullJsonbObjectWithIdUpdate {
    fn default() -> Self {
        Self(postgresql_crud::UniqueVec::default())
    }
}
impl postgresql_crud::PostgresqlJsonType for AnimalAsNotNullJsonbObject {
    type TableTypeDeclaration = AnimalAsNotNullJsonbObjectTableTypeDeclaration;
    type Create = AnimalAsNotNullJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_part(&value.field_0, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_0", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_part(&value.field_1, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_1", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("{increments}"))
    }
    //here mut
    fn create_query_bind(value: Self::Create, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        //here
        // value.create_query_bind(query)
        match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_bind(value.field_0, query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_bind(value.field_1, query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
    type Select = AnimalAsNotNullJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = if is_postgresql_type { column_name_and_maybe_field_getter.to_string() } else { format!("{column_name_and_maybe_field_getter}->'{field_ident}'") };
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in value.0.to_vec() {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalAsNotNullJsonbObjectSelectElement::Field0(value) => <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_0", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                    AnimalAsNotNullJsonbObjectSelectElement::Field1(value) => <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_1", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        if is_postgresql_type { format!("{acc}") } else { format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))") }
    }
    type WhereElement = AnimalAsNotNullJsonbObjectWhereElement;
    type Read = AnimalAsNotNullJsonbObjectRead;
    type ReadOnlyIds = AnimalAsNotNullJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        let mut acc = std::string::String::default();
        acc.push_str(&format!("jsonb_build_object('field_0',{})||", <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'field_0'"))));
        acc.push_str(&format!("jsonb_build_object('field_1',{})||", <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'field_1'"))));
        let _ = acc.pop();
        let _ = acc.pop();
        format!("jsonb_build_object('value',{})", acc)
    }
    type ReadInner = AnimalAsNotNullJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = AnimalAsNotNullJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut std_option_option_object_acc = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
        let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
        for element in value.0.to_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_0"), "field_0", increment) {
                    Ok(value) => {
                        std_option_option_object_acc = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_1"), "field_1", increment) {
                    Ok(value) => {
                        std_option_option_object_acc = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        if jsonb_set_path.is_empty() { Ok(std_option_option_object_acc) } else { Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',{std_option_option_object_acc})")) }
    }
    fn update_query_bind(value: Self::Update, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        for element in value.0.into_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        for element in value.0.to_vec() {
            match &element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_0", &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"), increment) {
                    Ok(mut value) => {
                        let _ = value.pop();
                        acc.push_str(&format!("jsonb_build_object({})||", value));
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_1", &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"), increment) {
                    Ok(mut value) => {
                        let _ = value.pop();
                        acc.push_str(&format!("jsonb_build_object({})||", value));
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        let _ = acc.pop();
        let _ = acc.pop();
        Ok(format!("'{field_ident}',jsonb_build_object('value',{}),", acc))
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        for element in value.0.to_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(&value.value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(&value.value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        Ok(query)
    }
}
impl postgresql_crud::PostgresqlType for AnimalAsNotNullJsonbObject {
    type TableTypeDeclaration = AnimalAsNotNullJsonbObjectTableTypeDeclaration;
    type Create = AnimalAsNotNullJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_part(value, increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        //here
        // value.create_query_bind(query)
        <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_bind(
            value,
            query
        )
    }
    type Select = AnimalAsNotNullJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{} as {column}", value.select_query_part_postgresql_type(column))
    }
    type WhereElement = AnimalAsNotNullJsonbObjectWhereElement;
    type Read = AnimalAsNotNullJsonbObjectRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadOnlyIds = AnimalAsNotNullJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(column: &std::primitive::str, is_primary_key: std::primitive::bool) -> std::string::String {
        format!("{} as {column},", <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&column))
    }
    type ReadInner = AnimalAsNotNullJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = AnimalAsNotNullJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut std_option_option_object_acc = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
        let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
        for element in value.0.to_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_0"), "field_0", increment) {
                    Ok(value) => {
                        std_option_option_object_acc = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_1"), "field_1", increment) {
                    Ok(value) => {
                        std_option_option_object_acc = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        if jsonb_set_path.is_empty() { Ok(std_option_option_object_acc) } else { Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',{std_option_option_object_acc})")) }
    }
    fn update_query_bind<'a>(value: Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        for element in value.0.into_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, column: &std::primitive::str, increment: &mut std::primitive::u64, is_primary_key: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        for element in value.0.to_vec() {
            match &element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_0", &column, increment) {
                    Ok(mut value) => {
                        let _ = value.pop();
                        acc.push_str(&format!("jsonb_build_object({})||", value));
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_1", &column, increment) {
                    Ok(mut value) => {
                        let _ = value.pop();
                        acc.push_str(&format!("jsonb_build_object({})||", value));
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        let _ = acc.pop();
        let _ = acc.pop();
        Ok(format!("jsonb_build_object('value',{}) as {column},", acc))
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        for element in value.0.to_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(&value.value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(&value.value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        Ok(query)
    }
}
impl postgresql_crud::PostgresqlJsonType for AnimalWithIdAsNotNullJsonbObjectWithId {
    type TableTypeDeclaration = AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = AnimalWithIdAsNotNullJsonbObjectWithIdCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        value.create_query_bind(query)
    }
    type Select = AnimalWithIdAsNotNullJsonbObjectWithIdSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = if is_postgresql_type { column_name_and_maybe_field_getter.to_string() } else { format!("{column_name_and_maybe_field_getter}->'{field_ident}'") };
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in value.0.to_vec() {
            acc.push_str(&format!(
                "{}||",
                match element {
                    AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Id(value) => <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "id", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                    AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Field0(value) => <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_0", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                    AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Field1(value) => <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_1", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        if is_postgresql_type { format!("{acc}") } else { format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))") }
    }
    type WhereElement = AnimalWithIdAsNotNullJsonbObjectWithIdWhereElement;
    type Read = AnimalWithIdAsNotNullJsonbObjectWithIdRead;
    type ReadOnlyIds = AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        let mut acc = std::string::String::default();
        acc.push_str(&format!("jsonb_build_object('id',{})||", <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'id'"))));
        acc.push_str(&format!("jsonb_build_object('field_0',{})||", <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'field_0'"))));
        acc.push_str(&format!("jsonb_build_object('field_1',{})||", <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'field_1'"))));
        let _ = acc.pop();
        let _ = acc.pop();
        format!("jsonb_build_object('value',{})", acc)
    }
    type ReadInner = AnimalWithIdAsNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = AnimalWithIdAsNotNullJsonbObjectWithIdUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        unreachable!()
    }
    fn update_query_bind(value: Self::Update, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        unreachable!()
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = std::string::String::new();
        for element in value.0.to_vec() {
            let mut current_acc = std::string::String::new();
            match <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&element.id, &"id", &column_name_and_maybe_field_getter, increment) {
                Ok(mut value) => {
                    let _ = value.pop();
                    current_acc.push_str(&format!("jsonb_build_object({})||", value));
                }
                Err(error) => {
                    return Err(error);
                }
            }
            for element in element.fields.0.to_vec() {
                match &element {
                    AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_0", &column_name_and_maybe_field_getter, increment) {
                        Ok(mut value) => {
                            let _ = value.pop();
                            current_acc.push_str(&format!("jsonb_build_object({})||", value));
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    },
                    AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_1", &column_name_and_maybe_field_getter, increment) {
                        Ok(mut value) => {
                            let _ = value.pop();
                            current_acc.push_str(&format!("jsonb_build_object({})||", value));
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    },
                }
            }
            let _ = current_acc.pop();
            let _ = current_acc.pop();
            acc.push_str(&format!("{}||", current_acc));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        Ok(format!("jsonb_build_object('value',{})", acc))
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        for element in value.0.to_vec() {
            match <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(&element.id, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
            match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(&element.fields, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlTypeTestCases for AnimalAsNotNullJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        let mut acc = vec![];
        let mut field_0_last = <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_0.clone());
        let mut field_1_last = <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_1.clone());
        for element0 in <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::test_cases(&read_only_ids.0.value.field_0) {
            for element1 in element0 {
                let field_0_current = Some(postgresql_crud::Value { value: element1 });
                field_0_last = field_0_current.clone();
                acc.push(AnimalAsNotNullJsonbObjectReadInner { field_0: field_0_current.clone(), field_1: field_1_last.clone() });
            }
        }
        for element0 in <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::test_cases(&read_only_ids.0.value.field_1) {
            for element1 in element0 {
                let field_1_current = Some(postgresql_crud::Value { value: element1 });
                field_1_last = field_1_current.clone();
                acc.push(AnimalAsNotNullJsonbObjectReadInner { field_0: field_0_last.clone(), field_1: field_1_current.clone() });
            }
        }
        drop(field_0_last);
        drop(field_1_last);
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: AnimalAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::try_new(
            match value.field_0 {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value) }),
                None => None,
            },
            match value.field_1 {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value) }),
                None => None,
            },
        )
        .unwrap()
    }
    fn update_new_or_try_new_unwraped_for_test(value: AnimalAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(
            postgresql_crud::NotEmptyUniqueEnumVec::try_new({
                let mut acc = vec![];
                if let Some(value) = value.field_0 {
                    acc.push(AnimalAsNotNullJsonbObjectUpdateElement::Field0(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(value.value) }));
                }
                if let Some(value) = value.field_1 {
                    acc.push(AnimalAsNotNullJsonbObjectUpdateElement::Field1(postgresql_crud::Value { value: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(value.value) }));
                }
                acc
            })
            .unwrap(),
        )
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        unreachable!()
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlType>::Update) -> <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        let mut field_0 = None;
        let mut field_1 = None;
        for element in value.0.to_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => {
                    field_0 = Some(<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value.value));
                }
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => {
                    field_1 = Some(<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value.value));
                }
            }
        }
        AnimalAsNotNullJsonbObjectReadOnlyIds(postgresql_crud::Value { value: AnimalAsNotNullJsonbObjectReadOnlyIdsHandle { field_0: field_0.expect("expect 106f16f2-ae03-48b3-9239-f4b1b60746d5"), field_1: field_1.expect("expect 106f16f2-ae03-48b3-9239-f4b1b60746d5") } })
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlJsonTypeTestCases for AnimalAsNotNullJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        let mut field_0_last = <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_0.clone());
        let mut field_1_last = <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_1.clone());
        for element0 in <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::test_cases(&read_only_ids.0.value.field_0) {
            for element1 in element0 {
                let field_0_current = Some(postgresql_crud::Value { value: element1 });
                field_0_last = field_0_current.clone();
                acc.push(AnimalAsNotNullJsonbObjectReadInner { field_0: field_0_current.clone(), field_1: field_1_last.clone() });
            }
        }
        for element0 in <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::test_cases(&read_only_ids.0.value.field_1) {
            for element1 in element0 {
                let field_1_current = Some(postgresql_crud::Value { value: element1 });
                field_1_last = field_1_current.clone();
                acc.push(AnimalAsNotNullJsonbObjectReadInner { field_0: field_0_last.clone(), field_1: field_1_current.clone() });
            }
        }
        drop(field_0_last);
        drop(field_1_last);
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: AnimalAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::try_new(
            match value.field_0 {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value) }),
                None => None,
            },
            match value.field_1 {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value) }),
                None => None,
            },
        )
        .unwrap()
    }
    fn update_new_or_try_new_unwraped_for_test(value: AnimalAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(
            postgresql_crud::NotEmptyUniqueEnumVec::try_new({
                let mut acc = vec![];
                if let Some(value) = value.field_0 {
                    acc.push(AnimalAsNotNullJsonbObjectUpdateElement::Field0(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(value.value) }));
                }
                if let Some(value) = value.field_1 {
                    acc.push(AnimalAsNotNullJsonbObjectUpdateElement::Field1(postgresql_crud::Value { value: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(value.value) }));
                }
                acc
            })
            .unwrap(),
        )
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: AnimalAsNotNullJsonbObjectReadInner {
                field_0: match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.field_0) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()) }),
                },
                field_1: match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.field_1) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value { value: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()) }),
                },
            },
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        let mut field_0 = None;
        let mut field_1 = None;
        for element in value.0.to_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => {
                    field_0 = Some(<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value.value));
                }
                AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => {
                    field_1 = Some(<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value.value));
                }
            }
        }
        AnimalAsNotNullJsonbObjectReadOnlyIds(postgresql_crud::Value { value: AnimalAsNotNullJsonbObjectReadOnlyIdsHandle { field_0: field_0.expect("expect 106f16f2-ae03-48b3-9239-f4b1b60746d5"), field_1: field_1.expect("expect 106f16f2-ae03-48b3-9239-f4b1b60746d5") } })
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlJsonTypeTestCases for AnimalWithIdAsNotNullJsonbObjectWithId {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        vec![vec![AnimalWithIdAsNotNullJsonbObjectWithIdReadInner { id: Some(postgresql_crud::Value { value: read_only_ids.0.value.id.0.value.clone() }), field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_0.clone()), field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_1.clone()) }]]
    }
    fn read_new_or_try_new_unwraped_for_test(value: AnimalWithIdAsNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        AnimalWithIdAsNotNullJsonbObjectWithIdRead {
            id: match value.id {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value) }),
                None => None,
            },
            field_0: match value.field_0 {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value) }),
                None => None,
            },
            field_1: match value.field_1 {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value) }),
                None => None,
            },
        }
    }
    fn update_new_or_try_new_unwraped_for_test(value: AnimalWithIdAsNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        unreachable!()
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: AnimalWithIdAsNotNullJsonbObjectWithIdReadInner {
                id: match <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.id) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value { value: <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::into_inner(<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()) }),
                },
                field_0: match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.field_0) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()) }),
                },
                field_1: match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.field_1) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value { value: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()) }),
                },
            },
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        unreachable!()
    }
}
#[derive(Debug)]
pub struct OptionAnimalAsNullableJsonbObject;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionAnimalAsNullableJsonbObjectTableTypeDeclaration(std::option::Option<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>);
impl OptionAnimalAsNullableJsonbObjectTableTypeDeclaration {
    pub fn new(value: std::option::Option<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionAnimalAsNullableJsonbObjectTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl OptionAnimalAsNullableJsonbObjectTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(OptionAnimalAsNullableJsonbObjectTableTypeDeclaration)).unwrap())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionAnimalAsNullableJsonbObjectCreate(std::option::Option<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Create>);
impl OptionAnimalAsNullableJsonbObjectCreate {
    pub fn new(value: std::option::Option<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Create>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionAnimalAsNullableJsonbObjectCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl std::fmt::Display for OptionAnimalAsNullableJsonbObjectCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for OptionAnimalAsNullableJsonbObjectCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl OptionAnimalAsNullableJsonbObjectCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self.0 {
            Some(value) => <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_part(value, increment),
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("${increment}"))
                }
                None => Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            },
        }
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        //here
        match self.0 {
            Some(value) => {
                // value.create_query_bind(query)
                <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_bind(value, query)
            },
            None => {
                if let Err(error) = query.try_bind(sqlx::types::Json(None::<std::option::Option<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Create>>)) {
                    return Err(error.to_string());
                } else {
                    Ok(query)
                }
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionAnimalAsNullableJsonbObjectSelect(std::option::Option<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select>);
impl OptionAnimalAsNullableJsonbObjectSelect {
    pub fn new(value: std::option::Option<postgresql_crud::NotEmptyUniqueEnumVec<AnimalAsNotNullJsonbObjectSelectElement>>) -> Self {
        Self(match value {
            Some(value) => Some(<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select::new(value)),
            None => None,
        })
    }
}
impl OptionAnimalAsNullableJsonbObjectSelect {
    fn select_query_part_postgresql_type(&self, column: &std::primitive::str) -> std::string::String {
        format!("case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({}) end", {
            let value = match &self.0 {
                Some(value) => value,
                None => &<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            };
            value.select_query_part_postgresql_type(column)
        })
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionAnimalAsNullableJsonbObjectSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionAnimalAsNullableJsonbObjectSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionAnimalAsNullableJsonbObjectSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
pub type OptionAnimalAsNullableJsonbObjectWhereElement = postgresql_crud::NullableJsonObjectPostgresqlTypeWhereFilter<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::WhereElement>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionAnimalAsNullableJsonbObjectRead(std::option::Option<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>);
impl OptionAnimalAsNullableJsonbObjectRead {
    fn into_inner(self) -> OptionAnimalAsNullableJsonbObjectReadInner {
        match self.0 {
            Some(value) => Some(<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::into_inner(value)),
            None => None,
        }
    }
}
impl OptionAnimalAsNullableJsonbObjectRead {
    pub fn new(value: std::option::Option<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionAnimalAsNullableJsonbObjectRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionAnimalAsNullableJsonbObjectRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionAnimalAsNullableJsonbObjectRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionAnimalAsNullableJsonbObjectReadOnlyIds(postgresql_crud::Value<std::option::Option<AnimalAsNotNullJsonbObjectReadOnlyIds>>);
impl sqlx::Decode<'_, sqlx::Postgres> for OptionAnimalAsNullableJsonbObjectReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionAnimalAsNullableJsonbObjectReadOnlyIds {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type OptionAnimalAsNullableJsonbObjectReadInner = std::option::Option<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::ReadInner>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionAnimalAsNullableJsonbObjectUpdate(std::option::Option<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update>);
impl OptionAnimalAsNullableJsonbObjectUpdate {
    pub fn new(value: std::option::Option<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionAnimalAsNullableJsonbObjectUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl postgresql_crud::PostgresqlJsonType for OptionAnimalAsNullableJsonbObject {
    type TableTypeDeclaration = OptionAnimalAsNullableJsonbObjectTableTypeDeclaration;
    type Create = OptionAnimalAsNullableJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        value.create_query_bind(query)
    }
    type Select = OptionAnimalAsNullableJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident}) = 'null' then 'null'::jsonb else ({}) end))", {
            let value = match &value.0 {
                Some(value) => value,
                None => &<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            };
            <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::select_query_part(value, field_ident, &column_name_and_maybe_field_getter_field_ident, column_name_and_maybe_field_getter_for_error_message, true)
        })
    }
    type WhereElement = OptionAnimalAsNullableJsonbObjectWhereElement;
    type Read = OptionAnimalAsNullableJsonbObjectRead;
    type ReadOnlyIds = OptionAnimalAsNullableJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        format!("jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter})='null' then 'null'::jsonb else {} end)", <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(column_name_and_maybe_field_getter),)
    }
    type ReadInner = OptionAnimalAsNullableJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = OptionAnimalAsNullableJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value) => <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_part(value, jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment),
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
                }
                None => Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            },
        }
    }
    fn update_query_bind(value: Self::Update, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match value.0 {
            Some(value) => <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_bind(value, query),
            None => {
                if let Err(error) = query.try_bind(sqlx::types::Json(<OptionAnimalAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Update::new(None))) {
                    return Err(error.to_string());
                } else {
                    Ok(query)
                }
            }
        }
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value) => {
                let mut acc = std::string::String::default();
                for element in value.0.to_vec() {
                    match &element {
                        AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_0", &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"), increment) {
                            Ok(mut value) => {
                                let _ = value.pop();
                                acc.push_str(&format!("jsonb_build_object({})||", value));
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        },
                        AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_1", &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"), increment) {
                            Ok(mut value) => {
                                let _ = value.pop();
                                acc.push_str(&format!("jsonb_build_object({})||", value));
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        },
                    }
                }
                let _ = acc.pop();
                let _ = acc.pop();
                Ok(format!("'{field_ident}',jsonb_build_object('value',jsonb_build_object('value',{})),", acc))
            }
            None => Ok(format!("'{field_ident}',jsonb_build_object('value','null'::jsonb),")),
        }
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Some(value) = &value.0 {
            match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(&value, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(query)
    }
}
impl postgresql_crud::PostgresqlType for OptionAnimalAsNullableJsonbObject {
    type TableTypeDeclaration = OptionAnimalAsNullableJsonbObjectTableTypeDeclaration;
    type Create = OptionAnimalAsNullableJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        value.create_query_bind(query)
    }
    type Select = OptionAnimalAsNullableJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{} as {column}", value.select_query_part_postgresql_type(column))
    }
    type WhereElement = OptionAnimalAsNullableJsonbObjectWhereElement;
    type Read = OptionAnimalAsNullableJsonbObjectRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadOnlyIds = OptionAnimalAsNullableJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(column: &std::primitive::str, is_primary_key: std::primitive::bool) -> std::string::String {
        format!("{} as {column},", <OptionAnimalAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&column))
    }
    type ReadInner = OptionAnimalAsNullableJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = OptionAnimalAsNullableJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value) => <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_part(value, jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment),
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
                }
                None => Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            },
        }
    }
    fn update_query_bind<'a>(value: Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match value.0 {
            Some(value) => <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_bind(value, query),
            None => {
                if let Err(error) = query.try_bind(sqlx::types::Json(<OptionAnimalAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Update::new(None))) {
                    return Err(error.to_string());
                } else {
                    Ok(query)
                }
            }
        }
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, column: &std::primitive::str, increment: &mut std::primitive::u64, is_primary_key: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        Ok(format!(
            "jsonb_build_object('value',{}) as {column},",
            match &value.0 {
                Some(value) => {
                    let mut acc = std::string::String::default();
                    for element in value.0.to_vec() {
                        match &element {
                            AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => match <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_0", &format!("{column}"), increment) {
                                Ok(mut value) => {
                                    let _ = value.pop();
                                    acc.push_str(&format!("jsonb_build_object({})||", value));
                                }
                                Err(error) => {
                                    return Err(error);
                                }
                            },
                            AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => match <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_1", &format!("{column}"), increment) {
                                Ok(mut value) => {
                                    let _ = value.pop();
                                    acc.push_str(&format!("jsonb_build_object({})||", value));
                                }
                                Err(error) => {
                                    return Err(error);
                                }
                            },
                        }
                    }
                    let _ = acc.pop();
                    let _ = acc.pop();
                    format!("jsonb_build_object('value',{})", acc)
                }
                None => "'null'::jsonb".to_string(),
            }
        ))
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Some(value) = &value.0 {
            match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(&value, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlTypeTestCases for OptionAnimalAsNullableJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        let mut acc = vec![];
        if let Some(value) = &read_only_ids.0.value {
            for element0 in <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonTypeTestCases>::test_cases(&value) {
                for element1 in element0 {
                    acc.push(Some(element1));
                }
            }
        }
        acc.push(None);
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: OptionAnimalAsNullableJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::new(match value {
            Some(value) => Some(<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn update_new_or_try_new_unwraped_for_test(value: OptionAnimalAsNullableJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(match value {
            Some(value) => Some(<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        unreachable!()
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlType>::Update) -> <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        OptionAnimalAsNullableJsonbObjectReadOnlyIds(postgresql_crud::Value {
            value: match &value.0 {
                Some(value) => Some(<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value)),
                None => None,
            },
        })
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlJsonTypeTestCases for OptionAnimalAsNullableJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        if let Some(value) = &read_only_ids.0.value {
            for element0 in <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonTypeTestCases>::test_cases(&value) {
                for element1 in element0 {
                    acc.push(Some(element1));
                }
            }
        }
        acc.push(None);
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: OptionAnimalAsNullableJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::new(match value {
            Some(value) => Some(<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn update_new_or_try_new_unwraped_for_test(value: OptionAnimalAsNullableJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(match value {
            Some(value) => Some(<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: match value.0.value {
                Some(value) => match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value) {
                    Some(value) => Some(value.value),
                    None => None,
                },
                None => None,
            },
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        OptionAnimalAsNullableJsonbObjectReadOnlyIds(postgresql_crud::Value {
            value: match &value.0 {
                Some(value) => Some(<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value)),
                None => None,
            },
        })
    }
}
#[derive(Debug)]
pub struct VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration(std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>);
impl VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn new(value: std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration)).unwrap())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate(std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>);
impl VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    pub fn new(value: std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl std::fmt::Display for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        for element in &self.0 {
            match element.create_query_part(increment) {
                Ok(value) => {
                    acc.push_str(&format!("{value},"));
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        let _ = acc.pop();
        Ok(format!("jsonb_build_array({})", acc))
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        for element in self.0 {
            match element.create_query_bind(query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    animal_with_id_as_not_null_jsonb_object_with_id_select: <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Select,
    dimension1_pagination: postgresql_crud::PaginationStartsWithZero,
}
impl VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    pub fn new(animal_with_id_as_not_null_jsonb_object_with_id_select: <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Select, dimension1_pagination: postgresql_crud::PaginationStartsWithZero) -> Self {
        Self { animal_with_id_as_not_null_jsonb_object_with_id_select, dimension1_pagination }
    }
}
impl VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    fn select_query_part_postgresql_type(&self, column: &std::primitive::str) -> std::string::String {
        let animal_with_id_as_not_null_jsonb_object_with_id_select = <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_query_part(&self.animal_with_id_as_not_null_jsonb_object_with_id_select, column, &"value", &"value", true);
        let dimension1_start = self.dimension1_pagination.start();
        let dimension1_end = self.dimension1_pagination.end();
        format!("(case when (jsonb_array_length({column}) = 0) then '[]'::jsonb else (select jsonb_agg(({animal_with_id_as_not_null_jsonb_object_with_id_select})) from jsonb_array_elements((select {column})) with ordinality where ordinality between {dimension1_start} and {dimension1_end}) end)")
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { animal_with_id_as_not_null_jsonb_object_with_id_select: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(), dimension1_pagination: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement {
    Equal(postgresql_crud::PostgresqlJsonTypeWhereElementEqual<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>),
    DimensionOneEqual(postgresql_crud::PostgresqlJsonTypeWhereElementDimensionOneEqual<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>),
    LengthEqual(postgresql_crud::PostgresqlJsonTypeWhereElementLengthEqual),
    LengthMoreThan(postgresql_crud::PostgresqlJsonTypeWhereElementLengthMoreThan),
    In(postgresql_crud::PostgresqlJsonTypeWhereElementIn<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>),
    DimensionOneIn(postgresql_crud::PostgresqlJsonTypeWhereElementDimensionOneIn<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>),
    ContainsAllElementsOfArray(postgresql_crud::PostgresqlJsonTypeWhereElementContainsAllElementsOfArray<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>),
    OverlapsWithArray(postgresql_crud::PostgresqlJsonTypeWhereElementOverlapsWithArray<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>),
    ElementId(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    ElementField0(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    ElementField1(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonType>::WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut generate_element_query = |value: &dyn postgresql_crud::PostgresqlTypeWhereFilter<'_>, field: &std::primitive::str| -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
            let elem = "elem";
            let value = match postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{elem}->'{field}'"), is_need_to_add_logical_operator) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error);
                }
            };
            Ok(format!("exists (select 1 from jsonb_array_elements({column}) as {elem} where {value})"))
        };
        match &self {
            Self::Equal(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneEqual(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::LengthEqual(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::LengthMoreThan(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::In(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::DimensionOneIn(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::ContainsAllElementsOfArray(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::OverlapsWithArray(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
            Self::ElementId(value) => generate_element_query(value, &"id"),
            Self::ElementField0(value) => generate_element_query(value, &"field_0"),
            Self::ElementField1(value) => generate_element_query(value, &"field_1"),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self {
            Self::Equal(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneEqual(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::LengthEqual(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::LengthMoreThan(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::In(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::DimensionOneIn(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::ContainsAllElementsOfArray(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::OverlapsWithArray(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::ElementId(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::ElementField0(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::ElementField1(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneEqual(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::LengthEqual(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::LengthMoreThan(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::In(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::DimensionOneIn(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::ContainsAllElementsOfArray(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::OverlapsWithArray(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::ElementId(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::ElementField0(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::ElementField1(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead(std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>);
impl VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    fn into_inner(self) -> VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner {
        self.0.into_iter().map(|element| <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::into_inner(element)).collect()
    }
}
impl VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    pub fn new(value: std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds(postgresql_crud::Value<std::vec::Vec<AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIds>>);
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner = std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::ReadInner>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    create: std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>,
    update: AnimalWithIdAsNotNullJsonbObjectWithIdUpdate,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    delete: std::vec::Vec<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed {
    CreateUpdateDeleteAreEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueIdInJsonUpdateArray {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueIdInJsonDeleteArray {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueIdInJsonUpdateAndDeleteArrays {
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate {
    pub fn try_new(create: std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>, update: AnimalWithIdAsNotNullJsonbObjectWithIdUpdate, delete: std::vec::Vec<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>) -> Result<Self, VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed> {
        if create.is_empty() && update.0.is_empty() && delete.is_empty() {
            return Err(VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed::CreateUpdateDeleteAreEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let update_acc = {
                let mut update_acc = vec![];
                for element in update.0.to_vec() {
                    let id = &element.id;
                    if update_acc.contains(&id) {
                        return Err(VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed::NotUniqueIdInJsonUpdateArray { error: format!("custom serde error deserializing VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate: not unique id in json update array: {}", id.get_inner()), code_occurence: error_occurence_lib::code_occurence!() });
                    } else {
                        update_acc.push(id);
                    }
                }
                update_acc
            };
            let delete_acc = {
                let mut delete_acc = vec![];
                for element in &delete {
                    if delete_acc.contains(&element) {
                        return Err(VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed::NotUniqueIdInJsonDeleteArray { error: format!("custom serde error deserializing VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate: not unique id in json delete array: {}", element.get_inner()), code_occurence: error_occurence_lib::code_occurence!() });
                    } else {
                        delete_acc.push(element);
                    }
                }
                delete_acc
            };
            for element in update_acc {
                if delete_acc.contains(&&element) {
                    return Err(VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed::NotUniqueIdInJsonUpdateAndDeleteArrays { error: format!("custom serde error deserializing VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate: not unique id in json update and delete arrays: {}", element.get_inner()), code_occurence: error_occurence_lib::code_occurence!() });
                }
            }
        }
        Ok(Self { create, update, delete })
    }
}
impl<'de> serde::Deserialize<'de> for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
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
        impl serde::de::Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__f, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "create" => serde::__private::Ok(__Field::__field0),
                    "update" => serde::__private::Ok(__Field::__field1),
                    "delete" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"create" => serde::__private::Ok(__Field::__field0),
                    b"update" => serde::__private::Ok(__Field::__field1),
                    b"delete" => serde::__private::Ok(__Field::__field2),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate;
            fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__f, "tuple struct VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<AnimalWithIdAsNotNullJsonbObjectWithIdUpdate>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => AnimalWithIdAsNotNullJsonbObjectWithIdUpdate::default(),
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::vec::Vec<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                match VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate::try_new(__field0, __field1, __field2) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>> = serde::__private::None;
                let mut __field1: serde::__private::Option<AnimalWithIdAsNotNullJsonbObjectWithIdUpdate> = serde::__private::None;
                let mut __field2: serde::__private::Option<std::vec::Vec<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("create"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("update"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<AnimalWithIdAsNotNullJsonbObjectWithIdUpdate>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("delete"));
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<<postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>>(&mut __map)?);
                        }
                        _ => {
                            let _ = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => AnimalWithIdAsNotNullJsonbObjectWithIdUpdate::default(),
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => {
                        vec![]
                    }
                };
                match VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate::try_new(__field0, __field1, __field2) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["create", "update", "delete"];
        serde::Deserializer::deserialize_struct(__deserializer, "VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate", FIELDS, __Visitor { marker: serde::__private::PhantomData::<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate>, lifetime: serde::__private::PhantomData })
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { create: vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()], update: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(), delete: vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()] }
    }
}
impl postgresql_crud::PostgresqlJsonType for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId {
    type TableTypeDeclaration = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        value.create_query_bind(query)
    }
    type Select = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let animal_with_id_as_not_null_jsonb_object_with_id_select = <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_query_part(&value.animal_with_id_as_not_null_jsonb_object_with_id_select, field_ident, &"value", &"value", true);
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',case when (jsonb_array_length({column_name_and_maybe_field_getter}->'{field_ident}') = 0) then '[]'::jsonb else (select jsonb_agg(({animal_with_id_as_not_null_jsonb_object_with_id_select})) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {dimension1_start} and {dimension1_end}) end ))")
    }
    type WhereElement = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement;
    type Read = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead;
    type ReadOnlyIds = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        format!("jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({column_name_and_maybe_field_getter}) as elem))", <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part("elem"),)
    }
    type ReadInner = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let update_query_part_acc = {
            if value.update.0.is_empty() {
                std::string::String::from("elem")
            } else {
                let mut update_query_part_acc = std::string::String::default();
                for element_handle in value.update.0.to_vec() {
                    let ident_with_id_handle = {
                        let id_increment = match increment.checked_add(1) {
                            Some(value) => {
                                *increment = value;
                                increment.to_string()
                            }
                            None => {
                                return Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                            }
                        };
                        match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_part(&element_handle.fields, &"", &"elem", &"", increment) {
                            Ok(value) => Ok(format!("when {jsonb_set_target}->>'id' = ${id_increment} then {value} ")),
                            Err(error) => Err(error),
                        }
                    };
                    match ident_with_id_handle {
                        Ok(value) => {
                            update_query_part_acc.push_str(&value);
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                let _ = update_query_part_acc.pop();
                format!("case {update_query_part_acc} else elem end")
            }
        };
        let delete_query_part_acc = {
            let mut delete_query_part_acc = std::string::String::default();
            for _ in &value.delete {
                match increment.checked_add(1) {
                    Some(value) => {
                        *increment = value;
                        let maybe_space_and_space = if delete_query_part_acc.is_empty() { "" } else { " and " };
                        delete_query_part_acc.push_str(&format!("{maybe_space_and_space}elem->>'id' <> ${increment}"));
                    }
                    None => {
                        return Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                    }
                }
            }
            delete_query_part_acc
        };
        let create_query_part_acc = {
            let mut create_query_part_acc = std::string::String::default();
            for element in &value.create {
                match element.create_query_part(increment) {
                    Ok(value) => {
                        create_query_part_acc.push_str(&format!("{value},"));
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            let _ = create_query_part_acc.pop();
            create_query_part_acc
        };
        let maybe_where = if value.delete.is_empty() { std::string::String::default() } else { format!(" where {delete_query_part_acc}") };
        let maybe_jsonb_build_array = if value.create.is_empty() { std::string::String::default() } else { format!(" || jsonb_build_array({create_query_part_acc})") };
        Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})"))
    }
    fn update_query_bind(value: Self::Update, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        for element in value.update.0.into_vec() {
            match element.id.query_bind_as_postgresql_text(query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
            match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_bind(element.fields, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        for element in value.delete {
            match element.query_bind_as_postgresql_text(query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        for element in value.create {
            match element.create_query_bind(query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        Ok(format!("'{field_ident}',jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({column_name_and_maybe_field_getter}->'{field_ident}') as elem)),", {
            match <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.update, &"", &"elem", increment) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error);
                }
            }
        }))
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(&value.update, query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
impl postgresql_crud::PostgresqlType for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId {
    type TableTypeDeclaration = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        value.create_query_bind(query)
    }
    type Select = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{} as {column}", value.select_query_part_postgresql_type(column))
    }
    type WhereElement = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement;
    type Read = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadOnlyIds = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds;
    fn select_only_ids_query_part(column: &std::primitive::str, is_primary_key: std::primitive::bool) -> std::string::String {
        format!("{} as {column},", <VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&column))
    }
    type ReadInner = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let update_query_part_acc = {
            if value.update.0.is_empty() {
                std::string::String::from("elem")
            } else {
                let mut update_query_part_acc = std::string::String::default();
                for element_handle in value.update.0.to_vec() {
                    let ident_with_id_handle = {
                        let id_increment = match increment.checked_add(1) {
                            Some(value) => {
                                *increment = value;
                                increment.to_string()
                            }
                            None => {
                                return Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                            }
                        };
                        match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_part(&element_handle.fields, &"", &"elem", &"", increment) {
                            Ok(value) => Ok(format!("when {jsonb_set_target}->>'id' = ${id_increment} then {value} ")),
                            Err(error) => Err(error),
                        }
                    };
                    match ident_with_id_handle {
                        Ok(value) => {
                            update_query_part_acc.push_str(&value);
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                let _ = update_query_part_acc.pop();
                format!("case {update_query_part_acc} else elem end")
            }
        };
        let delete_query_part_acc = {
            let mut delete_query_part_acc = std::string::String::default();
            for _ in &value.delete {
                match increment.checked_add(1) {
                    Some(value) => {
                        *increment = value;
                        let maybe_space_and_space = if delete_query_part_acc.is_empty() { "" } else { " and " };
                        delete_query_part_acc.push_str(&format!("{maybe_space_and_space}elem->>'id' <> ${increment}"));
                    }
                    None => {
                        return Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                    }
                }
            }
            delete_query_part_acc
        };
        let create_query_part_acc = {
            let mut create_query_part_acc = std::string::String::default();
            for element in &value.create {
                match element.create_query_part(increment) {
                    Ok(value) => {
                        create_query_part_acc.push_str(&format!("{value},"));
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            let _ = create_query_part_acc.pop();
            create_query_part_acc
        };
        let maybe_where = if value.delete.is_empty() { std::string::String::default() } else { format!(" where {delete_query_part_acc}") };
        let maybe_jsonb_build_array = if value.create.is_empty() { std::string::String::default() } else { format!(" || jsonb_build_array({create_query_part_acc})") };
        Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})"))
    }
    fn update_query_bind<'a>(value: Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        for element in value.update.0.into_vec() {
            match element.id.query_bind_as_postgresql_text(query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
            match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_bind(element.fields, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        for element in value.delete {
            match element.query_bind_as_postgresql_text(query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        for element in value.create {
            match element.create_query_bind(query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(query)
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, column: &std::primitive::str, increment: &mut std::primitive::u64, is_primary_key: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        Ok(format!("jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({column}) as elem)::jsonb) as {column},", {
            match <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.update, &"", &"elem", increment) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error);
                }
            }
        }))
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(&value.update, query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlTypeTestCases for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        read_only_ids
            .0
            .value
            .iter()
            .map(|element0| {
                let mut acc = vec![];
                for element1 in <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::test_cases(&element0.0.value.field_0.clone()) {
                    for element2 in element1 {
                        acc.push(vec![AnimalWithIdAsNotNullJsonbObjectWithIdReadInner { id: Some(postgresql_crud::Value { value: element0.0.value.id.0.value.clone() }), field_0: Some(postgresql_crud::Value { value: element2 }), field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(element0.0.value.field_1.clone()) }]);
                    }
                }
                for element1 in <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::test_cases(&element0.0.value.field_1.clone()) {
                    for element2 in element1 {
                        acc.push(vec![AnimalWithIdAsNotNullJsonbObjectWithIdReadInner { id: Some(postgresql_crud::Value { value: element0.0.value.id.0.value.clone() }), field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(element0.0.value.field_0.clone()), field_1: Some(postgresql_crud::Value { value: element2 }) }]);
                    }
                }
                acc
            })
            .collect()
    }
    fn read_new_or_try_new_unwraped_for_test(value: VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Read {
        VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead::new({
            let mut acc = vec![];
            for element in value {
                acc.push(<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(element));
            }
            acc
        })
    }
    fn update_new_or_try_new_unwraped_for_test(value: VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Update {
        VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate::try_new(vec![], AnimalWithIdAsNotNullJsonbObjectWithIdUpdate::new(postgresql_crud::UniqueVec::try_new(value.into_iter().map(|element| AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement { id: postgresql_crud::UuidUuidAsNotNullJsonbStringOrigin::new(element.id.clone().unwrap().value), fields: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(AnimalAsNotNullJsonbObjectReadInner { field_0: element.field_0, field_1: element.field_1 }) }).collect()).unwrap()), vec![]).unwrap()
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        unreachable!()
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlType>::Update) -> <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds(postgresql_crud::Value {
            value: value
                .update
                .0
                .to_vec()
                .iter()
                .map(|element| {
                    let mut field_0 = None;
                    let mut field_1 = None;
                    for element1 in element.fields.0.to_vec() {
                        match &element1 {
                            AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => {
                                field_0 = Some(value.value.clone());
                            }
                            AnimalAsNotNullJsonbObjectUpdateElement::Field1(_) => (),
                        }
                    }
                    for element1 in element.fields.0.to_vec() {
                        match &element1 {
                            AnimalAsNotNullJsonbObjectUpdateElement::Field0(_) => (),
                            AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => {
                                field_1 = Some(value.value.clone());
                            }
                        }
                    }
                    AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIds(postgresql_crud::Value {
                        value: AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIdsHandle {
                            id: <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&element.id),
                            field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&field_0.expect("expect a3ec7cae-94a0-49c1-b5d1-88f7b2a3dd1d")),
                            field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&field_1.expect("expect a3ec7cae-94a0-49c1-b5d1-88f7b2a3dd1d")),
                        },
                    })
                })
                .collect(),
        })
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlJsonTypeTestCases for VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        read_only_ids
            .0
            .value
            .iter()
            .map(|element0| {
                let mut acc = vec![];
                for element1 in <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::test_cases(&element0.0.value.field_0.clone()) {
                    for element2 in element1 {
                        acc.push(vec![AnimalWithIdAsNotNullJsonbObjectWithIdReadInner { id: Some(postgresql_crud::Value { value: element0.0.value.id.0.value.clone() }), field_0: Some(postgresql_crud::Value { value: element2 }), field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(element0.0.value.field_1.clone()) }]);
                    }
                }
                for element1 in <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::test_cases(&element0.0.value.field_1.clone()) {
                    for element2 in element1 {
                        acc.push(vec![AnimalWithIdAsNotNullJsonbObjectWithIdReadInner { id: Some(postgresql_crud::Value { value: element0.0.value.id.0.value.clone() }), field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(element0.0.value.field_0.clone()), field_1: Some(postgresql_crud::Value { value: element2 }) }]);
                    }
                }
                acc
            })
            .collect()
    }
    fn read_new_or_try_new_unwraped_for_test(value: VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead::new({
            let mut acc = vec![];
            for element in value {
                acc.push(<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(element));
            }
            acc
        })
    }
    fn update_new_or_try_new_unwraped_for_test(value: VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate::try_new(vec![], AnimalWithIdAsNotNullJsonbObjectWithIdUpdate::new(postgresql_crud::UniqueVec::try_new(value.into_iter().map(|element| AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement { id: postgresql_crud::UuidUuidAsNotNullJsonbStringOrigin::new(element.id.clone().unwrap().value), fields: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(AnimalAsNotNullJsonbObjectReadInner { field_0: element.field_0, field_1: element.field_1 }) }).collect()).unwrap()), vec![]).unwrap()
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: value.0.value.into_iter().fold(vec![], |mut acc, element| {
                if let Some(value) = <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(element) {
                    acc.push(value.value);
                }
                acc
            }),
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds(postgresql_crud::Value {
            value: value
                .update
                .0
                .to_vec()
                .iter()
                .map(|element| {
                    let mut field_0 = None;
                    let mut field_1 = None;
                    for element1 in element.fields.0.to_vec() {
                        match &element1 {
                            AnimalAsNotNullJsonbObjectUpdateElement::Field0(value) => {
                                field_0 = Some(value.value.clone());
                            }
                            AnimalAsNotNullJsonbObjectUpdateElement::Field1(_) => (),
                        }
                    }
                    for element1 in element.fields.0.to_vec() {
                        match &element1 {
                            AnimalAsNotNullJsonbObjectUpdateElement::Field0(_) => (),
                            AnimalAsNotNullJsonbObjectUpdateElement::Field1(value) => {
                                field_1 = Some(value.value.clone());
                            }
                        }
                    }
                    AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIds(postgresql_crud::Value {
                        value: AnimalWithIdAsNotNullJsonbObjectWithIdReadOnlyIdsHandle {
                            id: <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&element.id),
                            field_0: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&field_0.expect("expect a3ec7cae-94a0-49c1-b5d1-88f7b2a3dd1d")),
                            field_1: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&field_1.expect("expect a3ec7cae-94a0-49c1-b5d1-88f7b2a3dd1d")),
                        },
                    })
                })
                .collect(),
        })
    }
}
#[derive(Debug)]
pub struct OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration(std::option::Option<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>);
impl OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn new(value: std::option::Option<std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>>) -> Self {
        Self(match value {
            Some(value) => Some(<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration::new(value)),
            None => None,
        })
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration)).unwrap())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate(std::option::Option<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>);
impl OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate {
    pub fn new(value: std::option::Option<std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>>) -> Self {
        Self(match value {
            Some(value) => Some(<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create::new(value)),
            None => None,
        })
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl std::fmt::Display for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self.0 {
            Some(value) => <VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::create_query_part(value, increment),
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("${increment}"))
                }
                None => Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            },
        }
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.0 {
            Some(value) => value.create_query_bind(query),
            None => {
                if let Err(error) = query.try_bind(sqlx::types::Json(None::<std::option::Option<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>>)) {
                    return Err(error.to_string());
                } else {
                    Ok(query)
                }
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect(std::option::Option<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Select>);
impl OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect {
    pub fn new(value: std::option::Option<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Select>) -> Self {
        Self(value)
    }
}
impl OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect {
    fn select_query_part_postgresql_type(&self, column: &std::primitive::str) -> std::string::String {
        let field_ident = column;
        let column_name_and_maybe_field_getter = column;
        let column_name_and_maybe_field_getter_for_error_message = column;
        format!("case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then jsonb_build_object('{field_ident}',jsonb_build_object('value','null'::jsonb)) else ({}) end", {
            let value = match &self.0 {
                Some(value) => value,
                None => &<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            };
            value.select_query_part_postgresql_type(column)
        })
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
pub type OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdWhereElement = postgresql_crud::NullableJsonObjectPostgresqlTypeWhereFilter<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::WhereElement>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead(std::option::Option<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>);
impl OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead {
    fn into_inner(self) -> OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner {
        match self.0 {
            Some(value) => Some(value.0.into_iter().map(|element| <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::into_inner(element)).collect()),
            None => None,
        }
    }
}
impl OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead {
    pub fn new(value: std::option::Option<std::vec::Vec<<AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>>) -> Self {
        Self(match value {
            Some(value) => Some(<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read::new(value)),
            None => None,
        })
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadOnlyIds(postgresql_crud::Value<std::option::Option<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds>>);
impl sqlx::Decode<'_, sqlx::Postgres> for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadOnlyIds {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner = std::option::Option<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::ReadInner>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdate(std::option::Option<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Update>);
impl OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdate {
    pub fn new(value: std::option::Option<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Update>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl postgresql_crud::PostgresqlJsonType for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId {
    type TableTypeDeclaration = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        value.create_query_bind(query)
    }
    type Select = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        format!("case when jsonb_typeof({column_name_and_maybe_field_getter}->'{field_ident}') = 'null' then jsonb_build_object('{field_ident}',jsonb_build_object('value','null'::jsonb)) else ({}) end", {
            let value = match &value.0 {
                Some(value) => value,
                None => &<<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            };
            <VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_query_part(value, field_ident, &column_name_and_maybe_field_getter, column_name_and_maybe_field_getter_for_error_message, true)
        })
    }
    type WhereElement = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdWhereElement;
    type Read = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead;
    type ReadOnlyIds = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        format!("jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter})='null' then 'null'::jsonb else {} end)", <VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(column_name_and_maybe_field_getter),)
    }
    type ReadInner = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value) => <VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::update_query_part(value, jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment),
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
                }
                None => Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            },
        }
    }
    fn update_query_bind(value: Self::Update, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match value.0 {
            Some(value) => <VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::update_query_bind(value, query),
            None => {
                if let Err(error) = query.try_bind(sqlx::types::Json(<OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Update::new(None))) {
                    return Err(error.to_string());
                } else {
                    Ok(query)
                }
            }
        }
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value) => Ok(format!("'{field_ident}',jsonb_build_object('value',jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({column_name_and_maybe_field_getter}->'{field_ident}') as elem))),", {
                match <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.update, &"", &"elem", increment) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(error);
                    }
                }
            })),
            None => Ok(format!("'{field_ident}',jsonb_build_object('value','null'::jsonb),")),
        }
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Some(value) = &value.0 {
            match <VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(&value, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(query)
    }
}
impl postgresql_crud::PostgresqlType for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId {
    type TableTypeDeclaration = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        value.create_query_bind(query)
    }
    type Select = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{} as {column}", value.select_query_part_postgresql_type(column))
    }
    type WhereElement = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdWhereElement;
    type Read = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadOnlyIds = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadOnlyIds;
    fn select_only_ids_query_part(column: &std::primitive::str, is_primary_key: std::primitive::bool) -> std::string::String {
        format!("{} as {column},", <OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&column))
    }
    type ReadInner = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value) => <VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::update_query_part(value, jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment),
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
                }
                None => Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            },
        }
    }
    fn update_query_bind<'a>(value: Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match value.0 {
            Some(value) => <VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::update_query_bind(value, query),
            None => {
                if let Err(error) = query.try_bind(sqlx::types::Json(<OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Update::new(None))) {
                    return Err(error.to_string());
                } else {
                    Ok(query)
                }
            }
        }
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, column: &std::primitive::str, increment: &mut std::primitive::u64, is_primary_key: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        Ok(format!(
            "jsonb_build_object('value',{}) as {column},",
            match &value.0 {
                Some(value) => format!("jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({column}) as elem)::jsonb)", {
                    match <AnimalWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.update, &"", &"elem", increment) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }),
                None => "'null'::jsonb".to_string(),
            }
        ))
    }
    fn select_only_updated_ids_query_bind<'a>(value: &'a Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Some(value) = &value.0 {
            match <VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_bind(&value, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(query)
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlTypeTestCases for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        let mut acc = vec![];
        if let Some(value) = &read_only_ids.0.value {
            for element0 in <VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonTypeTestCases>::test_cases(&value) {
                for element1 in element0 {
                    acc.push(Some(element1));
                }
            }
        }
        acc.push(None);
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::new(match value {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| AnimalWithIdAsNotNullJsonbObjectWithIdRead {
                        id: match &element.id {
                            Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value.clone()) }),
                            None => None,
                        },
                        field_0: match &element.field_0 {
                            Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value.clone()) }),
                            None => None,
                        },
                        field_1: match &element.field_1 {
                            Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value.clone()) }),
                            None => None,
                        },
                    })
                    .collect(),
            ),
            None => None,
        })
    }
    fn update_new_or_try_new_unwraped_for_test(value: OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(match value {
            Some(value) => Some(<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        unreachable!()
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlType>::Update) -> <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadOnlyIds(postgresql_crud::Value {
            value: match &value.0 {
                Some(value) => Some(<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value)),
                None => None,
            },
        })
    }
}
#[cfg(feature = "test-utils")]
impl postgresql_crud::PostgresqlJsonTypeTestCases for OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        if let Some(value) = &read_only_ids.0.value {
            for element0 in <VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonTypeTestCases>::test_cases(&value) {
                for element1 in element0 {
                    acc.push(Some(element1));
                }
            }
        }
        acc.push(None);
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::new(match value {
            Some(value) => Some(
                value
                    .into_iter()
                    .map(|element| AnimalWithIdAsNotNullJsonbObjectWithIdRead {
                        id: match &element.id {
                            Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value.clone()) }),
                            None => None,
                        },
                        field_0: match &element.field_0 {
                            Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value.clone()) }),
                            None => None,
                        },
                        field_1: match &element.field_1 {
                            Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber as postgresql_crud::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value.clone()) }),
                            None => None,
                        },
                    })
                    .collect(),
            ),
            None => None,
        })
    }
    fn update_new_or_try_new_unwraped_for_test(value: OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(match value {
            Some(value) => Some(<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: match value.0.value {
                Some(value) => match <VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value) {
                    Some(value) => Some(value.value),
                    None => None,
                },
                None => None,
            },
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithIdReadOnlyIds(postgresql_crud::Value {
            value: match &value.0 {
                Some(value) => Some(<VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value)),
                None => None,
            },
        })
    }
}
