#[derive(Debug, Clone, Copy
    // , postgresql_crud::GeneratePostgresqlTable
)]
// #[postgresql_crud::create_many_additional_error_variants{enum CreateManyAdditionalErrorVariants{}}]
// #[postgresql_crud::create_one_additional_error_variants{enum CreateOneAdditionalErrorVariants{}}]
// #[postgresql_crud::read_many_additional_error_variants{enum ReadManyAdditionalErrorVariants{}}]
// #[postgresql_crud::read_one_additional_error_variants{enum ReadOneAdditionalErrorVariants{}}]
// #[postgresql_crud::update_many_additional_error_variants{enum UpdateManyAdditionalErrorVariants{}}]
// #[postgresql_crud::update_one_additional_error_variants{enum UpdateOneAdditionalErrorVariants{}}]
// #[postgresql_crud::delete_many_additional_error_variants{enum DeleteManyAdditionalErrorVariants{}}]
// #[postgresql_crud::delete_one_additional_error_variants{enum DeleteOneAdditionalErrorVariants{}}]
// #[postgresql_crud::common_additional_error_variants{
//     enum CommonAdditionalErrorVariants {
//         CheckCommit {
//             #[eo_error_occurence]
//             check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
//             code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//         },
//     }
// }]
// #[postgresql_crud::create_many_additional_logic{}]
// #[postgresql_crud::create_one_additional_logic{}]
// #[postgresql_crud::read_many_additional_logic{}]
// #[postgresql_crud::read_one_additional_logic{}]
// #[postgresql_crud::update_many_additional_logic{}]
// #[postgresql_crud::update_one_additional_logic{}]
// #[postgresql_crud::delete_many_additional_logic{}]
// #[postgresql_crud::delete_one_additional_logic{}]
// #[postgresql_crud::common_additional_logic{}]
pub struct TableExample {
    // #[generate_postgresql_crud_primary_key]
    // pub primary_key: postgresql_crud::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql,
    // #[generate_postgresql_table_primary_key]
    pub primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql,

    pub column_0: postgresql_crud::StdPrimitiveI16AsNotNullInt2,
    // pub column_1: postgresql_crud::OptionStdPrimitiveI16AsNullableInt2,
    // pub column_2: postgresql_crud::VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2,
    // pub column_3: postgresql_crud::OptionVecOfStdPrimitiveI16AsNullableArrayOfNotNullInt2,
    // pub column_4: postgresql_crud::VecOfOptionStdPrimitiveI16AsNotNullArrayOfNullableInt2,
    // pub column_5: postgresql_crud::OptionVecOfOptionStdPrimitiveI16AsNullableArrayOfNullableInt2,
    pub column_6: postgresql_crud::StdPrimitiveI32AsNotNullInt4,
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


// #[derive(Debug
//     , postgresql_crud::GeneratePostgresqlJsonObjectType
// )] //
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
//             // {
//             //     "not_null_or_nullable": "Nullable",
//             //     "postgresql_json_object_type_pattern": "Array",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // }
//         // ]
//     }
// }]
// pub struct Animal {
//     pub field_0: postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber,
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
//     // pub field_124: postgresql_crud::StdPrimitiveI32AsNotNullJsonbNumber,
//     // pub field_125: postgresql_crud::OptionStdPrimitiveI32AsNullableJsonbNumber,
//     // pub field_126: postgresql_crud::VecOfStdPrimitiveI32AsNotNullArrayOfNotNullJsonbNumber,
//     // pub field_127: postgresql_crud::VecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableJsonbNumber,
//     // pub field_128: postgresql_crud::OptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullJsonbNumber,
//     // pub field_129: postgresql_crud::OptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableJsonbNumber,
//     // pub field_130: postgresql_crud::VecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_131: postgresql_crud::VecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_132: postgresql_crud::VecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_133: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_134: postgresql_crud::OptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_135: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_136: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_137: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_138: postgresql_crud::VecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_139: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_140: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_141: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_142: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_143: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_144: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_145: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_146: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_147: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_148: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_149: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_150: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_151: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_152: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_153: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_154: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_155: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_156: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_157: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_158: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_159: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_160: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_161: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_162: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_163: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_164: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_165: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_166: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_167: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_168: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_169: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_170: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_171: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_172: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_173: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_174: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_175: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_176: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_177: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_178: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_179: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_180: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_181: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_182: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_183: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_184: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_185: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_186: postgresql_crud::StdPrimitiveI64AsNotNullJsonbNumber,
//     // pub field_187: postgresql_crud::OptionStdPrimitiveI64AsNullableJsonbNumber,
//     // pub field_188: postgresql_crud::VecOfStdPrimitiveI64AsNotNullArrayOfNotNullJsonbNumber,
//     // pub field_189: postgresql_crud::VecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableJsonbNumber,
//     // pub field_190: postgresql_crud::OptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullJsonbNumber,
//     // pub field_191: postgresql_crud::OptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableJsonbNumber,
//     // pub field_192: postgresql_crud::VecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_193: postgresql_crud::VecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_194: postgresql_crud::VecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_195: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_196: postgresql_crud::OptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_197: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_198: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_199: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_200: postgresql_crud::VecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_201: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_202: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_203: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_204: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_205: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_206: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_207: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_208: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_209: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_210: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_211: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_212: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_213: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_214: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_215: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_216: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_217: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_218: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_219: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_220: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_221: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_222: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_223: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_224: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_225: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_226: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_227: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_228: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_229: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_230: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_231: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_232: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_233: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_234: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_235: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_236: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_237: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_238: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_239: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_240: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_241: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_242: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_243: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_244: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_245: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_246: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_247: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_248: postgresql_crud::StdPrimitiveU8AsNotNullJsonbNumber,
//     // pub field_249: postgresql_crud::OptionStdPrimitiveU8AsNullableJsonbNumber,
//     // pub field_250: postgresql_crud::VecOfStdPrimitiveU8AsNotNullArrayOfNotNullJsonbNumber,
//     // pub field_251: postgresql_crud::VecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableJsonbNumber,
//     // pub field_252: postgresql_crud::OptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullJsonbNumber,
//     // pub field_253: postgresql_crud::OptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableJsonbNumber,
//     // pub field_254: postgresql_crud::VecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_255: postgresql_crud::VecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_256: postgresql_crud::VecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_257: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_258: postgresql_crud::OptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_259: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_260: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_261: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_262: postgresql_crud::VecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_263: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_264: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_265: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_266: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_267: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_268: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_269: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_270: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_271: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_272: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_273: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_274: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_275: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_276: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_277: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_278: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_279: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_280: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_281: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_282: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_283: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_284: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_285: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_286: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_287: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_288: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_289: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_290: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_291: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_292: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_293: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_294: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_295: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_296: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_297: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_298: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_299: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_300: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_301: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_302: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_303: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_304: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_305: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_306: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_307: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_308: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_309: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_310: postgresql_crud::StdPrimitiveU16AsNotNullJsonbNumber,
//     // pub field_311: postgresql_crud::OptionStdPrimitiveU16AsNullableJsonbNumber,
//     // pub field_312: postgresql_crud::VecOfStdPrimitiveU16AsNotNullArrayOfNotNullJsonbNumber,
//     // pub field_313: postgresql_crud::VecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableJsonbNumber,
//     // pub field_314: postgresql_crud::OptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullJsonbNumber,
//     // pub field_315: postgresql_crud::OptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableJsonbNumber,
//     // pub field_316: postgresql_crud::VecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_317: postgresql_crud::VecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_318: postgresql_crud::VecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_319: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_320: postgresql_crud::OptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_321: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_322: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_323: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_324: postgresql_crud::VecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_325: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_326: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_327: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_328: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_329: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_330: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_331: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_332: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_333: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_334: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_335: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_336: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_337: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_338: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_339: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_340: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_341: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_342: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_343: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_344: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_345: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_346: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_347: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_348: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_349: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_350: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_351: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_352: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_353: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_354: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_355: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_356: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_357: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_358: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_359: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_360: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_361: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_362: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_363: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_364: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_365: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_366: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_367: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_368: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_369: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_370: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_371: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU16AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_372: postgresql_crud::StdPrimitiveU32AsNotNullJsonbNumber,
//     // pub field_373: postgresql_crud::OptionStdPrimitiveU32AsNullableJsonbNumber,
//     // pub field_374: postgresql_crud::VecOfStdPrimitiveU32AsNotNullArrayOfNotNullJsonbNumber,
//     // pub field_375: postgresql_crud::VecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableJsonbNumber,
//     // pub field_376: postgresql_crud::OptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullJsonbNumber,
//     // pub field_377: postgresql_crud::OptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableJsonbNumber,
//     // pub field_378: postgresql_crud::VecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_379: postgresql_crud::VecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_380: postgresql_crud::VecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_381: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_382: postgresql_crud::OptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_383: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_384: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_385: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_386: postgresql_crud::VecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_387: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_388: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_389: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_390: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_391: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_392: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_393: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_394: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_395: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_396: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_397: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_398: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_399: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_400: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_401: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_402: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_403: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_404: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_405: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_406: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_407: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_408: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_409: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_410: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_411: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_412: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_413: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_414: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_415: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_416: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_417: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_418: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_419: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_420: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_421: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_422: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_423: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_424: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_425: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_426: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_427: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_428: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_429: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_430: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_431: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_432: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_433: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_434: postgresql_crud::StdPrimitiveU64AsNotNullJsonbNumber,
//     // pub field_435: postgresql_crud::OptionStdPrimitiveU64AsNullableJsonbNumber,
//     // pub field_436: postgresql_crud::VecOfStdPrimitiveU64AsNotNullArrayOfNotNullJsonbNumber,
//     // pub field_437: postgresql_crud::VecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableJsonbNumber,
//     // pub field_438: postgresql_crud::OptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullJsonbNumber,
//     // pub field_439: postgresql_crud::OptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableJsonbNumber,
//     // pub field_440: postgresql_crud::VecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_441: postgresql_crud::VecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_442: postgresql_crud::VecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_443: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_444: postgresql_crud::OptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_445: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_446: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_447: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_448: postgresql_crud::VecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_449: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_450: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_451: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_452: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_453: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_454: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_455: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_456: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_457: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_458: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_459: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_460: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_461: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_462: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_463: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_464: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_465: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_466: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_467: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_468: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_469: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_470: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_471: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_472: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_473: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_474: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_475: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_476: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_477: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_478: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_479: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_480: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_481: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_482: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_483: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_484: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_485: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_486: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_487: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_488: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_489: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_490: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_491: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_492: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_493: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_494: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_495: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveU64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_496: postgresql_crud::StdPrimitiveF32AsNotNullJsonbNumber,
//     // pub field_497: postgresql_crud::OptionStdPrimitiveF32AsNullableJsonbNumber,
//     // pub field_498: postgresql_crud::VecOfStdPrimitiveF32AsNotNullArrayOfNotNullJsonbNumber,
//     // pub field_499: postgresql_crud::VecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableJsonbNumber,
//     // pub field_500: postgresql_crud::OptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullJsonbNumber,
//     // pub field_501: postgresql_crud::OptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableJsonbNumber,
//     // pub field_502: postgresql_crud::VecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_503: postgresql_crud::VecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_504: postgresql_crud::VecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_505: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_506: postgresql_crud::OptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_507: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_508: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_509: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_510: postgresql_crud::VecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_511: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_512: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_513: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_514: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_515: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_516: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_517: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_518: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_519: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_520: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_521: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_522: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_523: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_524: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_525: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_526: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_527: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_528: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_529: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_530: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_531: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_532: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_533: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_534: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_535: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_536: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_537: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_538: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_539: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_540: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_541: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_542: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_543: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_544: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_545: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_546: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_547: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_548: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_549: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_550: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_551: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_552: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_553: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_554: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_555: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_556: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_557: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF32AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_558: postgresql_crud::StdPrimitiveF64AsNotNullJsonbNumber,
//     // pub field_559: postgresql_crud::OptionStdPrimitiveF64AsNullableJsonbNumber,
//     // pub field_560: postgresql_crud::VecOfStdPrimitiveF64AsNotNullArrayOfNotNullJsonbNumber,
//     // pub field_561: postgresql_crud::VecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableJsonbNumber,
//     // pub field_562: postgresql_crud::OptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullJsonbNumber,
//     // pub field_563: postgresql_crud::OptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableJsonbNumber,
//     // pub field_564: postgresql_crud::VecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_565: postgresql_crud::VecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_566: postgresql_crud::VecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_567: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_568: postgresql_crud::OptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_569: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_570: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_571: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_572: postgresql_crud::VecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_573: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_574: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_575: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_576: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_577: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_578: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_579: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_580: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_581: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_582: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_583: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_584: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_585: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_586: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_587: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_588: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_589: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_590: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_591: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_592: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_593: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_594: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_595: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_596: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_597: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_598: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_599: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_600: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_601: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_602: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_603: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_604: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_605: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_606: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_607: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_608: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_609: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_610: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_611: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_612: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_613: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_614: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_615: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_616: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_617: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_618: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_619: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveF64AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_620: postgresql_crud::StdPrimitiveBoolAsNotNullJsonbBoolean,
//     // pub field_621: postgresql_crud::OptionStdPrimitiveBoolAsNullableJsonbBoolean,
//     // pub field_622: postgresql_crud::VecOfStdPrimitiveBoolAsNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_623: postgresql_crud::VecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableJsonbBoolean,
//     // pub field_624: postgresql_crud::OptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullJsonbBoolean,
//     // pub field_625: postgresql_crud::OptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableJsonbBoolean,
//     // pub field_626: postgresql_crud::VecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_627: postgresql_crud::VecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_628: postgresql_crud::VecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_629: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_630: postgresql_crud::OptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_631: postgresql_crud::OptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_632: postgresql_crud::OptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_633: postgresql_crud::OptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_634: postgresql_crud::VecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_635: postgresql_crud::VecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_636: postgresql_crud::VecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_637: postgresql_crud::VecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_638: postgresql_crud::VecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_639: postgresql_crud::VecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_640: postgresql_crud::VecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_641: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_642: postgresql_crud::OptionVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_643: postgresql_crud::OptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_644: postgresql_crud::OptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_645: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_646: postgresql_crud::OptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_647: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_648: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_649: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_650: postgresql_crud::VecOfVecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_651: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_652: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_653: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_654: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_655: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_656: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_657: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_658: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_659: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_660: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_661: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_662: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_663: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_664: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_665: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_666: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_667: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_668: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_669: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_670: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_671: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_672: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_673: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_674: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_675: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_676: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_677: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_678: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbBoolean,
//     // pub field_679: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbBoolean,
//     // pub field_680: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbBoolean,
//     // pub field_681: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveBoolAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbBoolean,
//     // pub field_682: postgresql_crud::StdStringStringAsNotNullJsonbString,
//     // pub field_683: postgresql_crud::OptionStdStringStringAsNullableJsonbString,
//     // pub field_684: postgresql_crud::VecOfStdStringStringAsNotNullArrayOfNotNullJsonbString,
//     // pub field_685: postgresql_crud::VecOfOptionStdStringStringAsNotNullArrayOfNullableJsonbString,
//     // pub field_686: postgresql_crud::OptionVecOfStdStringStringAsNullableArrayOfNotNullJsonbString,
//     // pub field_687: postgresql_crud::OptionVecOfOptionStdStringStringAsNullableArrayOfNullableJsonbString,
//     // pub field_688: postgresql_crud::VecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_689: postgresql_crud::VecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_690: postgresql_crud::VecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_691: postgresql_crud::VecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_692: postgresql_crud::OptionVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_693: postgresql_crud::OptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_694: postgresql_crud::OptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_695: postgresql_crud::OptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_696: postgresql_crud::VecOfVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_697: postgresql_crud::VecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_698: postgresql_crud::VecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_699: postgresql_crud::VecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_700: postgresql_crud::VecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_701: postgresql_crud::VecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_702: postgresql_crud::VecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_703: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_704: postgresql_crud::OptionVecOfVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_705: postgresql_crud::OptionVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_706: postgresql_crud::OptionVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_707: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_708: postgresql_crud::OptionVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_709: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_710: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_711: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_712: postgresql_crud::VecOfVecOfVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_713: postgresql_crud::VecOfVecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_714: postgresql_crud::VecOfVecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_715: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_716: postgresql_crud::VecOfVecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_717: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_718: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_719: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_720: postgresql_crud::VecOfOptionVecOfVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_721: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_722: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_723: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_724: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_725: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_726: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_727: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_728: postgresql_crud::OptionVecOfVecOfVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_729: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_730: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_731: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_732: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_733: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_734: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_735: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_736: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_737: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_738: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_739: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_740: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_741: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_742: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_743: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdStringStringAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_744: postgresql_crud::UuidUuidAsNotNullJsonbString,
//     // pub field_745: postgresql_crud::OptionUuidUuidAsNullableJsonbString,
//     // pub field_746: postgresql_crud::VecOfUuidUuidAsNotNullArrayOfNotNullJsonbString,
//     // pub field_747: postgresql_crud::VecOfOptionUuidUuidAsNotNullArrayOfNullableJsonbString,
//     // pub field_748: postgresql_crud::OptionVecOfUuidUuidAsNullableArrayOfNotNullJsonbString,
//     // pub field_749: postgresql_crud::OptionVecOfOptionUuidUuidAsNullableArrayOfNullableJsonbString,
//     // pub field_750: postgresql_crud::VecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_751: postgresql_crud::VecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_752: postgresql_crud::VecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_753: postgresql_crud::VecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_754: postgresql_crud::OptionVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_755: postgresql_crud::OptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_756: postgresql_crud::OptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_757: postgresql_crud::OptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_758: postgresql_crud::VecOfVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_759: postgresql_crud::VecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_760: postgresql_crud::VecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_761: postgresql_crud::VecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_762: postgresql_crud::VecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_763: postgresql_crud::VecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_764: postgresql_crud::VecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_765: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_766: postgresql_crud::OptionVecOfVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_767: postgresql_crud::OptionVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_768: postgresql_crud::OptionVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_769: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_770: postgresql_crud::OptionVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_771: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_772: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_773: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_774: postgresql_crud::VecOfVecOfVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_775: postgresql_crud::VecOfVecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_776: postgresql_crud::VecOfVecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_777: postgresql_crud::VecOfVecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_778: postgresql_crud::VecOfVecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_779: postgresql_crud::VecOfVecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_780: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_781: postgresql_crud::VecOfVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_782: postgresql_crud::VecOfOptionVecOfVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_783: postgresql_crud::VecOfOptionVecOfVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_784: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_785: postgresql_crud::VecOfOptionVecOfVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_786: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_787: postgresql_crud::VecOfOptionVecOfOptionVecOfVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_788: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_789: postgresql_crud::VecOfOptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_790: postgresql_crud::OptionVecOfVecOfVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_791: postgresql_crud::OptionVecOfVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_792: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_793: postgresql_crud::OptionVecOfVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_794: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_795: postgresql_crud::OptionVecOfVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_796: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_797: postgresql_crud::OptionVecOfVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_798: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_799: postgresql_crud::OptionVecOfOptionVecOfVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_800: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_801: postgresql_crud::OptionVecOfOptionVecOfVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_802: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbString,
//     // pub field_803: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbString,
//     // pub field_804: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbString,
//     // pub field_805: postgresql_crud::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionUuidUuidAsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbString,
//     // pub field_806: DoggieAsNotNullJsonbObject,
//     // pub field_808: OptionDoggieAsNullableJsonbObject,
//     // pub field_807: VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithId,
//     // pub field_809: OptionVecOfDoggieWithIdAsNullableArrayOfNotNullJsonbObjectWithId,
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
//             // }
//             // ,
//             // {
//             //     "not_null_or_nullable": "NotNull",
//             //     "postgresql_json_object_type_pattern": "Array",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // }
//             //,
//             {
//                 "not_null_or_nullable": "Nullable",
//                 "postgresql_json_object_type_pattern": "Array",
//                 "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             }
//         // ]
//     }
// }]
// pub struct Doggie {
//     pub field_0: postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber,
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
//             // }
//             // ,
//             // {
//             //     "not_null_or_nullable": "Nullable",
//             //     "postgresql_json_object_type_pattern": "Standart",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // },
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
// pub struct Cat {
//     pub field_0: postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber,
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
// }

/////////////
impl TableExample {
    pub const fn table_name() -> &'static str {
        "table_example"
    }
    const fn primary_key() -> &'static str {
        "primary_key_column"
    }
    pub async fn prepare_extensions(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), TableExamplePreparePostgresqlErrorNamed> {
        if let Err(error) = sqlx::query("create extension if not exists pg_jsonschema").execute(pool).await {
            return Err(TableExamplePreparePostgresqlErrorNamed::CreateExtensionIfNotExistsPgJsonschema { error, code_occurence: error_occurence_lib::code_occurence!() });
        }
        if let Err(error) = sqlx::query("create extension if not exists \"uuid-ossp\"").execute(pool).await {
            return Err(TableExamplePreparePostgresqlErrorNamed::CreateExtensionIfNotExistsUuidOssp { error, code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(())
    }
    pub async fn prepare_postgresql_table(pool: &sqlx::Pool<sqlx::Postgres>, table: &str) -> Result<(), TableExamplePreparePostgresqlErrorNamed> {
        if let Err(error) = sqlx::query(&format!(
            "create table if not exists {table} ({},{},{})",
            <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::create_table_column_query_part(&"primary_key_column", true),
            <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::create_table_column_query_part(&"column_0", false),
            <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::create_table_column_query_part(&"column_6", false)
        ))
        .execute(pool)
        .await
        {
            return Err(TableExamplePreparePostgresqlErrorNamed::PreparePostgresql { error, code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(())
    }
    pub async fn prepare_postgresql(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), TableExamplePreparePostgresqlErrorNamed> {
        Self::prepare_extensions(pool).await?;
        Self::prepare_postgresql_table(pool, "table_example").await?;
        Ok(())
    }
    pub const fn allow_methods() -> [http::Method; 4] {
        [http::Method::GET, http::Method::POST, http::Method::PATCH, http::Method::DELETE]
    }
    fn generate_select_query_part(select: &postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let mut value = String::default();
        for element in select.to_vec() {
            value.push_str(&match element {
                TableExampleSelect::PrimaryKeyColumn(value) => match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::select_query_part(value, "primary_key_column") {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(error);
                    }
                },
                TableExampleSelect::Column0(value) => match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_query_part(value, "column_0") {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(error);
                    }
                },
                TableExampleSelect::Column6(value) => match <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::select_query_part(value, "column_6") {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(error);
                    }
                },
            });
            value.push(',');
        }
        let _: Option<char> = value.pop();
        Ok(value)
    }
    async fn create_many_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleCreateManyErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleCreateManyErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleCreateManyParameters {
            payload: match serde_json::from_slice::<TableExampleCreateManyPayload>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleCreateManyErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_create_many_query_string(
            table,
            "primary_key_column,column_0,column_6",
            &{
                let mut increment: u64 = 0;
                let mut acc = String::default();
                for element in &parameters.payload.0 {
                    match element.create_query_part(&mut increment) {
                        Ok(value) => {
                            use std::fmt::Write as _;
                            if write!(acc, "({value}),").is_err() {
                                let error_0 = postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() };
                                let error = TableExampleCreateManyErrorNamed::QueryPart {
                                    error: error_0,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2484,
                                            column: 241,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        }
                        Err(error_0) => {
                            let error = TableExampleCreateManyErrorNamed::QueryPart {
                                error: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2523,
                                        column: 254,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                let _: Option<char> = acc.pop();
                acc
            },
            &{
                let mut acc = String::new();
                match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::select_only_ids_query_part("primary_key_column") {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateManyErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2461,
                                    column: 245,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_only_ids_query_part("column_0") {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateManyErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2461,
                                    column: 245,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                match <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::select_only_ids_query_part("column_6") {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateManyErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2461,
                                    column: 245,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                let _: Option<char> = acc.pop();
                acc
            },
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            for element in parameters.payload.0 {
                match element.create_query_bind(query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateManyErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2548,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleCreateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleCreateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleCreateManyErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1945,
                                column: 245,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc = Vec::new();
                while let Some(value) = match postgresql_crud::TryStreamExt::try_next(&mut rows).await {
                    Ok(value) => match value {
                        Some(value) => match TableExampleReadOnlyIds::try_from(value) {
                            Ok(value) => Some(value),
                            Err(error_0) => {
                                drop(rows);
                                {
                                    if let Err(error_1) = executor.rollback().await {
                                        let error = TableExampleCreateManyErrorNamed::RowAndRollback {
                                            row: error_0,
                                            rollback: error_1,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                    line: 2579,
                                                    column: 259,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                    let error = TableExampleCreateManyErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                line: 2579,
                                                column: 230,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            }
                        },
                        None => None,
                    },
                    Err(error_0) => {
                        drop(rows);
                        {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleCreateManyErrorNamed::RowAndRollback {
                                    row: error_0,
                                    rollback: error_1,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2589,
                                            column: 178,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleCreateManyErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2589,
                                        column: 149,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                } {
                    acc.push(value);
                }
                acc
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleCreateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 1956,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::CREATED;
        response
    }
    pub async fn create_many(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::create_many_handle(app_state, request, Self::table_name()).await
    }
    async fn try_create_many_handle(endpoint_location: &str, parameters: TableExampleCreateManyParameters, table: &str) -> Result<Vec<TableExampleReadOnlyIds>, TableExampleTryCreateManyErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryCreateManyErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/create_many");
        let future = reqwest::Client::new().post(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryCreateManyErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(TableExampleTryCreateManyErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleCreateManyResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryCreateManyErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let create_many_error_named_with_serialize_deserialize = match expected_response {
            TableExampleCreateManyResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleCreateManyResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleCreateManyResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleCreateManyResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleCreateManyResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleCreateManyResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleCreateManyResponseVariants::QueryPart { error, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleCreateManyResponseVariants::RowAndRollback { row, rollback, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            TableExampleCreateManyResponseVariants::TryBind { try_bind, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryCreateManyErrorNamed::TableExampleCreateManyErrorNamedWithSerializeDeserialize {
            create_many_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_create_many(endpoint_location: &str, parameters: TableExampleCreateManyParameters) -> Result<Vec<TableExampleReadOnlyIds>, TableExampleTryCreateManyErrorNamed> {
        Self::try_create_many_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn create_many_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleCreateManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn create_one_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleCreateOneErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleCreateOneErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleCreateOneParameters {
            payload: match serde_json::from_slice::<TableExampleCreate>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleCreateOneErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_create_one_query_string(
            table,
            "primary_key_column,column_0,column_6",
            &match parameters.payload.create_query_part(&mut 0) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleCreateOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2640,
                                column: 254,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
            &{
                let mut acc = String::new();
                match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::select_only_ids_query_part("primary_key_column") {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateOneErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2461,
                                    column: 245,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_only_ids_query_part("column_0") {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateOneErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2461,
                                    column: 245,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                match <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::select_only_ids_query_part("column_6") {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateOneErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2461,
                                    column: 245,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                let _: Option<char> = acc.pop();
                acc
            },
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match parameters.payload.create_query_bind(query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleCreateOneErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2657,
                                column: 252,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleCreateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleCreateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleCreateOneErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1945,
                                column: 245,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                match binded_query.fetch_one(executor.as_mut()).await {
                    Ok(value) => match TableExampleReadOnlyIds::try_from(value) {
                        Ok(value) => value,
                        Err(error_0) => {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleCreateOneErrorNamed::RowAndRollback {
                                    row: error_0,
                                    rollback: error_1,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2689,
                                            column: 225,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleCreateOneErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2689,
                                        column: 196,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                    Err(error_0) => {
                        if let Err(error_1) = executor.rollback().await {
                            let error = TableExampleCreateOneErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2699,
                                        column: 161,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        let error = TableExampleCreateOneErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2699,
                                    column: 132,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleCreateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 1956,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::CREATED;
        response
    }
    pub async fn create_one(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::create_one_handle(app_state, request, Self::table_name()).await
    }
    async fn try_create_one_handle(endpoint_location: &str, parameters: TableExampleCreateOneParameters, table: &str) -> Result<TableExampleReadOnlyIds, TableExampleTryCreateOneErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryCreateOneErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/create_one");
        let future = reqwest::Client::new().post(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryCreateOneErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(TableExampleTryCreateOneErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleCreateOneResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryCreateOneErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let create_one_error_named_with_serialize_deserialize = match expected_response {
            TableExampleCreateOneResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleCreateOneResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleCreateOneResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleCreateOneResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleCreateOneResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleCreateOneResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleCreateOneResponseVariants::RowAndRollback { row, rollback, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            TableExampleCreateOneResponseVariants::QueryPart { error, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleCreateOneResponseVariants::TryBind { try_bind, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryCreateOneErrorNamed::TableExampleCreateOneErrorNamedWithSerializeDeserialize {
            create_one_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_create_one(endpoint_location: &str, parameters: TableExampleCreateOneParameters) -> Result<TableExampleReadOnlyIds, TableExampleTryCreateOneErrorNamed> {
        Self::try_create_one_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn create_one_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleCreate as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn read_many_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleReadManyErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleReadManyErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleReadManyParameters {
            payload: match serde_json::from_slice::<TableExampleReadManyPayload>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleReadManyErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_read_many_query_string(
            table,
            &match Self::generate_select_query_part(&parameters.payload.select) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleReadManyErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 859,
                                column: 241,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
            &{
                let mut increment: u64 = 0;
                let mut additional_parameters = match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.where_many, &mut increment, &"", false) {
                    Ok(value) => value,
                    Err(error_0) => {
                        let error = TableExampleReadManyErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 1159,
                                    column: 274,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                };
                {
                    let prefix = if additional_parameters.is_empty() { "" } else { " " };
                    let value = &parameters.payload.order_by;
                    let order = match &value.order {
                        Some(value) => value.to_snake_case_stringified(),
                        None => postgresql_crud::Order::default().to_snake_case_stringified(),
                    };
                    {
                        {
                            use std::fmt::Write as _;
                            if write!(
                                additional_parameters,
                                "{}order by {} {}",
                                prefix,
                                &match &value.column {
                                    TableExampleSelect::PrimaryKeyColumn(_) => "primary_key_column",
                                    TableExampleSelect::Column0(_) => "column_0",
                                    TableExampleSelect::Column6(_) => "column_6",
                                },
                                order,
                            )
                            .is_err()
                            {
                                let error_0 = postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() };
                                let error = TableExampleReadManyErrorNamed::QueryPart {
                                    error: error_0,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2484,
                                            column: 241,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        }
                    }
                };
                {
                    let prefix = if additional_parameters.is_empty() { "" } else { " " };
                    let value = match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.pagination, &mut increment, &"", bool::default()) {
                        Ok(value) => value,
                        Err(error_0) => {
                            let error = TableExampleReadManyErrorNamed::QueryPart {
                                error: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2778,
                                        column: 254,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    };
                    {
                        {
                            use std::fmt::Write as _;
                            if write!(additional_parameters, "{prefix}{value}").is_err() {
                                let error_0 = postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() };
                                let error = TableExampleReadManyErrorNamed::QueryPart {
                                    error: error_0,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2484,
                                            column: 241,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        }
                    }
                };
                additional_parameters
            },
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.where_many, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleReadManyErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1182,
                                column: 239,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.pagination, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleReadManyErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2835,
                                column: 252,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleReadManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleReadManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc = Vec::new();
                while let Some(value) = match postgresql_crud::TryStreamExt::try_next(&mut rows).await {
                    Ok(value) => match value {
                        Some(value) => Some(match TableExampleRead::try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_table_example_select(value, &parameters.payload.select) {
                            Ok(value) => value,
                            Err(error_0) => {
                                let error = TableExampleReadManyErrorNamed::Postgresql {
                                    postgresql: error_0,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 1203,
                                            column: 271,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }),
                        None => None,
                    },
                    Err(error_0) => {
                        let error = TableExampleReadManyErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2859,
                                    column: 169,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                } {
                    acc.push(value);
                }
                acc
            }
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    pub async fn read_many(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::read_many_handle(app_state, request, Self::table_name()).await
    }
    async fn try_read_many_handle(endpoint_location: &str, parameters: TableExampleReadManyParameters, table: &str) -> Result<Vec<TableExampleRead>, TableExampleTryReadManyErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryReadManyErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/read_many");
        let future = reqwest::Client::new().post(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryReadManyErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(TableExampleTryReadManyErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleReadManyResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryReadManyErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let read_many_error_named_with_serialize_deserialize = match expected_response {
            TableExampleReadManyResponseVariants::Desirable(value) => {
                return Ok(value.into_iter().fold(Vec::new(), |mut acc, element| {
                    acc.push(element);
                    acc
                }));
            }
            TableExampleReadManyResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleReadManyResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleReadManyResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleReadManyResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleReadManyResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleReadManyResponseVariants::QueryPart { error, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleReadManyResponseVariants::NotUniqueField { not_unique_field, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::NotUniqueField { not_unique_field, code_occurence },
            TableExampleReadManyResponseVariants::TryBind { try_bind, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryReadManyErrorNamed::TableExampleReadManyErrorNamedWithSerializeDeserialize {
            read_many_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_read_many(endpoint_location: &str, parameters: TableExampleReadManyParameters) -> Result<Vec<TableExampleRead>, TableExampleTryReadManyErrorNamed> {
        Self::try_read_many_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn read_many_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleReadManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn read_one_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleReadOneErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleReadOneErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleReadOneParameters {
            payload: match serde_json::from_slice::<TableExampleReadOnePayload>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_read_one_query_string(
            table,
            &match Self::generate_select_query_part(&parameters.payload.select) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 859,
                                column: 241,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
            &match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.primary_key_column, &mut 0, &Self::primary_key(), false) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2945,
                                column: 254,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.primary_key_column, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2964,
                                column: 256,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleReadOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleReadOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => match TableExampleRead::try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_table_example_select(value, &parameters.payload.select) {
                    Ok(value) => value,
                    Err(error_0) => {
                        let error = TableExampleReadOneErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 1203,
                                    column: 271,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                },
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2984,
                                column: 165,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    pub async fn read_one(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::read_one_handle(app_state, request, Self::table_name()).await
    }
    async fn try_read_one_handle(endpoint_location: &str, parameters: TableExampleReadOneParameters, table: &str) -> Result<TableExampleRead, TableExampleTryReadOneErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryReadOneErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/read_one");
        let future = reqwest::Client::new().post(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryReadOneErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(TableExampleTryReadOneErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleReadOneResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryReadOneErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let read_one_error_named_with_serialize_deserialize = match expected_response {
            TableExampleReadOneResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleReadOneResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleReadOneResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleReadOneResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleReadOneResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleReadOneResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleReadOneResponseVariants::NotUniqueField { not_unique_field, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::NotUniqueField { not_unique_field, code_occurence },
            TableExampleReadOneResponseVariants::QueryPart { error, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleReadOneResponseVariants::TryBind { try_bind, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize {
            read_one_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_read_one(endpoint_location: &str, parameters: TableExampleReadOneParameters) -> Result<TableExampleRead, TableExampleTryReadOneErrorNamed> {
        Self::try_read_one_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn read_one_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleReadOnePayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn update_many_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleUpdateManyErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleUpdateManyErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleUpdateManyParameters {
            payload: match serde_json::from_slice::<TableExampleUpdateManyPayload>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleUpdateManyErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let update_for_query_vec = parameters.payload.0.into_iter().map(|element| TableExampleUpdateForQuery::from(element)).collect::<Vec<TableExampleUpdateForQuery>>();
        let query_string = {
            let mut increment: u64 = 0;
            let elements = {
                let mut acc = String::default();
                {
                    let mut is_column_0_update_exist = false;
                    for element in &update_for_query_vec {
                        if element.column_0.is_some() {
                            is_column_0_update_exist = true;
                            break;
                        }
                    }
                    if is_column_0_update_exist {
                        acc.push_str(&postgresql_crud::generate_column_equals_case_acc_else_column_end_comma_update_many_query_part("column_0", &{
                            let mut acc = String::default();
                            for element in &update_for_query_vec {
                                if let Some(value) = &element.column_0 {
                                    acc.push_str(&postgresql_crud::generate_when_column_id_then_value_update_many_query_part(
                                        Self::primary_key(),
                                        &match element.update_query_part_primary_key(&mut increment) {
                                            Ok(value) => value,
                                            Err(error_0) => {
                                                let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                                    error: error_0,
                                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!(),
                                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                            line: 3181,
                                                            column: 254,
                                                        }),
                                                    ),
                                                };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                                return response;
                                            }
                                        },
                                        &match TableExampleUpdateForQuery::update_query_part_column_0(value, &mut increment) {
                                            Ok(value) => value,
                                            Err(error_0) => {
                                                let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                                    error: error_0,
                                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!(),
                                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                            line: 3181,
                                                            column: 254,
                                                        }),
                                                    ),
                                                };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                                return response;
                                            }
                                        },
                                    ));
                                }
                            }
                            acc
                        }));
                    }
                }
                {
                    let mut is_column_6_update_exist = false;
                    for element in &update_for_query_vec {
                        if element.column_6.is_some() {
                            is_column_6_update_exist = true;
                            break;
                        }
                    }
                    if is_column_6_update_exist {
                        acc.push_str(&postgresql_crud::generate_column_equals_case_acc_else_column_end_comma_update_many_query_part("column_6", &{
                            let mut acc = String::default();
                            for element in &update_for_query_vec {
                                if let Some(value) = &element.column_6 {
                                    acc.push_str(&postgresql_crud::generate_when_column_id_then_value_update_many_query_part(
                                        Self::primary_key(),
                                        &match element.update_query_part_primary_key(&mut increment) {
                                            Ok(value) => value,
                                            Err(error_0) => {
                                                let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                                    error: error_0,
                                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!(),
                                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                            line: 3181,
                                                            column: 254,
                                                        }),
                                                    ),
                                                };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                                return response;
                                            }
                                        },
                                        &match TableExampleUpdateForQuery::update_query_part_column_6(value, &mut increment) {
                                            Ok(value) => value,
                                            Err(error_0) => {
                                                let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                                    error: error_0,
                                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!(),
                                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                            line: 3181,
                                                            column: 254,
                                                        }),
                                                    ),
                                                };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                                return response;
                                            }
                                        },
                                    ));
                                }
                            }
                            acc
                        }));
                    }
                }
                let _: Option<char> = acc.pop();
                acc
            };
            let primary_keys = {
                let mut acc = String::default();
                for element in &update_for_query_vec {
                    {
                        use std::fmt::Write as _;
                        if write!(
                            acc,
                            "{},",
                            match element.update_query_part_primary_key(&mut increment) {
                                Ok(value) => value,
                                Err(error_0) => {
                                    let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                        error: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                line: 1722,
                                                column: 241,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                    return response;
                                }
                            }
                        )
                        .is_err()
                        {
                            let error_0 = postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() };
                            let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                error: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2484,
                                        column: 241,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                let _: Option<char> = acc.pop();
                acc
            };
            let return_columns = {
                let mut acc = String::new();
                for element in &update_for_query_vec {
                    match element.select_only_updated_ids_query_part(&mut increment) {
                        Ok(value) => {
                            acc.push_str(&value);
                        }
                        Err(error_0) => {
                            let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                error: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3181,
                                        column: 254,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                acc
            };
            postgresql_crud::generate_update_many_query_string(table, &elements, Self::primary_key(), &primary_keys, &return_columns)
        };
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            for element in &update_for_query_vec {
                if let Some(value) = &element.column_0 {
                    if let Err(error_0) = query.try_bind(element.primary_key_column.clone()) {
                        let error_0 = error_0.to_string();
                        let error = TableExampleUpdateManyErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3276,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::update_query_bind(value.value.clone(), query) {
                        Ok(value) => {
                            query = value;
                        }
                        Err(error_0) => {
                            let error = TableExampleUpdateManyErrorNamed::TryBind {
                                try_bind: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3276,
                                        column: 252,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            }
            for element in &update_for_query_vec {
                if let Some(value) = &element.column_6 {
                    if let Err(error_0) = query.try_bind(element.primary_key_column.clone()) {
                        let error_0 = error_0.to_string();
                        let error = TableExampleUpdateManyErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3276,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    match <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::update_query_bind(value.value.clone(), query) {
                        Ok(value) => {
                            query = value;
                        }
                        Err(error_0) => {
                            let error = TableExampleUpdateManyErrorNamed::TryBind {
                                try_bind: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3276,
                                        column: 252,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            }
            for element in &update_for_query_vec {
                match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_bind(element.primary_key_column.clone(), query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = TableExampleUpdateManyErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3276,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            for element in &update_for_query_vec {
                if let Some(value) = &element.column_0 {
                    match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_bind(&value.value, query) {
                        Ok(value) => {
                            query = value;
                        }
                        Err(error_0) => {
                            let error = TableExampleUpdateManyErrorNamed::TryBind {
                                try_bind: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3276,
                                        column: 252,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            }
            for element in &update_for_query_vec {
                if let Some(value) = &element.column_6 {
                    match <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_bind(&value.value, query) {
                        Ok(value) => {
                            query = value;
                        }
                        Err(error_0) => {
                            let error = TableExampleUpdateManyErrorNamed::TryBind {
                                try_bind: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3276,
                                        column: 252,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleUpdateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleUpdateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleUpdateManyErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1945,
                                column: 245,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc = Vec::new();
                while let Some(value) = match postgresql_crud::TryStreamExt::try_next(&mut rows).await {
                    Ok(value) => match value {
                        Some(value) => match TableExampleReadOnlyIds::try_from(value) {
                            Ok(value) => Some(value),
                            Err(error_0) => {
                                drop(rows);
                                {
                                    if let Err(error_1) = executor.rollback().await {
                                        let error = TableExampleUpdateManyErrorNamed::RowAndRollback {
                                            row: error_0,
                                            rollback: error_1,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                    line: 3375,
                                                    column: 259,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                    let error = TableExampleUpdateManyErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                line: 3375,
                                                column: 230,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            }
                        },
                        None => None,
                    },
                    Err(error_0) => {
                        drop(rows);
                        {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleUpdateManyErrorNamed::RowAndRollback {
                                    row: error_0,
                                    rollback: error_1,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 3385,
                                            column: 178,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleUpdateManyErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3385,
                                        column: 149,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                } {
                    acc.push(value);
                }
                acc
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleUpdateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 1956,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    pub async fn update_many(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::update_many_handle(app_state, request, Self::table_name()).await
    }
    async fn try_update_many_handle(endpoint_location: &str, parameters: TableExampleUpdateManyParameters, table: &str) -> Result<Vec<TableExampleReadOnlyIds>, TableExampleTryUpdateManyErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryUpdateManyErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/update_many");
        let future = reqwest::Client::new().patch(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryUpdateManyErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(TableExampleTryUpdateManyErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleUpdateManyResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryUpdateManyErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let update_many_error_named_with_serialize_deserialize = match expected_response {
            TableExampleUpdateManyResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleUpdateManyResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleUpdateManyResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleUpdateManyResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleUpdateManyResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleUpdateManyResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleUpdateManyResponseVariants::RowAndRollback { row, rollback, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            TableExampleUpdateManyResponseVariants::QueryPart { error, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleUpdateManyResponseVariants::TryBind { try_bind, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryUpdateManyErrorNamed::TableExampleUpdateManyErrorNamedWithSerializeDeserialize {
            update_many_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_update_many(endpoint_location: &str, parameters: TableExampleUpdateManyParameters) -> Result<Vec<TableExampleReadOnlyIds>, TableExampleTryUpdateManyErrorNamed> {
        Self::try_update_many_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn update_many_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleUpdateManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn update_one_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleUpdateOneErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleUpdateOneErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleUpdateOneParameters {
            payload: match serde_json::from_slice::<TableExampleUpdate>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let update_for_query = TableExampleUpdateForQuery::from(parameters.payload);
        let query_string = {
            let mut increment: u64 = 0;
            let columns = {
                let mut acc = String::default();
                if let Some(value) = &update_for_query.column_0 {
                    acc.push_str(&postgresql_crud::generate_column_queals_value_comma_update_one_query_part(
                        "column_0",
                        &match TableExampleUpdateForQuery::update_query_part_column_0(value, &mut increment) {
                            Ok(value) => value,
                            Err(error_0) => {
                                let error = TableExampleUpdateOneErrorNamed::QueryPart {
                                    error: error_0,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 3446,
                                            column: 258,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        },
                    ));
                }
                if let Some(value) = &update_for_query.column_6 {
                    acc.push_str(&postgresql_crud::generate_column_queals_value_comma_update_one_query_part(
                        "column_6",
                        &match TableExampleUpdateForQuery::update_query_part_column_6(value, &mut increment) {
                            Ok(value) => value,
                            Err(error_0) => {
                                let error = TableExampleUpdateOneErrorNamed::QueryPart {
                                    error: error_0,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 3446,
                                            column: 258,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        },
                    ));
                }
                let _: Option<char> = acc.pop();
                acc
            };
            let primary_key_query_part = match update_for_query.update_query_part_primary_key(&mut increment) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1722,
                                column: 241,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            };
            let return_columns = match update_for_query.select_only_updated_ids_query_part(&mut increment) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 3466,
                                column: 254,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            };
            postgresql_crud::generate_update_one_query_string(table, &columns, Self::primary_key(), &primary_key_query_part, &return_columns)
        };
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            if let Some(value) = &update_for_query.column_0 {
                match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::update_query_bind(value.value.clone(), query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = TableExampleUpdateOneErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3496,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            if let Some(value) = &update_for_query.column_6 {
                match <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::update_query_bind(value.value.clone(), query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = TableExampleUpdateOneErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3496,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_bind(update_for_query.primary_key_column, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 3496,
                                column: 252,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            if let Some(value) = &update_for_query.column_0 {
                match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_bind(&value.value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = TableExampleUpdateOneErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3496,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            if let Some(value) = &update_for_query.column_6 {
                match <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_bind(&value.value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = TableExampleUpdateOneErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3496,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleUpdateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleUpdateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1945,
                                column: 245,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                match binded_query.fetch_one(executor.as_mut()).await {
                    Ok(value) => match TableExampleReadOnlyIds::try_from(value) {
                        Ok(value) => value,
                        Err(error_0) => {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleUpdateOneErrorNamed::RowAndRollback {
                                    row: error_0,
                                    rollback: error_1,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 3568,
                                            column: 225,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleUpdateOneErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3568,
                                        column: 196,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                    Err(error_0) => {
                        if let Err(error_1) = executor.rollback().await {
                            let error = TableExampleUpdateOneErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3576,
                                        column: 161,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        let error = TableExampleUpdateOneErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3576,
                                    column: 132,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleUpdateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 1956,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    pub async fn update_one(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::update_one_handle(app_state, request, Self::table_name()).await
    }
    async fn try_update_one_handle(endpoint_location: &str, parameters: TableExampleUpdateOneParameters, table: &str) -> Result<TableExampleReadOnlyIds, TableExampleTryUpdateOneErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryUpdateOneErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/update_one");
        let future = reqwest::Client::new().patch(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryUpdateOneErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(TableExampleTryUpdateOneErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleUpdateOneResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryUpdateOneErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let update_one_error_named_with_serialize_deserialize = match expected_response {
            TableExampleUpdateOneResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleUpdateOneResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleUpdateOneResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleUpdateOneResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleUpdateOneResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleUpdateOneResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleUpdateOneResponseVariants::RowAndRollback { row, rollback, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            TableExampleUpdateOneResponseVariants::QueryPart { error, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleUpdateOneResponseVariants::TryBind { try_bind, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryUpdateOneErrorNamed::TableExampleUpdateOneErrorNamedWithSerializeDeserialize {
            update_one_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_update_one(endpoint_location: &str, parameters: TableExampleUpdateOneParameters) -> Result<TableExampleReadOnlyIds, TableExampleTryUpdateOneErrorNamed> {
        Self::try_update_one_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn update_one_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleUpdate as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn delete_many_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleDeleteManyErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleDeleteManyErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleDeleteManyParameters {
            payload: match serde_json::from_slice::<TableExampleDeleteManyPayload>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleDeleteManyErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_delete_many_query_string(
            table,
            &{
                let mut increment: u64 = 0;
                match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.where_many, &mut increment, &"", false) {
                    Ok(value) => value,
                    Err(error_0) => {
                        let error = TableExampleDeleteManyErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 1159,
                                    column: 274,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
            },
            Self::primary_key(),
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.where_many, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleDeleteManyErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1182,
                                column: 239,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleDeleteManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleDeleteManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleDeleteManyErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1945,
                                column: 245,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc = Vec::new();
                while let Some(value) = match postgresql_crud::TryStreamExt::try_next(&mut rows).await {
                    Ok(value) => match value {
                        Some(value) => match sqlx::Row::try_get::<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read, &str>(&value, Self::primary_key()) {
                            Ok(value) => Some(value),
                            Err(error_0) => {
                                drop(rows);
                                {
                                    if let Err(error_1) = executor.rollback().await {
                                        let error = TableExampleDeleteManyErrorNamed::RowAndRollback {
                                            row: error_0,
                                            rollback: error_1,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                    line: 2407,
                                                    column: 166,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                    let error = TableExampleDeleteManyErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                line: 2407,
                                                column: 137,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            }
                        },
                        None => None,
                    },
                    Err(error_0) => {
                        drop(rows);
                        {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleDeleteManyErrorNamed::RowAndRollback {
                                    row: error_0,
                                    rollback: error_1,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2409,
                                            column: 162,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleDeleteManyErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2409,
                                        column: 133,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                } {
                    acc.push(value);
                }
                acc
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleDeleteManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 1956,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    pub async fn delete_many(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::delete_many_handle(app_state, request, Self::table_name()).await
    }
    async fn try_delete_many_handle(endpoint_location: &str, parameters: TableExampleDeleteManyParameters, table: &str) -> Result<Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>, TableExampleTryDeleteManyErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryDeleteManyErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/delete_many");
        let future = reqwest::Client::new().delete(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryDeleteManyErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(TableExampleTryDeleteManyErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleDeleteManyResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryDeleteManyErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let delete_many_error_named_with_serialize_deserialize = match expected_response {
            TableExampleDeleteManyResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleDeleteManyResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleDeleteManyResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleDeleteManyResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleDeleteManyResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleDeleteManyResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleDeleteManyResponseVariants::RowAndRollback { row, rollback, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            TableExampleDeleteManyResponseVariants::QueryPart { error, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleDeleteManyResponseVariants::TryBind { try_bind, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryDeleteManyErrorNamed::TableExampleDeleteManyErrorNamedWithSerializeDeserialize {
            delete_many_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_delete_many(endpoint_location: &str, parameters: TableExampleDeleteManyParameters) -> Result<Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>, TableExampleTryDeleteManyErrorNamed> {
        Self::try_delete_many_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn delete_many_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleDeleteManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn delete_one_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleDeleteOneErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleDeleteOneErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleDeleteOneParameters {
            payload: match serde_json::from_slice::<TableExampleDeleteOnePayload>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleDeleteOneErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_delete_one_query_string(table, Self::primary_key());
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.primary_key_column, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleDeleteOneErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 3717,
                                column: 252,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleDeleteOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleDeleteOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleDeleteOneErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1945,
                                column: 245,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                match binded_query.fetch_one(executor.as_mut()).await {
                    Ok(value) => match sqlx::Row::try_get::<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read, &str>(&value, Self::primary_key()) {
                        Ok(value) => value,
                        Err(error_0) => {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleDeleteOneErrorNamed::RowAndRollback {
                                    row: error_0,
                                    rollback: error_1,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2423,
                                            column: 149,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleDeleteOneErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2423,
                                        column: 120,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                    Err(error_0) => {
                        if let Err(error_1) = executor.rollback().await {
                            let error = TableExampleDeleteOneErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2425,
                                        column: 145,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        let error = TableExampleDeleteOneErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2425,
                                    column: 116,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleDeleteOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 1956,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    pub async fn delete_one(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::delete_one_handle(app_state, request, Self::table_name()).await
    }
    async fn try_delete_one_handle(endpoint_location: &str, parameters: TableExampleDeleteOneParameters, table: &str) -> Result<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read, TableExampleTryDeleteOneErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryDeleteOneErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/delete_one");
        let future = reqwest::Client::new().delete(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryDeleteOneErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(TableExampleTryDeleteOneErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleDeleteOneResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryDeleteOneErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let delete_one_error_named_with_serialize_deserialize = match expected_response {
            TableExampleDeleteOneResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleDeleteOneResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleDeleteOneResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleDeleteOneResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleDeleteOneResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleDeleteOneResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleDeleteOneResponseVariants::RowAndRollback { row, rollback, code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            TableExampleDeleteOneResponseVariants::TryBind { try_bind, code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryDeleteOneErrorNamed::TableExampleDeleteOneErrorNamedWithSerializeDeserialize {
            delete_one_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_delete_one(endpoint_location: &str, parameters: TableExampleDeleteOneParameters) -> Result<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read, TableExampleTryDeleteOneErrorNamed> {
        Self::try_delete_one_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn delete_one_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleDeleteOnePayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    fn routes_handle(app_state: std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>, table: &str) -> axum::Router {
        axum::Router::new().nest(
            &format!("/{table}"),
            axum::Router::new()
                .route(
                    "/create_many",
                    axum::routing::post({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::create_many_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/create_many_payload_example", axum::routing::get(Self::create_many_payload_example))
                .route(
                    "/create_one",
                    axum::routing::post({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::create_one_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/create_one_payload_example", axum::routing::get(Self::create_one_payload_example))
                .route(
                    "/read_many",
                    axum::routing::post({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::read_many_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/read_many_payload_example", axum::routing::get(Self::read_many_payload_example))
                .route(
                    "/read_one",
                    axum::routing::post({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::read_one_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/read_one_payload_example", axum::routing::get(Self::read_one_payload_example))
                .route(
                    "/update_many",
                    axum::routing::patch({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::update_many_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/update_many_payload_example", axum::routing::get(Self::update_many_payload_example))
                .route(
                    "/update_one",
                    axum::routing::patch({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::update_one_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/update_one_payload_example", axum::routing::get(Self::update_one_payload_example))
                .route(
                    "/delete_many",
                    axum::routing::delete({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::delete_many_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/delete_many_payload_example", axum::routing::get(Self::delete_many_payload_example))
                .route(
                    "/delete_one",
                    axum::routing::delete({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::delete_one_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/delete_one_payload_example", axum::routing::get(Self::delete_one_payload_example))
                .with_state(app_state),
        )
    }
    pub fn routes(app_state: std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>) -> axum::Router {
        Self::routes_handle(app_state, Self::table_name())
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExamplePreparePostgresqlErrorNamed {
    CreateExtensionIfNotExistsPgJsonschema {
        #[eo_to_std_string_string]
        error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CreateExtensionIfNotExistsUuidOssp {
        #[eo_to_std_string_string]
        error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PreparePostgresql {
        #[eo_to_std_string_string]
        error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa ::
ToSchema,
)]
pub struct TableExampleCreate {
    pub column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create,
    pub column_6: <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Create,
}
impl TableExampleCreate {
    fn create_query_part(&self, increment: &mut u64) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = String::default();
        match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::create_query_part(&<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(), increment) {
            Ok(value) => {
                use std::fmt::Write as _;
                if write!(acc, "{value},").is_err() {
                    return Err(postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() });
                }
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::create_query_part(&self.column_0, increment) {
            Ok(value) => {
                use std::fmt::Write as _;
                if write!(acc, "{value},").is_err() {
                    return Err(postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() });
                }
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        match <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::create_query_part(&self.column_6, increment) {
            Ok(value) => {
                use std::fmt::Write as _;
                if write!(acc, "{value},").is_err() {
                    return Err(postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() });
                }
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        let _: Option<char> = acc.pop();
        Ok(acc)
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::create_query_bind(<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(), query) {
            Ok(value) => {
                query = value;
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::create_query_bind(self.column_0, query) {
            Ok(value) => {
                query = value;
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        match <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::create_query_bind(self.column_6, query) {
            Ok(value) => {
                query = value;
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        Ok(query)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            column_0: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            column_6: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, serde :: Serialize, utoipa :: ToSchema)]
pub struct TableExampleWhereMany {
    primary_key_column: Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where>>,
    column_0: Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Where>>,
    column_6: Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Where>>,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleWhereManyTryNewErrorNamed {
    NoFieldsProvided {
        #[eo_to_std_string_string]
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleWhereMany {
    pub fn try_new(
        primary_key_column: Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where>>,
        column_0: Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Where>>,
        column_6: Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Where>>,
    ) -> Result<Self, TableExampleWhereManyTryNewErrorNamed> {
        if let (None, None, None) = (&primary_key_column, &column_0, &column_6) {
            return Err(TableExampleWhereManyTryNewErrorNamed::NoFieldsProvided { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { primary_key_column, column_0, column_6 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for TableExampleWhereMany {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
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
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => Ok(__Field::__field0),
                        1u64 => Ok(__Field::__field1),
                        2u64 => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "primary_key_column" => Ok(__Field::__field0),
                        "column_0" => Ok(__Field::__field1),
                        "column_6" => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"primary_key_column" => Ok(__Field::__field0),
                        b"column_0" => Ok(__Field::__field1),
                        b"column_6" => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TableExampleWhereMany>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TableExampleWhereMany;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct TableExampleWhereMany")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(__field0) = serde::de::SeqAccess::next_element::<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where>>>(&mut __seq)? else {
                        return Err(serde::de::Error::invalid_length(0usize, &"struct TableExampleWhereMany with 3 elements"));
                    };
                    let Some(__field1) = serde::de::SeqAccess::next_element::<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Where>>>(&mut __seq)? else {
                        return Err(serde::de::Error::invalid_length(0usize, &"struct TableExampleWhereMany with 3 elements"));
                    };
                    let Some(__field2) = serde::de::SeqAccess::next_element::<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Where>>>(&mut __seq)? else {
                        return Err(serde::de::Error::invalid_length(0usize, &"struct TableExampleWhereMany with 3 elements"));
                    };
                    match TableExampleWhereMany::try_new(__field0, __field1, __field2) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: Option<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where>>> = None;
                    let mut __field1: Option<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Where>>> = None;
                    let mut __field2: Option<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Where>>> = None;
                    while let Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if Option::is_some(&__field0) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field("primary_key_column"));
                                }
                                __field0 = Some(serde::de::MapAccess::next_value::<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where>>>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if Option::is_some(&__field1) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field("column_0"));
                                }
                                __field1 = Some(serde::de::MapAccess::next_value::<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Where>>>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if Option::is_some(&__field2) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field("column_6"));
                                }
                                __field2 = Some(serde::de::MapAccess::next_value::<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Where>>>(&mut __map)?);
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        Some(__field0) => __field0,
                        None => serde::__private::de::missing_field("primary_key_column")?,
                    };
                    let __field1 = match __field1 {
                        Some(__field1) => __field1,
                        None => serde::__private::de::missing_field("column_0")?,
                    };
                    let __field2 = match __field2 {
                        Some(__field2) => __field2,
                        None => serde::__private::de::missing_field("column_6")?,
                    };
                    match TableExampleWhereMany::try_new(__field0, __field1, __field2) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &[&str] = &["primary_key_column", "column_0", "column_6"];
            _serde::Deserializer::deserialize_struct(__deserializer, "TableExampleWhereMany", FIELDS, __Visitor { marker: _serde::__private::PhantomData::<Self>, lifetime: _serde::__private::PhantomData })
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleWhereMany {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            primary_key_column: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            column_0: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            column_6: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        }
    }
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa ::
ToSchema,
)]
pub struct StdOptionOptionTableExampleWhereMany(pub Option<TableExampleWhereMany>);
impl<'lifetime> postgresql_crud::PostgresqlTypeWhereFilter<'lifetime> for StdOptionOptionTableExampleWhereMany {
    fn query_part(&self, increment: &mut u64, _: &dyn std::fmt::Display, _: bool) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        Ok(match &self.0 {
            Some(value) => {
                let mut additional_parameters = String::from("where");
                let mut is_first_push_to_additional_parameters_already_happend = false;
                if let Some(value) = &value.primary_key_column {
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &"primary_key_column", is_first_push_to_additional_parameters_already_happend) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                            is_first_push_to_additional_parameters_already_happend = true;
                        }
                        Err(error_0) => {
                            return Err(error_0);
                        }
                    }
                }
                if let Some(value) = &value.column_0 {
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &"column_0", is_first_push_to_additional_parameters_already_happend) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                            is_first_push_to_additional_parameters_already_happend = true;
                        }
                        Err(error_0) => {
                            return Err(error_0);
                        }
                    }
                }
                if let Some(value) = &value.column_6 {
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &"column_6", is_first_push_to_additional_parameters_already_happend) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                        }
                        Err(error_0) => {
                            return Err(error_0);
                        }
                    }
                }
                additional_parameters
            }
            None => String::default(),
        })
    }
    fn query_bind(self, mut query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        if let Some(value) = self.0 {
            if let Some(value) = value.primary_key_column {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                }
            }
            if let Some(value) = value.column_0 {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                }
            }
            if let Some(value) = value.column_6 {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                }
            }
        }
        Ok(query)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdOptionOptionTableExampleWhereMany {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, PartialEq, Clone, Copy)]
pub enum TableExampleSelect {
    #[serde(rename(serialize = "primary_key_column", deserialize = "primary_key_column"))]
    PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select),
    #[serde(rename(serialize = "column_0", deserialize = "column_0"))]
    Column0(<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Select),
    #[serde(rename(serialize = "column_6", deserialize = "column_6"))]
    Column6(<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Select),
}
impl std::fmt::Display for TableExampleSelect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_else(|element| format!("cannot serialize into json: {element:?}")))
    }
}
impl error_occurence_lib::ToStdStringString for TableExampleSelect {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleSelect {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Vec<Self> {
        vec![
            Self::PrimaryKeyColumn(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Column0(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Column6(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct TableExampleRead {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key_column: Option<postgresql_crud::Value<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_0: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_6: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Read>>,
}
impl TableExampleRead {
    fn try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_table_example_select(value: sqlx::postgres::PgRow, select: &postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>) -> Result<Self, sqlx::Error> {
        let mut primary_key_column: Option<postgresql_crud::Value<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>> = None;
        let mut column_0: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Read>> = None;
        let mut column_6: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Read>> = None;
        for element in select.to_vec() {
            match element {
                TableExampleSelect::PrimaryKeyColumn(_) => match sqlx::Row::try_get::<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read, &str>(&value, "primary_key_column") {
                    Ok(value) => {
                        primary_key_column = Some(postgresql_crud::Value { value });
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                },
                TableExampleSelect::Column0(_) => match sqlx::Row::try_get::<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Read, &str>(&value, "column_0") {
                    Ok(value) => {
                        column_0 = Some(postgresql_crud::Value { value });
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                },
                TableExampleSelect::Column6(_) => match sqlx::Row::try_get::<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Read, &str>(&value, "column_6") {
                    Ok(value) => {
                        column_6 = Some(postgresql_crud::Value { value });
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                },
            }
        }
        Ok(Self { primary_key_column, column_0, column_6 })
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct TableExampleReadOnlyIds {
    pub primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::ReadOnlyIds,
    pub column_0: Option<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::ReadOnlyIds>,
    pub column_6: Option<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::ReadOnlyIds>,
}
impl TryFrom<sqlx::postgres::PgRow> for TableExampleReadOnlyIds {
    type Error = sqlx::Error;
    fn try_from(value: sqlx::postgres::PgRow) -> Result<Self, Self::Error> {
        let primary_key_column = match sqlx::Row::try_get::<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::ReadOnlyIds, &str>(&value, "primary_key_column") {
            Ok(value) => value,
            Err(error_0) => {
                return Err(error_0);
            }
        };
        let column_0 = sqlx::Row::try_get::<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::ReadOnlyIds, &str>(&value, "column_0").ok();
        let column_6 = sqlx::Row::try_get::<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::ReadOnlyIds, &str>(&value, "column_6").ok();
        Ok(Self { primary_key_column, column_0, column_6 })
    }
}
#[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
pub struct TableExampleUpdate {
    primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate,
    column_0: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>,
    column_6: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Update>>,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleUpdateTryNewErrorNamed {
    NoFieldsProvided {
        #[eo_to_std_string_string]
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleUpdate {
    pub fn try_new(primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate, column_0: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>, column_6: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Update>>) -> Result<Self, TableExampleUpdateTryNewErrorNamed> {
        if let (None, None) = (&column_0, &column_6) {
            return Err(TableExampleUpdateTryNewErrorNamed::NoFieldsProvided { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { primary_key_column, column_0, column_6 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for TableExampleUpdate {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
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
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => Ok(__Field::__field0),
                        1u64 => Ok(__Field::__field1),
                        2u64 => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "primary_key_column" => Ok(__Field::__field0),
                        "column_0" => Ok(__Field::__field1),
                        "column_6" => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"primary_key_column" => Ok(__Field::__field0),
                        b"column_0" => Ok(__Field::__field1),
                        b"column_6" => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TableExampleUpdate>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TableExampleUpdate;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct TableExampleUpdate")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(__field0) = serde::de::SeqAccess::next_element::<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate>(&mut __seq)? else {
                        return Err(serde::de::Error::invalid_length(0usize, &"struct TableExampleUpdate with 3 elements"));
                    };
                    let Some(__field1) = serde::de::SeqAccess::next_element::<Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>>(&mut __seq)? else {
                        return Err(serde::de::Error::invalid_length(0usize, &"struct TableExampleUpdate with 3 elements"));
                    };
                    let Some(__field2) = serde::de::SeqAccess::next_element::<Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Update>>>(&mut __seq)? else {
                        return Err(serde::de::Error::invalid_length(0usize, &"struct TableExampleUpdate with 3 elements"));
                    };
                    match TableExampleUpdate::try_new(__field0, __field1, __field2) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: Option<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate> = None;
                    let mut __field1: Option<Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>> = None;
                    let mut __field2: Option<Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Update>>> = None;
                    while let Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if Option::is_some(&__field0) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field("primary_key_column"));
                                }
                                __field0 = Some(serde::de::MapAccess::next_value::<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if Option::is_some(&__field1) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field("column_0"));
                                }
                                __field1 = Some(serde::de::MapAccess::next_value::<Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if Option::is_some(&__field2) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field("column_6"));
                                }
                                __field2 = Some(serde::de::MapAccess::next_value::<Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Update>>>(&mut __map)?);
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        Some(__field0) => __field0,
                        None => serde::__private::de::missing_field("primary_key_column")?,
                    };
                    let __field1 = match __field1 {
                        Some(__field1) => __field1,
                        None => serde::__private::de::missing_field("column_0")?,
                    };
                    let __field2 = match __field2 {
                        Some(__field2) => __field2,
                        None => serde::__private::de::missing_field("column_6")?,
                    };
                    match TableExampleUpdate::try_new(__field0, __field1, __field2) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &[&str] = &["primary_key_column", "column_0", "column_6"];
            _serde::Deserializer::deserialize_struct(__deserializer, "TableExampleUpdate", FIELDS, __Visitor { marker: _serde::__private::PhantomData::<Self>, lifetime: _serde::__private::PhantomData })
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            primary_key_column: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            column_0: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
            column_6: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        }
    }
}
#[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
pub struct TableExampleUpdateForQuery {
    primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdateForQuery,
    column_0: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::UpdateForQuery>>,
    column_6: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::UpdateForQuery>>,
}
impl TableExampleUpdateForQuery {
    fn update_query_part_primary_key(&self, increment: &mut u64) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_part(&self.primary_key_column, "", TableExample::primary_key(), "", increment) {
            Ok(value_snake_case) => Ok(value_snake_case),
            Err(error_0) => Err(error_0),
        }
    }
    fn update_query_part_column_0(value: &postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::UpdateForQuery>, increment: &mut u64) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::update_query_part(&value.value, "column_0", "column_0", "", increment) {
            Ok(value) => Ok(value),
            Err(error_0) => Err(error_0),
        }
    }
    fn update_query_part_column_6(value: &postgresql_crud::Value<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::UpdateForQuery>, increment: &mut u64) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::update_query_part(&value.value, "column_6", "column_6", "", increment) {
            Ok(value) => Ok(value),
            Err(error_0) => Err(error_0),
        }
    }
    fn select_only_updated_ids_query_part(&self, increment: &mut u64) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = String::new();
        acc.push_str(&match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_part(&self.primary_key_column, "primary_key_column", increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        });
        if let Some(value) = &self.column_0 {
            acc.push_str(&match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_part(&value.value, "column_0", increment) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error);
                }
            });
        }
        if let Some(value) = &self.column_6 {
            acc.push_str(&match <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_part(&value.value, "column_6", increment) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error);
                }
            });
        }
        let _: Option<char> = acc.pop();
        Ok(acc)
    }
}
impl From<TableExampleUpdate> for TableExampleUpdateForQuery {
    fn from(value: TableExampleUpdate) -> Self {
        Self {
            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::UpdateForQuery::from(value.primary_key_column),
            column_0: match value.column_0 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::UpdateForQuery::from(value.value),
                }),
                None => None,
            },
            column_6: match value.column_6 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::UpdateForQuery::from(value.value),
                }),
                None => None,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleCreateManyPayload(pub Vec<TableExampleCreate>);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleCreateManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
#[derive(Debug)]
pub struct TableExampleCreateManyParameters {
    pub payload: TableExampleCreateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleCreateManyResponseVariants {
    Desirable(Vec<TableExampleReadOnlyIds>),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: String, rollback: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleCreateManyErrorNamed> for TableExampleCreateManyResponseVariants {
    fn from(value: TableExampleCreateManyErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleCreateManyErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryCreateManyErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleCreateManyErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        create_many_error_named_with_serialize_deserialize: TableExampleCreateManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug)]
pub struct TableExampleCreateOneParameters {
    pub payload: TableExampleCreate,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleCreateOneResponseVariants {
    Desirable(TableExampleReadOnlyIds),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: String, rollback: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleCreateOneErrorNamed> for TableExampleCreateOneResponseVariants {
    fn from(value: TableExampleCreateOneErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleCreateOneErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryCreateOneErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleCreateOneErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        create_one_error_named_with_serialize_deserialize: TableExampleCreateOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleReadManyPayload {
    pub where_many: StdOptionOptionTableExampleWhereMany,
    pub select: postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>,
    pub order_by: postgresql_crud::OrderBy<TableExampleSelect>,
    pub pagination: postgresql_crud::PaginationStartsWithZero,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleReadManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            where_many: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            select: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            order_by: postgresql_crud::OrderBy {
                column: TableExampleSelect::PrimaryKeyColumn(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                order: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            },
            pagination: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug)]
pub struct TableExampleReadManyParameters {
    pub payload: TableExampleReadManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleReadManyResponseVariants {
    Desirable(Vec<TableExampleRead>),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    NotUniqueField { not_unique_field: TableExampleSelect, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleReadManyErrorNamed> for TableExampleReadManyResponseVariants {
    fn from(value: TableExampleReadManyErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleReadManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::NotUniqueField { not_unique_field, code_occurence } => Self::NotUniqueField { not_unique_field, code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleReadManyErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueField {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_field: TableExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryReadManyErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueField {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_field: TableExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleReadManyErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        read_many_error_named_with_serialize_deserialize: TableExampleReadManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleReadOnePayload {
    pub primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead,
    pub select: postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleReadOnePayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            primary_key_column: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            select: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug)]
pub struct TableExampleReadOneParameters {
    pub payload: TableExampleReadOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleReadOneResponseVariants {
    Desirable(TableExampleRead),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    NotUniqueField { not_unique_field: TableExampleSelect, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleReadOneErrorNamed> for TableExampleReadOneResponseVariants {
    fn from(value: TableExampleReadOneErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleReadOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::NotUniqueField { not_unique_field, code_occurence } => Self::NotUniqueField { not_unique_field, code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleReadOneErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueField {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_field: TableExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryReadOneErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueField {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_field: TableExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleReadOneErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        read_one_error_named_with_serialize_deserialize: TableExampleReadOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
pub struct TableExampleUpdateManyPayload(Vec<TableExampleUpdate>);
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleUpdateManyPayloadTryNewErrorNamed {
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate,
        #[eo_to_std_string_string]
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleUpdateManyPayload {
    pub fn try_new(value: Vec<TableExampleUpdate>) -> Result<Self, TableExampleUpdateManyPayloadTryNewErrorNamed> {
        let mut acc = Vec::new();
        for element in &value {
            if acc.contains(&&element.primary_key_column) {
                return Err(TableExampleUpdateManyPayloadTryNewErrorNamed::NotUniquePrimaryKey {
                    not_unique_primary_key: element.primary_key_column.clone(),
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            acc.push(&element.primary_key_column);
        }
        Ok(Self(value))
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for TableExampleUpdateManyPayload {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TableExampleUpdateManyPayload>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TableExampleUpdateManyPayload;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct TableExampleUpdateManyPayload")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: Vec<TableExampleUpdate> = <Vec<TableExampleUpdate> as _serde::Deserialize>::deserialize(__e)?;
                    match TableExampleUpdateManyPayload::try_new(__field0) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<Vec<TableExampleUpdate>>(&mut __seq)? {
                        Some(__value) => __value,
                        None => {
                            return Err(_serde::de::Error::invalid_length(0usize, &"tuple struct TableExampleUpdateManyPayload with 1 element"));
                        }
                    };
                    match TableExampleUpdateManyPayload::try_new(__field0) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(__deserializer, "TableExampleUpdateManyPayload", __Visitor { marker: _serde::__private::PhantomData::<Self>, lifetime: _serde::__private::PhantomData })
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleUpdateManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
#[derive(Debug)]
pub struct TableExampleUpdateManyParameters {
    pub payload: TableExampleUpdateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleUpdateManyResponseVariants {
    Desirable(Vec<TableExampleReadOnlyIds>),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: String, rollback: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleUpdateManyErrorNamed> for TableExampleUpdateManyResponseVariants {
    fn from(value: TableExampleUpdateManyErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleUpdateManyErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryUpdateManyErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleUpdateManyErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        update_many_error_named_with_serialize_deserialize: TableExampleUpdateManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug)]
pub struct TableExampleUpdateOneParameters {
    pub payload: TableExampleUpdate,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleUpdateOneResponseVariants {
    Desirable(TableExampleReadOnlyIds),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: String, rollback: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleUpdateOneErrorNamed> for TableExampleUpdateOneResponseVariants {
    fn from(value: TableExampleUpdateOneErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleUpdateOneErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryUpdateOneErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleUpdateOneErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        update_one_error_named_with_serialize_deserialize: TableExampleUpdateOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleDeleteManyPayload {
    pub where_many: StdOptionOptionTableExampleWhereMany,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleDeleteManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            where_many: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug)]
pub struct TableExampleDeleteManyParameters {
    pub payload: TableExampleDeleteManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleDeleteManyResponseVariants {
    Desirable(Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: String, rollback: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleDeleteManyErrorNamed> for TableExampleDeleteManyResponseVariants {
    fn from(value: TableExampleDeleteManyErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleDeleteManyErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryDeleteManyErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleDeleteManyErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        delete_many_error_named_with_serialize_deserialize: TableExampleDeleteManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleDeleteOnePayload {
    pub primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleDeleteOnePayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            primary_key_column: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug)]
pub struct TableExampleDeleteOneParameters {
    pub payload: TableExampleDeleteOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleDeleteOneResponseVariants {
    Desirable(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: String, rollback: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleDeleteOneErrorNamed> for TableExampleDeleteOneResponseVariants {
    fn from(value: TableExampleDeleteOneErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleDeleteOneErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryDeleteOneErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleDeleteOneErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        delete_one_error_named_with_serialize_deserialize: TableExampleDeleteOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
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
                    let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_6 = format!("{table}_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_6");
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0 = format!("{table}_eb24448c_fa63_4259_bb05_3215802a78f6_column_0");
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_6 = format!("{table}_eb24448c_fa63_4259_bb05_3215802a78f6_column_6");
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0 = format!("{table}_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0");
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_6 = format!("{table}_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_6");
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0 = format!("{table}_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0");
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_6 = format!("{table}_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_6");
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0 = format!("{table}_5a52af33_a590_403b_808e_961df6d7e7aa_column_0");
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_6 = format!("{table}_5a52af33_a590_403b_808e_961df6d7e7aa_column_6");
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0 = format!("{table}_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0");
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_6 = format!("{table}_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_6");
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0 = format!("{table}_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0");
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_6 = format!("{table}_581c947f_9b0f_452f_8e52_524088bbb2e7_column_6");
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0 = format!("{table}_de556c26_9297_4adb_9483_22d474cf1e7d_column_0");
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_6 = format!("{table}_de556c26_9297_4adb_9483_22d474cf1e7d_column_6");
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0 = format!("{table}_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0");
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_6 = format!("{table}_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_6");
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0 = format!("{table}_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0");
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_6 = format!("{table}_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_6");
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
                    let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_6_cloned = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_6.clone();
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0.clone();
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_6_cloned = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_6.clone();
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0.clone();
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_6_cloned = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_6.clone();
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0_cloned = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0.clone();
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_6_cloned = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_6.clone();
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0_cloned = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0.clone();
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_6_cloned = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_6.clone();
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0_cloned = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0.clone();
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_6_cloned = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_6.clone();
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0_cloned = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0.clone();
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_6_cloned = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_6.clone();
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0_cloned = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0.clone();
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_6_cloned = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_6.clone();
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0_cloned = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0.clone();
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_6_cloned = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_6.clone();
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0_cloned = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0.clone();
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_6_cloned = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_6.clone();
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
                    let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_6_cloned2 = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_6.clone();
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned2 = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0.clone();
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_6_cloned2 = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_6.clone();
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned2 = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0.clone();
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_6_cloned2 = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_6.clone();
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0_cloned2 = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0.clone();
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_6_cloned2 = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_6.clone();
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0_cloned2 = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0.clone();
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_6_cloned2 = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_6.clone();
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0_cloned2 = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0.clone();
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_6_cloned2 = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_6.clone();
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0_cloned2 = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0.clone();
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_6_cloned2 = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_6.clone();
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0_cloned2 = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0.clone();
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_6_cloned2 = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_6.clone();
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0_cloned2 = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0.clone();
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_6_cloned2 = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_6.clone();
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0_cloned2 = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0.clone();
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_6_cloned2 = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_6.clone();
                    let table_read_one_cloned2 = table_read_one.clone();
                    let table_update_many_cloned2 = table_update_many.clone();
                    let table_update_one_cloned2 = table_update_one.clone();
                    let table_delete_one_cloned2 = table_delete_one.clone();
                    let drop_all_test_tables = async || {
                        async fn drop_table_if_exists(postgres_pool: &sqlx::Pool<sqlx::Postgres>, table: &str) {
                            let query = format!("drop table if exists {table}");
                            let _unused = sqlx::query(&query).execute(postgres_pool).await.expect("error 1b11bf1b-9180-419f-bae7-b1ab93cd9c57");
                        }
                        drop_table_if_exists(&postgres_pool, table).await;
                        drop_table_if_exists(&postgres_pool, &table_create_many).await;
                        drop_table_if_exists(&postgres_pool, &table_create_one).await;
                        drop_table_if_exists(&postgres_pool, &table_test_read_many_by_non_existent_primary_keys_cloned2).await;
                        drop_table_if_exists(&postgres_pool, &table_test_read_many_by_equal_to_created_primary_keys_cloned2).await;
                        drop_table_if_exists(&postgres_pool, &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_6).await;
                        drop_table_if_exists(&postgres_pool, &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_6).await;
                        drop_table_if_exists(&postgres_pool, &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_6).await;
                        drop_table_if_exists(&postgres_pool, &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_6).await;
                        drop_table_if_exists(&postgres_pool, &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_6).await;
                        drop_table_if_exists(&postgres_pool, &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_6).await;
                        drop_table_if_exists(&postgres_pool, &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_6).await;
                        drop_table_if_exists(&postgres_pool, &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_6).await;
                        drop_table_if_exists(&postgres_pool, &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_6).await;
                        drop_table_if_exists(&postgres_pool, &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_6).await;
                        drop_table_if_exists(&postgres_pool, &table_read_one).await;
                        drop_table_if_exists(&postgres_pool, &table_update_many).await;
                        drop_table_if_exists(&postgres_pool, &table_update_one).await;
                        drop_table_if_exists(&postgres_pool, &table_delete_many).await;
                        drop_table_if_exists(&postgres_pool, &table_delete_one).await;
                    };
                    drop_all_test_tables().await;
                    let postgres_pool_for_tokio_spawn_sync_move = postgres_pool.clone();
                    let (started_tx, started_rx) = tokio::sync::oneshot::channel();
                    let _unused = tokio::spawn(async move {
                        TableExample::prepare_extensions(&postgres_pool_for_tokio_spawn_sync_move).await.expect("error 0633ff48-ebc4-460f-a282-d750511f5d78");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, table).await.expect("error 0c29cf7d-1af7-459c-b0c6-69855ca98bef");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_create_many_cloned).await.expect("error 141d990c-91e5-4518-8978-7660fcf88784");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_create_one_cloned).await.expect("error cdd3b111-5e8b-4201-896e-bd38dc8b4d7c");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_test_read_many_by_non_existent_primary_keys_cloned).await.expect("error 56a27d70-0393-4759-9d02-f9eb1e623f5f");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_test_read_many_by_equal_to_created_primary_keys_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_6_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_6_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_6_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_6_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_6_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_6_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_6_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_6_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_6_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_6_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_read_one_cloned).await.expect("error 425e8574-6cdd-43b5-9b7b-75efce9b750d");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_update_many_cloned).await.expect("error ab50eb74-29ab-49b3-bdd4-ff6c6c6b700a");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_update_one_cloned).await.expect("error de8885ae-34f5-430b-a3b4-bf91c999b2c8");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_delete_many_cloned).await.expect("error 2bb3d5ec-1069-470c-a758-60afe3bd5224");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_delete_one_cloned).await.expect("error e5cc2f6f-65a2-472d-8a1e-56e23fbc165a");
                        let app_state = std::sync::Arc::new(server_app_state::ServerAppState {
                            postgres_pool: postgres_pool_for_tokio_spawn_sync_move.clone(),
                            config,
                            project_git_info: &git_info::PROJECT_GIT_INFO,
                        });
                        let tcp_listener = tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(&config)).await.expect("error 663ae29e-bc00-4ea1-a7e9-4dddceb5b53a");
                        if let Err(error) = started_tx.send(()) {
                            panic!("error aa3b8154-1fe2-4d3f-a164-26f9d21245cd {error:#?}");
                        }
                        axum::serve(
                            tcp_listener,
                            axum::Router::new()
                                .merge(TableExample::routes(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state)))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_create_many_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_create_one_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_test_read_many_by_non_existent_primary_keys_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_test_read_many_by_equal_to_created_primary_keys_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_6_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_6_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_6_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_6_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_6_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_6_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_6_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_6_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_6_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_6_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_read_one_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_update_many_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_update_one_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_delete_many_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_delete_one_cloned))
                                .into_make_service(),
                        )
                        .await
                        .unwrap_or_else(|error| panic!("axum builder serve await failed {error:#?}"));
                    });
                    started_rx.await.expect("error 87003141-43a4-4975-8ddf-273148add50f");
                    let select_primary_key = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default())]).expect("error 0776170e-4dd6-4c14-a412-ce10b0c746f1");
                    let ident_create_default = TableExampleCreate {
                        column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                        column_6: <<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                    };
                    let select_default_all_with_max_page_size = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![
                        TableExampleSelect::PrimaryKeyColumn(<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize>::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size()),
                        TableExampleSelect::Column0(<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize>::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size()),
                        TableExampleSelect::Column6(<<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize>::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size()),
                    ])
                    .expect("error 8f42ee4f-00d9-4b67-8ead-adddf5bcdf94");
                    let common_read_only_ids_returned_from_create_one = TableExample::try_create_one(&url, TableExampleCreateOneParameters { payload: ident_create_default.clone() }).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
                    let some_value_read_only_ids_returned_from_create_one = Some(postgresql_crud::Value {
                        value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(common_read_only_ids_returned_from_create_one.primary_key_column.clone()),
                    });
                    assert_eq!(
                        TableExampleRead {
                            primary_key_column: some_value_read_only_ids_returned_from_create_one.clone(),
                            column_0: None,
                            column_6: None
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
                    let read_only_ids_from_try_delete_one = TableExample::try_delete_one(
                        &url,
                        TableExampleDeleteOneParameters {
                            payload: TableExampleDeleteOnePayload {
                                primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(common_read_only_ids_returned_from_create_one.primary_key_column.clone()),
                            },
                        },
                    )
                    .await
                    .expect("error 006b18e9-c965-45ee-afc0-a4f6b850ed06");
                    assert_eq!(read_only_ids_from_try_delete_one, <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(common_read_only_ids_returned_from_create_one.primary_key_column.clone()), "error 26e2058b-4bc1-42da-8f35-0ab993904de5");
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
                    futures::StreamExt::for_each_concurrent(
                        futures::stream::iter({
                            let mut acc: Vec<futures::future::BoxFuture<'static, ()>> = vec![];
                            {
                                let current_table = table_update_many_cloned2.clone();
                                {
                                    let read_only_ids_to_two_dimensional_vec_read_inner_acc = {
                                        let mut acc = vec![];
                                        if let Some(value) = &common_read_only_ids_returned_from_create_one.column_0 {
                                            for element in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(value) {
                                                for _ in element {
                                                    acc.push(ident_create_default.clone());
                                                }
                                            }
                                        }
                                        acc
                                    };
                                    if read_only_ids_to_two_dimensional_vec_read_inner_acc.is_empty() {
                                        println!("PostgresqlTypeTestCases read_only_ids_to_two_dimensional_vec_read_inner is empty for column_0");
                                    } else {
                                        let current_table = current_table.clone();
                                        let read_only_ids_current_elements = {
                                            futures::StreamExt::collect::<Vec<Vec<TableExampleReadOnlyIds>>>(futures::StreamExt::buffer_unordered(
                                                futures::stream::iter(read_only_ids_to_two_dimensional_vec_read_inner_acc.chunks(25).map(Vec::from).map(|element| {
                                                    let current_table = current_table.clone();
                                                    let url_cloned = url.clone();
                                                    futures::FutureExt::boxed(async move { TableExample::try_create_many_handle(&url_cloned, TableExampleCreateManyParameters { payload: TableExampleCreateManyPayload(element) }, &current_table).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98") })
                                                })),
                                                5,
                                            ))
                                            .await
                                            .into_iter()
                                            .flatten()
                                            .collect::<Vec<TableExampleReadOnlyIds>>()
                                        };
                                        assert_eq!(
                                            {
                                                let mut acc = vec![];
                                                for element in &read_only_ids_current_elements {
                                                    acc.push(TableExampleRead {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&element.primary_key_column),
                                                        column_0: match &element.column_0 {
                                                            Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value),
                                                            None => Some(postgresql_crud::Value {
                                                                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                            }),
                                                        },
                                                        column_6: match &element.column_6 {
                                                            Some(value) => <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value),
                                                            None => Some(postgresql_crud::Value {
                                                                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                            }),
                                                        },
                                                    });
                                                }
                                                acc.sort_by(|first, second| {
                                                    if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                        value_first.value.cmp(&value_second.value)
                                                    } else {
                                                        panic!("must not be what");
                                                    }
                                                });
                                                acc
                                            },
                                            {
                                                let mut acc = TableExample::try_read_many_handle(
                                                    &url,
                                                    TableExampleReadManyParameters {
                                                        payload: TableExampleReadManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                TableExampleWhereMany::try_new(
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                            let mut acc = vec![];
                                                                            for element in &read_only_ids_current_elements {
                                                                                acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        element.primary_key_column.clone(),
                                                                                    ))),
                                                                                }));
                                                                            }
                                                                            acc
                                                                        })
                                                                        .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
                                                                    ),
                                                                    None,
                                                                    None,
                                                                )
                                                                .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
                                                            )),
                                                            select: select_default_all_with_max_page_size.clone(),
                                                            order_by: postgresql_crud::OrderBy {
                                                                column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                order: Some(postgresql_crud::Order::Asc),
                                                            },
                                                            pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                acc.sort_by(|first, second| if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) { value_first.value.cmp(&value_second.value) } else { panic!("must not be what") });
                                                acc
                                            },
                                            "error 50198a7f-e65c-4e4e-8d7f-9881cfd42453"
                                        );
                                        for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
                                            let current_table = table_update_many_cloned2.clone();
                                            let url_cloned = url.clone();
                                            let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                            acc.push(futures::FutureExt::boxed(async move {
                                                let previous_read = {
                                                    let mut acc = TableExample::try_read_many_handle(
                                                        &url_cloned,
                                                        TableExampleReadManyParameters {
                                                            payload: TableExampleReadManyPayload {
                                                                where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                    TableExampleWhereMany::try_new(
                                                                        Some(
                                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                postgresql_crud::LogicalOperator::Or,
                                                                                vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        read_only_ids_current_element.primary_key_column.clone(),
                                                                                    ))),
                                                                                })],
                                                                            )
                                                                            .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
                                                                        ),
                                                                        None,
                                                                        None,
                                                                    )
                                                                    .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
                                                                )),
                                                                select: select_default_all_with_max_page_size_cloned.clone(),
                                                                order_by: postgresql_crud::OrderBy {
                                                                    column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                    order: Some(postgresql_crud::Order::Asc),
                                                                },
                                                                pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                            },
                                                        },
                                                        &current_table,
                                                    )
                                                    .await
                                                    .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                    acc.sort_by(|first, second| {
                                                        if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                            value_first.value.cmp(&value_second.value)
                                                        } else {
                                                            panic!("must not be what");
                                                        }
                                                    });
                                                    acc
                                                };
                                                let update = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_inner_into_update_with_new_or_try_new_unwraped({
                                                    let mut local_increment: usize = 0;
                                                    let mut option_test_case = None;
                                                    for element_0 in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&read_only_ids_current_element.column_0.clone().expect("error c4d98a71-f30f-410e-b410-a75f4672f2f7")) {
                                                        let mut should_break = false;
                                                        for element_1 in element_0 {
                                                            if local_increment == increment {
                                                                option_test_case = Some(element_1);
                                                                should_break = true;
                                                                break;
                                                            }
                                                            local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
                                                        }
                                                        if should_break {
                                                            break;
                                                        }
                                                    }
                                                    option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
                                                });
                                                assert_eq!(
                                                    vec![TableExampleReadOnlyIds {
                                                        primary_key_column: read_only_ids_current_element.primary_key_column.clone(),
                                                        column_0: Some(<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update)),
                                                        column_6: None
                                                    }],
                                                    TableExample::try_update_many_handle(
                                                        &url_cloned,
                                                        TableExampleUpdateManyParameters {
                                                            payload: TableExampleUpdateManyPayload::try_new(vec![
                                                                TableExampleUpdate::try_new(
                                                                    <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_update(read_only_ids_current_element.primary_key_column.clone())),
                                                                    Some(postgresql_crud::Value { value: update.clone() }),
                                                                    None
                                                                )
                                                                .expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2")
                                                            ])
                                                            .expect("error 69e1bd8a-fe78-4301-85ca-f4f3958d7493")
                                                        },
                                                        &current_table
                                                    )
                                                    .await
                                                    .expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52"),
                                                    "error 34bfb3c7-7a53-479e-9d4f-0856003573e1"
                                                );
                                                assert_eq!(
                                                    {
                                                        let mut acc = vec![];
                                                        for element in previous_read {
                                                            acc.push(TableExampleRead {
                                                                primary_key_column: Some(postgresql_crud::Value {
                                                                    value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_current_element.primary_key_column.clone()),
                                                                }),
                                                                column_0: Some(postgresql_crud::Value {
                                                                    value: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::previous_read_merged_with_option_update_into_read(
                                                                        <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_current_element.column_0.clone().expect("error 4f19d0d2-a23f-4b77-a2bc-c7b04db7a129"))
                                                                            .expect("error c7685b19-9bca-47bc-a3a5-8fc543b174a5")
                                                                            .value,
                                                                        Some(update.clone()),
                                                                    ),
                                                                }),
                                                                column_6: element.column_6,
                                                            });
                                                        }
                                                        acc
                                                    },
                                                    {
                                                        let mut acc = TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                        primary_key_column: Some(
                                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                postgresql_crud::LogicalOperator::Or,
                                                                                vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        read_only_ids_current_element.primary_key_column.clone(),
                                                                                    ))),
                                                                                })],
                                                                            )
                                                                            .expect("error 5f1e5f9d-d189-4368-807e-a84348967610"),
                                                                        ),
                                                                        column_0: None,
                                                                        column_6: None,
                                                                    })),
                                                                    select: select_default_all_with_max_page_size_cloned,
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc),
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                                },
                                                            },
                                                            &current_table,
                                                        )
                                                        .await
                                                        .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                        acc.sort_by(|first, second| {
                                                            if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                                value_first.value.cmp(&value_second.value)
                                                            } else {
                                                                panic!("must not be what");
                                                            }
                                                        });
                                                        acc
                                                    },
                                                    "error ae2a2da5-3697-4fd7-9ad2-4a535618fbc3"
                                                );
                                            }));
                                        }
                                    }
                                };
                                {
                                    let read_only_ids_to_two_dimensional_vec_read_inner_acc = {
                                        let mut acc = vec![];
                                        if let Some(value) = &common_read_only_ids_returned_from_create_one.column_6 {
                                            for element in <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(value) {
                                                for _ in element {
                                                    acc.push(ident_create_default.clone());
                                                }
                                            }
                                        }
                                        acc
                                    };
                                    if read_only_ids_to_two_dimensional_vec_read_inner_acc.is_empty() {
                                        println!("PostgresqlTypeTestCases read_only_ids_to_two_dimensional_vec_read_inner is empty for column_6");
                                    } else {
                                        let current_table = current_table.clone();
                                        let read_only_ids_current_elements = {
                                            futures::StreamExt::collect::<Vec<Vec<TableExampleReadOnlyIds>>>(futures::StreamExt::buffer_unordered(
                                                futures::stream::iter(read_only_ids_to_two_dimensional_vec_read_inner_acc.chunks(25).map(Vec::from).map(|element| {
                                                    let current_table = current_table.clone();
                                                    let url_cloned = url.clone();
                                                    futures::FutureExt::boxed(async move { TableExample::try_create_many_handle(&url_cloned, TableExampleCreateManyParameters { payload: TableExampleCreateManyPayload(element) }, &current_table).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98") })
                                                })),
                                                5,
                                            ))
                                            .await
                                            .into_iter()
                                            .flatten()
                                            .collect::<Vec<TableExampleReadOnlyIds>>()
                                        };
                                        assert_eq!(
                                            {
                                                let mut acc = vec![];
                                                for element in &read_only_ids_current_elements {
                                                    acc.push(TableExampleRead {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&element.primary_key_column),
                                                        column_0: match &element.column_0 {
                                                            Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value),
                                                            None => Some(postgresql_crud::Value {
                                                                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                            }),
                                                        },
                                                        column_6: match &element.column_6 {
                                                            Some(value) => <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value),
                                                            None => Some(postgresql_crud::Value {
                                                                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                            }),
                                                        },
                                                    });
                                                }
                                                acc.sort_by(|first, second| {
                                                    if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                        value_first.value.cmp(&value_second.value)
                                                    } else {
                                                        panic!("must not be what");
                                                    }
                                                });
                                                acc
                                            },
                                            {
                                                let mut acc = TableExample::try_read_many_handle(
                                                    &url,
                                                    TableExampleReadManyParameters {
                                                        payload: TableExampleReadManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                TableExampleWhereMany::try_new(
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                            let mut acc = vec![];
                                                                            for element in &read_only_ids_current_elements {
                                                                                acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        element.primary_key_column.clone(),
                                                                                    ))),
                                                                                }));
                                                                            }
                                                                            acc
                                                                        })
                                                                        .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
                                                                    ),
                                                                    None,
                                                                    None,
                                                                )
                                                                .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
                                                            )),
                                                            select: select_default_all_with_max_page_size.clone(),
                                                            order_by: postgresql_crud::OrderBy {
                                                                column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                order: Some(postgresql_crud::Order::Asc),
                                                            },
                                                            pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                acc.sort_by(|first, second| if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) { value_first.value.cmp(&value_second.value) } else { panic!("must not be what") });
                                                acc
                                            },
                                            "error 50198a7f-e65c-4e4e-8d7f-9881cfd42453"
                                        );
                                        for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
                                            let current_table = table_update_many_cloned2.clone();
                                            let url_cloned = url.clone();
                                            let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                            acc.push(futures::FutureExt::boxed(async move {
                                                let previous_read = {
                                                    let mut acc = TableExample::try_read_many_handle(
                                                        &url_cloned,
                                                        TableExampleReadManyParameters {
                                                            payload: TableExampleReadManyPayload {
                                                                where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                    TableExampleWhereMany::try_new(
                                                                        Some(
                                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                postgresql_crud::LogicalOperator::Or,
                                                                                vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        read_only_ids_current_element.primary_key_column.clone(),
                                                                                    ))),
                                                                                })],
                                                                            )
                                                                            .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
                                                                        ),
                                                                        None,
                                                                        None,
                                                                    )
                                                                    .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
                                                                )),
                                                                select: select_default_all_with_max_page_size_cloned.clone(),
                                                                order_by: postgresql_crud::OrderBy {
                                                                    column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                    order: Some(postgresql_crud::Order::Asc),
                                                                },
                                                                pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                            },
                                                        },
                                                        &current_table,
                                                    )
                                                    .await
                                                    .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                    acc.sort_by(|first, second| {
                                                        if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                            value_first.value.cmp(&value_second.value)
                                                        } else {
                                                            panic!("must not be what");
                                                        }
                                                    });
                                                    acc
                                                };
                                                let update = <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlTypeTestCases>::read_inner_into_update_with_new_or_try_new_unwraped({
                                                    let mut local_increment: usize = 0;
                                                    let mut option_test_case = None;
                                                    for element_0 in <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&read_only_ids_current_element.column_6.clone().expect("error c4d98a71-f30f-410e-b410-a75f4672f2f7")) {
                                                        let mut should_break = false;
                                                        for element_1 in element_0 {
                                                            if local_increment == increment {
                                                                option_test_case = Some(element_1);
                                                                should_break = true;
                                                                break;
                                                            }
                                                            local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
                                                        }
                                                        if should_break {
                                                            break;
                                                        }
                                                    }
                                                    option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
                                                });
                                                assert_eq!(
                                                    vec![TableExampleReadOnlyIds {
                                                        primary_key_column: read_only_ids_current_element.primary_key_column.clone(),
                                                        column_0: None,
                                                        column_6: Some(<postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update))
                                                    }],
                                                    TableExample::try_update_many_handle(
                                                        &url_cloned,
                                                        TableExampleUpdateManyParameters {
                                                            payload: TableExampleUpdateManyPayload::try_new(vec![
                                                                TableExampleUpdate::try_new(
                                                                    <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_update(read_only_ids_current_element.primary_key_column.clone())),
                                                                    None,
                                                                    Some(postgresql_crud::Value { value: update.clone() })
                                                                )
                                                                .expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2")
                                                            ])
                                                            .expect("error 69e1bd8a-fe78-4301-85ca-f4f3958d7493")
                                                        },
                                                        &current_table
                                                    )
                                                    .await
                                                    .expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52"),
                                                    "error 34bfb3c7-7a53-479e-9d4f-0856003573e1"
                                                );
                                                assert_eq!(
                                                    {
                                                        let mut acc = vec![];
                                                        for element in previous_read {
                                                            acc.push(TableExampleRead {
                                                                primary_key_column: Some(postgresql_crud::Value {
                                                                    value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_current_element.primary_key_column.clone()),
                                                                }),
                                                                column_0: element.column_0,
                                                                column_6: Some(postgresql_crud::Value {
                                                                    value: <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlTypeTestCases>::previous_read_merged_with_option_update_into_read(
                                                                        <postgresql_crud::StdPrimitiveI32AsNotNullInt4 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_current_element.column_6.clone().expect("error 4f19d0d2-a23f-4b77-a2bc-c7b04db7a129"))
                                                                            .expect("error c7685b19-9bca-47bc-a3a5-8fc543b174a5")
                                                                            .value,
                                                                        Some(update.clone()),
                                                                    ),
                                                                }),
                                                            });
                                                        }
                                                        acc
                                                    },
                                                    {
                                                        let mut acc = TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                        primary_key_column: Some(
                                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                postgresql_crud::LogicalOperator::Or,
                                                                                vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        read_only_ids_current_element.primary_key_column.clone(),
                                                                                    ))),
                                                                                })],
                                                                            )
                                                                            .expect("error 5f1e5f9d-d189-4368-807e-a84348967610"),
                                                                        ),
                                                                        column_0: None,
                                                                        column_6: None,
                                                                    })),
                                                                    select: select_default_all_with_max_page_size_cloned,
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc),
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                                },
                                                            },
                                                            &current_table,
                                                        )
                                                        .await
                                                        .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                        acc.sort_by(|first, second| {
                                                            if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                                value_first.value.cmp(&value_second.value)
                                                            } else {
                                                                panic!("must not be what");
                                                            }
                                                        });
                                                        acc
                                                    },
                                                    "error ae2a2da5-3697-4fd7-9ad2-4a535618fbc3"
                                                );
                                            }));
                                        }
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
