#[derive(Debug
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
// #[postgresql_crud::create_many_additional_logic{
//     println!("GeneratePostgresqlTypesExample create_many log");
// }]
// #[postgresql_crud::create_one_additional_logic{}]
// #[postgresql_crud::read_many_additional_logic{}]
// #[postgresql_crud::read_one_additional_logic{}]
// #[postgresql_crud::update_many_additional_logic{}]
// #[postgresql_crud::update_one_additional_logic{}]
// #[postgresql_crud::delete_many_additional_logic{}]
// #[postgresql_crud::delete_one_additional_logic{}]
// #[postgresql_crud::common_additional_logic{}]
pub struct Example {
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
    pub column_30: postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql,
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

    // pub column_154: crate::repositories_types::server::routes::api::example::AnimalAsNotNullJsonbObject,
    // pub column_155: crate::repositories_types::server::routes::api::example::OptionAnimalAsNullableJsonbObject,
    // pub column_156: crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId,
    // pub column_157: crate::repositories_types::server::routes::api::example::OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId,
}

// #[derive(Debug
//     , postgresql_crud::GeneratePostgresqlJsonObjectType
// )] //
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

// #[derive(Debug
//     , postgresql_crud::GeneratePostgresqlJsonObjectType
// )]
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

// #[derive(Debug
//     , postgresql_crud::GeneratePostgresqlJsonObjectType
// )]
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
//     pub field_1: postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber,
//     pub field_2: postgresql_crud::VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber,
//     pub field_3: postgresql_crud::VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber,
//     pub field_4: postgresql_crud::OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber,
//     pub field_5: postgresql_crud::OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber,
//     // pub field_6: postgresql_crud::VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
//     // pub field_7: postgresql_crud::VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
//     // pub field_8: postgresql_crud::VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
//     // pub field_9: postgresql_crud::VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
//     // pub field_10: postgresql_crud::OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
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
//                         let app_state = std::sync::Arc::new(crate::repositories_types::server::routes::app_state::AppState {
//                             postgres_pool: postgres_pool_for_tokio_spawn_sync_move.clone(),
//                             config: &config,
//                             project_git_info: &git_info::PROJECT_GIT_INFO,
//                         });
//                         axum::serve(
//                             tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(&config)).await.expect("error 663ae29e-bc00-4ea1-a7e9-4dddceb5b53a"),
//                             axum::Router::new().merge(super::Example::routes(std::sync::Arc::<crate::repositories_types::server::routes::app_state::AppState<'_>>::clone(&app_state))).into_make_service(),
//                         )
//                         .await
//                         .unwrap_or_else(|error| panic!("axum builder serve await failed {error:#?}"));
//                     });
//                     tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
//                     let select_primary_key = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default())]).expect("error 0776170e-4dd6-4c14-a412-ce10b0c746f1");
//                     let sort_vec_of_ident_read_with_primary_key_by_primary_key = |mut value: std::vec::Vec<super::ExampleRead>| -> std::vec::Vec<super::ExampleRead> {
//                         value.sort_by_key(|element| element.primary_key_column.clone().expect("error 4f25860e-5b1a-408f-a4db-d49b6969ad4a").value);
//                         value
//                     };
//                     let ident_create_default = super::ExampleCreate {
//                         column_156: <<crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                     };
//                     let common_read_only_ids_returned_from_create_one = super::Example::try_create_one(&url, super::ExampleCreateOneParameters { payload: ident_create_default.clone() }).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
//                     let some_value_read_only_ids_returned_from_create_one = Some(postgresql_crud::Value { value: common_read_only_ids_returned_from_create_one.primary_key_column.clone() });
//                     assert_eq!(
//                         super::ExampleRead {
//                             primary_key_column: some_value_read_only_ids_returned_from_create_one.clone(),
//                             column_156: None
//                         },
//                         super::Example::try_read_one(
//                             &url,
//                             super::ExampleReadOneParameters {
//                                 payload: super::ExampleReadOnePayload {
//                                     primary_key_column: common_read_only_ids_returned_from_create_one.primary_key_column.clone(),
//                                     select: select_primary_key.clone(),
//                                 },
//                             },
//                         )
//                         .await
//                         .expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
//                         "try_read_one result different after try_create_one 3d9f2ec0-e374-48d2-a36b-486f5598b0b4"
//                     );
//                     let select_default_all = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![
//                         super::ExampleSelect::PrimaryKeyColumn(<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//                         super::ExampleSelect::Column156(<<crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
//                     ])
//                     .expect("error 0776170e-4dd6-4c14-a412-ce10b0c746f1");

//                     ////////////////////
//                     let start = std::time::Instant::now(); 
//                     futures::StreamExt::for_each_concurrent(
//                         futures::stream::iter({
//                             let mut acc: std::vec::Vec<futures::future::BoxFuture<'static, ()>> = vec![];
//                             let chunked = <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::create_vec()
//                                 .chunks(10)
//                                 .map(|element| element.to_vec())
//                                 .collect::<std::vec::Vec<std::vec::Vec<super::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate>>>();
//                             for chunk in chunked {
//                                 let url_cloned = url.clone();
//                                 let ident_create_default_cloned = ident_create_default.clone();
//                                 let select_default_all_cloned = select_default_all.clone();
//                                 acc.push(futures::FutureExt::boxed(async move {
//                                     let ident_create_vec = {
//                                         let mut acc = vec![];
//                                         for element in chunk {
//                                             acc.push(super::ExampleCreate {
//                                                 column_156: element,
//                                                 // column_156: <<crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                                             });
//                                         }
//                                         acc
//                                     };
//                                     let read_only_ids_from_try_create_many = super::Example::try_create_many(&url_cloned, super::ExampleCreateManyParameters {
//                                         payload: super::ExampleCreateManyPayload(ident_create_vec.clone())
//                                     }).await.expect("error 5eecedc4-bb02-454a-acd9-0af758f30b2e");
//                                     assert_eq!(
//                                         {
//                                             let mut acc = vec![];
//                                             assert_eq!(read_only_ids_from_try_create_many.len(), ident_create_vec.len(), "error 39572295-b6a4-49d7-a65a-16f8bcf44ede");
//                                             for (read_only_ids, create) in read_only_ids_from_try_create_many.clone().into_iter().zip(ident_create_vec.into_iter()).collect::<std::vec::Vec<(super::ExampleReadOnlyIds, super::ExampleCreate)>>() {
//                                                 acc.push(super::ExampleRead {
//                                                     primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids.primary_key_column),
//                                                     column_156: <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(
//                                                         read_only_ids.column_156.expect("error 2432bf87-cf90-45c9-9a7c-f1d2283d22f3"),
//                                                         create.column_156
//                                                     )
//                                                 });
//                                             }
//                                             acc.sort_by(|a, b| {
//                                                 if let (Some(value_a), Some(value_b)) = (&a.primary_key_column, &b.primary_key_column) {
//                                                     value_a.value.cmp(&value_b.value)
//                                                 } else {
//                                                     panic!("must not be what error 4428083a-53be-4184-a5b7-94ae2de21d40");
//                                                 }
//                                             });
//                                             acc
//                                         },
//                                         super::Example::try_read_many(
//                                             &url_cloned,
//                                             super::ExampleReadManyParameters {
//                                                 payload: super::ExampleReadManyPayload {
//                                                     where_many: super::StdOptionOptionExampleWhereMany(Some(
//                                                         super::ExampleWhereMany::try_new(
//                                                             Some(
//                                                                 postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
//                                                                     let mut acc = vec![];
//                                                                     for element in &read_only_ids_from_try_create_many {
//                                                                         acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual {
//                                                                             logical_operator: postgresql_crud::LogicalOperator::Or,
//                                                                             value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone())),
//                                                                         }));
//                                                                     }
//                                                                     acc
//                                                                 })
//                                                                 .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
//                                                             ),
//                                                             None,
//                                                         )
//                                                         .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
//                                                     )),
//                                                     select: select_default_all_cloned.clone(),
//                                                     order_by: postgresql_crud::OrderBy {
//                                                         column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
//                                                         order: Some(postgresql_crud::Order::Asc),
//                                                     },
//                                                     pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
//                                                 }
//                                             }
//                                         )
//                                         .await
//                                         .expect("error cb64ceaf-74a4-4501-b614-0c9d9e4e0598"),
//                                         "try_read_one result different after try_create_one error d19bbbf6-f64c-4151-8b5b-998a93e13af5"
//                                     );
//                                     let read_only_ids_from_try_delete_many = {
//                                         let mut acc = super::Example::try_delete_many(
//                                             &url_cloned,
//                                             super::ExampleDeleteManyParameters {
//                                                 payload: super::ExampleDeleteManyPayload {
//                                                     where_many: super::StdOptionOptionExampleWhereMany(Some(super::ExampleWhereMany {
//                                                         primary_key_column: Some(postgresql_crud::PostgresqlTypeWhere::try_new(
//                                                             postgresql_crud::LogicalOperator::Or,
//                                                             {
//                                                                 let mut acc = vec![];
//                                                                 for element in &read_only_ids_from_try_create_many {
//                                                                     acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual {
//                                                                         logical_operator: postgresql_crud::LogicalOperator::Or,
//                                                                         value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as     postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone())),
//                                                                     }));
//                                                                 }
//                                                                 acc
//                                                             }
//                                                         ).expect("error 5f1e5f9d-d189-4368-807e-a84348967610")),
//                                                         column_156: None,
//                                                     }))
//                                                 }
//                                             }
//                                         ).await.expect("error 716e470e-d738-4642-adfc-df1f9b945d27");
//                                         acc.sort_by(|a,b|a.cmp(&b));
//                                         acc
//                                     };
//                                     assert_eq!(
//                                         read_only_ids_from_try_delete_many,
//                                         {
//                                             let mut acc = read_only_ids_from_try_create_many.into_iter().map(|element|element.primary_key_column.clone()).collect::<std::vec::Vec<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead>>();
//                                             acc.sort_by(|a,b|a.cmp(&b));
//                                             acc
//                                         },
//                                         "error f58f5572-4286-4a74-8006-0507339910d4"
//                                     );
//                                     match super::Example::try_read_many(
//                                         &url_cloned,
//                                         super::ExampleReadManyParameters {
//                                             payload: super::ExampleReadManyPayload {
//                                                 where_many: super::StdOptionOptionExampleWhereMany(Some(
//                                                     super::ExampleWhereMany::try_new(
//                                                         Some(
//                                                             postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
//                                                                 let mut acc = vec![];
//                                                                 for element in &read_only_ids_from_try_delete_many {
//                                                                     acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual {
//                                                                         logical_operator: postgresql_crud::LogicalOperator::Or,
//                                                                         value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.clone())),
//                                                                     }));
//                                                                 }
//                                                                 acc
//                                                             })
//                                                             .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
//                                                         ),
//                                                         None,
//                                                     )
//                                                     .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
//                                                 )),
//                                                 select: select_default_all_cloned,
//                                                 order_by: postgresql_crud::OrderBy {
//                                                     column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
//                                                     order: Some(postgresql_crud::Order::Asc),
//                                                 },
//                                                 pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
//                                             }
//                                         }
//                                     )
//                                     .await {
//                                         Ok(value) => {
//                                             if value != std::vec::Vec::new() {
//                                                 panic!("error 4e88679a-0d23-418f-8767-4e9b7531429c");
//                                             }
//                                         },
//                                         Err(error) => {
//                                             panic!("error 24ab86d6-15c9-47f1-a43f-c5fac4b38188 {error:#?}");
//                                         }
//                                     }
//                                 }));
//                             }
//                             acc
//                         }),
//                         10,
//                         |fut| async move {
//                             fut.await;
//                         },
//                     )
//                     .await;
//                     let create_many_elapsed = start.elapsed();
//                     futures::StreamExt::for_each_concurrent(
//                         futures::stream::iter({
//                             let mut acc: std::vec::Vec<futures::future::BoxFuture<'static, ()>> = vec![];
//                             for element in <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::create_vec() {
//                                 let url_cloned = url.clone();
//                                 let ident_create_default_cloned = ident_create_default.clone();
//                                 let select_default_all_cloned = select_default_all.clone();
//                                 acc.push(futures::FutureExt::boxed(async move {
//                                     let ident_create = super::ExampleCreate {
//                                         column_156: element,
//                                         // column_156: <<crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                                     };
//                                     let read_only_ids_from_try_create_one = super::Example::try_create_one(&url_cloned, super::ExampleCreateOneParameters {
//                                             payload: ident_create.clone()
//                                     }).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
//                                     assert_eq!(
//                                         super::ExampleRead {
//                                             primary_key_column: Some(postgresql_crud::Value {
//                                                 value: read_only_ids_from_try_create_one.primary_key_column.clone()
//                                             }),
//                                             column_156: <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(
//                                                 read_only_ids_from_try_create_one.column_156.expect("error 2432bf87-cf90-45c9-9a7c-f1d2283d22f3"),
//                                                 ident_create.column_156
//                                             )
//                                         },
//                                         super::Example::try_read_one(
//                                             &url_cloned,
//                                             super::ExampleReadOneParameters {
//                                                 payload: super::ExampleReadOnePayload {
//                                                     primary_key_column: read_only_ids_from_try_create_one.primary_key_column.clone(),
//                                                     select: select_default_all_cloned.clone()
//                                                 }
//                                             }
//                                         )
//                                         .await
//                                         .expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
//                                         "try_read_one result different after try_create_one"
//                                     );
//                                     let read_only_ids_from_try_delete_one = super::Example::try_delete_one(
//                                         &url_cloned,
//                                         super::ExampleDeleteOneParameters {
//                                             payload: super::ExampleDeleteOnePayload {
//                                                 primary_key_column: read_only_ids_from_try_create_one.primary_key_column.clone()
//                                             }
//                                         }
//                                     ).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
//                                     assert_eq!(
//                                         read_only_ids_from_try_delete_one,
//                                         read_only_ids_from_try_create_one.primary_key_column.clone(),
//                                         "error 4f563faf-1d9b-4ef3-8636-f93fde8ef235"
//                                     );
//                                     if let Err(error) = super::Example::try_read_one(
//                                         &url_cloned,
//                                         super::ExampleReadOneParameters {
//                                             payload: super::ExampleReadOnePayload {
//                                                 primary_key_column: read_only_ids_from_try_create_one.primary_key_column.clone(),
//                                                 select: select_default_all_cloned
//                                             }
//                                         }
//                                     )
//                                     .await {
//                                         if let super::ExampleTryReadOneErrorNamed::ExampleReadOneErrorNamedWithSerializeDeserialize {
//                                             read_one_error_named_with_serialize_deserialize,
//                                             code_occurence,
//                                         } = error {
//                                             if let super::ExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql {
//                                                 postgresql,
//                                                 code_occurence
//                                             } = read_one_error_named_with_serialize_deserialize {
//                                                 if postgresql != "no rows returned by a query that expected to return at least one row" {
//                                                     panic!("error d7152378-3a59-4050-8710-87b7000c8e3d");
//                                                 }
//                                             }
//                                             else {
//                                                  panic!("error e1ac93a5-59e6-477e-a99d-c02e99497421");
//                                             }
//                                         }
//                                         else {
//                                             panic!("error bcd3f9bf-d6b7-4594-b078-8fe9c34bcf18")
//                                         }
//                                     }
//                                     else {
//                                         panic!("error 893263c9-7c62-4551-9225-74153c6e1c57")
//                                     }
//                                 }));
//                             }
//                             acc
//                         }),
//                         10,
//                         |fut| async move {
//                             fut.await;
//                         },
//                     )
//                     .await;
//                     let create_one_elapsed = start.elapsed();
//                     println!("Elapsed: create_many_elapsed {:?}, create_one_elapsed {:?}", create_many_elapsed, create_one_elapsed);
//                     ////////////////////

//                     futures::StreamExt::for_each_concurrent(
//                         futures::stream::iter({
//                             let mut acc: std::vec::Vec<futures::future::BoxFuture<'static, ()>> = vec![];
//                             {
//                                 let read_only_ids_current_elements = {
//                                     use futures::StreamExt;
//                                     futures::stream::iter(
//                                         {
//                                             let mut acc = vec![];
//                                             if let Some(value) = &common_read_only_ids_returned_from_create_one.column_156 {
//                                                 for element0 in <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::read_inner_vec_vec(&value) {
//                                                     for element1 in element0 {
//                                                         acc.push(ident_create_default.clone());
//                                                     }
//                                                 }
//                                             }
//                                             acc
//                                         }
//                                         .chunks(25)
//                                         .map(|element| element.to_vec())
//                                         .collect::<std::vec::Vec<std::vec::Vec<super::ExampleCreate>>>()
//                                         .into_iter()
//                                         .map(|element| {
//                                             let url_cloned = url.clone();
//                                             futures::FutureExt::boxed(async move { super::Example::try_create_many(&url_cloned, super::ExampleCreateManyParameters { payload: super::ExampleCreateManyPayload(element) }).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98") })
//                                         })
//                                         .collect::<std::vec::Vec<futures::future::BoxFuture<'static, std::vec::Vec<super::ExampleReadOnlyIds>>>>(),
//                                     )
//                                     .buffer_unordered(5)
//                                     .collect::<std::vec::Vec<std::vec::Vec<super::ExampleReadOnlyIds>>>()
//                                     .await
//                                     .into_iter()
//                                     .flatten()
//                                     .collect::<std::vec::Vec<super::ExampleReadOnlyIds>>()
//                                 };
//                                 assert_eq!(
//                                     {
//                                         let mut acc = vec![];
//                                         for element in &read_only_ids_current_elements {
//                                             acc.push(super::ExampleRead {
//                                                 primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&element.primary_key_column),
//                                                 column_156: match &element.column_156 {
//                                                     Some(value) => <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&value),
//                                                     None => Some(postgresql_crud::Value {
//                                                         value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                                                     }),
//                                                 },
//                                             });
//                                         }
//                                         acc.sort_by(|a, b| {
//                                             if let (Some(value_a), Some(value_b)) = (&a.primary_key_column, &b.primary_key_column) {
//                                                 value_a.value.cmp(&value_b.value)
//                                             } else {
//                                                 panic!("must not be what");
//                                             }
//                                         });
//                                         acc
//                                     },
//                                     {
//                                         let mut acc = super::Example::try_read_many(
//                                             &url,
//                                             super::ExampleReadManyParameters {
//                                                 payload: super::ExampleReadManyPayload {
//                                                     where_many: super::StdOptionOptionExampleWhereMany(Some(
//                                                         super::ExampleWhereMany::try_new(
//                                                             Some(
//                                                                 postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
//                                                                     let mut acc = vec![];
//                                                                     for element in &read_only_ids_current_elements {
//                                                                         acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual {
//                                                                             logical_operator: postgresql_crud::LogicalOperator::Or,
//                                                                             value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone())),
//                                                                         }));
//                                                                     }
//                                                                     acc
//                                                                 })
//                                                                 .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
//                                                             ),
//                                                             None,
//                                                         )
//                                                         .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
//                                                     )),
//                                                     select: select_default_all.clone(),
//                                                     order_by: postgresql_crud::OrderBy {
//                                                         column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
//                                                         order: Some(postgresql_crud::Order::Asc),
//                                                     },
//                                                     pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
//                                                 },
//                                             },
//                                         )
//                                         .await
//                                         .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
//                                         acc.sort_by(|a, b| if let (Some(value_a), Some(value_b)) = (&a.primary_key_column, &b.primary_key_column) { value_a.value.cmp(&value_b.value) } else { panic!("must not be what") });
//                                         acc
//                                     },
//                                     "try_read_many result different after try_create_many db146190-0496-42a7-93d6-8405eb641954"
//                                 );
//                                 for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
//                                     let url_cloned = url.clone();
//                                     let ident_create_default_cloned = ident_create_default.clone();
//                                     let select_default_all_cloned = select_default_all.clone();
//                                     acc.push(futures::FutureExt::boxed(async move {
//                                         let previous_read = super::Example::try_read_one(
//                                             &url_cloned,
//                                             super::ExampleReadOneParameters {
//                                                 payload: super::ExampleReadOnePayload {
//                                                     primary_key_column: read_only_ids_current_element.primary_key_column.clone(),
//                                                     select: select_default_all_cloned.clone(),
//                                                 },
//                                             },
//                                         )
//                                         .await
//                                         .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
//                                         let update = <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test({
//                                             let mut local_increment = 0;
//                                             let mut option_test_case = None;
//                                             for element_0 in <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::read_inner_vec_vec(&read_only_ids_current_element.column_156.clone().unwrap()) {
//                                                 let mut should_break = false;
//                                                 for element_1 in element_0 {
//                                                     if local_increment == increment {
//                                                         option_test_case = Some(element_1);
//                                                         should_break = true;
//                                                         break;
//                                                     } else {
//                                                         local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
//                                                     }
//                                                 }
//                                                 if should_break {
//                                                     break;
//                                                 }
//                                             }
//                                             option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
//                                         });
//                                         assert_eq!(
//                                             super::ExampleReadOnlyIds {
//                                                 primary_key_column: read_only_ids_current_element.primary_key_column.clone(),
//                                                 column_156: Some(<crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update))
//                                             },
//                                             super::Example::try_update_one(
//                                                 &url_cloned,
//                                                 {
//                                                     let f = super::ExampleUpdateOneParameters {
//                                                         payload: super::ExampleUpdate::try_new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(read_only_ids_current_element.primary_key_column.clone()), Some(postgresql_crud::Value { value: update.clone() })).expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2")
//                                                     };
//                                                     // println!("{f:#?}");
//                                                     f
//                                                 }
//                                             )
//                                             .await
//                                             .expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52"),
//                                             "try_update_one result different"
//                                         );
//                                         assert_eq!(
//                                             super::ExampleRead {
//                                                 primary_key_column: Some(postgresql_crud::Value { value: read_only_ids_current_element.primary_key_column.clone() }),
//                                                 column_156: Some(postgresql_crud::Value {
//                                                     value: <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::read_from_previous_read_unwraped_merged_with_update(
//                                                         <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_current_element.column_156.clone().unwrap())
//                                                             .unwrap()
//                                                             .value,
//                                                         Some(update.clone())
//                                                     )
//                                                 })
//                                             },
//                                             super::Example::try_read_one(
//                                                 &url_cloned,
//                                                 super::ExampleReadOneParameters {
//                                                     payload: super::ExampleReadOnePayload {
//                                                         primary_key_column: read_only_ids_current_element.primary_key_column.clone(),
//                                                         select: select_default_all_cloned
//                                                     }
//                                                 }
//                                             )
//                                             .await
//                                             .expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
//                                             "try_read_one result different after try_create_one"
//                                         );
//                                     }));
//                                 }
//                             }
//                             acc
//                         }),
//                         100,
//                         |fut| async move {
//                             fut.await;
//                         },
//                     )
//                     .await;
//                     let try_read_many_data = super::Example::try_read_many(
//                         &url,
//                         super::ExampleReadManyParameters {
//                             payload: super::ExampleReadManyPayload {
//                                 where_many: super::StdOptionOptionExampleWhereMany(None),
//                                 select: select_default_all.clone(),
//                                 order_by: postgresql_crud::OrderBy {
//                                     column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
//                                     order: Some(postgresql_crud::Order::Asc),
//                                 },
//                                 pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
//                             },
//                         },
//                     )
//                     .await
//                     .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
//                     // println!("try_read_many result len {}", try_read_many_data.len());
//                     /////////////////
//                     futures::StreamExt::for_each_concurrent(
//                         futures::stream::iter({
//                             let mut acc: std::vec::Vec<futures::future::BoxFuture<'static, ()>> = vec![];
//                             {
//                                 let read_only_ids_current_elements = {
//                                     use futures::StreamExt;
//                                     futures::stream::iter(
//                                         {
//                                             // let mut acc = vec![];
//                                             // if let Some(value) = &common_read_only_ids_returned_from_create_one.column_156 {
//                                             //     for element0 in <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::read_inner_vec_vec(&value) {
//                                             //         for element1 in element0 {
//                                             //             acc.push(ident_create_default.clone());
//                                             //         }
//                                             //     }
//                                             // }
//                                             // acc
//                                             //here
//                                             vec![ident_create_default.clone()]
//                                         }
//                                         .chunks(25)
//                                         .map(|element| element.to_vec())
//                                         .collect::<std::vec::Vec<std::vec::Vec<super::ExampleCreate>>>()
//                                         .into_iter()
//                                         .map(|element| {
//                                             let url_cloned = url.clone();
//                                             futures::FutureExt::boxed(async move { super::Example::try_create_many(&url_cloned, super::ExampleCreateManyParameters { payload: super::ExampleCreateManyPayload(element) }).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98") })
//                                         })
//                                         .collect::<std::vec::Vec<futures::future::BoxFuture<'static, std::vec::Vec<super::ExampleReadOnlyIds>>>>(),
//                                     )
//                                     .buffer_unordered(5)
//                                     .collect::<std::vec::Vec<std::vec::Vec<super::ExampleReadOnlyIds>>>()
//                                     .await
//                                     .into_iter()
//                                     .flatten()
//                                     .collect::<std::vec::Vec<super::ExampleReadOnlyIds>>()
//                                 };
//                                 // println!("@@@ {read_only_ids_current_elements:#?}");
//                                 assert_eq!(
//                                     {
//                                         let mut acc = vec![];
//                                         for element in &read_only_ids_current_elements {
//                                             acc.push(super::ExampleRead {
//                                                 primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&element.primary_key_column),
//                                                 column_156: match &element.column_156 {
//                                                     Some(value) => <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&value),
//                                                     None => Some(postgresql_crud::Value {
//                                                         value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                                                     }),
//                                                 },
//                                             });
//                                         }
//                                         acc.sort_by(|a, b| {
//                                             if let (Some(value_a), Some(value_b)) = (&a.primary_key_column, &b.primary_key_column) {
//                                                 value_a.value.cmp(&value_b.value)
//                                             } else {
//                                                 panic!("must not be what");
//                                             }
//                                         });
//                                         acc
//                                     },
//                                     {
//                                         let mut acc = super::Example::try_read_many(
//                                             &url,
//                                             super::ExampleReadManyParameters {
//                                                 payload: super::ExampleReadManyPayload {
//                                                     where_many: super::StdOptionOptionExampleWhereMany(Some(
//                                                         super::ExampleWhereMany::try_new(
//                                                             Some(
//                                                                 postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
//                                                                     let mut acc = vec![];
//                                                                     for element in &read_only_ids_current_elements {
//                                                                         acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual {
//                                                                             logical_operator: postgresql_crud::LogicalOperator::Or,
//                                                                             value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone())),
//                                                                         }));
//                                                                     }
//                                                                     acc
//                                                                 })
//                                                                 .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
//                                                             ),
//                                                             None,
//                                                         )
//                                                         .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
//                                                     )),
//                                                     select: select_default_all.clone(),
//                                                     order_by: postgresql_crud::OrderBy {
//                                                         column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
//                                                         order: Some(postgresql_crud::Order::Asc),
//                                                     },
//                                                     pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
//                                                 },
//                                             },
//                                         )
//                                         .await
//                                         .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
//                                         acc.sort_by(|a, b| if let (Some(value_a), Some(value_b)) = (&a.primary_key_column, &b.primary_key_column) { value_a.value.cmp(&value_b.value) } else { panic!("must not be what") });
//                                         acc
//                                     },
//                                     "try_read_many result different after try_create_many db146190-0496-42a7-93d6-8405eb641954"
//                                 );
//                                 for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
//                                     let url_cloned = url.clone();
//                                     let ident_create_default_cloned = ident_create_default.clone();
//                                     let select_default_all_cloned = select_default_all.clone();
//                                     acc.push(futures::FutureExt::boxed(async move {
//                                         let previous_read = super::Example::try_read_one(
//                                             &url_cloned,
//                                             super::ExampleReadOneParameters {
//                                                 payload: super::ExampleReadOnePayload {
//                                                     primary_key_column: read_only_ids_current_element.primary_key_column.clone(),
//                                                     select: select_default_all_cloned.clone(),
//                                                 },
//                                             },
//                                         )
//                                         .await
//                                         .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
//                                         // let update = <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test({
//                                         //     let mut local_increment = 0;
//                                         //     let mut option_test_case = None;
//                                         //     for element_0 in <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::read_inner_vec_vec(&read_only_ids_current_element.column_156.clone().unwrap()) {
//                                         //         let mut should_break = false;
//                                         //         for element_1 in element_0 {
//                                         //             if local_increment == increment {
//                                         //                 option_test_case = Some(element_1);
//                                         //                 should_break = true;
//                                         //                 break;
//                                         //             } else {
//                                         //                 local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
//                                         //             }
//                                         //         }
//                                         //         if should_break {
//                                         //             break;
//                                         //         }
//                                         //     }
//                                         //     option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
//                                         // });
//                                         // println!("{update:#?}");
//                                         //here
//                                         let update = crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate::try_new(
//                                             {
//                                                 let f = vec![
//                                                     crate::repositories_types::server::routes::api::example::AnimalWithIdAsNotNullJsonbObjectWithIdCreate::new(
//                                                         //todo make it Create
//                                                         <
//                                                             <postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create
//                                                             as
//                                                             postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
//                                                         >::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                                                         <
//                                                             <super::DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Create
//                                                             as
//                                                             postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
//                                                         >::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                                                         <
//                                                             <super::OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Create
//                                                             as
//                                                             postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
//                                                         >::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                                                         <
//                                                             <super::VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create
//                                                             as
//                                                             postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
//                                                         >::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                                                         <
//                                                             <super::OptionVecOfDoggieWithIdAsNullableArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create
//                                                             as
//                                                             postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
//                                                         >::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                                                     )
//                                                 ];
//                                                 let ddd = serde_json::to_string(&f).unwrap();
//                                                 // println!("DDDD {ddd:#?}");
//                                                 f
//                                             },
//                                             postgresql_crud::UniqueVec::try_new(vec![]).expect("error bb6d3378-d6bf-4e66-b70c-f0e48aa70772"),
//                                             vec![]
//                                         ).expect("error 530210f1-adb2-4632-8367-7031c1931cbc");

//                                         // VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate {
//                                         //     create: [],
//                                         //     update: AnimalWithIdAsNotNullJsonbObjectWithIdUpdate(
//                                         //         UniqueVec(
//                                         //             [
//                                         //                 AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement {
//                                         //                     id: UuidUuidAsNotNullJsonbStringOrigin(
//                                         //                         5845aaf9-3ff5-4c8d-959d-a3f61af8043b,
//                                         //                     ),
//                                         //                     fields: AnimalAsNotNullJsonbObjectUpdate(
//                                         //                         NotEmptyUniqueEnumVec(
//                                         //                             [
//                                         //                                 Field0(
//                                         //                                     Value {
//                                         //                                         value: StdPrimitiveI8AsNotNullJsonbNumberOrigin(
//                                         //                                             -128,
//                                         //                                         ),
//                                         //                                     },
//                                         //                                 ),
//                                         //                             ],
//                                         //                         ),
//                                         //                     ),
//                                         //                 },
//                                         //             ],
//                                         //         ),
//                                         //     ),
//                                         //     delete: [],
//                                         // }
//                                         let right = super::Example::try_update_one(
//                                             &url_cloned,
//                                             super::ExampleUpdateOneParameters {
//                                                 payload: {
//                                                     let p = super::ExampleUpdate::try_new(
//                                                         <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(
//                                                             read_only_ids_current_element.primary_key_column.clone()
//                                                         ),
//                                                         Some(postgresql_crud::Value { value: update.clone() })
//                                                     ).expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2");
//                                                     // println!("PPP {p:#?}");
//                                                     p
//                                                 }
//                                             }
//                                         )
//                                         .await
//                                         .expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52");
                                        
//                                         // assert_eq!(
//                                         //     super::ExampleReadOnlyIds {
//                                         //         primary_key_column: read_only_ids_current_element.primary_key_column.clone(),
//                                         //         column_156: Some(<crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update))
//                                         //     },
//                                         //     right,
//                                         //     "try_update_one result different"
//                                         // );
//                                         // assert_eq!(
//                                         //     super::ExampleRead {
//                                         //         primary_key_column: Some(postgresql_crud::Value { value: read_only_ids_current_element.primary_key_column.clone() }),
//                                         //         column_156: Some(postgresql_crud::Value {
//                                         //             value: <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::read_from_previous_read_unwraped_merged_with_update(
//                                         //                 <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_current_element.column_156.clone().unwrap())
//                                         //                     .unwrap()
//                                         //                     .value,
//                                         //                 Some(update.clone())
//                                         //             )
//                                         //         })
//                                         //     },
//                                         //     super::Example::try_read_one(
//                                         //         &url_cloned,
//                                         //         super::ExampleReadOneParameters {
//                                         //             payload: super::ExampleReadOnePayload {
//                                         //                 primary_key_column: read_only_ids_current_element.primary_key_column.clone(),
//                                         //                 select: select_default_all_cloned
//                                         //             }
//                                         //         }
//                                         //     )
//                                         //     .await
//                                         //     .expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
//                                         //     "try_read_one result different after try_create_one"
//                                         // );
//                                     }));
//                                 }
//                             }
//                             acc
//                         }),
//                         10,
//                         |fut| async move {
//                             fut.await;
//                         },
//                     )
//                     .await;
//                     //
//                     //here
//                     // let u = <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::create_vec();
//                     // println!("test cases: {}", u.len());
//                     // // println!("test cases: {}", serde_json::to_string(&u).unwrap());
//                     // /////////////////
//                     // let read_only_ids_current_elements = {
//                     //     use futures::StreamExt;
//                     //     futures::stream::iter(
//                     //         {
//                     //             let mut acc = vec![];
//                     //             for element in <crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlTypeTestCases>::create_vec() {
//                     //                 acc.push(super::ExampleCreate {
//                     //                     column_156: element,
//                     //                     // column_156: <<crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                     //                 });
//                     //             }
//                     //             acc
//                     //         }
//                     //         .chunks(25)
//                     //         .map(|element| element.to_vec())
//                     //         .collect::<std::vec::Vec<std::vec::Vec<super::ExampleCreate>>>()
//                     //         .into_iter()
//                     //         .map(|element| {
//                     //             let url_cloned = url.clone();
//                     //             futures::FutureExt::boxed(async move { 
//                     //                 (
//                     //                     super::Example::try_create_many(&url_cloned, super::ExampleCreateManyParameters { payload: super::ExampleCreateManyPayload(element.clone()) }).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98"),
//                     //                     element
//                     //                 )
//                     //             })
//                     //         })
//                     //         .collect::<std::vec::Vec<futures::future::BoxFuture<'static, (std::vec::Vec<super::ExampleReadOnlyIds>)>>>(),
//                     //     )
//                     //     .buffer_unordered(5)
//                     //     .collect::<std::vec::Vec<std::vec::Vec<super::ExampleReadOnlyIds>>>()
//                     //     .await
//                     //     .into_iter()
//                     //     .flatten()
//                     //     .collect::<std::vec::Vec<super::ExampleReadOnlyIds>>()
//                     // };
//                     // println!("read_only_ids_current_elements {read_only_ids_current_elements:#?}");

//                     //todo method read_only_ids + create into read
                    
//                     /////////////////
//                 });
//             })
//             .expect("error 4d329978-f5af-424e-8757-e8a32dbeb5a1")
//             .join()
//             .unwrap_or_else(|error| {
//                 panic!("error b2f21a5f-d9ce-435c-809f-bd40741c8795 {error:#?}");
//             });
//     }
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
//                         column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                         column_30: <<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//                     };
//                     let common_read_only_ids_returned_from_create_one = super::Example::try_create_one(&url, super::ExampleCreateOneParameters { payload: ident_create_default.clone() }).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
//                     let some_value_read_only_ids_returned_from_create_one = Some(postgresql_crud::Value { value: common_read_only_ids_returned_from_create_one.primary_key_column.clone().into_read() });
//                     assert_eq!(super::ExampleRead { primary_key_column: some_value_read_only_ids_returned_from_create_one.clone(), column_0: None, column_30: None }, super::Example::try_read_one(&url, super::ExampleReadOneParameters { payload: super::ExampleReadOnePayload { primary_key_column: common_read_only_ids_returned_from_create_one.primary_key_column.clone().into_read(), select: select_primary_key.clone() } },).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b"), "try_read_one result different after try_create_one 3d9f2ec0-e374-48d2-a36b-486f5598b0b4");
//                     let select_default_all = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![super::ExampleSelect::PrimaryKeyColumn(<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()), super::ExampleSelect::Column0(<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()), super::ExampleSelect::Column30(<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element())]).expect("error 0776170e-4dd6-4c14-a412-ce10b0c746f1");
//                     let start = std::time::Instant::now();
//                     futures::StreamExt::for_each_concurrent(
//                         futures::stream::iter({
//                             let mut acc: std::vec::Vec<futures::future::BoxFuture<'static, ()>> = vec![];
//                             for chunk in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::create_vec().chunks(10).map(|element| element.to_vec()).collect::<std::vec::Vec<std::vec::Vec<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create>>>() {
//                                 let url_cloned = url.clone();
//                                 let ident_create_default_cloned = ident_create_default.clone();
//                                 let select_default_all_cloned = select_default_all.clone();
//                                 acc.push(futures::FutureExt::boxed(async move {
//                                     let ident_create_vec = {
//                                         let mut acc = vec![];
//                                         for element in chunk {
//                                             acc.push(super::ExampleCreate { column_0: element, column_30: <<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element() });
//                                         }
//                                         acc
//                                     };
//                                     let read_only_ids_from_try_create_many = super::Example::try_create_many(&url_cloned, super::ExampleCreateManyParameters { payload: super::ExampleCreateManyPayload(ident_create_vec.clone()) }).await.expect("error 5eecedc4-bb02-454a-acd9-0af758f30b2e");
//                                     println!("@@@{read_only_ids_from_try_create_many:#?}");
//                                     let right = {
//                                         let mut acc = vec![];
//                                         assert_eq!(read_only_ids_from_try_create_many.len(), ident_create_vec.len(), "error 39572295-b6a4-49d7-a65a-16f8bcf44ede");
//                                         for (read_only_ids, create) in read_only_ids_from_try_create_many.clone().into_iter().zip(ident_create_vec.into_iter()).collect::<std::vec::Vec<(super::ExampleReadOnlyIds, super::ExampleCreate)>>() {
//                                             acc.push(super::ExampleRead {
//                                                 primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids.primary_key_column),
//                                                 column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_0, create.column_0),
//                                                 column_30: <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_30, create.column_30),
//                                             });
//                                         }
//                                         acc.sort_by(|a, b| {
//                                             if let (Some(a), Some(b)) = (&a.primary_key_column, &b.primary_key_column) {
//                                                 a.value.cmp(&b.value)
//                                             } else {
//                                                 panic!("must not be what error 4428083a-53be-4184-a5b7-94ae2de21d40");
//                                             }
//                                         });
//                                         acc
//                                     };
//                                     // let left = super::Example::try_read_many(
//                                     //     &url_cloned,
//                                     //     super::ExampleReadManyParameters {
//                                     //         payload: super::ExampleReadManyPayload {
//                                     //             where_many: super::StdOptionOptionExampleWhereMany(Some(
//                                     //                 super::ExampleWhereMany::try_new(
//                                     //                     Some(
//                                     //                         postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
//                                     //                             let mut acc = vec![];
//                                     //                             for element in &read_only_ids_from_try_create_many {
//                                     //                                 acc.push(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone().into_read())) }));
//                                     //                             }
//                                     //                             acc
//                                     //                         })
//                                     //                         .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
//                                     //                     ),
//                                     //                     None,
//                                     //                     None
//                                     //                 )
//                                     //                 .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
//                                     //             )),
//                                     //             select: select_default_all_cloned.clone(),
//                                     //             order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) },
//                                     //             pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
//                                     //         }
//                                     //     }
//                                     // )
//                                     // .await
//                                     // .expect("error cb64ceaf-74a4-4501-b614-0c9d9e4e0598");
//                                     // assert_eq!(
//                                     //     right,
//                                     //     left,
//                                     //     "try_read_many result different after try_create_many error d19bbbf6-f64c-4151-8b5b-998a93e13af5"
//                                     // );
//                                     // let read_only_ids_from_try_delete_many = {
//                                     //     let mut acc = super::Example::try_delete_many(
//                                     //         &url_cloned,
//                                     //         super::ExampleDeleteManyParameters {
//                                     //             payload: super::ExampleDeleteManyPayload {
//                                     //                 where_many: super::StdOptionOptionExampleWhereMany(Some(super::ExampleWhereMany {
//                                     //                     primary_key_column: Some(
//                                     //                         postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
//                                     //                             let mut acc = vec![];
//                                     //                             for element in &read_only_ids_from_try_create_many {
//                                     //                                 acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone().into_read())) }));
//                                     //                             }
//                                     //                             acc
//                                     //                         })
//                                     //                         .expect("error 5f1e5f9d-d189-4368-807e-a84348967610"),
//                                     //                     ),
//                                     //                     column_0: None,
//                                     //                     column_30: None,
//                                     //                 })),
//                                     //             },
//                                     //         },
//                                     //     )
//                                     //     .await
//                                     //     .expect("error 716e470e-d738-4642-adfc-df1f9b945d27");
//                                     //     acc.sort_by(|a, b| a.cmp(&b));
//                                     //     acc
//                                     // };
//                                     // assert_eq!(
//                                     //     read_only_ids_from_try_delete_many,
//                                     //     {
//                                     //         let mut acc = read_only_ids_from_try_create_many.into_iter().map(|element| element.primary_key_column.clone().into_read()).collect::<std::vec::Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>();
//                                     //         acc.sort_by(|a, b| a.cmp(&b));
//                                     //         acc
//                                     //     },
//                                     //     "error f58f5572-4286-4a74-8006-0507339910d4"
//                                     // );
//                                     // match super::Example::try_read_many(
//                                     //     &url_cloned,
//                                     //     super::ExampleReadManyParameters {
//                                     //         payload: super::ExampleReadManyPayload {
//                                     //             where_many: super::StdOptionOptionExampleWhereMany(Some(
//                                     //                 super::ExampleWhereMany::try_new(
//                                     //                     Some(
//                                     //                         postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
//                                     //                             let mut acc = vec![];
//                                     //                             for element in &read_only_ids_from_try_delete_many {
//                                     //                                 acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.clone())) }));
//                                     //                             }
//                                     //                             acc
//                                     //                         })
//                                     //                         .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
//                                     //                     ),
//                                     //                     None,
//                                     //                     None,
//                                     //                 )
//                                     //                 .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
//                                     //             )),
//                                     //             select: select_default_all_cloned,
//                                     //             order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) },
//                                     //             pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
//                                     //         },
//                                     //     },
//                                     // )
//                                     // .await
//                                     // {
//                                     //     Ok(value) => {
//                                     //         if value != std::vec::Vec::new() {
//                                     //             panic!("error 4e88679a-0d23-418f-8767-4e9b7531429c");
//                                     //         }
//                                     //     }
//                                     //     Err(error) => {
//                                     //         panic!("error 24ab86d6-15c9-47f1-a43f-c5fac4b38188 {error:#?}");
//                                     //     }
//                                     // }
//                                 }));
//                             }
//                             // for chunk in <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::create_vec().chunks(10).map(|element| element.to_vec()).collect::<std::vec::Vec<std::vec::Vec<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Create>>>() {
//                             //     let url_cloned = url.clone();
//                             //     let ident_create_default_cloned = ident_create_default.clone();
//                             //     let select_default_all_cloned = select_default_all.clone();
//                             //     acc.push(futures::FutureExt::boxed(async move {
//                             //         let ident_create_vec = {
//                             //             let mut acc = vec![];
//                             //             for element in chunk {
//                             //                 acc.push(super::ExampleCreate { column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(), column_30: element });
//                             //             }
//                             //             acc
//                             //         };
//                             //         let read_only_ids_from_try_create_many = super::Example::try_create_many(&url_cloned, super::ExampleCreateManyParameters { payload: super::ExampleCreateManyPayload(ident_create_vec.clone()) }).await.expect("error 5eecedc4-bb02-454a-acd9-0af758f30b2e");
//                             //         assert_eq!(
//                             //             {
//                             //                 let mut acc = vec![];
//                             //                 assert_eq!(read_only_ids_from_try_create_many.len(), ident_create_vec.len(), "error 39572295-b6a4-49d7-a65a-16f8bcf44ede");
//                             //                 for (read_only_ids, create) in read_only_ids_from_try_create_many.clone().into_iter().zip(ident_create_vec.into_iter()).collect::<std::vec::Vec<(super::ExampleReadOnlyIds, super::ExampleCreate)>>() {
//                             //                     acc.push(super::ExampleRead {
//                             //                         primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids.primary_key_column),
//                             //                         column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_0, create.column_0),
//                             //                         column_30: <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_30, create.column_30),
//                             //                     });
//                             //                 }
//                             //                 acc.sort_by(|a, b| {
//                             //                     if let (Some(a), Some(b)) = (&a.primary_key_column, &b.primary_key_column) {
//                             //                         a.value.cmp(&b.value)
//                             //                     } else {
//                             //                         panic!("must not be what error 4428083a-53be-4184-a5b7-94ae2de21d40");
//                             //                     }
//                             //                 });
//                             //                 acc
//                             //             },
//                             //             super::Example::try_read_many(
//                             //                 &url_cloned,
//                             //                 super::ExampleReadManyParameters {
//                             //                     payload: super::ExampleReadManyPayload {
//                             //                         where_many: super::StdOptionOptionExampleWhereMany(Some(
//                             //                             super::ExampleWhereMany::try_new(
//                             //                                 Some(
//                             //                                     postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
//                             //                                         let mut acc = vec![];
//                             //                                         for element in &read_only_ids_from_try_create_many {
//                             //                                             acc.push(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone().into_read())) }));
//                             //                                         }
//                             //                                         acc
//                             //                                     })
//                             //                                     .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
//                             //                                 ),
//                             //                                 None,
//                             //                                 None
//                             //                             )
//                             //                             .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
//                             //                         )),
//                             //                         select: select_default_all_cloned.clone(),
//                             //                         order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) },
//                             //                         pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
//                             //                     }
//                             //                 }
//                             //             )
//                             //             .await
//                             //             .expect("error cb64ceaf-74a4-4501-b614-0c9d9e4e0598"),
//                             //             "try_read_many result different after try_create_many error d19bbbf6-f64c-4151-8b5b-998a93e13af5"
//                             //         );
//                             //         let read_only_ids_from_try_delete_many = {
//                             //             let mut acc = super::Example::try_delete_many(
//                             //                 &url_cloned,
//                             //                 super::ExampleDeleteManyParameters {
//                             //                     payload: super::ExampleDeleteManyPayload {
//                             //                         where_many: super::StdOptionOptionExampleWhereMany(Some(super::ExampleWhereMany {
//                             //                             primary_key_column: Some(
//                             //                                 postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
//                             //                                     let mut acc = vec![];
//                             //                                     for element in &read_only_ids_from_try_create_many {
//                             //                                         acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone().into_read())) }));
//                             //                                     }
//                             //                                     acc
//                             //                                 })
//                             //                                 .expect("error 5f1e5f9d-d189-4368-807e-a84348967610"),
//                             //                             ),
//                             //                             column_0: None,
//                             //                             column_30: None,
//                             //                         })),
//                             //                     },
//                             //                 },
//                             //             )
//                             //             .await
//                             //             .expect("error 716e470e-d738-4642-adfc-df1f9b945d27");
//                             //             acc.sort_by(|a, b| a.cmp(&b));
//                             //             acc
//                             //         };
//                             //         assert_eq!(
//                             //             read_only_ids_from_try_delete_many,
//                             //             {
//                             //                 let mut acc = read_only_ids_from_try_create_many.into_iter().map(|element| element.primary_key_column.clone().into_read()).collect::<std::vec::Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>();
//                             //                 acc.sort_by(|a, b| a.cmp(&b));
//                             //                 acc
//                             //             },
//                             //             "error f58f5572-4286-4a74-8006-0507339910d4"
//                             //         );
//                             //         match super::Example::try_read_many(
//                             //             &url_cloned,
//                             //             super::ExampleReadManyParameters {
//                             //                 payload: super::ExampleReadManyPayload {
//                             //                     where_many: super::StdOptionOptionExampleWhereMany(Some(
//                             //                         super::ExampleWhereMany::try_new(
//                             //                             Some(
//                             //                                 postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
//                             //                                     let mut acc = vec![];
//                             //                                     for element in &read_only_ids_from_try_delete_many {
//                             //                                         acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.clone())) }));
//                             //                                     }
//                             //                                     acc
//                             //                                 })
//                             //                                 .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
//                             //                             ),
//                             //                             None,
//                             //                             None,
//                             //                         )
//                             //                         .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
//                             //                     )),
//                             //                     select: select_default_all_cloned,
//                             //                     order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) },
//                             //                     pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
//                             //                 },
//                             //             },
//                             //         )
//                             //         .await
//                             //         {
//                             //             Ok(value) => {
//                             //                 if value != std::vec::Vec::new() {
//                             //                     panic!("error 4e88679a-0d23-418f-8767-4e9b7531429c");
//                             //                 }
//                             //             }
//                             //             Err(error) => {
//                             //                 panic!("error 24ab86d6-15c9-47f1-a43f-c5fac4b38188 {error:#?}");
//                             //             }
//                             //         }
//                             //     }));
//                             // }
//                             acc
//                         }),
//                         10,
//                         |fut| async move {
//                             fut.await;
//                         },
//                     )
//                     .await;
//                     let create_many_elapsed = start.elapsed();
//                     let create_one_elapsed = start.elapsed();
//                     println!("Elapsed: create_many_elapsed {:?}, create_one_elapsed {:?}", create_many_elapsed, create_one_elapsed);
//                     // futures::StreamExt::for_each_concurrent(
//                     //     futures::stream::iter({
//                     //         let mut acc: std::vec::Vec<futures::future::BoxFuture<'static, ()>> = vec![];
//                     //         {
//                     //             let read_only_ids_current_elements = {
//                     //                 use futures::StreamExt;
//                     //                 futures::stream::iter(
//                     //                     {
//                     //                         let mut acc = vec![];
//                     //                         if let Some(value) = &common_read_only_ids_returned_from_create_one.column_0 {
//                     //                             for element0 in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_inner_vec_vec(&value) {
//                     //                                 for element1 in element0 {
//                     //                                     acc.push(ident_create_default.clone());
//                     //                                 }
//                     //                             }
//                     //                         }
//                     //                         acc
//                     //                     }
//                     //                     .chunks(25)
//                     //                     .map(|element| element.to_vec())
//                     //                     .collect::<std::vec::Vec<std::vec::Vec<super::ExampleCreate>>>()
//                     //                     .into_iter()
//                     //                     .map(|element| {
//                     //                         let url_cloned = url.clone();
//                     //                         futures::FutureExt::boxed(async move { super::Example::try_create_many(&url_cloned, super::ExampleCreateManyParameters { payload: super::ExampleCreateManyPayload(element) }).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98") })
//                     //                     })
//                     //                     .collect::<std::vec::Vec<futures::future::BoxFuture<'static, std::vec::Vec<super::ExampleReadOnlyIds>>>>(),
//                     //                 )
//                     //                 .buffer_unordered(5)
//                     //                 .collect::<std::vec::Vec<std::vec::Vec<super::ExampleReadOnlyIds>>>()
//                     //                 .await
//                     //                 .into_iter()
//                     //                 .flatten()
//                     //                 .collect::<std::vec::Vec<super::ExampleReadOnlyIds>>()
//                     //             };
//                     //             assert_eq!(
//                     //                 {
//                     //                     let mut acc = vec![];
//                     //                     for element in &read_only_ids_current_elements {
//                     //                         acc.push(super::ExampleRead {
//                     //                             primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&element.primary_key_column),
//                     //                             column_0: match &element.column_0 {
//                     //                                 Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&value),
//                     //                                 None => Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }),
//                     //                             },
//                     //                             column_30: match &element.column_30 {
//                     //                                 Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&value),
//                     //                                 None => Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }),
//                     //                             },
//                     //                         });
//                     //                     }
//                     //                     acc.sort_by(|a, b| {
//                     //                         if let (Some(value_a), Some(value_b)) = (&a.primary_key_column, &b.primary_key_column) {
//                     //                             value_a.value.cmp(&value_b.value)
//                     //                         } else {
//                     //                             panic!("must not be what");
//                     //                         }
//                     //                     });
//                     //                     acc
//                     //                 },
//                     //                 {
//                     //                     let mut acc = super::Example::try_read_many(
//                     //                         &url,
//                     //                         super::ExampleReadManyParameters {
//                     //                             payload: super::ExampleReadManyPayload {
//                     //                                 where_many: super::StdOptionOptionExampleWhereMany(Some(
//                     //                                     super::ExampleWhereMany::try_new(
//                     //                                         Some(
//                     //                                             postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
//                     //                                                 let mut acc = vec![];
//                     //                                                 for element in &read_only_ids_current_elements {
//                     //                                                     acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone().into_read())) }));
//                     //                                                 }
//                     //                                                 acc
//                     //                                             })
//                     //                                             .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
//                     //                                         ),
//                     //                                         None,
//                     //                                         None,
//                     //                                     )
//                     //                                     .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
//                     //                                 )),
//                     //                                 select: select_default_all.clone(),
//                     //                                 order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) },
//                     //                                 pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
//                     //                             },
//                     //                         },
//                     //                     )
//                     //                     .await
//                     //                     .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
//                     //                     acc.sort_by(|a, b| if let (Some(value_a), Some(value_b)) = (&a.primary_key_column, &b.primary_key_column) { value_a.value.cmp(&value_b.value) } else { panic!("must not be what") });
//                     //                     acc
//                     //                 },
//                     //                 "try_read_many result different after try_create_many db146190-0496-42a7-93d6-8405eb641954"
//                     //             );
//                     //             for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
//                     //                 let url_cloned = url.clone();
//                     //                 let ident_create_default_cloned = ident_create_default.clone();
//                     //                 let select_default_all_cloned = select_default_all.clone();
//                     //                 acc.push(futures::FutureExt::boxed(async move {
//                     //                     let previous_read = super::Example::try_read_one(&url_cloned, super::ExampleReadOneParameters { payload: super::ExampleReadOnePayload { primary_key_column: read_only_ids_current_element.primary_key_column.clone().into_read(), select: select_default_all_cloned.clone() } }).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b");
//                     //                     let update = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test({
//                     //                         let mut local_increment = 0;
//                     //                         let mut option_test_case = None;
//                     //                         for element_0 in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_inner_vec_vec(&read_only_ids_current_element.column_0.clone().expect("error c4d98a71-f30f-410e-b410-a75f4672f2f7")) {
//                     //                             let mut should_break = false;
//                     //                             for element_1 in element_0 {
//                     //                                 if local_increment == increment {
//                     //                                     option_test_case = Some(element_1);
//                     //                                     should_break = true;
//                     //                                     break;
//                     //                                 } else {
//                     //                                     local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
//                     //                                 }
//                     //                             }
//                     //                             if should_break {
//                     //                                 break;
//                     //                             }
//                     //                         }
//                     //                         option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
//                     //                     });
//                     //                     assert_eq!(super::ExampleReadOnlyIds { primary_key_column: read_only_ids_current_element.primary_key_column.clone(), column_0: Some(<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update)), column_30: None }, super::Example::try_update_one(&url_cloned, super::ExampleUpdateOneParameters { payload: super::ExampleUpdate::try_new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(read_only_ids_current_element.primary_key_column.clone().into_read()), Some(postgresql_crud::Value { value: update.clone() }), None).expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2") }).await.expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52"), "try_update_one result different");
//                     //                     assert_eq!(
//                     //                         super::ExampleRead {
//                     //                             primary_key_column: Some(postgresql_crud::Value { value: read_only_ids_current_element.primary_key_column.clone().into_read() }),
//                     //                             column_0: Some(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_from_previous_read_unwraped_merged_with_update(<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_current_element.column_0.clone().expect("error 4f19d0d2-a23f-4b77-a2bc-c7b04db7a129")).expect("error c7685b19-9bca-47bc-a3a5-8fc543b174a5").value, Some(update.clone())) }),
//                     //                             column_30: previous_read.column_30
//                     //                         },
//                     //                         super::Example::try_read_one(&url_cloned, super::ExampleReadOneParameters { payload: super::ExampleReadOnePayload { primary_key_column: read_only_ids_current_element.primary_key_column.clone().into_read(), select: select_default_all_cloned } }).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
//                     //                         "try_read_one result different after try_create_one"
//                     //                     );
//                     //                 }));
//                     //             }
//                     //         }
//                     //         {
//                     //             let read_only_ids_current_elements = {
//                     //                 use futures::StreamExt;
//                     //                 futures::stream::iter(
//                     //                     {
//                     //                         let mut acc = vec![];
//                     //                         if let Some(value) = &common_read_only_ids_returned_from_create_one.column_30 {
//                     //                             for element0 in <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_inner_vec_vec(&value) {
//                     //                                 for element1 in element0 {
//                     //                                     acc.push(ident_create_default.clone());
//                     //                                 }
//                     //                             }
//                     //                         }
//                     //                         acc
//                     //                     }
//                     //                     .chunks(25)
//                     //                     .map(|element| element.to_vec())
//                     //                     .collect::<std::vec::Vec<std::vec::Vec<super::ExampleCreate>>>()
//                     //                     .into_iter()
//                     //                     .map(|element| {
//                     //                         let url_cloned = url.clone();
//                     //                         futures::FutureExt::boxed(async move { super::Example::try_create_many(&url_cloned, super::ExampleCreateManyParameters { payload: super::ExampleCreateManyPayload(element) }).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98") })
//                     //                     })
//                     //                     .collect::<std::vec::Vec<futures::future::BoxFuture<'static, std::vec::Vec<super::ExampleReadOnlyIds>>>>(),
//                     //                 )
//                     //                 .buffer_unordered(5)
//                     //                 .collect::<std::vec::Vec<std::vec::Vec<super::ExampleReadOnlyIds>>>()
//                     //                 .await
//                     //                 .into_iter()
//                     //                 .flatten()
//                     //                 .collect::<std::vec::Vec<super::ExampleReadOnlyIds>>()
//                     //             };
//                     //             assert_eq!(
//                     //                 {
//                     //                     let mut acc = vec![];
//                     //                     for element in &read_only_ids_current_elements {
//                     //                         acc.push(super::ExampleRead {
//                     //                             primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&element.primary_key_column),
//                     //                             column_0: match &element.column_0 {
//                     //                                 Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&value),
//                     //                                 None => Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }),
//                     //                             },
//                     //                             column_30: match &element.column_30 {
//                     //                                 Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&value),
//                     //                                 None => Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }),
//                     //                             },
//                     //                         });
//                     //                     }
//                     //                     acc.sort_by(|a, b| {
//                     //                         if let (Some(value_a), Some(value_b)) = (&a.primary_key_column, &b.primary_key_column) {
//                     //                             value_a.value.cmp(&value_b.value)
//                     //                         } else {
//                     //                             panic!("must not be what");
//                     //                         }
//                     //                     });
//                     //                     acc
//                     //                 },
//                     //                 {
//                     //                     let mut acc = super::Example::try_read_many(
//                     //                         &url,
//                     //                         super::ExampleReadManyParameters {
//                     //                             payload: super::ExampleReadManyPayload {
//                     //                                 where_many: super::StdOptionOptionExampleWhereMany(Some(
//                     //                                     super::ExampleWhereMany::try_new(
//                     //                                         Some(
//                     //                                             postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
//                     //                                                 let mut acc = vec![];
//                     //                                                 for element in &read_only_ids_current_elements {
//                     //                                                     acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone().into_read())) }));
//                     //                                                 }
//                     //                                                 acc
//                     //                                             })
//                     //                                             .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
//                     //                                         ),
//                     //                                         None,
//                     //                                         None,
//                     //                                     )
//                     //                                     .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
//                     //                                 )),
//                     //                                 select: select_default_all.clone(),
//                     //                                 order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) },
//                     //                                 pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
//                     //                             },
//                     //                         },
//                     //                     )
//                     //                     .await
//                     //                     .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
//                     //                     acc.sort_by(|a, b| if let (Some(value_a), Some(value_b)) = (&a.primary_key_column, &b.primary_key_column) { value_a.value.cmp(&value_b.value) } else { panic!("must not be what") });
//                     //                     acc
//                     //                 },
//                     //                 "try_read_many result different after try_create_many db146190-0496-42a7-93d6-8405eb641954"
//                     //             );
//                     //             for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
//                     //                 let url_cloned = url.clone();
//                     //                 let ident_create_default_cloned = ident_create_default.clone();
//                     //                 let select_default_all_cloned = select_default_all.clone();
//                     //                 acc.push(futures::FutureExt::boxed(async move {
//                     //                     let previous_read = super::Example::try_read_one(&url_cloned, super::ExampleReadOneParameters { payload: super::ExampleReadOnePayload { primary_key_column: read_only_ids_current_element.primary_key_column.clone().into_read(), select: select_default_all_cloned.clone() } }).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b");
//                     //                     let update = <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test({
//                     //                         let mut local_increment = 0;
//                     //                         let mut option_test_case = None;
//                     //                         for element_0 in <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_inner_vec_vec(&read_only_ids_current_element.column_30.clone().expect("error c4d98a71-f30f-410e-b410-a75f4672f2f7")) {
//                     //                             let mut should_break = false;
//                     //                             for element_1 in element_0 {
//                     //                                 if local_increment == increment {
//                     //                                     option_test_case = Some(element_1);
//                     //                                     should_break = true;
//                     //                                     break;
//                     //                                 } else {
//                     //                                     local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
//                     //                                 }
//                     //                             }
//                     //                             if should_break {
//                     //                                 break;
//                     //                             }
//                     //                         }
//                     //                         option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
//                     //                     });
//                     //                     assert_eq!(super::ExampleReadOnlyIds { primary_key_column: read_only_ids_current_element.primary_key_column.clone(), column_0: None, column_30: Some(<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update)) }, super::Example::try_update_one(&url_cloned, super::ExampleUpdateOneParameters { payload: super::ExampleUpdate::try_new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(read_only_ids_current_element.primary_key_column.clone().into_read()), None, Some(postgresql_crud::Value { value: update.clone() })).expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2") }).await.expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52"), "try_update_one result different");
//                     //                     assert_eq!(
//                     //                         super::ExampleRead {
//                     //                             primary_key_column: Some(postgresql_crud::Value { value: read_only_ids_current_element.primary_key_column.clone().into_read() }),
//                     //                             column_0: previous_read.column_0,
//                     //                             column_30: Some(postgresql_crud::Value {
//                     //                                 value: <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_from_previous_read_unwraped_merged_with_update(<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_current_element.column_30.clone().expect("error 4f19d0d2-a23f-4b77-a2bc-c7b04db7a129")).expect("error c7685b19-9bca-47bc-a3a5-8fc543b174a5").value, Some(update.clone()))
//                     //                             })
//                     //                         },
//                     //                         super::Example::try_read_one(&url_cloned, super::ExampleReadOneParameters { payload: super::ExampleReadOnePayload { primary_key_column: read_only_ids_current_element.primary_key_column.clone().into_read(), select: select_default_all_cloned } }).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
//                     //                         "try_read_one result different after try_create_one"
//                     //                     );
//                     //                 }));
//                     //             }
//                     //         }
//                     //         acc
//                     //     }),
//                     //     100,
//                     //     |fut| async move {
//                     //         fut.await;
//                     //     },
//                     // )
//                     // .await;
//                     // let try_read_many_data = super::Example::try_read_many(&url, super::ExampleReadManyParameters { payload: super::ExampleReadManyPayload { where_many: super::StdOptionOptionExampleWhereMany(None), select: select_default_all.clone(), order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) }, pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a") } }).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b");
//                     // println!("try_read_many result len {}", try_read_many_data.len());
//                 });
//             })
//             .expect("error 4d329978-f5af-424e-8757-e8a32dbeb5a1")
//             .join()
//             .unwrap_or_else(|error| {
//                 panic!("error b2f21a5f-d9ce-435c-809f-bd40741c8795 {error:#?}");
//             });
//     }
// }

#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExamplePreparePostgresqlErrorNamed {
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
impl Example {
    pub fn table_name() -> &'static str {
        "example"
    }
    fn primary_key() -> &'static std::primitive::str {
        "primary_key_column"
    }
    pub async fn prepare_postgresql(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), ExamplePreparePostgresqlErrorNamed> {
        let create_extension_if_not_exists_pg_jsonschema_query_stringified = "create extension if not exists pg_jsonschema";
        if let Err(error) = sqlx::query(create_extension_if_not_exists_pg_jsonschema_query_stringified).execute(pool).await {
            return Err(ExamplePreparePostgresqlErrorNamed::CreateExtensionIfNotExistsPgJsonschema { error, code_occurence: error_occurence_lib::code_occurence!() });
        }
        let create_extension_if_not_exists_uuid_ossp_query_stringified = "create extension if not exists \"uuid-ossp\"";
        if let Err(error) = sqlx::query(create_extension_if_not_exists_uuid_ossp_query_stringified).execute(pool).await {
            return Err(ExamplePreparePostgresqlErrorNamed::CreateExtensionIfNotExistsUuidOssp { error, code_occurence: error_occurence_lib::code_occurence!() });
        }
        let prepare_postgresql_query_stringified = format!("create table if not exists example ({},{},{})", <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"primary_key_column", true), <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"column_0", false), <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"column_30", false));
        if let Err(error) = sqlx::query(&prepare_postgresql_query_stringified).execute(pool).await {
            return Err(ExamplePreparePostgresqlErrorNamed::PreparePostgresql { error, code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(())
    }
    pub fn allow_methods() -> [http::Method; 4] {
        [http::Method::GET, http::Method::POST, http::Method::PATCH, http::Method::DELETE]
    }
    fn generate_select_query_part(select: &postgresql_crud::NotEmptyUniqueEnumVec<ExampleSelect>) -> std::string::String {
        let mut value = std::string::String::default();
        for element in select.to_vec() {
            value.push_str(&match element {
                ExampleSelect::PrimaryKeyColumn(value) => <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::select_query_part(value, "primary_key_column"),
                ExampleSelect::Column0(value) => <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_query_part(value, "column_0"),
                ExampleSelect::Column30(value) => <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::select_query_part(value, "column_30"),
            });
            value.push_str(",");
        }
        let _: std::option::Option<std::primitive::char> = value.pop();
        value
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
pub struct ExampleCreate {
    pub column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create,
    pub column_30: <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Create,
}
impl ExampleCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::create_query_part(&<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(), increment) {
            Ok(value) => {
                acc.push_str(&format!("{value},"));
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::create_query_part(&self.column_0, increment) {
            Ok(value) => {
                acc.push_str(&format!("{value},"));
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        match <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::create_query_part(&self.column_30, increment) {
            Ok(value) => {
                acc.push_str(&format!("{value},"));
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        let _: Option<char> = acc.pop();
        Ok(acc)
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
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
        match <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::create_query_bind(self.column_30, query) {
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
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ExampleCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { column_0: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(), column_30: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }
    }
}
#[derive(Debug, Clone, serde :: Serialize, utoipa :: ToSchema)]
pub struct ExampleWhereMany {
    primary_key_column: std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement>>,
    column_0: std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::WhereElement>>,
    column_30: std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement>>,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleWhereManyTryNewErrorNamed {
    NoFieldsProvided {
        #[eo_to_std_string_string]
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl ExampleWhereMany {
    pub fn try_new(primary_key_column: std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement>>, column_0: std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::WhereElement>>, column_30: std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement>>) -> Result<ExampleWhereMany, ExampleWhereManyTryNewErrorNamed> {
        if let (None, None, None) = (&primary_key_column, &column_0, &column_30) {
            return Err(ExampleWhereManyTryNewErrorNamed::NoFieldsProvided { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { primary_key_column, column_0, column_30 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ExampleWhereMany {
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
                        "primary_key_column" => serde::__private::Ok(__Field::__field0),
                        "column_0" => serde::__private::Ok(__Field::__field1),
                        "column_30" => serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"primary_key_column" => serde::__private::Ok(__Field::__field0),
                        b"column_0" => serde::__private::Ok(__Field::__field1),
                        b"column_30" => serde::__private::Ok(__Field::__field2),
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
                marker: _serde::__private::PhantomData<ExampleWhereMany>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ExampleWhereMany;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct ExampleWhereMany")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct ExampleWhereMany with 3 elements"));
                        }
                    };
                    let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::WhereElement>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct ExampleWhereMany with 3 elements"));
                        }
                    };
                    let __field2 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct ExampleWhereMany with 3 elements"));
                        }
                    };
                    match ExampleWhereMany::try_new(__field0, __field1, __field2) {
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
                    let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement>>> = serde::__private::None;
                    let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::WhereElement>>> = serde::__private::None;
                    let mut __field2: serde::__private::Option<std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement>>> = serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("primary_key_column"));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement>>>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if serde::__private::Option::is_some(&__field1) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("column_0"));
                                }
                                __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::WhereElement>>>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if serde::__private::Option::is_some(&__field2) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("column_30"));
                                }
                                __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement>>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        serde::__private::Some(__field0) => __field0,
                        serde::__private::None => serde::__private::de::missing_field("primary_key_column")?,
                    };
                    let __field1 = match __field1 {
                        serde::__private::Some(__field1) => __field1,
                        serde::__private::None => serde::__private::de::missing_field("column_0")?,
                    };
                    let __field2 = match __field2 {
                        serde::__private::Some(__field2) => __field2,
                        serde::__private::None => serde::__private::de::missing_field("column_30")?,
                    };
                    match ExampleWhereMany::try_new(__field0, __field1, __field2) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["primary_key_column", "column_0", "column_30"];
            _serde::Deserializer::deserialize_struct(__deserializer, "ExampleWhereMany", FIELDS, __Visitor { marker: _serde::__private::PhantomData::<ExampleWhereMany>, lifetime: _serde::__private::PhantomData })
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ExampleWhereMany {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { primary_key_column: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()), column_0: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()), column_30: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()) }
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
pub struct StdOptionOptionExampleWhereMany(pub std::option::Option<ExampleWhereMany>);
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for StdOptionOptionExampleWhereMany {
    fn query_part(&self, increment: &mut std::primitive::u64, _: &dyn std::fmt::Display, _: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        Ok(match &self.0 {
            Some(value) => {
                let mut additional_parameters = std::string::String::from("where");
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
                if let Some(value) = &value.column_30 {
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &"column_30", is_first_push_to_additional_parameters_already_happend) {
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
            None => std::string::String::default(),
        })
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
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
            if let Some(value) = value.column_30 {
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
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdOptionOptionExampleWhereMany {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, PartialEq, Clone)]
pub enum ExampleSelect {
    #[serde(rename(serialize = "primary_key_column", deserialize = "primary_key_column"))]
    PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select),
    #[serde(rename(serialize = "column_0", deserialize = "column_0"))]
    Column0(<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Select),
    #[serde(rename(serialize = "column_30", deserialize = "column_30"))]
    Column30(<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Select),
}
impl std::fmt::Display for ExampleSelect {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", serde_json::to_string(&self).unwrap_or_else(|e| format!("cannot serialize into json: {e:?}")))
    }
}
impl error_occurence_lib::ToStdStringString for ExampleSelect {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ExampleSelect {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![ExampleSelect::PrimaryKeyColumn(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()), ExampleSelect::Column0(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()), ExampleSelect::Column30(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
#[derive(Debug, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ExampleRead {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key_column: std::option::Option<postgresql_crud::Value<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_30: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>,
}
impl ExampleRead {
    fn try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_example_select(value: sqlx::postgres::PgRow, select: &postgresql_crud::NotEmptyUniqueEnumVec<ExampleSelect>) -> Result<Self, sqlx::Error> {
        let mut primary_key_column: std::option::Option<postgresql_crud::Value<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>> = None;
        let mut column_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Read>> = None;
        let mut column_30: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>> = None;
        for element in select.to_vec() {
            match element {
                ExampleSelect::PrimaryKeyColumn(_) => match sqlx::Row::try_get::<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, &std::primitive::str>(&value, "primary_key_column") {
                    Ok(value) => {
                        primary_key_column = Some(postgresql_crud::Value { value: value });
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                },
                ExampleSelect::Column0(_) => match sqlx::Row::try_get::<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Read, &std::primitive::str>(&value, "column_0") {
                    Ok(value) => {
                        column_0 = Some(postgresql_crud::Value { value: value });
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                },
                ExampleSelect::Column30(_) => match sqlx::Row::try_get::<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Read, &std::primitive::str>(&value, "column_30") {
                    Ok(value) => {
                        column_30 = Some(postgresql_crud::Value { value: value });
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                },
            }
        }
        Ok(Self { primary_key_column, column_0, column_30 })
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ExampleReadOnlyIds {
    pub primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::ReadOnlyIds,
    pub column_0: std::option::Option<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::ReadOnlyIds>,
    pub column_30: std::option::Option<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::ReadOnlyIds>,
}
impl std::convert::TryFrom<sqlx::postgres::PgRow> for ExampleReadOnlyIds {
    type Error = sqlx::Error;
    fn try_from(value: sqlx::postgres::PgRow) -> Result<Self, Self::Error> {
        let primary_key_column = match sqlx::Row::try_get::<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::ReadOnlyIds, &std::primitive::str>(&value, "primary_key_column") {
            Ok(value) => value,
            Err(error_0) => {
                return Err(error_0);
            }
        };
        let column_0 = if let Ok(value) = sqlx::Row::try_get::<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::ReadOnlyIds, &std::primitive::str>(&value, "column_0") { Some(value) } else { None };
        let column_30 = if let Ok(value) = sqlx::Row::try_get::<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::ReadOnlyIds, &std::primitive::str>(&value, "column_30") { Some(value) } else { None };
        Ok(Self { primary_key_column, column_0, column_30 })
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ExampleColumnReadPermission {
    primary_key_column: std::primitive::bool,
    column_0: std::primitive::bool,
    column_30: std::primitive::bool,
}
#[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
pub struct ExampleUpdate {
    primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate,
    column_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>,
    column_30: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Update>>,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleUpdateTryNewErrorNamed {
    NoFieldsProvided {
        #[eo_to_std_string_string]
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl ExampleUpdate {
    pub fn try_new(primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate, column_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>, column_30: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Update>>) -> Result<ExampleUpdate, ExampleUpdateTryNewErrorNamed> {
        if let (None, None) = (&column_0, &column_30) {
            return Err(ExampleUpdateTryNewErrorNamed::NoFieldsProvided { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { primary_key_column, column_0, column_30 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ExampleUpdate {
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
                        "primary_key_column" => serde::__private::Ok(__Field::__field0),
                        "column_0" => serde::__private::Ok(__Field::__field1),
                        "column_30" => serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"primary_key_column" => serde::__private::Ok(__Field::__field0),
                        b"column_0" => serde::__private::Ok(__Field::__field1),
                        b"column_30" => serde::__private::Ok(__Field::__field2),
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
                marker: _serde::__private::PhantomData<ExampleUpdate>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ExampleUpdate;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct ExampleUpdate")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct ExampleUpdate with 3 elements"));
                        }
                    };
                    let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct ExampleUpdate with 3 elements"));
                        }
                    };
                    let __field2 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Update>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct ExampleUpdate with 3 elements"));
                        }
                    };
                    match ExampleUpdate::try_new(__field0, __field1, __field2) {
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
                    let mut __field0: serde::__private::Option<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate> = serde::__private::None;
                    let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>> = serde::__private::None;
                    let mut __field2: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Update>>> = serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("primary_key_column"));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if serde::__private::Option::is_some(&__field1) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("column_0"));
                                }
                                __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if serde::__private::Option::is_some(&__field2) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("column_30"));
                                }
                                __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Update>>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        serde::__private::Some(__field0) => __field0,
                        serde::__private::None => serde::__private::de::missing_field("primary_key_column")?,
                    };
                    let __field1 = match __field1 {
                        serde::__private::Some(__field1) => __field1,
                        serde::__private::None => serde::__private::de::missing_field("column_0")?,
                    };
                    let __field2 = match __field2 {
                        serde::__private::Some(__field2) => __field2,
                        serde::__private::None => serde::__private::de::missing_field("column_30")?,
                    };
                    match ExampleUpdate::try_new(__field0, __field1, __field2) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["primary_key_column", "column_0", "column_30"];
            _serde::Deserializer::deserialize_struct(__deserializer, "ExampleUpdate", FIELDS, __Visitor { marker: _serde::__private::PhantomData::<ExampleUpdate>, lifetime: _serde::__private::PhantomData })
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ExampleUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            primary_key_column: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            column_0: Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }),
            column_30: Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }),
        }
    }
}
#[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
pub struct ExampleUpdateForQuery {
    primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdateForQuery,
    column_0: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::UpdateForQuery>>,
    column_30: std::option::Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::UpdateForQuery>>,
}
impl ExampleUpdateForQuery {
    fn update_query_part_primary_key(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_part(&self.primary_key_column, &"", &Example::primary_key(), &"", increment) {
            Ok(value_snake_case) => Ok(value_snake_case),
            Err(error_0) => Err(error_0),
        }
    }
    fn update_query_part_column_0(value: &postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::UpdateForQuery>, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::update_query_part(&value.value, &"column_0", &"column_0", &"", increment) {
            Ok(value) => Ok(value),
            Err(error_0) => Err(error_0),
        }
    }
    fn update_query_part_column_30(value: &postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::UpdateForQuery>, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_part(&value.value, &"column_30", &"column_30", &"", increment) {
            Ok(value) => Ok(value),
            Err(error_0) => Err(error_0),
        }
    }
    fn select_only_updated_ids_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = std::string::String::new();
        acc.push_str(&match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_part(&self.primary_key_column, "primary_key_column", increment, true) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        });
        if let Some(value) = &self.column_0 {
            acc.push_str(&match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_part(&value.value, "column_0", increment, false) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error);
                }
            });
        }
        if let Some(value) = &self.column_30 {
            acc.push_str(&match <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_part(&value.value, "column_30", increment, false) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error);
                }
            });
        }
        let _ = acc.pop();
        Ok(acc)
    }
}
impl std::convert::From<ExampleUpdate> for ExampleUpdateForQuery {
    fn from(value: ExampleUpdate) -> Self {
        Self {
            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::UpdateForQuery::from(value.primary_key_column),
            column_0: match value.column_0 {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::UpdateForQuery::from(value.value) }),
                None => None,
            },
            column_30: match value.column_30 {
                Some(value) => Some(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::UpdateForQuery::from(value.value) }),
                None => None,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ExampleCreateManyPayload(pub std::vec::Vec<ExampleCreate>);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ExampleCreateManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
#[derive(Debug)]
pub struct ExampleCreateManyParameters {
    pub payload: ExampleCreateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum ExampleCreateManyResponseVariants {
    Desirable(std::vec::Vec<ExampleReadOnlyIds>),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: std::string::String, rollback: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl std::convert::From<ExampleCreateManyErrorNamed> for ExampleCreateManyResponseVariants {
    fn from(value: ExampleCreateManyErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            ExampleCreateManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            ExampleCreateManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            ExampleCreateManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            ExampleCreateManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleCreateManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            ExampleCreateManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            ExampleCreateManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            ExampleCreateManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleCreateManyErrorNamed {
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
        try_bind: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn create_many(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = ExampleCreateManyErrorNamed::HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2117, column: 330 })) };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleCreateManyErrorNamed::CheckBodySize { check_body_size: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2118, column: 268 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        println!("GeneratePostgresqlTypesExample create_many log");
        let parameters = ExampleCreateManyParameters {
            payload: match serde_json::from_slice::<ExampleCreateManyPayload>(&body_bytes) {
                Ok(value) => {
                    let value = ExampleCreateManyPayload::from(value);
                    value
                }
                Err(error_0) => {
                    let error = ExampleCreateManyErrorNamed::SerdeJson { serde_json: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2193, column: 249 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_create_many_query_string(
            &Example::table_name(),
            "primary_key_column,column_0,column_30",
            {
                let mut increment: std::primitive::u64 = 0;
                let mut acc = std::string::String::default();
                for element in &parameters.payload.0 {
                    match element.create_query_part(&mut increment) {
                        Ok(value) => {
                            acc.push_str(&format!("({value}),"));
                        }
                        Err(error_0) => {
                            let error = ExampleCreateManyErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2507, column: 254 })) };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                let _: Option<char> = acc.pop();
                acc
            },
            &{
                let mut acc = std::string::String::new();
                acc.push_str(&<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::select_only_ids_query_part("primary_key_column", true));
                acc.push_str(&<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_only_ids_query_part("column_0", false));
                acc.push_str(&<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::select_only_ids_query_part("column_30", false));
                let _ = acc.pop();
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
                        let error = ExampleCreateManyErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2535, column: 25 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::from(error)));
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
                let error = ExampleCreateManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleCreateManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = ExampleCreateManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1911, column: 245 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc = std::vec::Vec::new();
                while let Some(value) = match {
                    use postgresql_crud::TryStreamExt;
                    rows.try_next()
                }
                .await
                {
                    Ok(value) => match value {
                        Some(value) => match ExampleReadOnlyIds::try_from(value) {
                            Ok(value) => Some(value),
                            Err(error_0) => {
                                drop(rows);
                                match executor.rollback().await {
                                    Ok(_) => {
                                        let error = ExampleCreateManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2567, column: 230 })) };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                    Err(error_1) => {
                                        let error = ExampleCreateManyErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2567, column: 259 })) };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                }
                            }
                        },
                        None => None,
                    },
                    Err(error_0) => {
                        drop(rows);
                        match executor.rollback().await {
                            Ok(_) => {
                                let error = ExampleCreateManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2577, column: 149 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            Err(error_1) => {
                                let error = ExampleCreateManyErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2577, column: 178 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                } {
                    acc.push(value);
                }
                acc
            };
            if let Err(error_0) = executor.commit().await {
                let error = ExampleCreateManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1922, column: 245 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateManyResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::CREATED;
        return response;
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleTryCreateManyErrorNamed {
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
    ExampleCreateManyErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        create_many_error_named_with_serialize_deserialize: ExampleCreateManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn try_create_many(endpoint_location: &std::primitive::str, parameters: ExampleCreateManyParameters) -> Result<std::vec::Vec<ExampleReadOnlyIds>, ExampleTryCreateManyErrorNamed> {
        let payload = {
            let value = ExampleCreateManyPayload::from(parameters.payload);
            match serde_json::to_string(&value) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(ExampleTryCreateManyErrorNamed::SerdeJsonToString { serde_json_to_string: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2218, column: 178 })) });
                }
            }
        };
        let url = format!("{}/example/create_many", endpoint_location,);
        let future = reqwest::Client::new().post(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(ExampleTryCreateManyErrorNamed::Reqwest { reqwest: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2267, column: 152 })) });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(ExampleTryCreateManyErrorNamed::FailedToGetResponseText { status_code: error_0, headers: error_1, reqwest: error_2, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2284, column: 192 })) });
            }
        };
        let expected_response = match serde_json::from_str::<ExampleCreateManyResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(ExampleTryCreateManyErrorNamed::DeserializeResponse { status_code: error_0, headers: error_1, response_text: error_2, serde: error_3, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2296, column: 178 })) });
            }
        };
        let create_many_error_named_with_serialize_deserialize = match expected_response {
            ExampleCreateManyResponseVariants::Desirable(value) => {
                let value = value;
                return Ok(value);
            }
            ExampleCreateManyResponseVariants::CheckBodySize { check_body_size, code_occurence } => ExampleCreateManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            ExampleCreateManyResponseVariants::Postgresql { postgresql, code_occurence } => ExampleCreateManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            ExampleCreateManyResponseVariants::SerdeJson { serde_json, code_occurence } => ExampleCreateManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            ExampleCreateManyResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => ExampleCreateManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleCreateManyResponseVariants::CheckCommit { check_commit, code_occurence } => ExampleCreateManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            ExampleCreateManyResponseVariants::QueryPart { error, code_occurence } => ExampleCreateManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            ExampleCreateManyResponseVariants::RowAndRollback { row, rollback, code_occurence } => ExampleCreateManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            ExampleCreateManyResponseVariants::TryBind { try_bind, code_occurence } => ExampleCreateManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(ExampleTryCreateManyErrorNamed::ExampleCreateManyErrorNamedWithSerializeDeserialize { create_many_error_named_with_serialize_deserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2334, column: 223 })) })
    }
}
impl Example {
    pub async fn create_many_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<ExampleCreateManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
#[derive(Debug)]
pub struct ExampleCreateOneParameters {
    pub payload: ExampleCreate,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum ExampleCreateOneResponseVariants {
    Desirable(ExampleReadOnlyIds),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: std::string::String, rollback: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl std::convert::From<ExampleCreateOneErrorNamed> for ExampleCreateOneResponseVariants {
    fn from(value: ExampleCreateOneErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            ExampleCreateOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            ExampleCreateOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            ExampleCreateOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            ExampleCreateOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleCreateOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            ExampleCreateOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            ExampleCreateOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            ExampleCreateOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleCreateOneErrorNamed {
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
        try_bind: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn create_one(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = ExampleCreateOneErrorNamed::HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2117, column: 330 })) };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleCreateOneErrorNamed::CheckBodySize { check_body_size: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2118, column: 268 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = ExampleCreateOneParameters {
            payload: match serde_json::from_slice::<ExampleCreate>(&body_bytes) {
                Ok(value) => {
                    let value = ExampleCreate::from(value);
                    value
                }
                Err(error_0) => {
                    let error = ExampleCreateOneErrorNamed::SerdeJson { serde_json: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2193, column: 249 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_create_one_query_string(
            &Example::table_name(),
            "primary_key_column,column_0,column_30",
            match parameters.payload.create_query_part(&mut 0) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = ExampleCreateOneErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2629, column: 254 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
            &{
                let mut acc = std::string::String::new();
                acc.push_str(&<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::select_only_ids_query_part("primary_key_column", true));
                acc.push_str(&<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_only_ids_query_part("column_0", false));
                acc.push_str(&<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::select_only_ids_query_part("column_30", false));
                let _ = acc.pop();
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
                    let error = ExampleCreateOneErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2649, column: 25 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleCreateOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleCreateOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = ExampleCreateOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1911, column: 245 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                match binded_query.fetch_one(executor.as_mut()).await {
                    Ok(value) => match ExampleReadOnlyIds::try_from(value) {
                        Ok(value) => value,
                        Err(error_0) => match executor.rollback().await {
                            Ok(_) => {
                                let error = ExampleCreateOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2683, column: 196 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            Err(error_1) => {
                                let error = ExampleCreateOneErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2683, column: 225 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        },
                    },
                    Err(error_0) => match executor.rollback().await {
                        Ok(_) => {
                            let error = ExampleCreateOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2693, column: 132 })) };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = ExampleCreateOneErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2693, column: 161 })) };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                }
            };
            if let Err(error_0) = executor.commit().await {
                let error = ExampleCreateOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1922, column: 245 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleCreateOneResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::CREATED;
        return response;
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleTryCreateOneErrorNamed {
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
    ExampleCreateOneErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        create_one_error_named_with_serialize_deserialize: ExampleCreateOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn try_create_one(endpoint_location: &std::primitive::str, parameters: ExampleCreateOneParameters) -> Result<ExampleReadOnlyIds, ExampleTryCreateOneErrorNamed> {
        let payload = {
            let value = ExampleCreate::from(parameters.payload);
            match serde_json::to_string(&value) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(ExampleTryCreateOneErrorNamed::SerdeJsonToString { serde_json_to_string: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2218, column: 178 })) });
                }
            }
        };
        let url = format!("{}/example/create_one", endpoint_location,);
        let future = reqwest::Client::new().post(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(ExampleTryCreateOneErrorNamed::Reqwest { reqwest: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2267, column: 152 })) });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(ExampleTryCreateOneErrorNamed::FailedToGetResponseText { status_code: error_0, headers: error_1, reqwest: error_2, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2284, column: 192 })) });
            }
        };
        let expected_response = match serde_json::from_str::<ExampleCreateOneResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(ExampleTryCreateOneErrorNamed::DeserializeResponse { status_code: error_0, headers: error_1, response_text: error_2, serde: error_3, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2296, column: 178 })) });
            }
        };
        let create_one_error_named_with_serialize_deserialize = match expected_response {
            ExampleCreateOneResponseVariants::Desirable(value) => {
                let value = value;
                return Ok(value);
            }
            ExampleCreateOneResponseVariants::CheckBodySize { check_body_size, code_occurence } => ExampleCreateOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            ExampleCreateOneResponseVariants::Postgresql { postgresql, code_occurence } => ExampleCreateOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            ExampleCreateOneResponseVariants::SerdeJson { serde_json, code_occurence } => ExampleCreateOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            ExampleCreateOneResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => ExampleCreateOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleCreateOneResponseVariants::CheckCommit { check_commit, code_occurence } => ExampleCreateOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            ExampleCreateOneResponseVariants::RowAndRollback { row, rollback, code_occurence } => ExampleCreateOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            ExampleCreateOneResponseVariants::QueryPart { error, code_occurence } => ExampleCreateOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            ExampleCreateOneResponseVariants::TryBind { try_bind, code_occurence } => ExampleCreateOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(ExampleTryCreateOneErrorNamed::ExampleCreateOneErrorNamedWithSerializeDeserialize { create_one_error_named_with_serialize_deserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2334, column: 223 })) })
    }
}
impl Example {
    pub async fn create_one_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<ExampleCreate as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ExampleReadManyPayload {
    pub where_many: StdOptionOptionExampleWhereMany,
    pub select: postgresql_crud::NotEmptyUniqueEnumVec<ExampleSelect>,
    pub order_by: postgresql_crud::OrderBy<ExampleSelect>,
    pub pagination: postgresql_crud::PaginationStartsWithZero,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ExampleReadManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            where_many: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            select: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            order_by: postgresql_crud::OrderBy { column: ExampleSelect::PrimaryKeyColumn(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()), order: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()) },
            pagination: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug)]
pub struct ExampleReadManyParameters {
    pub payload: ExampleReadManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum ExampleReadManyResponseVariants {
    Desirable(std::vec::Vec<ExampleRead>),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    NotUniqueField { not_unique_field: ExampleSelect, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl std::convert::From<ExampleReadManyErrorNamed> for ExampleReadManyResponseVariants {
    fn from(value: ExampleReadManyErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            ExampleReadManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            ExampleReadManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            ExampleReadManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            ExampleReadManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleReadManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            ExampleReadManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            ExampleReadManyErrorNamedWithSerializeDeserialize::NotUniqueField { not_unique_field, code_occurence } => Self::NotUniqueField { not_unique_field, code_occurence },
            ExampleReadManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleReadManyErrorNamed {
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
        not_unique_field: ExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn read_many(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = ExampleReadManyErrorNamed::HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2117, column: 330 })) };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadManyResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleReadManyErrorNamed::CheckBodySize { check_body_size: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2118, column: 268 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = ExampleReadManyParameters {
            payload: match serde_json::from_slice::<ExampleReadManyPayload>(&body_bytes) {
                Ok(value) => {
                    let value = ExampleReadManyPayload::from(value);
                    value
                }
                Err(error_0) => {
                    let error = ExampleReadManyErrorNamed::SerdeJson { serde_json: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2193, column: 249 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_read_many_query_string(&Example::table_name(), Example::generate_select_query_part(&parameters.payload.select), {
            let mut increment: std::primitive::u64 = 0;
            let mut additional_parameters = match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.where_many, &mut increment, &"", false) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = ExampleReadManyErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1020, column: 274 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            };
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "",
                    false => " ",
                };
                let value = &parameters.payload.order_by;
                let order = match &value.order {
                    Some(value) => value.to_snake_case_stringified(),
                    None => postgresql_crud::Order::default().to_snake_case_stringified(),
                };
                additional_parameters.push_str(&format!(
                    "{}order by {} {}",
                    prefix,
                    &match &value.column {
                        ExampleSelect::PrimaryKeyColumn(_) => "primary_key_column",
                        ExampleSelect::Column0(_) => "column_0",
                        ExampleSelect::Column30(_) => "column_30",
                    },
                    order,
                ));
            }
            {
                let prefix = match additional_parameters.is_empty() {
                    true => "",
                    false => " ",
                };
                let value = match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.pagination, &mut increment, &"", std::primitive::bool::default()) {
                    Ok(value) => value,
                    Err(error_0) => {
                        let error = ExampleReadManyErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2785, column: 254 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                };
                additional_parameters.push_str(&format!("{}{}", prefix, value));
            }
            additional_parameters
        });
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.where_many, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = ExampleReadManyErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1051, column: 13 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.pagination, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = ExampleReadManyErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2846, column: 25 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleReadManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleReadManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let value = {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc = std::vec::Vec::new();
                while let Some(value) = match {
                    use postgresql_crud::TryStreamExt;
                    rows.try_next()
                }
                .await
                {
                    Ok(value) => match value {
                        Some(value) => Some(match ExampleRead::try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_example_select(value, &parameters.payload.select) {
                            Ok(value) => value,
                            Err(error_0) => {
                                let error = ExampleReadManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1086, column: 271 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }),
                        None => None,
                    },
                    Err(error_0) => {
                        let error = ExampleReadManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2872, column: 169 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                } {
                    acc.push(value);
                }
                acc
            };
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadManyResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleTryReadManyErrorNamed {
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
    NotUniqueField {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_field: ExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ExampleReadManyErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        read_many_error_named_with_serialize_deserialize: ExampleReadManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn try_read_many(endpoint_location: &std::primitive::str, parameters: ExampleReadManyParameters) -> Result<std::vec::Vec<ExampleRead>, ExampleTryReadManyErrorNamed> {
        let payload = {
            let value = ExampleReadManyPayload::from(parameters.payload);
            match serde_json::to_string(&value) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(ExampleTryReadManyErrorNamed::SerdeJsonToString { serde_json_to_string: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2218, column: 178 })) });
                }
            }
        };
        let url = format!("{}/example/read_many", endpoint_location,);
        let future = reqwest::Client::new().post(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(ExampleTryReadManyErrorNamed::Reqwest { reqwest: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2267, column: 152 })) });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(ExampleTryReadManyErrorNamed::FailedToGetResponseText { status_code: error_0, headers: error_1, reqwest: error_2, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2284, column: 192 })) });
            }
        };
        let expected_response = match serde_json::from_str::<ExampleReadManyResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(ExampleTryReadManyErrorNamed::DeserializeResponse { status_code: error_0, headers: error_1, response_text: error_2, serde: error_3, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2296, column: 178 })) });
            }
        };
        let read_many_error_named_with_serialize_deserialize = match expected_response {
            ExampleReadManyResponseVariants::Desirable(value) => {
                let value = value.into_iter().fold(std::vec::Vec::new(), |mut acc, element| {
                    acc.push(element);
                    acc
                });
                return Ok(value);
            }
            ExampleReadManyResponseVariants::CheckBodySize { check_body_size, code_occurence } => ExampleReadManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            ExampleReadManyResponseVariants::Postgresql { postgresql, code_occurence } => ExampleReadManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            ExampleReadManyResponseVariants::SerdeJson { serde_json, code_occurence } => ExampleReadManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            ExampleReadManyResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => ExampleReadManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleReadManyResponseVariants::CheckCommit { check_commit, code_occurence } => ExampleReadManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            ExampleReadManyResponseVariants::QueryPart { error, code_occurence } => ExampleReadManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            ExampleReadManyResponseVariants::NotUniqueField { not_unique_field, code_occurence } => ExampleReadManyErrorNamedWithSerializeDeserialize::NotUniqueField { not_unique_field, code_occurence },
            ExampleReadManyResponseVariants::TryBind { try_bind, code_occurence } => ExampleReadManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(ExampleTryReadManyErrorNamed::ExampleReadManyErrorNamedWithSerializeDeserialize { read_many_error_named_with_serialize_deserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2334, column: 223 })) })
    }
}
impl Example {
    pub async fn read_many_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<ExampleReadManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ExampleReadOnePayload {
    pub primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead,
    pub select: postgresql_crud::NotEmptyUniqueEnumVec<ExampleSelect>,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ExampleReadOnePayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { primary_key_column: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(), select: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }
    }
}
#[derive(Debug)]
pub struct ExampleReadOneParameters {
    pub payload: ExampleReadOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum ExampleReadOneResponseVariants {
    Desirable(ExampleRead),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    NotUniqueField { not_unique_field: ExampleSelect, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl std::convert::From<ExampleReadOneErrorNamed> for ExampleReadOneResponseVariants {
    fn from(value: ExampleReadOneErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            ExampleReadOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            ExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            ExampleReadOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            ExampleReadOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleReadOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            ExampleReadOneErrorNamedWithSerializeDeserialize::NotUniqueField { not_unique_field, code_occurence } => Self::NotUniqueField { not_unique_field, code_occurence },
            ExampleReadOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            ExampleReadOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleReadOneErrorNamed {
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
        not_unique_field: ExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn read_one(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = ExampleReadOneErrorNamed::HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2117, column: 330 })) };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadOneResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleReadOneErrorNamed::CheckBodySize { check_body_size: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2118, column: 268 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = ExampleReadOneParameters {
            payload: match serde_json::from_slice::<ExampleReadOnePayload>(&body_bytes) {
                Ok(value) => {
                    let value = ExampleReadOnePayload::from(value);
                    value
                }
                Err(error_0) => {
                    let error = ExampleReadOneErrorNamed::SerdeJson { serde_json: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2193, column: 249 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_read_one_query_string(
            &Example::table_name(),
            Example::generate_select_query_part(&parameters.payload.select),
            match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.primary_key_column, &mut 0, &Example::primary_key(), false) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = ExampleReadOneErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2960, column: 254 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadOneResponseVariants::from(error)));
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
                    let error = ExampleReadOneErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2983, column: 29 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleReadOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleReadOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let value = {
                match binded_query.fetch_one(executor.as_mut()).await {
                    Ok(value) => match ExampleRead::try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_example_select(value, &parameters.payload.select) {
                        Ok(value) => value,
                        Err(error_0) => {
                            let error = ExampleReadOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1086, column: 271 })) };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                    Err(error_0) => {
                        let error = ExampleReadOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3006, column: 169 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            };
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleReadOneResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleTryReadOneErrorNamed {
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
    NotUniqueField {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_field: ExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ExampleReadOneErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        read_one_error_named_with_serialize_deserialize: ExampleReadOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn try_read_one(endpoint_location: &std::primitive::str, parameters: ExampleReadOneParameters) -> Result<ExampleRead, ExampleTryReadOneErrorNamed> {
        let payload = {
            let value = ExampleReadOnePayload::from(parameters.payload);
            match serde_json::to_string(&value) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(ExampleTryReadOneErrorNamed::SerdeJsonToString { serde_json_to_string: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2218, column: 178 })) });
                }
            }
        };
        let url = format!("{}/example/read_one", endpoint_location,);
        let future = reqwest::Client::new().post(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(ExampleTryReadOneErrorNamed::Reqwest { reqwest: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2267, column: 152 })) });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(ExampleTryReadOneErrorNamed::FailedToGetResponseText { status_code: error_0, headers: error_1, reqwest: error_2, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2284, column: 192 })) });
            }
        };
        let expected_response = match serde_json::from_str::<ExampleReadOneResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(ExampleTryReadOneErrorNamed::DeserializeResponse { status_code: error_0, headers: error_1, response_text: error_2, serde: error_3, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2296, column: 178 })) });
            }
        };
        let read_one_error_named_with_serialize_deserialize = match expected_response {
            ExampleReadOneResponseVariants::Desirable(value) => {
                let value = value;
                return Ok(value);
            }
            ExampleReadOneResponseVariants::CheckBodySize { check_body_size, code_occurence } => ExampleReadOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            ExampleReadOneResponseVariants::Postgresql { postgresql, code_occurence } => ExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            ExampleReadOneResponseVariants::SerdeJson { serde_json, code_occurence } => ExampleReadOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            ExampleReadOneResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => ExampleReadOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleReadOneResponseVariants::CheckCommit { check_commit, code_occurence } => ExampleReadOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            ExampleReadOneResponseVariants::NotUniqueField { not_unique_field, code_occurence } => ExampleReadOneErrorNamedWithSerializeDeserialize::NotUniqueField { not_unique_field, code_occurence },
            ExampleReadOneResponseVariants::QueryPart { error, code_occurence } => ExampleReadOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            ExampleReadOneResponseVariants::TryBind { try_bind, code_occurence } => ExampleReadOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(ExampleTryReadOneErrorNamed::ExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2334, column: 223 })) })
    }
}
impl Example {
    pub async fn read_one_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<ExampleReadOnePayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
#[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
pub struct ExampleUpdateManyPayload(std::vec::Vec<ExampleUpdate>);
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleUpdateManyPayloadTryNewErrorNamed {
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate,
        #[eo_to_std_string_string]
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl ExampleUpdateManyPayload {
    fn try_new(value: std::vec::Vec<ExampleUpdate>) -> Result<Self, ExampleUpdateManyPayloadTryNewErrorNamed> {
        let mut acc = std::vec::Vec::new();
        for element in &value {
            if !acc.contains(&&element.primary_key_column) {
                acc.push(&element.primary_key_column);
            } else {
                return Err(ExampleUpdateManyPayloadTryNewErrorNamed::NotUniquePrimaryKey { not_unique_primary_key: element.primary_key_column.clone(), code_occurence: error_occurence_lib::code_occurence!() });
            }
        }
        Ok(Self(value))
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ExampleUpdateManyPayload {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<ExampleUpdateManyPayload>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ExampleUpdateManyPayload;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct ExampleUpdateManyPayload")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: std::vec::Vec<ExampleUpdate> = <std::vec::Vec<ExampleUpdate> as _serde::Deserialize>::deserialize(__e)?;
                    match ExampleUpdateManyPayload::try_new(__field0) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<ExampleUpdate>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct ExampleUpdateManyPayload with 1 element"));
                        }
                    };
                    match ExampleUpdateManyPayload::try_new(__field0) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(__deserializer, "ExampleUpdateManyPayload", __Visitor { marker: _serde::__private::PhantomData::<ExampleUpdateManyPayload>, lifetime: _serde::__private::PhantomData })
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ExampleUpdateManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
#[derive(Debug)]
pub struct ExampleUpdateManyParameters {
    pub payload: ExampleUpdateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum ExampleUpdateManyResponseVariants {
    Desirable(std::vec::Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: std::string::String, rollback: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl std::convert::From<ExampleUpdateManyErrorNamed> for ExampleUpdateManyResponseVariants {
    fn from(value: ExampleUpdateManyErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            ExampleUpdateManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            ExampleUpdateManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            ExampleUpdateManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            ExampleUpdateManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleUpdateManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            ExampleUpdateManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            ExampleUpdateManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            ExampleUpdateManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleUpdateManyErrorNamed {
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
        try_bind: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn update_many(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = ExampleUpdateManyErrorNamed::HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2117, column: 330 })) };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleUpdateManyErrorNamed::CheckBodySize { check_body_size: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2118, column: 268 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = ExampleUpdateManyParameters {
            payload: match serde_json::from_slice::<ExampleUpdateManyPayload>(&body_bytes) {
                Ok(value) => {
                    let value = ExampleUpdateManyPayload::from(value);
                    value
                }
                Err(error_0) => {
                    let error = ExampleUpdateManyErrorNamed::SerdeJson { serde_json: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2193, column: 249 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let update_for_query_vec = parameters.payload.0.into_iter().map(|element| ExampleUpdateForQuery::from(element)).collect::<std::vec::Vec<ExampleUpdateForQuery>>();
        let query_string = {
            let mut increment: std::primitive::u64 = 0;
            let elements = {
                let mut acc = std::string::String::default();
                {
                    let mut is_column_0_update_exist = false;
                    for element in &update_for_query_vec {
                        if element.column_0.is_some() {
                            is_column_0_update_exist = true;
                            break;
                        }
                    }
                    if is_column_0_update_exist {
                        acc.push_str(&postgresql_crud::generate_column_equals_case_acc_else_column_end_comma_update_many_query_part(&"column_0", {
                            let mut acc = std::string::String::default();
                            for element in &update_for_query_vec {
                                if let Some(value) = &element.column_0 {
                                    acc.push_str(&postgresql_crud::generate_when_column_id_then_value_update_many_query_part(
                                        &Example::primary_key(),
                                        match element.update_query_part_primary_key(&mut increment) {
                                            Ok(value) => value,
                                            Err(error_0) => {
                                                let error = ExampleUpdateManyErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3220, column: 258 })) };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                                return response;
                                            }
                                        },
                                        match ExampleUpdateForQuery::update_query_part_column_0(&value, &mut increment) {
                                            Ok(value) => value,
                                            Err(error_0) => {
                                                let error = ExampleUpdateManyErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3220, column: 258 })) };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
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
                    let mut is_column_30_update_exist = false;
                    for element in &update_for_query_vec {
                        if element.column_30.is_some() {
                            is_column_30_update_exist = true;
                            break;
                        }
                    }
                    if is_column_30_update_exist {
                        acc.push_str(&postgresql_crud::generate_column_equals_case_acc_else_column_end_comma_update_many_query_part(&"column_30", {
                            let mut acc = std::string::String::default();
                            for element in &update_for_query_vec {
                                if let Some(value) = &element.column_30 {
                                    acc.push_str(&postgresql_crud::generate_when_column_id_then_value_update_many_query_part(
                                        &Example::primary_key(),
                                        match element.update_query_part_primary_key(&mut increment) {
                                            Ok(value) => value,
                                            Err(error_0) => {
                                                let error = ExampleUpdateManyErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3220, column: 258 })) };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                                return response;
                                            }
                                        },
                                        match ExampleUpdateForQuery::update_query_part_column_30(&value, &mut increment) {
                                            Ok(value) => value,
                                            Err(error_0) => {
                                                let error = ExampleUpdateManyErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3220, column: 258 })) };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
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
                let mut acc = std::string::String::default();
                for element in &update_for_query_vec {
                    acc.push_str(&format!(
                        "{},",
                        match element.update_query_part_primary_key(&mut increment) {
                            Ok(value) => value,
                            Err(error_0) => {
                                let error = ExampleUpdateManyErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1676, column: 241 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        }
                    ));
                }
                let _: Option<char> = acc.pop();
                acc
            };
            postgresql_crud::generate_update_many_query_string(&Example::table_name(), elements, &Example::primary_key(), primary_keys)
        };
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            for element in &update_for_query_vec {
                if let Some(value) = &element.column_0 {
                    if let Err(error_0) = query.try_bind(element.primary_key_column.clone()) {
                        let error_0 = error_0.to_string();
                        let error = ExampleUpdateManyErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3297, column: 25 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::update_query_bind(value.value.clone(), query) {
                        Ok(value) => {
                            query = value;
                        }
                        Err(error_0) => {
                            let error = ExampleUpdateManyErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3297, column: 25 })) };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            }
            for element in &update_for_query_vec {
                if let Some(value) = &element.column_30 {
                    if let Err(error_0) = query.try_bind(element.primary_key_column.clone()) {
                        let error_0 = error_0.to_string();
                        let error = ExampleUpdateManyErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3297, column: 25 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    match <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_bind(value.value.clone(), query) {
                        Ok(value) => {
                            query = value;
                        }
                        Err(error_0) => {
                            let error = ExampleUpdateManyErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3297, column: 25 })) };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            }
            for element in update_for_query_vec {
                match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_bind(element.primary_key_column, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = ExampleUpdateManyErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3297, column: 25 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
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
                let error = ExampleUpdateManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleUpdateManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = ExampleUpdateManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1911, column: 245 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc = std::vec::Vec::new();
                while let Some(value) = match {
                    use postgresql_crud::TryStreamExt;
                    rows.try_next()
                }
                .await
                {
                    Ok(value) => match value {
                        Some(value) => match sqlx::Row::try_get::<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, &std::primitive::str>(&value, &Example::primary_key()) {
                            Ok(value) => Some(value),
                            Err(error_0) => {
                                drop(rows);
                                match executor.rollback().await {
                                    Ok(_) => {
                                        let error = ExampleUpdateManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2386, column: 137 })) };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                    Err(error_1) => {
                                        let error = ExampleUpdateManyErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2386, column: 166 })) };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                }
                            }
                        },
                        None => None,
                    },
                    Err(error_0) => {
                        drop(rows);
                        match executor.rollback().await {
                            Ok(_) => {
                                let error = ExampleUpdateManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2388, column: 133 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            Err(error_1) => {
                                let error = ExampleUpdateManyErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2388, column: 162 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                } {
                    acc.push(value);
                }
                acc
            };
            if let Err(error_0) = executor.commit().await {
                let error = ExampleUpdateManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1922, column: 245 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateManyResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleTryUpdateManyErrorNamed {
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
    ExampleUpdateManyErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        update_many_error_named_with_serialize_deserialize: ExampleUpdateManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn try_update_many(endpoint_location: &std::primitive::str, parameters: ExampleUpdateManyParameters) -> Result<std::vec::Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>, ExampleTryUpdateManyErrorNamed> {
        let payload = {
            let value = ExampleUpdateManyPayload::from(parameters.payload);
            match serde_json::to_string(&value) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(ExampleTryUpdateManyErrorNamed::SerdeJsonToString { serde_json_to_string: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2218, column: 178 })) });
                }
            }
        };
        let url = format!("{}/example/update_many", endpoint_location,);
        let future = reqwest::Client::new().patch(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(ExampleTryUpdateManyErrorNamed::Reqwest { reqwest: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2267, column: 152 })) });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(ExampleTryUpdateManyErrorNamed::FailedToGetResponseText { status_code: error_0, headers: error_1, reqwest: error_2, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2284, column: 192 })) });
            }
        };
        let expected_response = match serde_json::from_str::<ExampleUpdateManyResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(ExampleTryUpdateManyErrorNamed::DeserializeResponse { status_code: error_0, headers: error_1, response_text: error_2, serde: error_3, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2296, column: 178 })) });
            }
        };
        let update_many_error_named_with_serialize_deserialize = match expected_response {
            ExampleUpdateManyResponseVariants::Desirable(value) => {
                let value = value;
                return Ok(value);
            }
            ExampleUpdateManyResponseVariants::CheckBodySize { check_body_size, code_occurence } => ExampleUpdateManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            ExampleUpdateManyResponseVariants::Postgresql { postgresql, code_occurence } => ExampleUpdateManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            ExampleUpdateManyResponseVariants::SerdeJson { serde_json, code_occurence } => ExampleUpdateManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            ExampleUpdateManyResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => ExampleUpdateManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleUpdateManyResponseVariants::CheckCommit { check_commit, code_occurence } => ExampleUpdateManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            ExampleUpdateManyResponseVariants::RowAndRollback { row, rollback, code_occurence } => ExampleUpdateManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            ExampleUpdateManyResponseVariants::QueryPart { error, code_occurence } => ExampleUpdateManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            ExampleUpdateManyResponseVariants::TryBind { try_bind, code_occurence } => ExampleUpdateManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(ExampleTryUpdateManyErrorNamed::ExampleUpdateManyErrorNamedWithSerializeDeserialize { update_many_error_named_with_serialize_deserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2334, column: 223 })) })
    }
}
impl Example {
    pub async fn update_many_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<ExampleUpdateManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
#[derive(Debug)]
pub struct ExampleUpdateOneParameters {
    pub payload: ExampleUpdate,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum ExampleUpdateOneResponseVariants {
    Desirable(ExampleReadOnlyIds),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: std::string::String, rollback: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl std::convert::From<ExampleUpdateOneErrorNamed> for ExampleUpdateOneResponseVariants {
    fn from(value: ExampleUpdateOneErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            ExampleUpdateOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            ExampleUpdateOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            ExampleUpdateOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            ExampleUpdateOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleUpdateOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            ExampleUpdateOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            ExampleUpdateOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            ExampleUpdateOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleUpdateOneErrorNamed {
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
        try_bind: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn update_one(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = ExampleUpdateOneErrorNamed::HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2117, column: 330 })) };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleUpdateOneErrorNamed::CheckBodySize { check_body_size: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2118, column: 268 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = ExampleUpdateOneParameters {
            payload: match serde_json::from_slice::<ExampleUpdate>(&body_bytes) {
                Ok(value) => {
                    let value = ExampleUpdate::from(value);
                    value
                }
                Err(error_0) => {
                    let error = ExampleUpdateOneErrorNamed::SerdeJson { serde_json: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2193, column: 249 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let update_for_query = ExampleUpdateForQuery::from(parameters.payload);
        let query_string = {
            let mut increment: std::primitive::u64 = 0;
            let columns = {
                let mut acc = std::string::String::default();
                if let Some(value) = &update_for_query.column_0 {
                    acc.push_str(&postgresql_crud::generate_column_queals_value_comma_update_one_query_part(
                        "column_0",
                        match ExampleUpdateForQuery::update_query_part_column_0(&value, &mut increment) {
                            Ok(value) => value,
                            Err(error_0) => {
                                let error = ExampleUpdateOneErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3406, column: 258 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        },
                    ));
                }
                if let Some(value) = &update_for_query.column_30 {
                    acc.push_str(&postgresql_crud::generate_column_queals_value_comma_update_one_query_part(
                        "column_30",
                        match ExampleUpdateForQuery::update_query_part_column_30(&value, &mut increment) {
                            Ok(value) => value,
                            Err(error_0) => {
                                let error = ExampleUpdateOneErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3406, column: 258 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
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
                    let error = ExampleUpdateOneErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1676, column: 241 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            };
            let return_columns = match update_for_query.select_only_updated_ids_query_part(&mut increment) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = ExampleUpdateOneErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3426, column: 254 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            };
            postgresql_crud::generate_update_one_query_string(&Example::table_name(), columns, &Example::primary_key(), primary_key_query_part, &return_columns)
        };
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            if let Some(ref value) = update_for_query.column_0 {
                match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::update_query_bind(value.value.clone(), query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = ExampleUpdateOneErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3460, column: 25 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            if let Some(ref value) = update_for_query.column_30 {
                match <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_bind(value.value.clone(), query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = ExampleUpdateOneErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3460, column: 25 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
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
                    let error = ExampleUpdateOneErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3460, column: 25 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
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
                        let error = ExampleUpdateOneErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3460, column: 25 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            if let Some(value) = &update_for_query.column_30 {
                match <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_bind(&value.value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = ExampleUpdateOneErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3460, column: 25 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
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
                let error = ExampleUpdateOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleUpdateOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = ExampleUpdateOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1911, column: 245 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                match binded_query.fetch_one(executor.as_mut()).await {
                    Ok(value) => match ExampleReadOnlyIds::try_from(value) {
                        Ok(value) => value,
                        Err(error_0) => match executor.rollback().await {
                            Ok(_) => {
                                let error = ExampleUpdateOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3534, column: 196 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            Err(error_1) => {
                                let error = ExampleUpdateOneErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3534, column: 225 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        },
                    },
                    Err(error_0) => match executor.rollback().await {
                        Ok(_) => {
                            let error = ExampleUpdateOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3542, column: 132 })) };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = ExampleUpdateOneErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3542, column: 161 })) };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                }
            };
            if let Err(error_0) = executor.commit().await {
                let error = ExampleUpdateOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1922, column: 245 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleUpdateOneResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleTryUpdateOneErrorNamed {
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
    ExampleUpdateOneErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        update_one_error_named_with_serialize_deserialize: ExampleUpdateOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn try_update_one(endpoint_location: &std::primitive::str, parameters: ExampleUpdateOneParameters) -> Result<ExampleReadOnlyIds, ExampleTryUpdateOneErrorNamed> {
        let payload = {
            let value = ExampleUpdate::from(parameters.payload);
            match serde_json::to_string(&value) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(ExampleTryUpdateOneErrorNamed::SerdeJsonToString { serde_json_to_string: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2218, column: 178 })) });
                }
            }
        };
        let url = format!("{}/example/update_one", endpoint_location,);
        let future = reqwest::Client::new().patch(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(ExampleTryUpdateOneErrorNamed::Reqwest { reqwest: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2267, column: 152 })) });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(ExampleTryUpdateOneErrorNamed::FailedToGetResponseText { status_code: error_0, headers: error_1, reqwest: error_2, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2284, column: 192 })) });
            }
        };
        let expected_response = match serde_json::from_str::<ExampleUpdateOneResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(ExampleTryUpdateOneErrorNamed::DeserializeResponse { status_code: error_0, headers: error_1, response_text: error_2, serde: error_3, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2296, column: 178 })) });
            }
        };
        let update_one_error_named_with_serialize_deserialize = match expected_response {
            ExampleUpdateOneResponseVariants::Desirable(value) => {
                let value = value;
                return Ok(value);
            }
            ExampleUpdateOneResponseVariants::CheckBodySize { check_body_size, code_occurence } => ExampleUpdateOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            ExampleUpdateOneResponseVariants::Postgresql { postgresql, code_occurence } => ExampleUpdateOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            ExampleUpdateOneResponseVariants::SerdeJson { serde_json, code_occurence } => ExampleUpdateOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            ExampleUpdateOneResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => ExampleUpdateOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleUpdateOneResponseVariants::CheckCommit { check_commit, code_occurence } => ExampleUpdateOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            ExampleUpdateOneResponseVariants::RowAndRollback { row, rollback, code_occurence } => ExampleUpdateOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            ExampleUpdateOneResponseVariants::QueryPart { error, code_occurence } => ExampleUpdateOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            ExampleUpdateOneResponseVariants::TryBind { try_bind, code_occurence } => ExampleUpdateOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(ExampleTryUpdateOneErrorNamed::ExampleUpdateOneErrorNamedWithSerializeDeserialize { update_one_error_named_with_serialize_deserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2334, column: 223 })) })
    }
}
impl Example {
    pub async fn update_one_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<ExampleUpdate as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ExampleDeleteManyPayload {
    pub where_many: StdOptionOptionExampleWhereMany,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ExampleDeleteManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { where_many: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }
    }
}
#[derive(Debug)]
pub struct ExampleDeleteManyParameters {
    pub payload: ExampleDeleteManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum ExampleDeleteManyResponseVariants {
    Desirable(std::vec::Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: std::string::String, rollback: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl std::convert::From<ExampleDeleteManyErrorNamed> for ExampleDeleteManyResponseVariants {
    fn from(value: ExampleDeleteManyErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            ExampleDeleteManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            ExampleDeleteManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            ExampleDeleteManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            ExampleDeleteManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleDeleteManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            ExampleDeleteManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            ExampleDeleteManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            ExampleDeleteManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleDeleteManyErrorNamed {
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
        try_bind: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn delete_many(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = ExampleDeleteManyErrorNamed::HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2117, column: 330 })) };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleDeleteManyErrorNamed::CheckBodySize { check_body_size: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2118, column: 268 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = ExampleDeleteManyParameters {
            payload: match serde_json::from_slice::<ExampleDeleteManyPayload>(&body_bytes) {
                Ok(value) => {
                    let value = ExampleDeleteManyPayload::from(value);
                    value
                }
                Err(error_0) => {
                    let error = ExampleDeleteManyErrorNamed::SerdeJson { serde_json: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2193, column: 249 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_delete_many_query_string(
            &Example::table_name(),
            {
                let mut increment: std::primitive::u64 = 0;
                let additional_parameters = match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.where_many, &mut increment, &"", false) {
                    Ok(value) => value,
                    Err(error_0) => {
                        let error = ExampleDeleteManyErrorNamed::QueryPart { error: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1020, column: 274 })) };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                };
                additional_parameters
            },
            &Example::primary_key(),
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.where_many, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = ExampleDeleteManyErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1051, column: 13 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleDeleteManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleDeleteManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = ExampleDeleteManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1911, column: 245 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc = std::vec::Vec::new();
                while let Some(value) = match {
                    use postgresql_crud::TryStreamExt;
                    rows.try_next()
                }
                .await
                {
                    Ok(value) => match value {
                        Some(value) => match sqlx::Row::try_get::<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, &std::primitive::str>(&value, &Example::primary_key()) {
                            Ok(value) => Some(value),
                            Err(error_0) => {
                                drop(rows);
                                match executor.rollback().await {
                                    Ok(_) => {
                                        let error = ExampleDeleteManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2386, column: 137 })) };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                    Err(error_1) => {
                                        let error = ExampleDeleteManyErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2386, column: 166 })) };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                }
                            }
                        },
                        None => None,
                    },
                    Err(error_0) => {
                        drop(rows);
                        match executor.rollback().await {
                            Ok(_) => {
                                let error = ExampleDeleteManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2388, column: 133 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            Err(error_1) => {
                                let error = ExampleDeleteManyErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2388, column: 162 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                } {
                    acc.push(value);
                }
                acc
            };
            if let Err(error_0) = executor.commit().await {
                let error = ExampleDeleteManyErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1922, column: 245 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteManyResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleTryDeleteManyErrorNamed {
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
    ExampleDeleteManyErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        delete_many_error_named_with_serialize_deserialize: ExampleDeleteManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn try_delete_many(endpoint_location: &std::primitive::str, parameters: ExampleDeleteManyParameters) -> Result<std::vec::Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>, ExampleTryDeleteManyErrorNamed> {
        let payload = {
            let value = ExampleDeleteManyPayload::from(parameters.payload);
            match serde_json::to_string(&value) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(ExampleTryDeleteManyErrorNamed::SerdeJsonToString { serde_json_to_string: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2218, column: 178 })) });
                }
            }
        };
        let url = format!("{}/example/delete_many", endpoint_location,);
        let future = reqwest::Client::new().delete(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(ExampleTryDeleteManyErrorNamed::Reqwest { reqwest: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2267, column: 152 })) });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(ExampleTryDeleteManyErrorNamed::FailedToGetResponseText { status_code: error_0, headers: error_1, reqwest: error_2, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2284, column: 192 })) });
            }
        };
        let expected_response = match serde_json::from_str::<ExampleDeleteManyResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(ExampleTryDeleteManyErrorNamed::DeserializeResponse { status_code: error_0, headers: error_1, response_text: error_2, serde: error_3, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2296, column: 178 })) });
            }
        };
        let delete_many_error_named_with_serialize_deserialize = match expected_response {
            ExampleDeleteManyResponseVariants::Desirable(value) => {
                let value = value;
                return Ok(value);
            }
            ExampleDeleteManyResponseVariants::CheckBodySize { check_body_size, code_occurence } => ExampleDeleteManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            ExampleDeleteManyResponseVariants::Postgresql { postgresql, code_occurence } => ExampleDeleteManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            ExampleDeleteManyResponseVariants::SerdeJson { serde_json, code_occurence } => ExampleDeleteManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            ExampleDeleteManyResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => ExampleDeleteManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleDeleteManyResponseVariants::CheckCommit { check_commit, code_occurence } => ExampleDeleteManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            ExampleDeleteManyResponseVariants::RowAndRollback { row, rollback, code_occurence } => ExampleDeleteManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            ExampleDeleteManyResponseVariants::QueryPart { error, code_occurence } => ExampleDeleteManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            ExampleDeleteManyResponseVariants::TryBind { try_bind, code_occurence } => ExampleDeleteManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(ExampleTryDeleteManyErrorNamed::ExampleDeleteManyErrorNamedWithSerializeDeserialize { delete_many_error_named_with_serialize_deserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2334, column: 223 })) })
    }
}
impl Example {
    pub async fn delete_many_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<ExampleDeleteManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ExampleDeleteOnePayload {
    pub primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ExampleDeleteOnePayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { primary_key_column: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }
    }
}
#[derive(Debug)]
pub struct ExampleDeleteOneParameters {
    pub payload: ExampleDeleteOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum ExampleDeleteOneResponseVariants {
    Desirable(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: std::string::String, rollback: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: std::string::String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl std::convert::From<ExampleDeleteOneErrorNamed> for ExampleDeleteOneResponseVariants {
    fn from(value: ExampleDeleteOneErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            ExampleDeleteOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            ExampleDeleteOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            ExampleDeleteOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            ExampleDeleteOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleDeleteOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            ExampleDeleteOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            ExampleDeleteOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleDeleteOneErrorNamed {
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
        try_bind: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn delete_one(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = ExampleDeleteOneErrorNamed::HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2117, column: 330 })) };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteOneResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleDeleteOneErrorNamed::CheckBodySize { check_body_size: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2118, column: 268 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = ExampleDeleteOneParameters {
            payload: match serde_json::from_slice::<ExampleDeleteOnePayload>(&body_bytes) {
                Ok(value) => {
                    let value = ExampleDeleteOnePayload::from(value);
                    value
                }
                Err(error_0) => {
                    let error = ExampleDeleteOneErrorNamed::SerdeJson { serde_json: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2193, column: 249 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_delete_one_query_string(&Example::table_name(), &Example::primary_key());
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.primary_key_column, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = ExampleDeleteOneErrorNamed::TryBind { try_bind: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 3694, column: 25 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleDeleteOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = ExampleDeleteOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2144, column: 257 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = ExampleDeleteOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1911, column: 245 })) };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                match binded_query.fetch_one(executor.as_mut()).await {
                    Ok(value) => match sqlx::Row::try_get::<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, &std::primitive::str>(&value, &Example::primary_key()) {
                        Ok(value) => value,
                        Err(error_0) => match executor.rollback().await {
                            Ok(_) => {
                                let error = ExampleDeleteOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2412, column: 261 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            Err(error_1) => {
                                let error = ExampleDeleteOneErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2412, column: 290 })) };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        },
                    },
                    Err(error_0) => match executor.rollback().await {
                        Ok(_) => {
                            let error = ExampleDeleteOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2413, column: 116 })) };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = ExampleDeleteOneErrorNamed::RowAndRollback { row: error_0, rollback: error_1, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2413, column: 145 })) };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                }
            };
            if let Err(error_0) = executor.commit().await {
                let error = ExampleDeleteOneErrorNamed::Postgresql { postgresql: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 1922, column: 245 })) };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(ExampleDeleteOneResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum ExampleTryDeleteOneErrorNamed {
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
    ExampleDeleteOneErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        delete_one_error_named_with_serialize_deserialize: ExampleDeleteOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl Example {
    pub async fn try_delete_one(endpoint_location: &std::primitive::str, parameters: ExampleDeleteOneParameters) -> Result<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, ExampleTryDeleteOneErrorNamed> {
        let payload = {
            let value = ExampleDeleteOnePayload::from(parameters.payload);
            match serde_json::to_string(&value) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(ExampleTryDeleteOneErrorNamed::SerdeJsonToString { serde_json_to_string: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2218, column: 178 })) });
                }
            }
        };
        let url = format!("{}/example/delete_one", endpoint_location,);
        let future = reqwest::Client::new().delete(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(ExampleTryDeleteOneErrorNamed::Reqwest { reqwest: error_0, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2267, column: 152 })) });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value) => value,
            Err(error_2) => {
                return Err(ExampleTryDeleteOneErrorNamed::FailedToGetResponseText { status_code: error_0, headers: error_1, reqwest: error_2, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2284, column: 192 })) });
            }
        };
        let expected_response = match serde_json::from_str::<ExampleDeleteOneResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(ExampleTryDeleteOneErrorNamed::DeserializeResponse { status_code: error_0, headers: error_1, response_text: error_2, serde: error_3, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2296, column: 178 })) });
            }
        };
        let delete_one_error_named_with_serialize_deserialize = match expected_response {
            ExampleDeleteOneResponseVariants::Desirable(value) => {
                let value = value;
                return Ok(value);
            }
            ExampleDeleteOneResponseVariants::CheckBodySize { check_body_size, code_occurence } => ExampleDeleteOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            ExampleDeleteOneResponseVariants::Postgresql { postgresql, code_occurence } => ExampleDeleteOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            ExampleDeleteOneResponseVariants::SerdeJson { serde_json, code_occurence } => ExampleDeleteOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            ExampleDeleteOneResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => ExampleDeleteOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            ExampleDeleteOneResponseVariants::CheckCommit { check_commit, code_occurence } => ExampleDeleteOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            ExampleDeleteOneResponseVariants::RowAndRollback { row, rollback, code_occurence } => ExampleDeleteOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            ExampleDeleteOneResponseVariants::TryBind { try_bind, code_occurence } => ExampleDeleteOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(ExampleTryDeleteOneErrorNamed::ExampleDeleteOneErrorNamedWithSerializeDeserialize { delete_one_error_named_with_serialize_deserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(file!().to_owned(), line!(), column!(), Some(error_occurence_lib::code_occurence::MacroOccurence { file: std::string::String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"), line: 2334, column: 223 })) })
    }
}
impl Example {
    pub async fn delete_one_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<ExampleDeleteOnePayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        return response;
    }
}
impl Example {
    pub fn routes(app_state: std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>) -> axum::Router {
        axum::Router::new().nest(&format!("/{}", Example::table_name()), axum::Router::new().route("/create_many", axum::routing::post(Example::create_many)).route("/create_many_payload_example", axum::routing::get(Example::create_many_payload_example)).route("/create_one", axum::routing::post(Example::create_one)).route("/create_one_payload_example", axum::routing::get(Example::create_one_payload_example)).route("/read_many", axum::routing::post(Example::read_many)).route("/read_many_payload_example", axum::routing::get(Example::read_many_payload_example)).route("/read_one", axum::routing::post(Example::read_one)).route("/read_one_payload_example", axum::routing::get(Example::read_one_payload_example)).route("/update_many", axum::routing::patch(Example::update_many)).route("/update_many_payload_example", axum::routing::get(Example::update_many_payload_example)).route("/update_one", axum::routing::patch(Example::update_one)).route("/update_one_payload_example", axum::routing::get(Example::update_one_payload_example)).route("/delete_many", axum::routing::delete(Example::delete_many)).route("/delete_many_payload_example", axum::routing::get(Example::delete_many_payload_example)).route("/delete_one", axum::routing::delete(Example::delete_one)).route("/delete_one_payload_example", axum::routing::get(Example::delete_one_payload_example)).with_state(app_state))
    }
}
#[cfg(test)]
mod example_tests {
    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<super::Example>(), 0);
    }
    #[test]
    fn test_crud() {
        std::thread::Builder::new()
            .stack_size(16 * 1024 * 1024)
            .spawn(|| {
                tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build().expect("error 38823c21-1879-449c-9b60-ce7293709959").block_on(async {
                    tracing_subscriber::fmt::init();
                    static CONFIG: std::sync::OnceLock<crate::repositories_types::server::config::Config> = std::sync::OnceLock::new();
                    let config = CONFIG.get_or_init(|| crate::repositories_types::server::config::Config::try_from_env().expect("error d7a6ef78-c306-40e7-b560-297ce4e8a8d1"));
                    let postgres_pool = sqlx::postgres::PgPoolOptions::new().max_connections(50).connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&config))).await.expect("error e3044bb9-7b76-4c0c-bc5f-eb34da05a103");
                    let url = format!("http://{}", app_state::GetServiceSocketAddress::get_service_socket_address(&config));
                    async fn drop_table_if_exists(postgres_pool: &sqlx::Pool<sqlx::Postgres>) {
                        let query = "drop table if exists example";
                        println!("{query}");
                        let _unused = sqlx::query(query).execute(postgres_pool).await.expect("error 1b11bf1b-9180-419f-bae7-b1ab93cd9c57");
                    }
                    drop_table_if_exists(&postgres_pool).await;
                    let postgres_pool_for_tokio_spawn_sync_move = postgres_pool.clone();
                    let _unused = tokio::spawn(async move {
                        super::Example::prepare_postgresql(&postgres_pool_for_tokio_spawn_sync_move).await.expect("error 0a7889da-c2b5-4205-adf1-75904ad80cc0");
                        let app_state = std::sync::Arc::new(crate::repositories_types::server::routes::app_state::AppState { postgres_pool: postgres_pool_for_tokio_spawn_sync_move.clone(), config: &config, project_git_info: &git_info::PROJECT_GIT_INFO });
                        axum::serve(tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(&config)).await.expect("error 663ae29e-bc00-4ea1-a7e9-4dddceb5b53a"), axum::Router::new().merge(super::Example::routes(std::sync::Arc::<crate::repositories_types::server::routes::app_state::AppState<'_>>::clone(&app_state))).into_make_service()).await.unwrap_or_else(|error| panic!("axum builder serve await failed {error:#?}"));
                    });
                    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                    let select_primary_key = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default())]).expect("error 0776170e-4dd6-4c14-a412-ce10b0c746f1");
                    let sort_vec_of_ident_read_with_primary_key_by_primary_key = |mut value: std::vec::Vec<super::ExampleRead>| -> std::vec::Vec<super::ExampleRead> {
                        value.sort_by_key(|element| element.primary_key_column.clone().expect("error 4f25860e-5b1a-408f-a4db-d49b6969ad4a").value);
                        value
                    };
                    let ident_create_default = super::ExampleCreate {
                        column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                        column_30: <<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                    };
                    let common_read_only_ids_returned_from_create_one = super::Example::try_create_one(&url, super::ExampleCreateOneParameters { payload: ident_create_default.clone() }).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
                    let some_value_read_only_ids_returned_from_create_one = Some(postgresql_crud::Value { value: common_read_only_ids_returned_from_create_one.primary_key_column.clone().into_read() });
                    assert_eq!(super::ExampleRead { primary_key_column: some_value_read_only_ids_returned_from_create_one.clone(), column_0: None, column_30: None }, super::Example::try_read_one(&url, super::ExampleReadOneParameters { payload: super::ExampleReadOnePayload { primary_key_column: common_read_only_ids_returned_from_create_one.primary_key_column.clone().into_read(), select: select_primary_key.clone() } },).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b"), "try_read_one result different after try_create_one 3d9f2ec0-e374-48d2-a36b-486f5598b0b4");
                    let select_default_all = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![super::ExampleSelect::PrimaryKeyColumn(<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()), super::ExampleSelect::Column0(<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()), super::ExampleSelect::Column30(<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element())]).expect("error 0776170e-4dd6-4c14-a412-ce10b0c746f1");
                    let start = std::time::Instant::now();
                    futures::StreamExt::for_each_concurrent(
                        futures::stream::iter({
                            let mut acc: std::vec::Vec<futures::future::BoxFuture<'static, ()>> = vec![];
                            for chunk in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::create_vec().chunks(10).map(|element| element.to_vec()).collect::<std::vec::Vec<std::vec::Vec<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create>>>() {
                                let url_cloned = url.clone();
                                let ident_create_default_cloned = ident_create_default.clone();
                                let select_default_all_cloned = select_default_all.clone();
                                acc.push(futures::FutureExt::boxed(async move {
                                    let ident_create_vec = {
                                        let mut acc = vec![];
                                        for element in chunk {
                                            acc.push(super::ExampleCreate { column_0: element, column_30: <<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element() });
                                        }
                                        acc
                                    };
                                    let read_only_ids_from_try_create_many = super::Example::try_create_many(&url_cloned, super::ExampleCreateManyParameters { payload: super::ExampleCreateManyPayload(ident_create_vec.clone()) }).await.expect("error 5eecedc4-bb02-454a-acd9-0af758f30b2e");
                                    assert_eq!(
                                        {
                                            let mut acc = vec![];
                                            assert_eq!(read_only_ids_from_try_create_many.len(), ident_create_vec.len(), "error 39572295-b6a4-49d7-a65a-16f8bcf44ede");
                                            for (read_only_ids, create) in read_only_ids_from_try_create_many.clone().into_iter().zip(ident_create_vec.into_iter()).collect::<std::vec::Vec<(super::ExampleReadOnlyIds, super::ExampleCreate)>>() {
                                                acc.push(super::ExampleRead {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids.primary_key_column),
                                                    column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_0, create.column_0),
                                                    column_30: <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_30, create.column_30),
                                                });
                                            }
                                            acc.sort_by(|a, b| {
                                                if let (Some(a), Some(b)) = (&a.primary_key_column, &b.primary_key_column) {
                                                    a.value.cmp(&b.value)
                                                } else {
                                                    panic!("must not be what error 4428083a-53be-4184-a5b7-94ae2de21d40");
                                                }
                                            });
                                            acc
                                        },
                                        super::Example::try_read_many(
                                            &url_cloned,
                                            super::ExampleReadManyParameters {
                                                payload: super::ExampleReadManyPayload {
                                                    where_many: super::StdOptionOptionExampleWhereMany(Some(
                                                        super::ExampleWhereMany::try_new(
                                                            Some(
                                                                postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                    let mut acc = vec![];
                                                                    for element in &read_only_ids_from_try_create_many {
                                                                        acc.push(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone().into_read())) }));
                                                                    }
                                                                    acc
                                                                })
                                                                .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
                                                            ),
                                                            None,
                                                            None
                                                        )
                                                        .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
                                                    )),
                                                    select: select_default_all_cloned.clone(),
                                                    order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) },
                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
                                                }
                                            }
                                        )
                                        .await
                                        .expect("error cb64ceaf-74a4-4501-b614-0c9d9e4e0598"),
                                        "try_read_many result different after try_create_many error d19bbbf6-f64c-4151-8b5b-998a93e13af5"
                                    );
                                    let read_only_ids_from_try_delete_many = {
                                        let mut acc = super::Example::try_delete_many(
                                            &url_cloned,
                                            super::ExampleDeleteManyParameters {
                                                payload: super::ExampleDeleteManyPayload {
                                                    where_many: super::StdOptionOptionExampleWhereMany(Some(super::ExampleWhereMany {
                                                        primary_key_column: Some(
                                                            postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                let mut acc = vec![];
                                                                for element in &read_only_ids_from_try_create_many {
                                                                    acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone().into_read())) }));
                                                                }
                                                                acc
                                                            })
                                                            .expect("error 5f1e5f9d-d189-4368-807e-a84348967610"),
                                                        ),
                                                        column_0: None,
                                                        column_30: None,
                                                    })),
                                                },
                                            },
                                        )
                                        .await
                                        .expect("error 716e470e-d738-4642-adfc-df1f9b945d27");
                                        acc.sort_by(|a, b| a.cmp(&b));
                                        acc
                                    };
                                    assert_eq!(
                                        read_only_ids_from_try_delete_many,
                                        {
                                            let mut acc = read_only_ids_from_try_create_many.into_iter().map(|element| element.primary_key_column.clone().into_read()).collect::<std::vec::Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>();
                                            acc.sort_by(|a, b| a.cmp(&b));
                                            acc
                                        },
                                        "error f58f5572-4286-4a74-8006-0507339910d4"
                                    );
                                    match super::Example::try_read_many(
                                        &url_cloned,
                                        super::ExampleReadManyParameters {
                                            payload: super::ExampleReadManyPayload {
                                                where_many: super::StdOptionOptionExampleWhereMany(Some(
                                                    super::ExampleWhereMany::try_new(
                                                        Some(
                                                            postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                let mut acc = vec![];
                                                                for element in &read_only_ids_from_try_delete_many {
                                                                    acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.clone())) }));
                                                                }
                                                                acc
                                                            })
                                                            .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
                                                        ),
                                                        None,
                                                        None,
                                                    )
                                                    .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
                                                )),
                                                select: select_default_all_cloned,
                                                order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) },
                                                pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
                                            },
                                        },
                                    )
                                    .await
                                    {
                                        Ok(value) => {
                                            if value != std::vec::Vec::new() {
                                                panic!("error 4e88679a-0d23-418f-8767-4e9b7531429c");
                                            }
                                        }
                                        Err(error) => {
                                            panic!("error 24ab86d6-15c9-47f1-a43f-c5fac4b38188 {error:#?}");
                                        }
                                    }
                                }));
                            }
                            for chunk in <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::create_vec().chunks(10).map(|element| element.to_vec()).collect::<std::vec::Vec<std::vec::Vec<<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Create>>>() {
                                let url_cloned = url.clone();
                                let ident_create_default_cloned = ident_create_default.clone();
                                let select_default_all_cloned = select_default_all.clone();
                                acc.push(futures::FutureExt::boxed(async move {
                                    let ident_create_vec = {
                                        let mut acc = vec![];
                                        for element in chunk {
                                            acc.push(super::ExampleCreate { column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(), column_30: element });
                                        }
                                        acc
                                    };
                                    let read_only_ids_from_try_create_many = super::Example::try_create_many(&url_cloned, super::ExampleCreateManyParameters { payload: super::ExampleCreateManyPayload(ident_create_vec.clone()) }).await.expect("error 5eecedc4-bb02-454a-acd9-0af758f30b2e");
                                    assert_eq!(
                                        {
                                            let mut acc = vec![];
                                            assert_eq!(read_only_ids_from_try_create_many.len(), ident_create_vec.len(), "error 39572295-b6a4-49d7-a65a-16f8bcf44ede");
                                            for (read_only_ids, create) in read_only_ids_from_try_create_many.clone().into_iter().zip(ident_create_vec.into_iter()).collect::<std::vec::Vec<(super::ExampleReadOnlyIds, super::ExampleCreate)>>() {
                                                acc.push(super::ExampleRead {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids.primary_key_column),
                                                    column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_0, create.column_0),
                                                    column_30: <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_30, create.column_30),
                                                });
                                            }
                                            acc.sort_by(|a, b| {
                                                if let (Some(a), Some(b)) = (&a.primary_key_column, &b.primary_key_column) {
                                                    a.value.cmp(&b.value)
                                                } else {
                                                    panic!("must not be what error 4428083a-53be-4184-a5b7-94ae2de21d40");
                                                }
                                            });
                                            acc
                                        },
                                        super::Example::try_read_many(
                                            &url_cloned,
                                            super::ExampleReadManyParameters {
                                                payload: super::ExampleReadManyPayload {
                                                    where_many: super::StdOptionOptionExampleWhereMany(Some(
                                                        super::ExampleWhereMany::try_new(
                                                            Some(
                                                                postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                    let mut acc = vec![];
                                                                    for element in &read_only_ids_from_try_create_many {
                                                                        acc.push(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone().into_read())) }));
                                                                    }
                                                                    acc
                                                                })
                                                                .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
                                                            ),
                                                            None,
                                                            None
                                                        )
                                                        .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
                                                    )),
                                                    select: select_default_all_cloned.clone(),
                                                    order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) },
                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
                                                }
                                            }
                                        )
                                        .await
                                        .expect("error cb64ceaf-74a4-4501-b614-0c9d9e4e0598"),
                                        "try_read_many result different after try_create_many error d19bbbf6-f64c-4151-8b5b-998a93e13af5"
                                    );
                                    let read_only_ids_from_try_delete_many = {
                                        let mut acc = super::Example::try_delete_many(
                                            &url_cloned,
                                            super::ExampleDeleteManyParameters {
                                                payload: super::ExampleDeleteManyPayload {
                                                    where_many: super::StdOptionOptionExampleWhereMany(Some(super::ExampleWhereMany {
                                                        primary_key_column: Some(
                                                            postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                let mut acc = vec![];
                                                                for element in &read_only_ids_from_try_create_many {
                                                                    acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone().into_read())) }));
                                                                }
                                                                acc
                                                            })
                                                            .expect("error 5f1e5f9d-d189-4368-807e-a84348967610"),
                                                        ),
                                                        column_0: None,
                                                        column_30: None,
                                                    })),
                                                },
                                            },
                                        )
                                        .await
                                        .expect("error 716e470e-d738-4642-adfc-df1f9b945d27");
                                        acc.sort_by(|a, b| a.cmp(&b));
                                        acc
                                    };
                                    assert_eq!(
                                        read_only_ids_from_try_delete_many,
                                        {
                                            let mut acc = read_only_ids_from_try_create_many.into_iter().map(|element| element.primary_key_column.clone().into_read()).collect::<std::vec::Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>();
                                            acc.sort_by(|a, b| a.cmp(&b));
                                            acc
                                        },
                                        "error f58f5572-4286-4a74-8006-0507339910d4"
                                    );
                                    match super::Example::try_read_many(
                                        &url_cloned,
                                        super::ExampleReadManyParameters {
                                            payload: super::ExampleReadManyPayload {
                                                where_many: super::StdOptionOptionExampleWhereMany(Some(
                                                    super::ExampleWhereMany::try_new(
                                                        Some(
                                                            postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                let mut acc = vec![];
                                                                for element in &read_only_ids_from_try_delete_many {
                                                                    acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.clone())) }));
                                                                }
                                                                acc
                                                            })
                                                            .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
                                                        ),
                                                        None,
                                                        None,
                                                    )
                                                    .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
                                                )),
                                                select: select_default_all_cloned,
                                                order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) },
                                                pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
                                            },
                                        },
                                    )
                                    .await
                                    {
                                        Ok(value) => {
                                            if value != std::vec::Vec::new() {
                                                panic!("error 4e88679a-0d23-418f-8767-4e9b7531429c");
                                            }
                                        }
                                        Err(error) => {
                                            panic!("error 24ab86d6-15c9-47f1-a43f-c5fac4b38188 {error:#?}");
                                        }
                                    }
                                }));
                            }
                            acc
                        }),
                        10,
                        |fut| async move {
                            fut.await;
                        },
                    )
                    .await;
                    let create_many_elapsed = start.elapsed();
                    let create_one_elapsed = start.elapsed();
                    println!("Elapsed: create_many_elapsed {:?}, create_one_elapsed {:?}", create_many_elapsed, create_one_elapsed);
                    futures::StreamExt::for_each_concurrent(
                        futures::stream::iter({
                            let mut acc: std::vec::Vec<futures::future::BoxFuture<'static, ()>> = vec![];
                            {
                                let read_only_ids_current_elements = {
                                    use futures::StreamExt;
                                    futures::stream::iter(
                                        {
                                            let mut acc = vec![];
                                            if let Some(value) = &common_read_only_ids_returned_from_create_one.column_0 {
                                                for element0 in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_inner_vec_vec(&value) {
                                                    for element1 in element0 {
                                                        acc.push(ident_create_default.clone());
                                                    }
                                                }
                                            }
                                            acc
                                        }
                                        .chunks(25)
                                        .map(|element| element.to_vec())
                                        .collect::<std::vec::Vec<std::vec::Vec<super::ExampleCreate>>>()
                                        .into_iter()
                                        .map(|element| {
                                            let url_cloned = url.clone();
                                            futures::FutureExt::boxed(async move { super::Example::try_create_many(&url_cloned, super::ExampleCreateManyParameters { payload: super::ExampleCreateManyPayload(element) }).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98") })
                                        })
                                        .collect::<std::vec::Vec<futures::future::BoxFuture<'static, std::vec::Vec<super::ExampleReadOnlyIds>>>>(),
                                    )
                                    .buffer_unordered(5)
                                    .collect::<std::vec::Vec<std::vec::Vec<super::ExampleReadOnlyIds>>>()
                                    .await
                                    .into_iter()
                                    .flatten()
                                    .collect::<std::vec::Vec<super::ExampleReadOnlyIds>>()
                                };
                                assert_eq!(
                                    {
                                        let mut acc = vec![];
                                        for element in &read_only_ids_current_elements {
                                            acc.push(super::ExampleRead {
                                                primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&element.primary_key_column),
                                                column_0: match &element.column_0 {
                                                    Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&value),
                                                    None => Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }),
                                                },
                                                column_30: match &element.column_30 {
                                                    Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&value),
                                                    None => Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }),
                                                },
                                            });
                                        }
                                        acc.sort_by(|a, b| {
                                            if let (Some(value_a), Some(value_b)) = (&a.primary_key_column, &b.primary_key_column) {
                                                value_a.value.cmp(&value_b.value)
                                            } else {
                                                panic!("must not be what");
                                            }
                                        });
                                        acc
                                    },
                                    {
                                        let mut acc = super::Example::try_read_many(
                                            &url,
                                            super::ExampleReadManyParameters {
                                                payload: super::ExampleReadManyPayload {
                                                    where_many: super::StdOptionOptionExampleWhereMany(Some(
                                                        super::ExampleWhereMany::try_new(
                                                            Some(
                                                                postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                    let mut acc = vec![];
                                                                    for element in &read_only_ids_current_elements {
                                                                        acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone().into_read())) }));
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
                                                    select: select_default_all.clone(),
                                                    order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) },
                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                },
                                            },
                                        )
                                        .await
                                        .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                        acc.sort_by(|a, b| if let (Some(value_a), Some(value_b)) = (&a.primary_key_column, &b.primary_key_column) { value_a.value.cmp(&value_b.value) } else { panic!("must not be what") });
                                        acc
                                    },
                                    "try_read_many result different after try_create_many db146190-0496-42a7-93d6-8405eb641954"
                                );
                                for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
                                    let url_cloned = url.clone();
                                    let ident_create_default_cloned = ident_create_default.clone();
                                    let select_default_all_cloned = select_default_all.clone();
                                    acc.push(futures::FutureExt::boxed(async move {
                                        let previous_read = super::Example::try_read_one(&url_cloned, super::ExampleReadOneParameters { payload: super::ExampleReadOnePayload { primary_key_column: read_only_ids_current_element.primary_key_column.clone().into_read(), select: select_default_all_cloned.clone() } }).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                        let update = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test({
                                            let mut local_increment = 0;
                                            let mut option_test_case = None;
                                            for element_0 in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_inner_vec_vec(&read_only_ids_current_element.column_0.clone().expect("error c4d98a71-f30f-410e-b410-a75f4672f2f7")) {
                                                let mut should_break = false;
                                                for element_1 in element_0 {
                                                    if local_increment == increment {
                                                        option_test_case = Some(element_1);
                                                        should_break = true;
                                                        break;
                                                    } else {
                                                        local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
                                                    }
                                                }
                                                if should_break {
                                                    break;
                                                }
                                            }
                                            option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
                                        });
                                        assert_eq!(super::ExampleReadOnlyIds { primary_key_column: read_only_ids_current_element.primary_key_column.clone(), column_0: Some(<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update)), column_30: None }, super::Example::try_update_one(&url_cloned, super::ExampleUpdateOneParameters { payload: super::ExampleUpdate::try_new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(read_only_ids_current_element.primary_key_column.clone().into_read()), Some(postgresql_crud::Value { value: update.clone() }), None).expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2") }).await.expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52"), "try_update_one result different");
                                        assert_eq!(
                                            super::ExampleRead {
                                                primary_key_column: Some(postgresql_crud::Value { value: read_only_ids_current_element.primary_key_column.clone().into_read() }),
                                                column_0: Some(postgresql_crud::Value { value: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_from_previous_read_unwraped_merged_with_update(<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_current_element.column_0.clone().expect("error 4f19d0d2-a23f-4b77-a2bc-c7b04db7a129")).expect("error c7685b19-9bca-47bc-a3a5-8fc543b174a5").value, Some(update.clone())) }),
                                                column_30: previous_read.column_30
                                            },
                                            super::Example::try_read_one(&url_cloned, super::ExampleReadOneParameters { payload: super::ExampleReadOnePayload { primary_key_column: read_only_ids_current_element.primary_key_column.clone().into_read(), select: select_default_all_cloned } }).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
                                            "try_read_one result different after try_create_one"
                                        );
                                    }));
                                }
                            }
                            {
                                let read_only_ids_current_elements = {
                                    use futures::StreamExt;
                                    futures::stream::iter(
                                        {
                                            let mut acc = vec![];
                                            if let Some(value) = &common_read_only_ids_returned_from_create_one.column_30 {
                                                for element0 in <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_inner_vec_vec(&value) {
                                                    for element1 in element0 {
                                                        acc.push(ident_create_default.clone());
                                                    }
                                                }
                                            }
                                            acc
                                        }
                                        .chunks(25)
                                        .map(|element| element.to_vec())
                                        .collect::<std::vec::Vec<std::vec::Vec<super::ExampleCreate>>>()
                                        .into_iter()
                                        .map(|element| {
                                            let url_cloned = url.clone();
                                            futures::FutureExt::boxed(async move { super::Example::try_create_many(&url_cloned, super::ExampleCreateManyParameters { payload: super::ExampleCreateManyPayload(element) }).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98") })
                                        })
                                        .collect::<std::vec::Vec<futures::future::BoxFuture<'static, std::vec::Vec<super::ExampleReadOnlyIds>>>>(),
                                    )
                                    .buffer_unordered(5)
                                    .collect::<std::vec::Vec<std::vec::Vec<super::ExampleReadOnlyIds>>>()
                                    .await
                                    .into_iter()
                                    .flatten()
                                    .collect::<std::vec::Vec<super::ExampleReadOnlyIds>>()
                                };
                                assert_eq!(
                                    {
                                        let mut acc = vec![];
                                        for element in &read_only_ids_current_elements {
                                            acc.push(super::ExampleRead {
                                                primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&element.primary_key_column),
                                                column_0: match &element.column_0 {
                                                    Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&value),
                                                    None => Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }),
                                                },
                                                column_30: match &element.column_30 {
                                                    Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&value),
                                                    None => Some(postgresql_crud::Value { value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element() }),
                                                },
                                            });
                                        }
                                        acc.sort_by(|a, b| {
                                            if let (Some(value_a), Some(value_b)) = (&a.primary_key_column, &b.primary_key_column) {
                                                value_a.value.cmp(&value_b.value)
                                            } else {
                                                panic!("must not be what");
                                            }
                                        });
                                        acc
                                    },
                                    {
                                        let mut acc = super::Example::try_read_many(
                                            &url,
                                            super::ExampleReadManyParameters {
                                                payload: super::ExampleReadManyPayload {
                                                    where_many: super::StdOptionOptionExampleWhereMany(Some(
                                                        super::ExampleWhereMany::try_new(
                                                            Some(
                                                                postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                    let mut acc = vec![];
                                                                    for element in &read_only_ids_current_elements {
                                                                        acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhereElement::Equal(postgresql_crud::PostgresqlTypeWhereElementEqual { logical_operator: postgresql_crud::LogicalOperator::Or, value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlOrigin::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.primary_key_column.clone().into_read())) }));
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
                                                    select: select_default_all.clone(),
                                                    order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) },
                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                },
                                            },
                                        )
                                        .await
                                        .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                        acc.sort_by(|a, b| if let (Some(value_a), Some(value_b)) = (&a.primary_key_column, &b.primary_key_column) { value_a.value.cmp(&value_b.value) } else { panic!("must not be what") });
                                        acc
                                    },
                                    "try_read_many result different after try_create_many db146190-0496-42a7-93d6-8405eb641954"
                                );
                                for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
                                    let url_cloned = url.clone();
                                    let ident_create_default_cloned = ident_create_default.clone();
                                    let select_default_all_cloned = select_default_all.clone();
                                    acc.push(futures::FutureExt::boxed(async move {
                                        let previous_read = super::Example::try_read_one(&url_cloned, super::ExampleReadOneParameters { payload: super::ExampleReadOnePayload { primary_key_column: read_only_ids_current_element.primary_key_column.clone().into_read(), select: select_default_all_cloned.clone() } }).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                        let update = <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test({
                                            let mut local_increment = 0;
                                            let mut option_test_case = None;
                                            for element_0 in <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_inner_vec_vec(&read_only_ids_current_element.column_30.clone().expect("error c4d98a71-f30f-410e-b410-a75f4672f2f7")) {
                                                let mut should_break = false;
                                                for element_1 in element_0 {
                                                    if local_increment == increment {
                                                        option_test_case = Some(element_1);
                                                        should_break = true;
                                                        break;
                                                    } else {
                                                        local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
                                                    }
                                                }
                                                if should_break {
                                                    break;
                                                }
                                            }
                                            option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
                                        });
                                        assert_eq!(super::ExampleReadOnlyIds { primary_key_column: read_only_ids_current_element.primary_key_column.clone(), column_0: None, column_30: Some(<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update)) }, super::Example::try_update_one(&url_cloned, super::ExampleUpdateOneParameters { payload: super::ExampleUpdate::try_new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(read_only_ids_current_element.primary_key_column.clone().into_read()), None, Some(postgresql_crud::Value { value: update.clone() })).expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2") }).await.expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52"), "try_update_one result different");
                                        assert_eq!(
                                            super::ExampleRead {
                                                primary_key_column: Some(postgresql_crud::Value { value: read_only_ids_current_element.primary_key_column.clone().into_read() }),
                                                column_0: previous_read.column_0,
                                                column_30: Some(postgresql_crud::Value {
                                                    value: <postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_from_previous_read_unwraped_merged_with_update(<postgresql_crud::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_current_element.column_30.clone().expect("error 4f19d0d2-a23f-4b77-a2bc-c7b04db7a129")).expect("error c7685b19-9bca-47bc-a3a5-8fc543b174a5").value, Some(update.clone()))
                                                })
                                            },
                                            super::Example::try_read_one(&url_cloned, super::ExampleReadOneParameters { payload: super::ExampleReadOnePayload { primary_key_column: read_only_ids_current_element.primary_key_column.clone().into_read(), select: select_default_all_cloned } }).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
                                            "try_read_one result different after try_create_one"
                                        );
                                    }));
                                }
                            }
                            acc
                        }),
                        100,
                        |fut| async move {
                            fut.await;
                        },
                    )
                    .await;
                    let try_read_many_data = super::Example::try_read_many(&url, super::ExampleReadManyParameters { payload: super::ExampleReadManyPayload { where_many: super::StdOptionOptionExampleWhereMany(None), select: select_default_all.clone(), order_by: postgresql_crud::OrderBy { column: super::ExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()), order: Some(postgresql_crud::Order::Asc) }, pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a") } }).await.expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                    println!("try_read_many result len {}", try_read_many_data.len());
                });
            })
            .expect("error 4d329978-f5af-424e-8757-e8a32dbeb5a1")
            .join()
            .unwrap_or_else(|error| {
                panic!("error b2f21a5f-d9ce-435c-809f-bd40741c8795 {error:#?}");
            });
    }
}
